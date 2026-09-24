//! Codex app-server metadata. Native subscriptions are never converted to API keys.
mod metadata;
mod models;
mod usage;
use lai_contracts::{DomainError, ErrorCode, Result};
pub use metadata::parse_metadata;
use serde_json::Value;
fn invalid() -> DomainError {
    DomainError::new(
        ErrorCode::ProtocolIncompatible,
        "Codex returned unsupported metadata",
    )
}
fn text(value: &Value) -> Option<&str> {
    value
        .as_str()
        .filter(|s| !s.is_empty() && s.len() <= 512 && !s.chars().any(char::is_control))
}
fn required<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    text(&value[key]).ok_or_else(invalid)
}
mod launch;
pub use launch::{probe, resolve_executable};
