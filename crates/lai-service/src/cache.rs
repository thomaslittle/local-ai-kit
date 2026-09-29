use crate::InventorySource;
use lai_contracts::{
    ConnectionMetadata, DataMode, DomainError, ErrorCode, Observation, Result, Snapshot,
};
use lai_discovery::{LocatedClient, ScanResult};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::watch;

type RefreshResult = Option<Result<()>>;
const METADATA_PROBE_CACHE_TTL: Duration = Duration::from_secs(60);
const METADATA_FRESHNESS_TTL: Duration = Duration::from_secs(15 * 60);
/// Persisted observations older than this are dropped instead of shown as stale.
const PERSISTED_MAX_AGE: Duration = Duration::from_secs(7 * 24 * 60 * 60);
const PERSISTED_VERSION: u32 = 1;
/// The on-disk form of the last metadata observations (stale-while-revalidate across restarts).
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PersistedMetadata {
    version: u32,
    entries: Vec<PersistedEntry>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PersistedEntry {
    saved_at_ms: u64,
    report: ConnectionMetadata,
}
fn unix_ms(at: SystemTime) -> u64 {
    at.duration_since(UNIX_EPOCH)
        .map(|d| u64::try_from(d.as_millis()).unwrap_or(u64::MAX))
        .unwrap_or(0)
}
/// Public snapshots already mask account identity, so the disk copy never stores it either.
fn without_identity(mut report: ConnectionMetadata) -> ConnectionMetadata {
    if let Some(account) = report.account.as_mut() {
        account.display_label = Observation::unknown("NOT_PERSISTED");
        account.identity_source = Observation::unknown("NOT_PERSISTED");
    }
    report
}
fn write_private(path: &Path, bytes: &[u8]) {
    if let Some(directory) = path.parent() {
        let _ = std::fs::create_dir_all(directory);
    }
    let temporary = path.with_extension("json.tmp");
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let written = options.open(&temporary).and_then(|mut file| {
        use std::io::Write;
        file.write_all(bytes)
    });
    if written.is_ok() {
        let _ = std::fs::rename(&temporary, path);
    } else {
        let _ = std::fs::remove_file(&temporary);
    }
}
fn read_persisted(path: &Path) -> BTreeMap<String, (Instant, ConnectionMetadata)> {
    let Ok(bytes) = std::fs::read(path) else {
        return BTreeMap::new();
    };
    let Ok(file) = serde_json::from_slice::<PersistedMetadata>(&bytes) else {
        return BTreeMap::new();
    };
    if file.version != PERSISTED_VERSION {
        return BTreeMap::new();
    }
    let now = unix_ms(SystemTime::now());
    file.entries
        .into_iter()
        .filter_map(|entry| {
            let age = Duration::from_millis(now.saturating_sub(entry.saved_at_ms));
            (age < PERSISTED_MAX_AGE).then(|| {
                let observed = Instant::now().checked_sub(age).unwrap_or_else(Instant::now);
                (entry.report.connection_id.clone(), (observed, entry.report))
            })
        })
        .collect()
}
fn metadata_is_stale(age: Duration) -> bool {
    age >= METADATA_FRESHNESS_TTL
}
#[derive(Clone)]
struct MetadataFailure {
    until: Instant,
    attempts: u32,
    error: DomainError,
}
struct CacheState {
    snapshot: Snapshot,
    metadata: BTreeMap<String, (Instant, lai_contracts::ConnectionMetadata)>,
    metadata_failures: BTreeMap<String, MetadataFailure>,
    /// Observations loaded from disk, adopted on the first scan for connections that still exist.
    restored: BTreeMap<String, (Instant, ConnectionMetadata)>,
    persist_path: Option<PathBuf>,
    // Native-only: never serialize filesystem locations into public responses.
    _locations: BTreeMap<String, LocatedClient>,
    updated: Option<Instant>,
    failure: Option<(Instant, DomainError)>,
    flight: Option<watch::Receiver<RefreshResult>>,
}
pub(crate) struct InventoryCache {
    state: Mutex<CacheState>,
    source: InventorySource,
    ttl: Duration,
}
fn metadata_backoff(connection: &str, attempts: u32, error: &DomainError) -> Duration {
    if let Some(milliseconds) = error.retry_after_ms {
        return Duration::from_millis(milliseconds.min(86_400_000));
    }
    if !error.retryable {
        return Duration::from_secs(5);
    }
    let shift = attempts.saturating_sub(1).min(7);
    let base_ms = 250u64.saturating_mul(1u64 << shift).min(30_000);
    let mut hash = 0xcbf29ce484222325u64;
    for byte in connection.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash ^= u64::from(attempts).wrapping_mul(0x9e3779b97f4a7c15);
    let jitter_window = (base_ms / 4).max(1);
    Duration::from_millis(base_ms.saturating_add(hash % jitter_window).min(30_000))
}
impl InventoryCache {
    pub fn new(source: InventorySource, ttl: Duration) -> Self {
        Self {
            source,
            ttl,
            state: Mutex::new(CacheState {
                snapshot: Snapshot::empty(DataMode::Live),
                metadata: BTreeMap::new(),
                metadata_failures: BTreeMap::new(),
                restored: BTreeMap::new(),
                persist_path: None,
                _locations: BTreeMap::new(),
                updated: None,
                failure: None,
                flight: None,
            }),
        }
    }
    /// Keep the last metadata observations on disk and reload them after a restart. They come back
    /// with their original age, so anything past the freshness window is served as stale and the
    /// caller refreshes it. Account identity is never written.
    pub fn enable_persistence(&self, path: PathBuf) {
        let restored = read_persisted(&path);
        if let Ok(mut state) = self.state.lock() {
            state.restored = restored;
            state.persist_path = Some(path);
        }
    }
    fn persist(&self) {
        let (path, entries) = {
            let Ok(state) = self.state.lock() else {
                return;
            };
            let Some(path) = state.persist_path.clone() else {
                return;
            };
            let entries = state
                .metadata
                .values()
                .map(|(at, report)| PersistedEntry {
                    saved_at_ms: unix_ms(
                        SystemTime::now()
                            .checked_sub(at.elapsed())
                            .unwrap_or_else(SystemTime::now),
                    ),
                    report: without_identity(report.clone()),
                })
                .collect::<Vec<_>>();
            (path, entries)
        };
        let file = PersistedMetadata {
            version: PERSISTED_VERSION,
            entries,
        };
        if let Ok(bytes) = serde_json::to_vec(&file) {
            write_private(&path, &bytes);
        }
    }
    pub fn snapshot(&self) -> Result<Snapshot> {
        let state = self.state.lock().map_err(|_| unavailable())?;
        let mut snapshot = state.snapshot.clone();
        for (at, report) in state.metadata.values() {
            crate::metadata::merge_report(&mut snapshot, report, metadata_is_stale(at.elapsed()));
        }
        Ok(snapshot)
    }
    pub(crate) fn probe_input(&self, connection: &str) -> Result<(LocatedClient, String, String)> {
        let state = self.state.lock().map_err(|_| unavailable())?;
        let client = state._locations.get(connection).cloned().ok_or_else(|| {
            lai_contracts::DomainError::new(ErrorCode::NotFound, "Connection is unavailable")
        })?;
        let route = state
            .snapshot
            .routes
            .iter()
            .find(|r| r.connection_id == connection)
            .ok_or_else(unavailable)?;
        Ok((client, route.id.clone(), state.snapshot.revision.clone()))
    }
    pub(crate) fn has_fresh_metadata(&self, connection: &str) -> Result<bool> {
        let state = self.state.lock().map_err(|_| unavailable())?;
        Ok(state
            .metadata
            .get(connection)
            .is_some_and(|(at, _)| at.elapsed() < METADATA_PROBE_CACHE_TTL))
    }
    pub(crate) fn publish_metadata(
        &self,
        base: &str,
        report: lai_contracts::ConnectionMetadata,
    ) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        if state.snapshot.revision != base {
            return Err(DomainError::new(
                ErrorCode::SelectionStale,
                "Inventory changed during the metadata probe",
            ));
        }
        state.metadata_failures.remove(&report.connection_id);
        state
            .metadata
            .insert(report.connection_id.clone(), (Instant::now(), report));
        drop(state);
        self.persist();
        Ok(())
    }
    pub(crate) fn metadata_failure(&self, connection: &str) -> Result<Option<DomainError>> {
        let state = self.state.lock().map_err(|_| unavailable())?;
        let now = Instant::now();
        Ok(state.metadata_failures.get(connection).and_then(|failure| {
            (failure.until > now).then(|| {
                let mut error = failure.error.clone();
                if error.retryable {
                    let remaining = failure.until.saturating_duration_since(now);
                    error.retry_after_ms = Some(
                        u64::try_from(remaining.as_millis())
                            .unwrap_or(u64::MAX)
                            .max(1),
                    );
                }
                error
            })
        }))
    }
    pub(crate) fn fail_metadata(&self, connection: &str, error: DomainError) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        state.metadata.remove(connection);
        let attempts = state
            .metadata_failures
            .get(connection)
            .map(|failure| failure.attempts.saturating_add(1))
            .unwrap_or(1)
            .min(32);
        let delay = metadata_backoff(connection, attempts, &error);
        state.metadata_failures.insert(
            connection.into(),
            MetadataFailure {
                until: Instant::now() + delay,
                attempts,
                error,
            },
        );
        drop(state);
        self.persist();
        Ok(())
    }
    pub(crate) fn watch_roots(&self) -> Result<BTreeMap<String, PathBuf>> {
        let state = self.state.lock().map_err(|_| unavailable())?;
        Ok(state
            ._locations
            .iter()
            .map(|(id, client)| (id.clone(), client.profile_home.clone()))
            .collect())
    }
    pub(crate) fn invalidate_metadata(&self, connections: &[String]) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        for id in connections {
            state.metadata.remove(id);
            state.metadata_failures.remove(id);
        }
        state.updated = None;
        drop(state);
        self.persist();
        Ok(())
    }
    pub async fn refresh(self: &Arc<Self>) -> Result<()> {
        let mut receiver = {
            let mut state = self.state.lock().map_err(|_| unavailable())?;
            if state.updated.is_some_and(|at| at.elapsed() < self.ttl) {
                return Ok(());
            }
            if let Some(receiver) = &state.flight {
                receiver.clone()
            } else {
                if let Some((at, error)) = &state.failure {
                    if at.elapsed() < Duration::from_secs(1) {
                        return Err(error.clone());
                    }
                }
                let (sender, receiver) = watch::channel(None);
                state.flight = Some(receiver.clone());
                let cache = self.clone();
                tokio::spawn(async move {
                    cache.run_refresh(sender).await;
                });
                receiver
            }
        };
        loop {
            let result = receiver.borrow().clone();
            if let Some(result) = result {
                return result;
            }
            receiver.changed().await.map_err(|_| unavailable())?;
        }
    }
    async fn run_refresh(self: Arc<Self>, sender: watch::Sender<RefreshResult>) {
        let cancelled = Arc::new(AtomicBool::new(false));
        let flag = cancelled.clone();
        let source = self.source.clone();
        let mut worker = tokio::task::spawn_blocking(move || source(&flag));
        let outcome = match tokio::time::timeout(Duration::from_secs(5), &mut worker).await {
            Ok(Ok(result)) => result,
            Ok(Err(_)) => Err(unavailable()),
            Err(_) => {
                cancelled.store(true, Ordering::SeqCst);
                let error =
                    DomainError::new(ErrorCode::Timeout, "Native inventory deadline exceeded");
                sender.send_replace(Some(Err(error.clone())));
                // Keep the in-flight slot occupied until the blocking OS operation exits.
                // Timing out must not spawn an unbounded succession of blocked workers.
                let _ = worker.await;
                Err(error)
            }
        };
        let result = self.publish(outcome);
        sender.send_replace(Some(result));
    }
    fn publish(&self, outcome: Result<ScanResult>) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        state.flight = None;
        match outcome {
            Ok(result) => {
                state
                    .metadata
                    .retain(|id, _| result.snapshot.connections.iter().any(|c| c.id == *id));
                state
                    .metadata_failures
                    .retain(|id, _| result.snapshot.connections.iter().any(|c| c.id == *id));
                state.snapshot = result.snapshot;
                state._locations = result.locations;
                // Adopt the observations kept on disk for connections that are still here and have no
                // newer live observation. They keep their original age (stale after the window).
                let restored = std::mem::take(&mut state.restored);
                for (id, entry) in restored {
                    if state.snapshot.connections.iter().any(|c| c.id == id)
                        && !state.metadata.contains_key(&id)
                    {
                        state.metadata.insert(id, entry);
                    }
                }
                state.updated = Some(Instant::now());
                state.failure = None;
                Ok(())
            }
            Err(error) => {
                // Never echo an arbitrary adapter/OS error, which may include private paths.
                let safe = DomainError::new(error.code, "Native inventory is unavailable");
                state.failure = Some((Instant::now(), safe.clone()));
                Err(safe)
            }
        }
    }
}
fn unavailable() -> DomainError {
    DomainError::new(
        ErrorCode::ProviderUnavailable,
        "Native inventory is unavailable",
    )
}
#[cfg(test)]
mod backoff_tests {
    use super::metadata_backoff;
    use lai_contracts::{DomainError, ErrorCode};
    use std::time::Duration;

    #[test]
    fn retry_jitter_never_exceeds_the_thirty_second_cap() {
        let error = DomainError::new(ErrorCode::ProviderUnavailable, "safe").retryable();
        assert!(metadata_backoff("cap", 32, &error) <= Duration::from_secs(30));
    }
}

#[cfg(test)]
mod metadata_freshness_tests {
    use super::metadata_is_stale;
    use std::time::Duration;

    #[test]
    fn a_recent_authentication_check_stays_fresh_while_the_user_checks_other_profiles() {
        assert!(!metadata_is_stale(Duration::from_secs(61)));
        assert!(!metadata_is_stale(Duration::from_secs(15 * 60 - 1)));
        assert!(metadata_is_stale(Duration::from_secs(15 * 60)));
    }
}
