//! One native service shared by every transport. Only the trusted host sets policies.
mod execution;
mod inputs;
pub use execution::{
    ExecutionGrant, ExecutionInput, ExecutionSource, PERSISTENT_RESUME, PersistentResume,
};
mod cache;
mod catalog;
mod metadata;
mod methods;
mod params;
mod resume_token;
use cache::InventoryCache;
use lai_chatgpt_auth::AuthManager;
use lai_contracts::{DomainError, ErrorCode, MAX_TURN_START_PARAMS_BYTES, Result, Snapshot};
use lai_core::{ClientPolicy, project_snapshot};
use lai_discovery::ScanResult;
use lai_platform::watcher::MetadataWatcher;
pub use metadata::{MetadataFuture, MetadataSource};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    sync::{Arc, RwLock, atomic::AtomicBool},
    time::Duration,
};

pub type InventorySource = Arc<dyn Fn(&AtomicBool) -> Result<ScanResult> + Send + Sync>;
pub struct NativeService {
    clients: Arc<RwLock<BTreeMap<String, ClientPolicy>>>,
    adapters: Vec<lai_contracts::AdapterDescriptor>,
    execution_state: execution::ExecutionState,
    cache: Arc<InventoryCache>,
    metadata_source: Option<MetadataSource>,
    metadata_locks: std::sync::Mutex<BTreeMap<String, Arc<tokio::sync::Mutex<()>>>>,
    watchers: std::sync::Mutex<BTreeMap<String, MetadataWatcher>>,
    /// Stable application identity per connection, for durable resume tokens.
    app_identities: RwLock<BTreeMap<String, String>>,
    chatgpt_auth: Option<Arc<AuthManager>>,
    /// Serializes managed sign-out with session publication and turn enqueue.
    chatgpt_lifecycle: tokio::sync::Mutex<()>,
}
impl NativeService {
    pub fn new(source: InventorySource, ttl: Duration) -> Self {
        Self {
            clients: Arc::new(RwLock::new(BTreeMap::new())),
            adapters: Vec::new(),
            execution_state: Default::default(),
            metadata_source: None,
            metadata_locks: std::sync::Mutex::new(BTreeMap::new()),
            watchers: std::sync::Mutex::new(BTreeMap::new()),
            app_identities: RwLock::new(BTreeMap::new()),
            chatgpt_auth: None,
            chatgpt_lifecycle: tokio::sync::Mutex::new(()),
            cache: Arc::new(InventoryCache::new(source, ttl)),
        }
    }
    /// Host-only: persist the last metadata observations at `path` (identity excluded) and reload
    /// them after a restart, so status is shown immediately and refreshed in the background.
    pub fn enable_metadata_cache(&self, path: std::path::PathBuf) {
        self.cache.enable_persistence(path);
    }
    pub fn with_adapters(mut self, adapters: Vec<lai_contracts::AdapterDescriptor>) -> Self {
        self.adapters = adapters;
        self
    }
    /// Opt-in host-managed SIWC authentication. Callers still require an
    /// explicit managed-provider metadata grant for every auth RPC.
    pub fn with_chatgpt_auth(mut self, auth: Arc<AuthManager>) -> Self {
        self.chatgpt_auth = Some(auth);
        self
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
    /// Host-only: names the application behind a connection. Connection ids
    /// change on every pairing or restart; durable resume tokens are bound to
    /// this identity instead (default: the connection id itself).
    pub fn set_app_identity(&self, client: &str, app: &str) -> Result<()> {
        if app.is_empty() || app.len() > 128 || app.chars().any(char::is_control) {
            return Err(params::invalid());
        }
        self.policy(client)?;
        self.app_identities
            .write()
            .map_err(|_| denied())?
            .insert(client.to_owned(), app.to_owned());
        Ok(())
    }
    pub(crate) fn app_identity(&self, client: &str) -> String {
        self.app_identities
            .read()
            .ok()
            .and_then(|ids| ids.get(client).cloned())
            .unwrap_or_else(|| client.to_owned())
    }
    /// Host-only cleanup for an application connection; other clients remain live.
    pub fn unregister_client(&self, id: &str) {
        if let Ok(mut ids) = self.app_identities.write() {
            ids.remove(id);
        }
        self.execution_state.revoke(id);
        if let Ok(mut watchers) = self.watchers.lock() {
            watchers.remove(id);
        }
        if let Ok(mut clients) = self.clients.write() {
            clients.remove(id);
        }
    }
    /// Release only this host-registered application. Other clients and shared metadata survive.
    pub fn close_client(&self, id: &str) {
        if let Ok(mut watchers) = self.watchers.lock() {
            watchers.remove(id);
        }
        if let Ok(mut clients) = self.clients.write() {
            clients.remove(id);
        }
        self.execution_state.revoke(id);
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
        let mut snapshot = project_snapshot(&snapshot, &self.policy(id)?);
        execution::decorate(
            &mut snapshot,
            &self.execution_state.providers,
            &self.execution_state.connections,
            &self.execution_state.persistent,
        );
        Ok(snapshot)
    }
    fn set_watch(&self, client: &str, enabled: bool) -> Result<Value> {
        if !self.policy(client)?.allow_passive_discovery {
            return Err(denied());
        }
        let mut watchers = self.watchers.lock().map_err(|_| denied())?;
        if !enabled {
            watchers.remove(client);
            return Ok(serde_json::json!({"watching":false}));
        }
        if watchers.contains_key(client) {
            return Ok(serde_json::json!({"watching":true}));
        }
        let roots = self.cache.watch_roots()?;
        if roots.is_empty() {
            return Err(DomainError::new(
                ErrorCode::Conflict,
                "Run passive discovery before enabling local metadata watching",
            ));
        }
        let cache = self.cache.clone();
        let watcher = MetadataWatcher::start(
            roots,
            Arc::new(move |ids| {
                let _ = cache.invalidate_metadata(&ids);
            }),
        )?;
        watchers.insert(client.to_owned(), watcher);
        Ok(serde_json::json!({"watching":true}))
    }
    pub async fn handle(&self, client: &str, method: &str, params: Value) -> Result<Value> {
        let _chatgpt_lifecycle =
            if self.chatgpt_auth.is_some() && method == "auth.chatgpt.disconnect" {
                Some(self.chatgpt_lifecycle.lock().await)
            } else {
                None
            };
        self.policy(client)?;
        // Staged images and turn text have separate bounds and closed typed params.
        if method == "inputs.put" {
            params.as_object().ok_or_else(params::invalid)?;
        } else if method == "turns.start" {
            params::object_with_limit(&params, MAX_TURN_START_PARAMS_BYTES)?;
        } else {
            params::object(&params)?;
        }
        if method.len() > 128 || params.get("clientId").is_some() {
            return Err(params::invalid());
        }
        if method == "runtime.closeClient" {
            params::allow(&params, &[])?;
            self.close_client(client);
            return Ok(serde_json::json!({"closed":true}));
        }
        if method.starts_with("auth.chatgpt.") {
            if !self
                .policy(client)?
                .probe_provider_ids
                .contains("codex_managed")
            {
                return Err(denied());
            }
            let auth = self.chatgpt_auth.as_ref().ok_or_else(unsupported)?;
            let map_auth = |error: lai_chatgpt_auth::AuthError| {
                DomainError::new(lai_contracts::ErrorCode::ProviderUnavailable, error.message)
            };
            let value = match method {
                "auth.chatgpt.status" => {
                    params::allow(&params, &[])?;
                    serde_json::to_value(auth.status().await.map_err(map_auth)?)
                }
                "auth.chatgpt.start" => {
                    params::allow(&params, &[])?;
                    serde_json::to_value(auth.start().await.map_err(map_auth)?)
                }
                "auth.chatgpt.cancel" => {
                    params::allow(&params, &["loginId"])?;
                    let id = params::text(&params, "loginId")?.ok_or_else(params::invalid)?;
                    auth.cancel(id).await.map_err(map_auth)?;
                    Ok(serde_json::json!({}))
                }
                "auth.chatgpt.disconnect" => {
                    params::allow(&params, &[])?;
                    // Clear durable credentials before sweeping sessions. The
                    // lifecycle lease prevents a concurrent create from
                    // publishing a new session after this sweep.
                    let mut close_result = Ok(());
                    let disconnect = auth
                        .disconnect_with(|| async {
                            close_result = self.close_managed_sessions().await;
                        })
                        .await;
                    close_result?;
                    // Remote revocation can fail after local sign-out. Never
                    // restore secrets or report the local disconnect as failed.
                    if let Err(error) = disconnect {
                        if error.code != "REVOCATION_UNCONFIRMED" {
                            return Err(map_auth(error));
                        }
                    }
                    Ok(serde_json::json!({}))
                }
                _ => return Err(unsupported()),
            };
            return value.map_err(|_| params::invalid());
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
        if method == "discovery.watch" {
            params::allow(&params, &["enabled"])?;
            return self.set_watch(client, params::boolean(&params, "enabled")?);
        }
        if method == "connections.probe" {
            return self.probe_connection(client, &params).await;
        }
        if method == "grants.revoke" {
            let value = self.dispatch(client, method, &params)?;
            self.execution_state.revoke(client);
            if let Ok(mut watchers) = self.watchers.lock() {
                watchers.remove(client);
            }
            return Ok(value);
        }
        if method.starts_with("sessions.")
            || method.starts_with("turns.")
            || method.starts_with("inputs.")
            || method == "approvals.respond"
            || method == "events.read"
            || method == "artifacts.read"
        {
            return self.execute_method(client, method, params).await;
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
