use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    InvalidRequest, LocalAccessDenied, AuthRequired, KeyringLocked, RuntimeMissing,
    IntegrationRestricted, ProtocolIncompatible, CapabilityUnknown, CapabilityUnsupported,
    OptionUnsupported, SelectionStale, RateLimited, BillingConsentRequired, ApprovalRequired,
    SessionRouteMismatch, EventGap, ProviderUnavailable, Timeout, Cancelled, Conflict,
    FrameTooLarge, NotFound, NotImplemented,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, thiserror::Error)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[error("{code:?}: {message}")]
pub struct DomainError { pub code: ErrorCode, pub message: String, pub retryable: bool }
impl DomainError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self { Self { code, message: message.into(), retryable: false } }
}
pub type Result<T> = std::result::Result<T, DomainError>;
