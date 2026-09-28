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
    /// Validated host MCP servers attached to the thread.
    pub host_mcp_servers: Vec<HostMcpServer>,
    /// Non-ephemeral thread that `thread/resume` can reattach later.
    pub persistent: bool,
    /// Native thread id to reattach instead of starting a thread.
    pub resume_thread: Option<String>,
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
    let mut agent = codex::CodexAgent {
        peer,
        host: crate::host_mcp::HostTools::new(&config.host_mcp_servers),
        config,
        workspace,
        thread: None,
        resumable: false,
    };
    if agent.config.resume_thread.is_some() {
        // Reattach now so a missing thread fails the resume, not a later turn.
        match tokio::time::timeout(Duration::from_secs(25), agent.ensure_thread()).await {
            Ok(Ok(_)) => agent.resumable = true,
            Ok(Err(error)) if error.code == ErrorCode::Timeout => {
                agent.peer.shutdown().await;
                return Err(error);
            }
            Ok(Err(_)) => {
                agent.peer.shutdown().await;
                return Err(DomainError::new(
                    ErrorCode::ResumeUnavailable,
                    "Codex could not reattach the native thread",
                ));
            }
            Err(_) => {
                agent.peer.shutdown().await;
                return Err(failure(ErrorCode::Timeout));
            }
        }
    }
    Ok(Box::new(agent))
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
