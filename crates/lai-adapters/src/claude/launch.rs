use crate::process::LaunchSpec;
use lai_contracts::*;
use lai_discovery::LocatedClient;
use std::{collections::BTreeMap, path::PathBuf};
pub fn launch(located: &LocatedClient, cwd: PathBuf) -> Result<LaunchSpec> {
    if located.provider_id != "claude_code" {
        return Err(DomainError::new(
            ErrorCode::CapabilityUnsupported,
            "Unsupported Claude route",
        ));
    }
    let executable = located
        .executable
        .clone()
        .filter(|p| {
            p.is_absolute()
                && p.is_file()
                && (!cfg!(windows) || p.extension().is_some_and(|v| v.eq_ignore_ascii_case("exe")))
        })
        .ok_or_else(|| {
            DomainError::new(
                ErrorCode::RuntimeMissing,
                "Native Claude executable required",
            )
        })?;
    let args = [
        "--safe-mode",
        "--restricted",
        "--tools",
        "",
        "--strict-mcp-config",
        "--no-chrome",
        "--disable-slash-commands",
        "--permission-mode",
        "dontAsk",
        "--permission-prompts",
        "none",
        "--no-session-persistence",
        "--print",
        "--input-format",
        "stream-json",
        "--output-format",
        "stream-json",
        "--include-partial-messages",
        "--verbose",
    ]
    .iter()
    .map(|s| (*s).into())
    .collect();
    Ok(LaunchSpec {
        executable,
        args,
        cwd,
        env: BTreeMap::from([
            (
                "CLAUDE_CONFIG_DIR".into(),
                located.profile_home.clone().into_os_string(),
            ),
            ("ENABLE_CLAUDEAI_MCP_SERVERS".into(), "false".into()),
            ("CLAUDE_CODE_AUTO_CONNECT_IDE".into(), "0".into()),
        ]),
    })
}
pub async fn probe(
    located: LocatedClient,
    connection: String,
    route: String,
) -> Result<ConnectionMetadata> {
    let dir = tempfile::tempdir().map_err(|_| {
        DomainError::new(
            ErrorCode::ProviderUnavailable,
            "Isolated workspace unavailable",
        )
    })?;
    super::client::probe_spec(launch(&located, dir.path().into())?, &connection, &route).await
}
