use crate::InventorySource;
use lai_contracts::{DataMode, DomainError, ErrorCode, Result, Snapshot};
use lai_discovery::{LocatedClient, ScanResult};
use std::{
    collections::BTreeMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};
use tokio::sync::watch;

type RefreshResult = Option<Result<()>>;
struct CacheState {
    snapshot: Snapshot,
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
impl InventoryCache {
    pub fn new(source: InventorySource, ttl: Duration) -> Self {
        Self {
            source,
            ttl,
            state: Mutex::new(CacheState {
                snapshot: Snapshot::empty(DataMode::Live),
                _locations: BTreeMap::new(),
                updated: None,
                failure: None,
                flight: None,
            }),
        }
    }
    pub fn snapshot(&self) -> Result<Snapshot> {
        Ok(self
            .state
            .lock()
            .map_err(|_| unavailable())?
            .snapshot
            .clone())
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
                state.snapshot = result.snapshot;
                state._locations = result.locations;
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
