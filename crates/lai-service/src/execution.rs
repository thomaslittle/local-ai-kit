//! Host-authorized execution. RPC callers cannot select executables, credentials, or arbitrary roots.
use crate::{NativeService, denied, params, unsupported};
use lai_contracts::*;
use lai_core::{project_snapshot, validate_selection};
use lai_discovery::LocatedClient;
use lai_runtime::{Agent, BoxFuture, Runtime};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    sync::{Arc, RwLock},
    time::Duration,
};
#[derive(Clone, Default)]
pub struct ExecutionGrant {
    pub connection_ids: BTreeSet<String>,
    pub provider_ids: BTreeSet<String>,
    pub workspaces: BTreeMap<String, PathBuf>,
    pub allow_paid: bool,
    pub allow_writes: bool,
    pub review_native: bool,
    /// Explicit host trust in the installed client's native permission policy; not a Kit OS sandbox.
    pub allow_native_policy: bool,
}
impl ExecutionGrant {
    fn allows(&self, c: &Connection) -> bool {
        self.connection_ids.contains(&c.id) || self.provider_ids.contains(&c.provider_id)
    }
}
#[derive(Clone)]
pub struct ExecutionInput {
    pub located: LocatedClient,
    pub connection_id: String,
    pub route_id: String,
    pub route_provider_id: String,
    pub model: Model,
    pub workspace: PathBuf,
    pub writable: bool,
    pub review_requests: bool,
    pub native_policy: bool,
}
pub type ExecutionSource =
    Arc<dyn Fn(ExecutionInput) -> BoxFuture<'static, Box<dyn Agent>> + Send + Sync>;
#[derive(Default)]
pub(crate) struct ExecutionState {
    pub providers: BTreeSet<String>,
    pub source: Option<ExecutionSource>,
    pub runtime: Arc<Runtime>,
    pub grants: Arc<RwLock<BTreeMap<String, ExecutionGrant>>>,
    session_billing: RwLock<BTreeMap<String, (String, bool)>>,
}
impl ExecutionState {
    pub fn revoke(&self, client: &str) {
        if let Ok(mut grants) = self.grants.write() {
            grants.remove(client);
        }
        self.runtime.revoke(client);
        if let Ok(mut sessions) = self.session_billing.write() {
            sessions.retain(|_, (owner, _)| owner != client);
        }
    }
}
type Create = SessionCreateParams;
type Start = TurnStartParams;
type Update = SessionOptionsParams;
fn decode<T: serde::de::DeserializeOwned>(p: Value) -> Result<T> {
    serde_json::from_value(p).map_err(|_| params::invalid())
}
pub(crate) fn decorate(snapshot: &mut Snapshot, providers: &BTreeSet<String>) {
    for route in &mut snapshot.routes {
        if snapshot
            .connections
            .iter()
            .any(|c| c.id == route.connection_id && providers.contains(&c.provider_id))
        {
            for cap in &mut route.capabilities {
                if cap.id == "generation.text" && cap.origin == CapabilityOrigin::Runtime {
                    cap.support = Support::Supported;
                    if let Observation::Known { value, .. } = &mut cap.evidence {
                        *value =
                            "Native execution driver registered; separate grant required".into();
                    }
                }
            }
        }
    }
}
impl NativeService {
    pub fn with_execution(mut self, providers: BTreeSet<String>, source: ExecutionSource) -> Self {
        self.execution_state.providers = providers;
        self.execution_state.source = Some(source);
        self
    }
    pub fn register_execution(&self, client: &str, mut grant: ExecutionGrant) -> Result<()> {
        self.policy(client)?;
        if grant.workspaces.is_empty()
            || grant.workspaces.len() > 16
            || grant.connection_ids.len() > 128
            || grant.provider_ids.len() > 64
        {
            return Err(params::invalid());
        }
        for (id, path) in &mut grant.workspaces {
            if id.is_empty()
                || id.len() > 64
                || id.chars().any(char::is_control)
                || !path.is_absolute()
            {
                return Err(params::invalid());
            }
            *path = path.canonicalize().map_err(|_| denied())?;
            if !path.is_dir() {
                return Err(denied());
            }
        }
        let mut grants = self.execution_state.grants.write().map_err(|_| denied())?;
        if grants.contains_key(client) {
            return Err(DomainError::new(
                ErrorCode::Conflict,
                "Execution grant already exists",
            ));
        }
        grants.insert(client.into(), grant);
        Ok(())
    }
    pub(crate) fn execution_methods(&self) -> Vec<&'static str> {
        if self.execution_state.source.is_none() {
            vec![]
        } else {
            vec![
                "artifacts.read",
                "sessions.create",
                "sessions.get",
                "sessions.updateOptions",
                "sessions.close",
                "turns.start",
                "turns.cancel",
                "approvals.respond",
                "events.read",
                "events.subscribe",
                "events.unsubscribe",
            ]
        }
    }
    fn accepted_paid(&self, client: &str, session: &str) -> Result<bool> {
        self.execution_state
            .session_billing
            .read()
            .map_err(|_| denied())?
            .get(session)
            .filter(|(owner, _)| owner == client)
            .map(|(_, allowed)| *allowed)
            .ok_or_else(denied)
    }
    fn execution_grant(&self, client: &str) -> Result<ExecutionGrant> {
        self.execution_state
            .grants
            .read()
            .map_err(|_| denied())?
            .get(client)
            .cloned()
            .ok_or_else(denied)
    }
    fn check_session(&self, client: &str, id: &str) -> Result<Session> {
        let session = self.execution_state.runtime.get(client, id)?;
        let snapshot = self.projected(client)?;
        let c = snapshot
            .connections
            .iter()
            .find(|c| c.id == session.connection_id)
            .ok_or_else(denied)?;
        if !self.execution_grant(client)?.allows(c) {
            return Err(denied());
        }
        Ok(session)
    }
    pub(crate) async fn execute_method(
        &self,
        client: &str,
        method: &str,
        p: Value,
    ) -> Result<Value> {
        if self.execution_state.source.is_none() {
            return Err(unsupported());
        }
        let runtime = &self.execution_state.runtime;
        match method {
            "artifacts.read" => {
                let request: ArtifactReadParams = decode(p)?;
                self.check_session(client, &request.session_id)?;
                let offset = if request.offset.is_empty() {
                    0
                } else {
                    request
                        .offset
                        .parse::<u64>()
                        .map_err(|_| params::invalid())?
                };
                serde_json::to_value(runtime.artifact(
                    client,
                    &request.session_id,
                    &request.artifact_id,
                    offset,
                    request.max_bytes as usize,
                )?)
                .map_err(|_| params::invalid())
            }
            "sessions.create" => {
                let request: Create = decode(p)?;
                let grant = self.execution_grant(client)?;
                let snapshot = self.projected(client)?;
                let model = validate_selection(
                    &snapshot,
                    &request.selection,
                    request.allow_paid && grant.allow_paid,
                )?
                .clone();
                let route = snapshot
                    .routes
                    .iter()
                    .find(|r| r.id == model.route_id)
                    .ok_or_else(denied)?;
                let connection = snapshot
                    .connections
                    .iter()
                    .find(|c| c.id == route.connection_id)
                    .ok_or_else(denied)?;
                if !grant.allows(connection)
                    || !self
                        .execution_state
                        .providers
                        .contains(&connection.provider_id)
                {
                    return Err(denied());
                }
                let workspace = grant
                    .workspaces
                    .get(&request.workspace_id)
                    .ok_or_else(denied)?
                    .clone();
                if workspace.canonicalize().ok().as_ref() != Some(&workspace) {
                    return Err(denied());
                }
                let writable = match request.permission_mode.as_deref() {
                    None | Some("read_only") => false,
                    Some("workspace_write") if grant.allow_writes => true,
                    Some("native_policy") if grant.allow_native_policy => false,
                    _ => return Err(denied()),
                };
                if (request.allow_paid && !grant.allow_paid)
                    || (request.review_requests && !grant.review_native)
                {
                    return Err(denied());
                }
                let located = self.cache.probe_input(&connection.id)?.0;
                let input = ExecutionInput {
                    located,
                    connection_id: connection.id.clone(),
                    route_id: route.id.clone(),
                    route_provider_id: route.provider_id.clone(),
                    model,
                    workspace: workspace.clone(),
                    writable,
                    review_requests: request.review_requests,
                    native_policy: request.permission_mode.as_deref() == Some("native_policy"),
                };
                let clients = self.clients.clone();
                let grants = self.execution_state.grants.clone();
                let cache = self.cache.clone();
                let providers = self.execution_state.providers.clone();
                let owner = client.to_owned();
                let selected = request.selection.clone();
                let connection_id = connection.id.clone();
                let workspace_id = request.workspace_id.clone();
                let paid = request.allow_paid;
                let native_policy = request.permission_mode.as_deref() == Some("native_policy");
                let expected_account = connection.account_id.clone();
                let expected_billing = route.billing.value().cloned();
                let guard = Arc::new(move || {
                    let policy = clients
                        .read()
                        .map_err(|_| denied())?
                        .get(&owner)
                        .cloned()
                        .ok_or_else(denied)?;
                    let grant = grants
                        .read()
                        .map_err(|_| denied())?
                        .get(&owner)
                        .cloned()
                        .ok_or_else(denied)?;
                    let mut snapshot = project_snapshot(&cache.snapshot()?, &policy);
                    decorate(&mut snapshot, &providers);
                    let c = snapshot
                        .connections
                        .iter()
                        .find(|c| c.id == connection_id)
                        .ok_or_else(denied)?;
                    if !grant.allows(c)
                        || grant.workspaces.get(&workspace_id) != Some(&workspace)
                        || workspace.canonicalize().ok().as_ref() != Some(&workspace)
                    {
                        return Err(denied());
                    }
                    // Pre-turn validation requires fresh metadata; active streaming checks identity
                    // and revocation without treating the passage of time as an account change.
                    if c.account_id != expected_account
                        || c.auth.value() != Some(&AuthState::Authenticated)
                        || (paid && !grant.allow_paid)
                        || (native_policy && !grant.allow_native_policy)
                    {
                        return Err(denied());
                    }
                    let current = snapshot
                        .models
                        .iter()
                        .find(|m| m.id == selected.model_id && m.route_id == selected.route_id)
                        .ok_or_else(denied)?;
                    let route = snapshot
                        .routes
                        .iter()
                        .find(|r| r.id == selected.route_id)
                        .ok_or_else(denied)?;
                    if current.revision != selected.revision
                        || route.billing.value() != expected_billing.as_ref()
                    {
                        return Err(DomainError::new(
                            ErrorCode::SelectionStale,
                            "Session identity changed",
                        ));
                    }
                    Ok(())
                });
                guard()?;
                let mut agent = tokio::time::timeout(
                    Duration::from_secs(25),
                    self.execution_state.source.as_ref().unwrap()(input),
                )
                .await
                .map_err(|_| {
                    DomainError::new(ErrorCode::Timeout, "Native session startup timed out")
                })??;
                if let Err(error) = guard() {
                    let _ = agent.close().await;
                    return Err(error);
                }
                let session = runtime.insert(
                    client,
                    connection.id.clone(),
                    request.selection,
                    agent,
                    guard,
                )?;
                self.execution_state
                    .session_billing
                    .write()
                    .map_err(|_| denied())?
                    .insert(session.id.clone(), (client.into(), request.allow_paid));
                serde_json::to_value(session).map_err(|_| params::invalid())
            }
            "sessions.get" => {
                params::allow(&p, &["sessionId"])?;
                serde_json::to_value(self.check_session(
                    client,
                    params::text(&p, "sessionId")?.ok_or_else(params::invalid)?,
                )?)
                .map_err(|_| params::invalid())
            }
            "sessions.close" => {
                params::allow(&p, &["sessionId"])?;
                runtime
                    .close(
                        client,
                        params::text(&p, "sessionId")?.ok_or_else(params::invalid)?,
                    )
                    .await
            }
            "sessions.updateOptions" => {
                let request: Update = decode(p)?;
                let session = self.check_session(client, &request.session_id)?;
                let mut selected = session.selection;
                selected.options = request.options.clone();
                validate_selection(
                    &self.projected(client)?,
                    &selected,
                    self.execution_grant(client)?.allow_paid
                        && self.accepted_paid(client, &request.session_id)?,
                )?;
                serde_json::to_value(runtime.update_options(
                    client,
                    &request.session_id,
                    request.options,
                )?)
                .map_err(|_| params::invalid())
            }
            "turns.start" => {
                let request: Start = decode(p)?;
                let session = self.check_session(client, &request.session_id)?;
                validate_selection(
                    &self.projected(client)?,
                    &session.selection,
                    self.execution_grant(client)?.allow_paid
                        && self.accepted_paid(client, &request.session_id)?,
                )?;
                runtime
                    .start(
                        client,
                        &request.session_id,
                        request.prompt,
                        request.request_id,
                    )
                    .await
            }
            "turns.cancel" => {
                params::allow(&p, &["turnId"])?;
                runtime
                    .cancel(
                        client,
                        params::text(&p, "turnId")?.ok_or_else(params::invalid)?,
                    )
                    .await
            }
            "approvals.respond" => {
                params::allow(&p, &["approvalId", "allow"])?;
                let allow = p["allow"].as_bool().ok_or_else(params::invalid)?;
                runtime.approve(
                    client,
                    params::text(&p, "approvalId")?.ok_or_else(params::invalid)?,
                    allow,
                )
            }
            "events.read" => {
                params::allow(&p, &["sessionId", "after"])?;
                let after = params::text(&p, "after")?
                    .map(|s| s.parse::<u64>().map_err(|_| params::invalid()))
                    .transpose()?;
                runtime.events(client, params::text(&p, "sessionId")?, after)
            }
            _ => Err(unsupported()),
        }
    }
    pub fn revoke_execution(&self, client: &str) {
        self.execution_state.revoke(client);
    }
}
