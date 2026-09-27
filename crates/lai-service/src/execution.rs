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
    pub native_resume: Option<String>,
}
pub type ExecutionSource =
    Arc<dyn Fn(ExecutionInput) -> BoxFuture<'static, Box<dyn Agent>> + Send + Sync>;
#[derive(Clone)]
struct SessionContext {
    owner: String,
    lifetime: Arc<()>,
    workspace_id: String,
    workspace_path: PathBuf,
    permission_mode: Option<String>,
    review_requests: bool,
    allow_paid: bool,
    connection_id: String,
    account_id: Option<String>,
    route_id: String,
    billing: Option<BillingMode>,
}
#[derive(Clone)]
struct ResumeRecord {
    owner: String,
    native_session_id: String,
    connection_id: String,
    selection: Selection,
    context: SessionContext,
}
#[derive(Default)]
pub(crate) struct ExecutionState {
    pub providers: BTreeSet<String>,
    pub connections: BTreeSet<String>,
    pub source: Option<ExecutionSource>,
    pub runtime: Arc<Runtime>,
    pub grants: Arc<RwLock<BTreeMap<String, ExecutionGrant>>>,
    // A fresh identity for each grant lifetime. Publication holds a read lease;
    // revocation holds the write lease until sessions and tokens are removed.
    lifecycle: Arc<RwLock<BTreeMap<String, Arc<()>>>>,
    session_billing: RwLock<BTreeMap<String, (String, bool)>>,
    session_context: RwLock<BTreeMap<String, SessionContext>>,
    resumptions: RwLock<BTreeMap<String, ResumeRecord>>,
    inputs: std::sync::Mutex<crate::inputs::InputStore>,
}
impl ExecutionState {
    fn publish_resumption(&self, token: String, record: ResumeRecord) -> Result<()> {
        let lifetimes = self.lifecycle.read().map_err(|_| denied())?;
        if !lifetimes
            .get(&record.owner)
            .is_some_and(|current| Arc::ptr_eq(current, &record.context.lifetime))
        {
            return Err(denied());
        }
        self.resumptions
            .write()
            .map_err(|_| denied())?
            .insert(token, record);
        Ok(())
    }
    pub fn revoke(&self, client: &str) {
        let Ok(mut lifecycle) = self.lifecycle.write() else {
            return;
        };
        lifecycle.remove(client);
        if let Ok(mut grants) = self.grants.write() {
            grants.remove(client);
        }
        self.runtime.revoke(client);
        if let Ok(mut sessions) = self.session_billing.write() {
            sessions.retain(|_, (owner, _)| owner != client);
        }
        if let Ok(mut contexts) = self.session_context.write() {
            contexts.retain(|_, context| context.owner != client);
        }
        if let Ok(mut resumptions) = self.resumptions.write() {
            resumptions.retain(|_, record| record.owner != client);
        }
        if let Ok(mut inputs) = self.inputs.lock() {
            inputs.clear_owner(client);
        }
    }
    fn clear_session_inputs(&self, client: &str, session: &str) {
        if let Ok(mut inputs) = self.inputs.lock() {
            inputs.clear_session(client, session);
        }
    }
}
type Create = SessionCreateParams;
type Start = TurnStartParams;
type Update = SessionOptionsParams;
fn decode<T: serde::de::DeserializeOwned>(p: Value) -> Result<T> {
    serde_json::from_value(p).map_err(|_| params::invalid())
}
fn resume_token() -> Result<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|_| {
        DomainError::new(
            ErrorCode::ProviderUnavailable,
            "Secure resumption token generation failed",
        )
    })?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}
pub(crate) fn decorate(
    snapshot: &mut Snapshot,
    providers: &BTreeSet<String>,
    connections: &BTreeSet<String>,
) {
    for route in &mut snapshot.routes {
        if snapshot.connections.iter().any(|c| {
            c.id == route.connection_id
                && (providers.contains(&c.provider_id) || connections.contains(&c.id))
        }) {
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
    pub fn with_execution(self, providers: BTreeSet<String>, source: ExecutionSource) -> Self {
        self.with_execution_scoped(providers, BTreeSet::new(), source)
    }
    pub fn with_execution_scoped(
        mut self,
        providers: BTreeSet<String>,
        connections: BTreeSet<String>,
        source: ExecutionSource,
    ) -> Self {
        self.execution_state.providers = providers;
        self.execution_state.connections = connections;
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
        let mut lifecycle = self
            .execution_state
            .lifecycle
            .write()
            .map_err(|_| denied())?;
        let mut grants = self.execution_state.grants.write().map_err(|_| denied())?;
        if grants.contains_key(client) {
            return Err(DomainError::new(
                ErrorCode::Conflict,
                "Execution grant already exists",
            ));
        }
        grants.insert(client.into(), grant);
        lifecycle.insert(client.into(), Arc::new(()));
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
                "sessions.suspend",
                "sessions.resume",
                "sessions.updateOptions",
                "sessions.close",
                "inputs.put",
                "turns.start",
                "turns.cancel",
                "approvals.respond",
                "events.read",
                "events.subscribe",
                "events.unsubscribe",
            ]
        }
    }
    /// Runtime features that exist only when an execution driver is registered.
    pub(crate) fn execution_capabilities(&self) -> Vec<&'static str> {
        if self.execution_state.source.is_none() {
            vec![]
        } else {
            vec!["turn.input.image"]
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
    async fn open_session(
        &self,
        client: &str,
        request: Create,
        native_resume: Option<String>,
        expected_lifetime: Option<Arc<()>>,
    ) -> Result<Session> {
        let lifetime = self
            .execution_state
            .lifecycle
            .read()
            .map_err(|_| denied())?
            .get(client)
            .cloned()
            .ok_or_else(denied)?;
        if expected_lifetime
            .as_ref()
            .is_some_and(|expected| !Arc::ptr_eq(expected, &lifetime))
        {
            return Err(denied());
        }
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
            .find(|route| route.id == model.route_id)
            .ok_or_else(denied)?;
        let connection = snapshot
            .connections
            .iter()
            .find(|connection| connection.id == route.connection_id)
            .ok_or_else(denied)?;
        if !grant.allows(connection)
            || !(self
                .execution_state
                .providers
                .contains(&connection.provider_id)
                || self.execution_state.connections.contains(&connection.id))
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
            native_resume,
        };
        let clients = self.clients.clone();
        let grants = self.execution_state.grants.clone();
        let cache = self.cache.clone();
        let providers = self.execution_state.providers.clone();
        let execution_connections = self.execution_state.connections.clone();
        let owner = client.to_owned();
        let selected = request.selection.clone();
        let connection_id = connection.id.clone();
        let workspace_id = request.workspace_id.clone();
        let paid = request.allow_paid;
        let native_policy = request.permission_mode.as_deref() == Some("native_policy");
        let expected_account = connection.account_id.clone();
        let expected_billing = route.billing.value().cloned();
        let guard_workspace = workspace.clone();
        let lifecycle = self.execution_state.lifecycle.clone();
        let guard_lifetime = lifetime.clone();
        let guard = Arc::new(move || {
            let lifetimes = lifecycle.read().map_err(|_| denied())?;
            if !lifetimes
                .get(&owner)
                .is_some_and(|current| Arc::ptr_eq(current, &guard_lifetime))
            {
                return Err(denied());
            }
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
            decorate(&mut snapshot, &providers, &execution_connections);
            let current_connection = snapshot
                .connections
                .iter()
                .find(|current| current.id == connection_id)
                .ok_or_else(denied)?;
            if !grant.allows(current_connection)
                || grant.workspaces.get(&workspace_id) != Some(&guard_workspace)
                || guard_workspace.canonicalize().ok().as_ref() != Some(&guard_workspace)
            {
                return Err(denied());
            }
            if current_connection.account_id != expected_account
                || (paid && !grant.allow_paid)
                || (native_policy && !grant.allow_native_policy)
            {
                return Err(denied());
            }
            let current = snapshot
                .models
                .iter()
                .find(|model| model.id == selected.model_id && model.route_id == selected.route_id)
                .ok_or_else(denied)?;
            let current_route = snapshot
                .routes
                .iter()
                .find(|current| current.id == selected.route_id)
                .ok_or_else(denied)?;
            if current_connection.auth.value() != Some(&AuthState::Authenticated)
                && !lai_core::is_accountless_local_route(current_route, current_connection)
            {
                return Err(denied());
            }
            if current.revision != selected.revision
                || current_route.billing.value() != expected_billing.as_ref()
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
        .map_err(|_| DomainError::new(ErrorCode::Timeout, "Native session startup timed out"))??;
        if let Err(error) = guard() {
            let _ = tokio::time::timeout(Duration::from_secs(2), agent.close()).await;
            return Err(error);
        }
        let mut unpublished = Some(agent);
        let result = (|| {
            let lifetimes = self
                .execution_state
                .lifecycle
                .read()
                .map_err(|_| denied())?;
            if !lifetimes
                .get(client)
                .is_some_and(|current| Arc::ptr_eq(current, &lifetime))
            {
                return Err(denied());
            }
            let context = SessionContext {
                owner: client.into(),
                lifetime: lifetime.clone(),
                workspace_id: request.workspace_id.clone(),
                workspace_path: workspace,
                permission_mode: request.permission_mode.clone(),
                review_requests: request.review_requests,
                allow_paid: request.allow_paid,
                connection_id: connection.id.clone(),
                account_id: connection.account_id.clone(),
                route_id: route.id.clone(),
                billing: route.billing.value().cloned(),
            };
            let session = self.execution_state.runtime.insert(
                client,
                connection.id.clone(),
                request.selection.clone(),
                unpublished.take().ok_or_else(denied)?,
                guard,
            )?;
            self.execution_state
                .session_billing
                .write()
                .map_err(|_| denied())?
                .insert(session.id.clone(), (client.into(), request.allow_paid));
            self.execution_state
                .session_context
                .write()
                .map_err(|_| denied())?
                .insert(session.id.clone(), context);
            Ok(session)
        })();
        // The lease has been released before asynchronous provider cleanup. Some
        // remote session adapters require close(), not merely dropping a process.
        if let Some(mut agent) = unpublished {
            let _ = tokio::time::timeout(Duration::from_secs(2), agent.close()).await;
        }
        result
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
                serde_json::to_value(self.open_session(client, request, None, None).await?)
                    .map_err(|_| params::invalid())
            }
            "sessions.get" => {
                params::allow(&p, &["sessionId"])?;
                serde_json::to_value(self.check_session(
                    client,
                    params::text(&p, "sessionId")?.ok_or_else(params::invalid)?,
                )?)
                .map_err(|_| params::invalid())
            }
            "sessions.suspend" => {
                let request: SessionParams = decode(p)?;
                let session = self.check_session(client, &request.session_id)?;
                if !session.native_resumption {
                    return Err(DomainError::new(
                        ErrorCode::CapabilityUnsupported,
                        "Selected native route does not support session resumption",
                    ));
                }
                let context = self
                    .execution_state
                    .session_context
                    .read()
                    .map_err(|_| denied())?
                    .get(&request.session_id)
                    .filter(|context| context.owner == client)
                    .cloned()
                    .ok_or_else(denied)?;
                let token = resume_token()?;
                {
                    let resumptions = self
                        .execution_state
                        .resumptions
                        .read()
                        .map_err(|_| denied())?;
                    if resumptions.len() >= 128
                        || resumptions
                            .values()
                            .filter(|record| record.owner == client)
                            .count()
                            >= 16
                        || resumptions.contains_key(&token)
                    {
                        return Err(DomainError::new(
                            ErrorCode::RateLimited,
                            "Native resumption capacity reached",
                        ));
                    }
                }
                let native_session_id = runtime.suspend(client, &request.session_id).await?;
                self.execution_state.publish_resumption(
                    token.clone(),
                    ResumeRecord {
                        owner: client.into(),
                        native_session_id,
                        connection_id: session.connection_id,
                        selection: session.selection,
                        context,
                    },
                )?;
                self.execution_state
                    .session_billing
                    .write()
                    .map_err(|_| denied())?
                    .remove(&request.session_id);
                self.execution_state
                    .session_context
                    .write()
                    .map_err(|_| denied())?
                    .remove(&request.session_id);
                self.execution_state
                    .clear_session_inputs(client, &request.session_id);
                serde_json::to_value(SessionSuspendResult {
                    resume_token: token,
                })
                .map_err(|_| params::invalid())
            }
            "sessions.resume" => {
                let request: SessionResumeParams = decode(p)?;
                if request.resume_token.len() != 64
                    || !request
                        .resume_token
                        .bytes()
                        .all(|byte| byte.is_ascii_hexdigit())
                {
                    return Err(params::invalid());
                }
                let record = {
                    let mut resumptions = self
                        .execution_state
                        .resumptions
                        .write()
                        .map_err(|_| denied())?;
                    if !resumptions
                        .get(&request.resume_token)
                        .is_some_and(|record| record.owner == client)
                    {
                        return Err(DomainError::new(
                            ErrorCode::NotFound,
                            "Resume token is unavailable",
                        ));
                    }
                    resumptions.remove(&request.resume_token).ok_or_else(|| {
                        DomainError::new(ErrorCode::NotFound, "Resume token is unavailable")
                    })?
                };
                let create = SessionCreateParams {
                    selection: record.selection.clone(),
                    workspace_id: record.context.workspace_id.clone(),
                    permission_mode: record.context.permission_mode.clone(),
                    review_requests: record.context.review_requests,
                    allow_paid: record.context.allow_paid,
                };
                let snapshot = self.projected(client)?;
                let grant = self.execution_grant(client)?;
                let connection = snapshot
                    .connections
                    .iter()
                    .find(|item| item.id == record.connection_id)
                    .ok_or_else(denied)?;
                let route = snapshot
                    .routes
                    .iter()
                    .find(|item| item.id == record.context.route_id)
                    .ok_or_else(|| {
                        DomainError::new(
                            ErrorCode::SessionRouteMismatch,
                            "Native session route changed",
                        )
                    })?;
                let model = snapshot
                    .models
                    .iter()
                    .find(|item| {
                        item.id == record.selection.model_id
                            && item.route_id == record.selection.route_id
                    })
                    .ok_or_else(|| {
                        DomainError::new(ErrorCode::SelectionStale, "Native session model changed")
                    })?;
                if connection.account_id != record.context.account_id
                    || route.connection_id != record.context.connection_id
                    || model.revision != record.selection.revision
                    || route.billing.value().cloned() != record.context.billing
                {
                    return Err(DomainError::new(
                        ErrorCode::SessionRouteMismatch,
                        "Native session identity changed",
                    ));
                }
                if grant.workspaces.get(&record.context.workspace_id)
                    != Some(&record.context.workspace_path)
                    || record.context.workspace_path.canonicalize().ok().as_ref()
                        != Some(&record.context.workspace_path)
                {
                    return Err(denied());
                }
                let session = self
                    .open_session(
                        client,
                        create,
                        Some(record.native_session_id),
                        Some(record.context.lifetime),
                    )
                    .await?;
                if session.connection_id != record.connection_id || !session.native_resumption {
                    let _ = runtime.close(client, &session.id).await;
                    return Err(DomainError::new(
                        ErrorCode::SessionRouteMismatch,
                        "Native session resumed on a different route",
                    ));
                }
                serde_json::to_value(session).map_err(|_| params::invalid())
            }
            "sessions.close" => {
                params::allow(&p, &["sessionId"])?;
                let session_id = params::text(&p, "sessionId")?.ok_or_else(params::invalid)?;
                let result = runtime.close(client, session_id).await;
                if result.is_ok() {
                    if let Ok(mut billing) = self.execution_state.session_billing.write() {
                        billing.remove(session_id);
                    }
                    if let Ok(mut contexts) = self.execution_state.session_context.write() {
                        contexts.remove(session_id);
                    }
                    self.execution_state
                        .clear_session_inputs(client, session_id);
                }
                result
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
            "inputs.put" => {
                let request: InputPutParams = decode(p)?;
                self.check_session(client, &request.session_id)?;
                let image = crate::inputs::decode_image(&request)?;
                let result = self
                    .execution_state
                    .inputs
                    .lock()
                    .map_err(|_| denied())?
                    .put(client, &request.session_id, image)?;
                serde_json::to_value(result).map_err(|_| params::invalid())
            }
            "turns.start" => {
                let request: Start = decode(p)?;
                let session = self.check_session(client, &request.session_id)?;
                let snapshot = self.projected(client)?;
                let model = validate_selection(
                    &snapshot,
                    &session.selection,
                    self.execution_grant(client)?.allow_paid
                        && self.accepted_paid(client, &request.session_id)?,
                )?;
                let images = if request.inputs.is_empty() {
                    Vec::new()
                } else {
                    let images = self
                        .execution_state
                        .inputs
                        .lock()
                        .map_err(|_| denied())?
                        .resolve(client, &request.session_id, &request.inputs)?;
                    // Only provider-reported model support counts; unknown is not supported.
                    if !model
                        .capabilities
                        .iter()
                        .any(|cap| cap.id == "input.image" && cap.support == Support::Supported)
                    {
                        return Err(DomainError::new(
                            ErrorCode::CapabilityUnsupported,
                            "Selected model does not report image input support",
                        ));
                    }
                    images
                };
                let started = runtime
                    .start_with_inputs(
                        client,
                        &request.session_id,
                        request.prompt,
                        request.request_id,
                        images,
                    )
                    .await?;
                if !request.inputs.is_empty() {
                    if let Ok(mut inputs) = self.execution_state.inputs.lock() {
                        inputs.consume(&request.inputs);
                    }
                }
                Ok(started)
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

#[cfg(test)]
mod lifecycle_tests {
    use super::*;
    fn record(lifetime: Arc<()>) -> ResumeRecord {
        ResumeRecord {
            owner: "app".into(),
            native_session_id: "private".into(),
            connection_id: "c".into(),
            selection: Selection {
                route_id: "r".into(),
                model_id: "m".into(),
                revision: "1".into(),
                options: Default::default(),
            },
            context: SessionContext {
                owner: "app".into(),
                lifetime,
                workspace_id: "w".into(),
                workspace_path: std::env::temp_dir(),
                permission_mode: None,
                review_requests: false,
                allow_paid: false,
                connection_id: "c".into(),
                account_id: None,
                route_id: "r".into(),
                billing: Some(BillingMode::Local),
            },
        }
    }
    #[test]
    fn revoke_between_native_suspend_and_token_publication_cannot_revive_old_access() {
        let state = ExecutionState::default();
        let original = Arc::new(());
        state
            .lifecycle
            .write()
            .unwrap()
            .insert("app".into(), original.clone());
        // Model a completed native suspend whose service publication is still pending.
        let pending = record(original);
        state.revoke("app");
        state
            .lifecycle
            .write()
            .unwrap()
            .insert("app".into(), Arc::new(()));
        assert_eq!(
            state
                .publish_resumption("old-token".into(), pending)
                .unwrap_err()
                .code,
            ErrorCode::LocalAccessDenied
        );
        assert!(state.resumptions.read().unwrap().is_empty());
        let current = state.lifecycle.read().unwrap()["app"].clone();
        state
            .publish_resumption("new-token".into(), record(current))
            .unwrap();
        state.revoke("app");
        assert!(state.resumptions.read().unwrap().is_empty());
    }
}
