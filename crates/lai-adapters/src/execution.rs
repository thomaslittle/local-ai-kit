//! Native Codex execution: no token extraction, shell launch, or generation on construction.
mod codex;
use crate::process::{LaunchSpec, Peer, exchange};
use lai_contracts::*;
use lai_runtime::Agent;
use std::{path::PathBuf, time::Duration};
#[derive(Clone)]
pub struct CodexExecution {
    pub model: Model,
    pub connection: String,
    pub route: String,
    pub writable: bool,
    pub review_requests: bool,
}
pub async fn open_codex(spec: LaunchSpec, config: CodexExecution) -> Result<Box<dyn Agent>> {
    let workspace = spec.cwd.clone();
    let mut peer = Peer::spawn(spec).await?;
    let report = tokio::time::timeout(
        Duration::from_secs(20),
        exchange(&mut peer, &config.connection, &config.route),
    )
    .await
    .map_err(|_| failure(ErrorCode::Timeout))??;
    let model = report
        .models
        .iter()
        .find(|m| m.id == config.model.id)
        .ok_or_else(|| failure(ErrorCode::SelectionStale))?;
    if model.revision != config.model.revision
        || report.auth.fresh_value() != Some(&AuthState::Authenticated)
    {
        return Err(failure(ErrorCode::SelectionStale));
    }
    if report.included_usage_allowed.fresh_value() == Some(&false) {
        return Err(failure(ErrorCode::RateLimited));
    }
    Ok(Box::new(codex::CodexAgent {
        peer,
        config,
        workspace,
        thread: None,
    }))
}
fn failure(code: ErrorCode) -> DomainError {
    DomainError::new(code, "Native execution failed")
}
fn native_id(value: &serde_json::Value) -> Result<String> {
    value
        .as_str()
        .filter(|s| !s.is_empty() && s.len() <= 256 && !s.chars().any(char::is_control))
        .map(str::to_owned)
        .ok_or_else(|| failure(ErrorCode::ProtocolIncompatible))
}
