use crate::{NativeService, denied, params, unsupported};
use lai_contracts::{DomainError, ErrorCode, Result, Selection};
use lai_core::{ClientPolicy, Requirements, resolve, validate_selection};
use serde_json::{Value, json};

const METHODS: &[&str] = &[
    "runtime.handshake",
    "runtime.health",
    "snapshot.get",
    "connections.list",
    "models.list",
    "usage.read",
    "capabilities.read",
    "adapters.list",
    "diagnostics.report",
    "discovery.refresh",
    "selection.resolve",
    "selection.validate",
    "grants.request",
    "grants.list",
    "grants.revoke",
];
impl NativeService {
    pub(crate) fn dispatch(&self, client: &str, method: &str, p: &Value) -> Result<Value> {
        match method {
            "runtime.handshake" => {
                params::allow(p, &["protocolVersion", "clientName"])?;
                let version = params::text(p, "protocolVersion")?.unwrap_or("1.0");
                if version.split_once('.').is_none_or(|(major, minor)| {
                    major != "1" || minor.is_empty() || !minor.bytes().all(|b| b.is_ascii_digit())
                }) {
                    return Err(DomainError::new(
                        ErrorCode::ProtocolIncompatible,
                        "Protocol major version is not supported",
                    ));
                }
                params::text(p, "clientName")?;
                Ok(
                    json!({"protocolVersion":"1.0","runtimeVersion":env!("CARGO_PKG_VERSION"),"methods":METHODS}),
                )
            }
            "runtime.health" => {
                params::allow(p, &[])?;
                Ok(json!({"status":"ok","mode":"live","executionAvailable":false}))
            }
            "grants.request" => {
                params::allow(p, &["scopes", "connectionIds", "allowPaid"])?;
                // This is an approval request, never a permission mutation.
                Ok(json!({"status":"requires_host_approval"}))
            }
            "grants.list" => {
                params::allow(p, &[])?;
                let policy = self.policy(client)?;
                Ok(
                    json!({"allowPassiveDiscovery":policy.allow_passive_discovery,"connectionIds":policy.connection_ids,"revealIdentity":policy.reveal_identity,"execution":false}),
                )
            }
            "grants.revoke" => {
                params::allow(p, &[])?;
                let mut clients = self.clients.write().map_err(|_| denied())?;
                *clients.get_mut(client).ok_or_else(denied)? = ClientPolicy::denied();
                Ok(json!({"revoked":true}))
            }
            "adapters.list" => {
                params::allow(p, &[])?;
                Ok(
                    json!({"operation":"passive_inventory","liveQualified":false,"executionAvailable":false}),
                )
            }
            _ => self.read_data(client, method, p),
        }
    }
    fn read_data(&self, client: &str, method: &str, p: &Value) -> Result<Value> {
        let snapshot = self.projected(client)?;
        match method {
            "snapshot.get" => {
                params::allow(p, &["fields"])?;
                let data = serde_json::to_value(snapshot).map_err(|_| params::invalid())?;
                if let Some(fields) = p.get("fields") {
                    let fields = fields
                        .as_array()
                        .filter(|fields| fields.len() <= 32)
                        .ok_or_else(params::invalid)?;
                    let mut selected = serde_json::Map::new();
                    for field in fields {
                        let name = field.as_str().ok_or_else(params::invalid)?;
                        selected.insert(
                            name.to_owned(),
                            data.get(name).ok_or_else(params::invalid)?.clone(),
                        );
                    }
                    Ok(Value::Object(selected))
                } else {
                    Ok(data)
                }
            }
            "connections.list" => {
                params::allow(p, &[])?;
                Ok(json!(snapshot.connections))
            }
            "models.list" => {
                params::allow(p, &["routeId"])?;
                let route = params::text(p, "routeId")?;
                Ok(json!(
                    snapshot
                        .models
                        .into_iter()
                        .filter(|m| route.is_none_or(|id| id == m.route_id))
                        .collect::<Vec<_>>()
                ))
            }
            "usage.read" => {
                params::allow(p, &[])?;
                Ok(json!(snapshot.quota_pools))
            }
            "capabilities.read" => {
                params::allow(p, &["modelId"])?;
                let id = params::text(p, "modelId")?.ok_or_else(params::invalid)?;
                let model =
                    snapshot.models.iter().find(|m| m.id == id).ok_or_else(|| {
                        DomainError::new(ErrorCode::NotFound, "Model is unavailable")
                    })?;
                Ok(json!(model.capabilities))
            }
            "selection.resolve" => {
                let requirements: Requirements = params::decode(p.clone())?;
                Ok(json!(resolve(&snapshot, &requirements)))
            }
            "selection.validate" => {
                params::allow(p, &["selection", "allowPaid"])?;
                let selection: Selection =
                    params::decode(p.get("selection").ok_or_else(params::invalid)?.clone())?;
                validate_selection(&snapshot, &selection, params::boolean(p, "allowPaid")?)?;
                Ok(json!({"valid":true,"mode":snapshot.mode}))
            }
            "diagnostics.report" => {
                params::allow(p, &[])?;
                Ok(
                    json!({"mode":snapshot.mode,"connections":snapshot.connections.len(),"models":snapshot.models.len(),"quotaPools":snapshot.quota_pools.len(),"executionAvailable":false,"limitations":["Passive discovery does not verify authentication, plans, or available models."]}),
                )
            }
            _ => Err(unsupported()),
        }
    }
}
