use crate::InventorySource;
use crate::health::{derive_issues, merge_issue, storage_issue, summarize};
use lai_contracts::{
    ConnectionMetadata, DataMode, DomainError, ErrorCode, InventoryChange, InventoryEventPage,
    Issue, IssueState, NotificationPreferences, ResourceChange, ResourceChangeKind, Result,
    Snapshot, Timestamp,
};
use lai_discovery::{LocatedClient, ScanResult};
use lai_store::{DurableMetadata, StateStore, StoreError};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    path::PathBuf,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::{Notify, watch};

type RefreshResult = Option<Result<()>>;
const METADATA_PROBE_CACHE_TTL: Duration = Duration::from_secs(60);
const METADATA_FRESHNESS_TTL: Duration = Duration::from_secs(15 * 60);
const JOURNAL_LIMIT: usize = 2048;
const JOURNAL_PAGE: usize = 256;

#[derive(Clone)]
struct MetadataEntry {
    observed: Instant,
    report: ConnectionMetadata,
    /// A local filesystem event says this must be re-read. The old value stays
    /// visible as stale until a probe succeeds.
    invalidated: bool,
}

#[derive(Clone)]
struct MetadataFailure {
    until: Instant,
    attempts: u32,
    error: DomainError,
}

struct CacheState {
    snapshot: Snapshot,
    metadata: BTreeMap<String, MetadataEntry>,
    metadata_failures: BTreeMap<String, MetadataFailure>,
    issues: BTreeMap<String, Issue>,
    revision: u64,
    epoch: String,
    cursor: String,
    journal: VecDeque<InventoryChange>,
    store: Option<Arc<StateStore>>,
    storage_error: Option<DomainError>,
    notification_preferences: NotificationPreferences,
    notification_history: BTreeMap<String, u64>,
    // Native-only: never serialize filesystem locations into public responses.
    _locations: BTreeMap<String, LocatedClient>,
    updated: Option<Instant>,
    failure: Option<(Instant, DomainError)>,
    flight: Option<watch::Receiver<RefreshResult>>,
    dirty: BTreeSet<String>,
    hydrated: bool,
}

pub(crate) struct ProbeTarget {
    pub connection_id: String,
    pub located: LocatedClient,
    pub route_id: String,
    pub base: String,
}

pub(crate) struct InventoryCache {
    state: Mutex<CacheState>,
    source: InventorySource,
    ttl: Duration,
    wake: Notify,
}

fn unix_ms(at: SystemTime) -> u64 {
    at.duration_since(UNIX_EPOCH)
        .map(|duration| u64::try_from(duration.as_millis()).unwrap_or(u64::MAX))
        .unwrap_or(0)
}

fn instance_epoch() -> String {
    let mut bytes = [0u8; 8];
    let _ = getrandom::fill(&mut bytes);
    format!("local-{:x}", u64::from_be_bytes(bytes))
}

fn metadata_is_stale(age: Duration) -> bool {
    age >= METADATA_FRESHNESS_TTL
}

fn unavailable() -> DomainError {
    DomainError::new(
        ErrorCode::ProviderUnavailable,
        "Native inventory is unavailable",
    )
}

fn store_error(error: StoreError) -> DomainError {
    error.domain()
}

fn storage_failure(error: DomainError) -> DomainError {
    let mut safe = DomainError::new(
        ErrorCode::StorageUnavailable,
        "Local state could not be committed; the previous snapshot remains visible",
    );
    safe.retryable = true;
    safe.retry_after_ms = error.retry_after_ms;
    safe
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

fn mark_observation_stale<T>(observation: &mut lai_contracts::Observation<T>) {
    observation.mark_stale();
}

fn mark_snapshot_stale(snapshot: &mut Snapshot) {
    for connection in &mut snapshot.connections {
        mark_observation_stale(&mut connection.auth);
        mark_observation_stale(&mut connection.plan);
        if connection.status == "metadata_ready" {
            connection.status = "metadata_stale".into();
        }
    }
    for installation in &mut snapshot.installations {
        mark_observation_stale(&mut installation.version);
    }
    for route in &mut snapshot.routes {
        mark_observation_stale(&mut route.billing);
        for capability in &mut route.capabilities {
            mark_observation_stale(&mut capability.evidence);
        }
    }
    for model in &mut snapshot.models {
        for capability in &mut model.capabilities {
            mark_observation_stale(&mut capability.evidence);
        }
        mark_observation_stale(&mut model.context_window);
    }
    for pool in &mut snapshot.quota_pools {
        for window in &mut pool.windows {
            mark_observation_stale(&mut window.reading);
        }
    }
    snapshot.health.hydrated = true;
}

fn json_eq<T: serde::Serialize>(left: &T, right: &T) -> bool {
    serde_json::to_string(left).ok() == serde_json::to_string(right).ok()
}

fn push_changes<T: serde::Serialize>(
    changes: &mut Vec<ResourceChange>,
    resource: &str,
    before: &[T],
    after: &[T],
    id: impl Fn(&T) -> String,
) {
    let before: BTreeMap<String, &T> = before.iter().map(|item| (id(item), item)).collect();
    let after: BTreeMap<String, &T> = after.iter().map(|item| (id(item), item)).collect();
    for (id, item) in &after {
        match before.get(id) {
            None => changes.push(ResourceChange {
                resource: resource.into(),
                id: id.clone(),
                change: ResourceChangeKind::Added,
                reason: None,
            }),
            Some(previous) if !json_eq(previous, item) => changes.push(ResourceChange {
                resource: resource.into(),
                id: id.clone(),
                change: ResourceChangeKind::Updated,
                reason: None,
            }),
            Some(_) => {}
        }
    }
    for id in before.keys() {
        if !after.contains_key(id) {
            changes.push(ResourceChange {
                resource: resource.into(),
                id: id.clone(),
                change: ResourceChangeKind::Removed,
                reason: None,
            });
        }
    }
}

fn diff_snapshot(before: &Snapshot, after: &Snapshot) -> Vec<ResourceChange> {
    let mut changes = Vec::new();
    push_changes(
        &mut changes,
        "provider",
        &before.providers,
        &after.providers,
        |item| item.id.clone(),
    );
    push_changes(
        &mut changes,
        "installation",
        &before.installations,
        &after.installations,
        |item| item.id.clone(),
    );
    push_changes(
        &mut changes,
        "profile",
        &before.profiles,
        &after.profiles,
        |item| item.id.clone(),
    );
    push_changes(
        &mut changes,
        "account",
        &before.accounts,
        &after.accounts,
        |item| item.id.clone(),
    );
    push_changes(
        &mut changes,
        "connection",
        &before.connections,
        &after.connections,
        |item| item.id.clone(),
    );
    push_changes(
        &mut changes,
        "route",
        &before.routes,
        &after.routes,
        |item| item.id.clone(),
    );
    push_changes(
        &mut changes,
        "model",
        &before.models,
        &after.models,
        |item| item.id.clone(),
    );
    push_changes(
        &mut changes,
        "quota_pool",
        &before.quota_pools,
        &after.quota_pools,
        |item| item.id.clone(),
    );
    push_changes(
        &mut changes,
        "issue",
        &before.issues,
        &after.issues,
        |item| item.id.clone(),
    );
    if changes.len() > 64 {
        changes.truncate(63);
        changes.push(ResourceChange {
            resource: "inventory".into(),
            id: "*".into(),
            change: ResourceChangeKind::Updated,
            reason: Some("additional changes omitted from the journal record".into()),
        });
    }
    changes
}

fn cursor_epoch(cursor: &str) -> Option<(&str, u64)> {
    let (epoch, sequence) = cursor.split_once(':')?;
    Some((epoch, sequence.parse().ok()?))
}

impl InventoryCache {
    pub fn new(source: InventorySource, ttl: Duration) -> Self {
        Self {
            source,
            ttl,
            wake: Notify::new(),
            state: Mutex::new(CacheState {
                snapshot: Snapshot::empty(DataMode::Live),
                metadata: BTreeMap::new(),
                metadata_failures: BTreeMap::new(),
                issues: BTreeMap::new(),
                revision: 0,
                epoch: instance_epoch(),
                cursor: String::new(),
                journal: VecDeque::new(),
                store: None,
                storage_error: None,
                notification_preferences: NotificationPreferences::default(),
                notification_history: BTreeMap::new(),
                _locations: BTreeMap::new(),
                updated: None,
                failure: None,
                flight: None,
                dirty: BTreeSet::new(),
                hydrated: false,
            }),
        }
    }

    /// Open the broker-owned SQLite store and hydrate the ready-to-serve
    /// snapshot before the first request. Hydrated fields are labeled stale and
    /// the background lane reverifies them.
    pub fn enable_store(&self, path: PathBuf) -> Result<()> {
        let (store, notice) = StateStore::open(path).map_err(store_error)?;
        let loaded = store.load().map_err(store_error)?;
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        let had_snapshot = loaded.snapshot.is_some();
        if let Some(mut snapshot) = loaded.snapshot {
            mark_snapshot_stale(&mut snapshot);
            snapshot.issues = loaded.issues.clone();
            snapshot.health = summarize(&snapshot.issues, true);
            snapshot.revision = loaded.revision.to_string();
            snapshot.cursor = loaded.cursor.clone();
            state.snapshot = snapshot;
            state.hydrated = true;
        }
        let now = unix_ms(SystemTime::now());
        for entry in loaded.metadata {
            let age = Duration::from_millis(now.saturating_sub(entry.observed_at_ms));
            let observed = Instant::now().checked_sub(age).unwrap_or_else(Instant::now);
            // Hydrated auth is never treated as current without a fresh check.
            let report = entry.report;
            state.metadata.insert(
                report.connection_id.clone(),
                MetadataEntry {
                    observed,
                    report,
                    invalidated: true,
                },
            );
        }
        for issue in loaded.issues {
            state.issues.insert(issue.id.clone(), issue);
        }
        if !had_snapshot && !state.issues.is_empty() {
            state.snapshot.issues = state.issues.values().cloned().collect();
            state.snapshot.health = summarize(&state.snapshot.issues, true);
        }
        state.notification_preferences = loaded.notifications;
        state.revision = loaded.revision;
        state.cursor = loaded.cursor;
        state.store = Some(Arc::new(store));
        if notice.is_some() {
            state.storage_error = Some(DomainError::new(
                ErrorCode::StorageUnavailable,
                "Damaged local state was preserved; the runtime rebuilt its working state",
            ));
        }
        state.updated = None;
        let had_issues = !state.issues.is_empty();
        drop(state);
        if had_snapshot || had_issues {
            // The durable view is already ready to serve. Do not invent a new
            // revision merely because the process restarted.
            return Ok(());
        }
        self.compose_and_commit("hydrate", None, false)?;
        Ok(())
    }

    pub(crate) fn status(&self) -> (u64, String, bool, Option<DomainError>) {
        self.state
            .lock()
            .map(|state| {
                (
                    state.revision,
                    state.cursor.clone(),
                    state.hydrated,
                    state.storage_error.clone(),
                )
            })
            .unwrap_or((0, String::new(), false, Some(unavailable())))
    }

    pub fn snapshot(&self) -> Result<Snapshot> {
        let state = self.state.lock().map_err(|_| unavailable())?;
        let mut snapshot = state.snapshot.clone();
        for entry in state.metadata.values() {
            let stale = entry.invalidated || metadata_is_stale(entry.observed.elapsed());
            crate::metadata::merge_report(&mut snapshot, &entry.report, stale);
        }
        Ok(snapshot)
    }

    pub(crate) fn inventory_page(&self, after: &str) -> Result<InventoryEventPage> {
        let state = self.state.lock().map_err(|_| unavailable())?;
        if let Some(store) = &state.store {
            return store.read_journal(after).map_err(store_error);
        }
        let Some((epoch, after_sequence)) = cursor_epoch(after) else {
            return Err(DomainError::new(
                ErrorCode::EventGap,
                "Invalid inventory cursor",
            ));
        };
        if epoch != state.epoch {
            return Err(DomainError::new(
                ErrorCode::EventGap,
                "Inventory cursor belongs to a previous runtime epoch",
            ));
        }
        let tail = state
            .journal
            .back()
            .map(|event| event.sequence.parse::<u64>().unwrap_or(0))
            .unwrap_or(0);
        if after_sequence > tail {
            return Err(DomainError::new(
                ErrorCode::EventGap,
                "Inventory cursor is ahead of the journal",
            ));
        }
        if let Some(oldest) = state.journal.front() {
            let oldest = oldest.sequence.parse::<u64>().unwrap_or(0);
            if after_sequence.saturating_add(1) < oldest {
                return Err(DomainError::new(
                    ErrorCode::EventGap,
                    "Inventory cursor is no longer replayable",
                ));
            }
        }
        let events: Vec<_> = state
            .journal
            .iter()
            .filter(|event| event.sequence.parse::<u64>().unwrap_or(0) > after_sequence)
            .take(JOURNAL_PAGE)
            .cloned()
            .collect();
        let cursor = events
            .last()
            .map(|event| event.sequence.clone())
            .unwrap_or_else(|| after_sequence.to_string());
        Ok(InventoryEventPage {
            events,
            cursor: format!("{}:{cursor}", state.epoch),
            tail_cursor: format!("{}:{tail}", state.epoch),
            has_more: cursor.parse::<u64>().unwrap_or(0) < tail,
            revision: state.revision.to_string(),
        })
    }

    pub(crate) fn probe_input(&self, connection: &str) -> Result<ProbeTarget> {
        let state = self.state.lock().map_err(|_| unavailable())?;
        let located =
            state._locations.get(connection).cloned().ok_or_else(|| {
                DomainError::new(ErrorCode::NotFound, "Connection is unavailable")
            })?;
        let route = state
            .snapshot
            .routes
            .iter()
            .find(|route| route.connection_id == connection)
            .ok_or_else(unavailable)?;
        Ok(ProbeTarget {
            connection_id: connection.to_owned(),
            located,
            route_id: route.id.clone(),
            base: state.revision.to_string(),
        })
    }

    pub(crate) fn has_fresh_metadata(&self, connection: &str) -> Result<bool> {
        let state = self.state.lock().map_err(|_| unavailable())?;
        Ok(state.metadata.get(connection).is_some_and(|entry| {
            !entry.invalidated && entry.observed.elapsed() < METADATA_PROBE_CACHE_TTL
        }))
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

    pub(crate) fn publish_metadata(&self, base: &str, report: ConnectionMetadata) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        if state.revision.to_string() != base {
            return Err(DomainError::new(
                ErrorCode::SelectionStale,
                "Inventory changed during the metadata probe",
            ));
        }
        let mut next = state.snapshot.clone();
        crate::metadata::merge_report(&mut next, &report, false);
        next.observed_at = Timestamp::now();
        state.metadata_failures.remove(&report.connection_id);
        let observed_at_ms = unix_ms(SystemTime::now());
        let mut entry = MetadataEntry {
            observed: Instant::now(),
            report,
            invalidated: false,
        };
        let changes = {
            let mut preview = next.clone();
            let issue_set = self.compose_issues_locked(&state, &preview);
            preview.issues = issue_set.clone();
            preview.health = summarize(&issue_set, state.hydrated);
            let mut changes = diff_snapshot(&state.snapshot, &preview);
            if changes.is_empty() {
                changes.push(ResourceChange {
                    resource: "connection".into(),
                    id: entry.report.connection_id.clone(),
                    change: ResourceChangeKind::Updated,
                    reason: Some("metadata observation refreshed".into()),
                });
            }
            next = preview;
            changes
        };
        self.commit_locked(
            &mut state,
            &mut next,
            Some(&mut entry),
            "metadata",
            changes,
            observed_at_ms,
        )?;
        state
            .metadata
            .insert(entry.report.connection_id.clone(), entry);
        Ok(())
    }

    pub(crate) fn fail_metadata(&self, connection: &str, error: DomainError) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
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
                error: error.clone(),
            },
        );
        if let Some(entry) = state.metadata.get_mut(connection) {
            entry.invalidated = true;
        }
        state.dirty.insert(connection.into());
        let mut next = state.snapshot.clone();
        // The last-known observation remains visible as stale; failure never
        // rewrites it as fresh or pretends the account signed out.
        for entry in state.metadata.values() {
            crate::metadata::merge_report(&mut next, &entry.report, true);
        }
        next.observed_at = Timestamp::now();
        let issues = self.compose_issues_locked(&state, &next);
        next.issues = issues.clone();
        next.health = summarize(&issues, state.hydrated);
        let changes = diff_snapshot(&state.snapshot, &next);
        if !changes.is_empty() {
            self.commit_locked(&mut state, &mut next, None, "metadata_failure", changes, 0)?;
        }
        drop(state);
        self.wake.notify_waiters();
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

    /// Invalidate stale facts and immediately publish the stale transition.
    /// The background lane performs the actual re-read with debounce/backoff.
    pub(crate) fn invalidate_metadata(&self, connections: &[String]) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        let mut changed = Vec::new();
        for id in connections {
            if let Some(entry) = state.metadata.get_mut(id) {
                if !entry.invalidated {
                    entry.invalidated = true;
                    changed.push(id.clone());
                }
            }
            state.dirty.insert(id.clone());
        }
        if changed.is_empty() {
            drop(state);
            self.wake.notify_waiters();
            return Ok(());
        }
        let mut next = state.snapshot.clone();
        for entry in state.metadata.values() {
            crate::metadata::merge_report(
                &mut next,
                &entry.report,
                entry.invalidated || metadata_is_stale(entry.observed.elapsed()),
            );
        }
        next.observed_at = Timestamp::now();
        let issues = self.compose_issues_locked(&state, &next);
        next.issues = issues.clone();
        next.health = summarize(&issues, state.hydrated);
        let changes: Vec<_> = changed
            .iter()
            .map(|id| ResourceChange {
                resource: "connection".into(),
                id: id.clone(),
                change: ResourceChangeKind::Updated,
                reason: Some("local metadata changed; refresh scheduled".into()),
            })
            .collect();
        self.commit_locked(&mut state, &mut next, None, "invalidate", changes, 0)?;
        drop(state);
        self.wake.notify_waiters();
        Ok(())
    }

    pub(crate) fn due_probes(
        &self,
        allowed_providers: &BTreeSet<String>,
        limit: usize,
    ) -> Result<Vec<ProbeTarget>> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        let now = Instant::now();
        let connections: Vec<_> = state.snapshot.connections.clone();
        let mut due = Vec::new();
        for connection in connections {
            if !allowed_providers.contains(&connection.provider_id) {
                continue;
            }
            if state
                .metadata_failures
                .get(&connection.id)
                .is_some_and(|failure| failure.until > now)
            {
                continue;
            }
            let needs = match state.metadata.get(&connection.id) {
                None => true,
                Some(entry) => {
                    entry.invalidated
                        || entry.observed.elapsed() >= METADATA_FRESHNESS_TTL
                        || state.dirty.contains(&connection.id)
                }
            };
            if !needs {
                continue;
            }
            let Some(located) = state._locations.get(&connection.id).cloned() else {
                continue;
            };
            let Some(route) = state
                .snapshot
                .routes
                .iter()
                .find(|route| route.connection_id == connection.id)
                .cloned()
            else {
                continue;
            };
            state.dirty.remove(&connection.id);
            due.push(ProbeTarget {
                connection_id: connection.id,
                located,
                route_id: route.id,
                base: state.revision.to_string(),
            });
            if due.len() >= limit {
                break;
            }
        }
        Ok(due)
    }

    pub(crate) fn set_notification_preferences(
        &self,
        preferences: NotificationPreferences,
    ) -> Result<()> {
        let state = self.state.lock().map_err(|_| unavailable())?;
        if let Some(store) = &state.store {
            store.set_preferences(&preferences).map_err(store_error)?;
        }
        drop(state);
        self.state
            .lock()
            .map_err(|_| unavailable())?
            .notification_preferences = preferences;
        Ok(())
    }

    pub(crate) fn poll_notifications(
        &self,
        issues: &[Issue],
    ) -> Result<Vec<lai_contracts::NotificationDecision>> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        let now_ms = unix_ms(SystemTime::now());
        let decisions = crate::notifications::poll(
            issues,
            &state.notification_preferences,
            state.store.as_deref(),
            &state.notification_history,
            crate::notifications::local_minute(),
            now_ms,
        );
        for decision in &decisions {
            if decision.should_notify {
                state
                    .notification_history
                    .insert(decision.dedupe_key.clone(), now_ms);
            }
        }
        Ok(decisions)
    }

    pub(crate) fn update_issue(
        &self,
        issue_id: &str,
        action: impl FnOnce(&mut Issue) -> Result<()>,
    ) -> Result<Issue> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        let mut issue = state
            .issues
            .get(issue_id)
            .cloned()
            .ok_or_else(|| DomainError::new(ErrorCode::NotFound, "Issue is unavailable"))?;
        action(&mut issue)?;
        state.issues.insert(issue.id.clone(), issue.clone());
        let mut next = state.snapshot.clone();
        next.issues = state.issues.values().cloned().collect();
        next.health = summarize(&next.issues, state.hydrated);
        let changes = vec![ResourceChange {
            resource: "issue".into(),
            id: issue.id.clone(),
            change: ResourceChangeKind::Updated,
            reason: Some(format!("{:?}", issue.state)),
        }];
        self.commit_locked(&mut state, &mut next, None, "issue_update", changes, 0)?;
        Ok(issue)
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
                let _ = worker.await;
                Err(error)
            }
        };
        let result = self.publish_scan(outcome);
        sender.send_replace(Some(result));
    }

    fn publish_scan(&self, outcome: Result<ScanResult>) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        state.flight = None;
        match outcome {
            Ok(result) => {
                let mut next = result.snapshot;
                state.metadata.retain(|id, _| {
                    next.connections
                        .iter()
                        .any(|connection| connection.id == *id)
                });
                state.metadata_failures.retain(|id, _| {
                    next.connections
                        .iter()
                        .any(|connection| connection.id == *id)
                });
                state.dirty.retain(|id| {
                    next.connections
                        .iter()
                        .any(|connection| connection.id == *id)
                });
                for entry in state.metadata.values() {
                    crate::metadata::merge_report(
                        &mut next,
                        &entry.report,
                        entry.invalidated || metadata_is_stale(entry.observed.elapsed()),
                    );
                }
                next.observed_at = Timestamp::now();
                let issues = self.compose_issues_locked(&state, &next);
                next.issues = issues.clone();
                next.health = summarize(&issues, state.hydrated);
                let changes = diff_snapshot(&state.snapshot, &next);
                state._locations = result.locations;
                let committed = self.commit_locked(&mut state, &mut next, None, "scan", changes, 0);
                match committed {
                    Ok(()) => {
                        for connection in &next.connections {
                            if !state.metadata.contains_key(&connection.id) {
                                state.dirty.insert(connection.id.clone());
                            }
                        }
                        state.snapshot = next;
                        state.updated = Some(Instant::now());
                        state.failure = None;
                        drop(state);
                        self.wake.notify_waiters();
                        Ok(())
                    }
                    Err(error) => {
                        state.failure = Some((Instant::now(), error.clone()));
                        state.storage_error = Some(error.clone());
                        Err(error)
                    }
                }
            }
            Err(error) => {
                // Never echo an arbitrary adapter/OS error, which may include private paths.
                let safe = DomainError::new(error.code, "Native inventory is unavailable");
                state.failure = Some((Instant::now(), safe.clone()));
                Err(safe)
            }
        }
    }

    fn compose_issues_locked(&self, state: &CacheState, snapshot: &Snapshot) -> Vec<Issue> {
        let mut issues = derive_issues(snapshot, &state.issues, state.hydrated);
        for (connection, failure) in &state.metadata_failures {
            let now = Timestamp::now();
            let extra = Issue {
                id: lai_contracts::stable_id("issue", &["adapter_failure", connection]),
                fingerprint: connection.clone(),
                kind: lai_contracts::IssueKind::AdapterFailure,
                severity: lai_contracts::IssueSeverity::Warning,
                state: IssueState::Open,
                title: "Provider metadata could not be refreshed".into(),
                summary: "Last-known data remains visible as stale. The runtime retries with backoff.".into(),
                provider_id: snapshot
                    .connections
                    .iter()
                    .find(|candidate| &candidate.id == connection)
                    .map(|candidate| candidate.provider_id.clone()),
                connection_id: Some(connection.clone()),
                account_id: None,
                route_id: None,
                pool_id: None,
                window_id: None,
                first_seen_at: now.clone(),
                last_seen_at: now,
                resolved_at: None,
                snoozed_until: None,
                occurrences: failure.attempts.max(1),
                evidence: vec![lai_contracts::IssueEvidence {
                    code: format!("{:?}", failure.error.code),
                    message: failure.error.message.clone(),
                    source: Some(lai_contracts::EvidenceSource::new(
                        lai_contracts::EvidenceKind::Derived,
                        "lai-service",
                        "metadata refresh",
                    )),
                    observed_at: Some(Timestamp::now()),
                    freshness: Some(lai_contracts::Freshness::Fresh),
                    verification: Some(lai_contracts::Verification::Observed),
                }],
                remediation: vec![lai_contracts::Remediation {
                    kind: lai_contracts::RemediationKind::RetryLater,
                    label: "Retry later".into(),
                    description: "The runtime retries with bounded backoff. Use Refresh for an explicit retry.".into(),
                    requires_user_action: false,
                }],
                last_observation: None,
            };
            if let Some(existing) = state.issues.get(&extra.id) {
                let merged = merge_issue(Some(existing), extra);
                if let Some(slot) = issues.iter_mut().find(|issue| issue.id == merged.id) {
                    *slot = merged;
                } else {
                    issues.push(merged);
                }
            } else {
                issues.push(extra);
            }
        }
        if let Some(error) = &state.storage_error {
            let issue = storage_issue(error, true);
            if let Some(existing) = state.issues.get(&issue.id) {
                let merged = merge_issue(Some(existing), issue);
                if let Some(slot) = issues
                    .iter_mut()
                    .find(|candidate| candidate.id == merged.id)
                {
                    *slot = merged;
                } else {
                    issues.push(merged);
                }
            } else {
                issues.push(issue);
            }
        }
        issues.sort_by(|left, right| {
            right
                .severity
                .cmp(&left.severity)
                .then_with(|| right.last_seen_at.as_str().cmp(left.last_seen_at.as_str()))
        });
        issues.truncate(crate::health::MAX_ISSUES);
        issues
    }

    fn compose_and_commit(
        &self,
        reason: &str,
        metadata: Option<DurableMetadata>,
        force_change: bool,
    ) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| unavailable())?;
        let mut next = state.snapshot.clone();
        for entry in state.metadata.values() {
            crate::metadata::merge_report(
                &mut next,
                &entry.report,
                entry.invalidated || metadata_is_stale(entry.observed.elapsed()),
            );
        }
        next.observed_at = Timestamp::now();
        let issues = self.compose_issues_locked(&state, &next);
        next.issues = issues.clone();
        next.health = summarize(&issues, state.hydrated);
        let mut changes = diff_snapshot(&state.snapshot, &next);
        if changes.is_empty() && force_change {
            changes.push(ResourceChange {
                resource: "inventory".into(),
                id: "*".into(),
                change: ResourceChangeKind::Updated,
                reason: Some(reason.into()),
            });
        }
        self.commit_locked(
            &mut state,
            &mut next,
            None,
            reason,
            changes,
            metadata.as_ref().map(|m| m.observed_at_ms).unwrap_or(0),
        )?;
        if let Some(metadata) = metadata {
            if let Some(entry) = state.metadata.get_mut(&metadata.report.connection_id) {
                entry.report = metadata.report;
            }
        }
        Ok(())
    }

    fn commit_locked(
        &self,
        state: &mut CacheState,
        next: &mut Snapshot,
        metadata: Option<&mut MetadataEntry>,
        reason: &str,
        changes: Vec<ResourceChange>,
        observed_at_ms: u64,
    ) -> Result<()> {
        let issues_changed = !json_eq(&state.snapshot.issues, &next.issues);
        if let Some(store) = &state.store {
            let durable = metadata.as_ref().map(|entry| DurableMetadata {
                observed_at_ms: if observed_at_ms == 0 {
                    unix_ms(SystemTime::now())
                } else {
                    observed_at_ms
                },
                report: entry.report.clone(),
            });
            match store.commit_snapshot(next, durable, reason, changes, issues_changed) {
                Ok(committed) => {
                    state.revision = committed.revision.parse().unwrap_or(state.revision + 1);
                    state.cursor = committed.cursor.clone();
                    state.journal.push_back(committed.change);
                    while state.journal.len() > JOURNAL_LIMIT {
                        state.journal.pop_front();
                    }
                }
                Err(error) => {
                    let safe = storage_failure(error.domain());
                    state.storage_error = Some(safe.clone());
                    return Err(safe);
                }
            }
        } else {
            let revision = state.revision.saturating_add(1);
            let sequence = revision;
            let cursor = format!("{}:{sequence}", state.epoch);
            next.revision = revision.to_string();
            next.cursor = cursor.clone();
            let change = InventoryChange {
                sequence: sequence.to_string(),
                cursor,
                revision: revision.to_string(),
                previous_revision: state.revision.to_string(),
                observed_at: next.observed_at.clone(),
                reason: reason.into(),
                changes,
                issues_changed,
            };
            state.revision = revision;
            state.cursor = change.cursor.clone();
            state.journal.push_back(change);
            while state.journal.len() > JOURNAL_LIMIT {
                state.journal.pop_front();
            }
        }
        state.snapshot = next.clone();
        for issue in &next.issues {
            state.issues.insert(issue.id.clone(), issue.clone());
        }
        state
            .issues
            .retain(|id, _| next.issues.iter().any(|issue| &issue.id == id));
        Ok(())
    }
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
