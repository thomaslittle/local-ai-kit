//! One native service shared by every transport. Only the trusted host sets policies.
mod cache;
mod metadata;
mod methods;
mod params;
use cache::InventoryCache;
use lai_contracts::{DomainError, ErrorCode, Result, Snapshot};
use lai_core::{ClientPolicy, project_snapshot};
use lai_discovery::ScanResult;
pub use metadata::{MetadataFuture, MetadataSource};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock, atomic::AtomicBool},
    time::Duration,
};

pub type InventorySource = Arc<dyn Fn(&AtomicBool) -> Result<ScanResult> + Send + Sync>;
pub struct NativeService {
    clients: RwLock<BTreeMap<String, ClientPolicy>>,
    cache: Arc<InventoryCache>,
    metadata_source: Option<MetadataSource>,
    metadata_locks: std::sync::Mutex<BTreeMap<String, Arc<tokio::sync::Mutex<()>>>>,
}
impl NativeService {
    pub fn new(source: InventorySource, ttl: Duration) -> Self {
        Self {
            clients: RwLock::new(BTreeMap::new()),
            metadata_source: None,
            metadata_locks: std::sync::Mutex::new(BTreeMap::new()),
            cache: Arc::new(InventoryCache::new(source, ttl)),
        }
    }
    /// Host-only API. Duplicate registration cannot silently replace a grant.
    pub fn register_client(&self, id: &str, policy: ClientPolicy) -> Result<()> {
        if id.is_empty() || id.len() > 128 || id.chars().any(char::is_control) {
            return Err(params::invalid());
        }
        let mut clients = self.clients.write().map_err(|_| denied())?;
        if clients.contains_key(id) {
            return Err(DomainError::new(
                ErrorCode::Conflict,
                "Client is already registered",
            ));
        }
        if clients.len() >= 64 {
            return Err(DomainError::new(
                ErrorCode::Conflict,
                "Client capacity reached",
            ));
        }
        clients.insert(id.to_owned(), policy);
        Ok(())
    }
    fn policy(&self, id: &str) -> Result<ClientPolicy> {
        self.clients
            .read()
            .map_err(|_| denied())?
            .get(id)
            .cloned()
            .ok_or_else(denied)
    }
    fn projected(&self, id: &str) -> Result<Snapshot> {
        // Acquire policy after loading data; never reuse a policy from before an await.
        let snapshot = self.cache.snapshot()?;
        Ok(project_snapshot(&snapshot, &self.policy(id)?))
    }
    pub async fn handle(&self, client: &str, method: &str, params: Value) -> Result<Value> {
        self.policy(client)?;
        params::object(&params)?;
        if method.len() > 128 || params.get("clientId").is_some() {
            return Err(params::invalid());
        }
        if method == "discovery.refresh" {
            params::allow(&params, &["level"])?;
            if params::text(&params, "level")?.is_some_and(|level| level != "passive") {
                return Err(unsupported());
            }
            if !self.policy(client)?.allow_passive_discovery {
                return Err(denied());
            }
            self.cache.refresh().await?;
            if !self.policy(client)?.allow_passive_discovery {
                return Err(denied());
            }
            let snapshot = self.projected(client)?;
            return Ok(serde_json::json!({"revision":snapshot.revision,"mode":snapshot.mode}));
        }
        if method == "connections.probe" {
            return self.probe_connection(client, &params).await;
        }
        self.dispatch(client, method, &params)
    }
}
fn denied() -> DomainError {
    DomainError::new(
        ErrorCode::LocalAccessDenied,
        "Local access is not authorized",
    )
}
fn unsupported() -> DomainError {
    DomainError::new(
        ErrorCode::CapabilityUnsupported,
        "Operation is not available in this runtime",
    )
}
