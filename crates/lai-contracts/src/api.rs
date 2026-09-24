//! Canonical public method contracts, shared by the service and generated SDK types.
use crate::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct EmptyParams {}
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HandshakeParams {
    #[serde(default)]
    pub protocol_version: Option<String>,
    #[serde(default)]
    pub client_name: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct HandshakeResult {
    pub protocol_version: String,
    pub runtime_version: String,
    pub methods: Vec<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SnapshotParams {
    #[serde(default)]
    pub fields: Option<Vec<String>>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModelsParams {
    #[serde(default)]
    pub route_id: Option<String>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DiscoveryParams {
    #[serde(default)]
    pub level: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ConnectionParams {
    pub connection_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModelParams {
    pub model_id: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Requirements {
    #[serde(default)]
    pub required_capabilities: Vec<String>,
    #[serde(default)]
    pub allow_paid_fallback: bool,
    #[serde(default)]
    pub eligible_only: bool,
    #[serde(default)]
    pub billing_preference: Option<String>,
    #[serde(default)]
    pub query: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub selection: Selection,
    pub connection_id: String,
    pub eligible: bool,
    pub reasons: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelectionParams {
    pub selection: Selection,
    #[serde(default)]
    pub allow_paid: bool,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GrantParams {
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub connection_ids: Vec<String>,
    #[serde(default)]
    pub allow_paid: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SessionCreateParams {
    pub selection: Selection,
    pub workspace_id: String,
    #[serde(default)]
    pub permission_mode: Option<String>,
    #[serde(default)]
    pub review_requests: bool,
    #[serde(default)]
    pub allow_paid: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SessionParams {
    pub session_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SessionOptionsParams {
    pub session_id: String,
    pub options: BTreeMap<String, Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TurnStartParams {
    pub session_id: String,
    pub prompt: String,
    pub request_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TurnParams {
    pub turn_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ApprovalParams {
    pub approval_id: String,
    pub allow: bool,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct EventsParams {
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub after: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SubscriptionParams {
    pub subscription_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EventPage {
    pub events: Vec<Event>,
    pub cursor: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TurnStarted {
    pub turn_id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryResult {
    pub revision: String,
    pub mode: DataMode,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub valid: bool,
    pub mode: DataMode,
}

/// This is the superset of compiled public methods, not a grant or an assertion of live provider support.
#[derive(JsonSchema)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct MethodInputs {
    #[serde(rename = "runtime.handshake")]
    handshake: HandshakeParams,
    #[serde(rename = "runtime.health")]
    health: EmptyParams,
    #[serde(rename = "snapshot.get")]
    snapshot: SnapshotParams,
    #[serde(rename = "discovery.refresh")]
    discovery: DiscoveryParams,
    #[serde(rename = "connections.list")]
    connections: EmptyParams,
    #[serde(rename = "connections.probe")]
    probe: ConnectionParams,
    #[serde(rename = "models.list")]
    models: ModelsParams,
    #[serde(rename = "usage.read")]
    usage: EmptyParams,
    #[serde(rename = "capabilities.read")]
    capabilities: ModelParams,
    #[serde(rename = "selection.resolve")]
    resolve: Requirements,
    #[serde(rename = "selection.validate")]
    validate: SelectionParams,
    #[serde(rename = "grants.request")]
    grant_request: GrantParams,
    #[serde(rename = "grants.list")]
    grants: EmptyParams,
    #[serde(rename = "grants.revoke")]
    revoke: EmptyParams,
    #[serde(rename = "sessions.create")]
    create: SessionCreateParams,
    #[serde(rename = "sessions.get")]
    session: SessionParams,
    #[serde(rename = "sessions.close")]
    close: SessionParams,
    #[serde(rename = "sessions.updateOptions")]
    options: SessionOptionsParams,
    #[serde(rename = "turns.start")]
    start: TurnStartParams,
    #[serde(rename = "turns.cancel")]
    cancel: TurnParams,
    #[serde(rename = "approvals.respond")]
    approve: ApprovalParams,
    #[serde(rename = "events.read")]
    events: EventsParams,
    #[serde(rename = "events.subscribe")]
    subscribe: EventsParams,
    #[serde(rename = "events.unsubscribe")]
    unsubscribe: SubscriptionParams,
    #[serde(rename = "adapters.list")]
    adapters: EmptyParams,
    #[serde(rename = "diagnostics.report")]
    diagnostics: EmptyParams,
}
#[derive(JsonSchema)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub struct MethodOutputs {
    #[serde(rename = "runtime.handshake")]
    handshake: HandshakeResult,
    #[serde(rename = "runtime.health")]
    health: RuntimeHealth,
    #[serde(rename = "snapshot.get")]
    snapshot: Snapshot,
    #[serde(rename = "discovery.refresh")]
    discovery: DiscoveryResult,
    #[serde(rename = "connections.list")]
    connections: Vec<Connection>,
    #[serde(rename = "connections.probe")]
    probe: Value,
    #[serde(rename = "models.list")]
    models: Vec<Model>,
    #[serde(rename = "usage.read")]
    usage: Vec<QuotaPool>,
    #[serde(rename = "capabilities.read")]
    capabilities: Vec<Capability>,
    #[serde(rename = "selection.resolve")]
    resolve: Vec<Candidate>,
    #[serde(rename = "selection.validate")]
    validate: ValidationResult,
    #[serde(rename = "grants.request")]
    grant_request: Value,
    #[serde(rename = "grants.list")]
    grants: Value,
    #[serde(rename = "grants.revoke")]
    revoke: Value,
    #[serde(rename = "sessions.create")]
    create: Session,
    #[serde(rename = "sessions.get")]
    session: Session,
    #[serde(rename = "sessions.close")]
    close: Value,
    #[serde(rename = "sessions.updateOptions")]
    options: Session,
    #[serde(rename = "turns.start")]
    start: TurnStarted,
    #[serde(rename = "turns.cancel")]
    cancel: Value,
    #[serde(rename = "approvals.respond")]
    approve: Value,
    #[serde(rename = "events.read")]
    events: EventPage,
    #[serde(rename = "events.subscribe")]
    subscribe: Value,
    #[serde(rename = "events.unsubscribe")]
    unsubscribe: Value,
    #[serde(rename = "adapters.list")]
    adapters: Vec<AdapterDescriptor>,
    #[serde(rename = "diagnostics.report")]
    diagnostics: Value,
}
