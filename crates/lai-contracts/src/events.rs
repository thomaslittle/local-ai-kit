use crate::{Selection, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SessionState {
    Ready,
    Running,
    AwaitingApproval,
    Completed,
    Error,
    Cancelled,
    Closed,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Session {
    pub id: String,
    pub connection_id: String,
    pub selection: Selection,
    pub state: SessionState,
    pub created_at: Timestamp,
    pub native_resumption: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(
    tag = "type",
    rename_all = "snake_case",
    rename_all_fields = "camelCase"
)]
pub enum EventPayload {
    State {
        state: SessionState,
    },
    TextDelta {
        text: String,
    },
    ReasoningSummary {
        text: String,
    },
    ApprovalRequest {
        approval_id: String,
        action: String,
        detail: String,
    },
    ApprovalResolved {
        approval_id: String,
        allowed: bool,
    },
    Tool {
        name: String,
        status: String,
    },
    Usage {
        unit: String,
        used: String,
    },
    Artifact {
        artifact_id: String,
        media_type: String,
        size: String,
    },
    OptionsChanged {
        revision: String,
    },
    Warning {
        code: String,
        message: String,
    },
    Error {
        code: String,
        message: String,
    },
    Final {
        status: String,
    },
    Extension {
        namespace: String,
        data: Value,
    },
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Event {
    pub session_id: String,
    pub turn_id: Option<String>,
    pub sequence: String,
    pub timestamp: Timestamp,
    pub adapter_id: String,
    pub schema_version: String,
    pub payload: EventPayload,
}
