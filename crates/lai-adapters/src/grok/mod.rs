//! Grok native status and ACP catalog; no credential-file access or token export.
use crate::{
    acp::metadata::{self, AccountEvidence},
    process::{LaunchSpec, capture},
};
use lai_contracts::*;
use lai_discovery::LocatedClient;
use std::{collections::BTreeMap, path::PathBuf, time::Duration};
pub fn parse_status(output: &str) -> (Observation<AuthState>, Observation<BillingMode>) {
    let source = EvidenceSource::new(
        EvidenceKind::OfficialProtocol,
        "grok",
        "grok models login status",
    );
    let lower = output.to_ascii_lowercase();
    let auth = if lower.contains("not authenticated") || lower.contains("not logged in") {
        Observation::reported(AuthState::Unauthenticated, source)
    } else if lower
        .lines()
        .any(|line| line.trim() == "you are logged in with grok.com.")
    {
        Observation::reported(AuthState::Authenticated, source)
    } else {
        Observation::unknown("NATIVE_AUTH_STATUS_NOT_REPORTED")
    };
    (
        auth,
        Observation::unknown("NATIVE_MODEL_ROUTE_BILLING_NOT_REPORTED"),
    )
}
fn invalid() -> DomainError {
    DomainError::new(
        ErrorCode::IntegrationRestricted,
        "Grok requires its supported native executable and default local profile",
    )
}
/// The supported Grok executable for the default local profile, or an error.
pub fn supported_executable(located: &LocatedClient) -> Result<PathBuf> {
    let home = std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" })
        .map(PathBuf::from)
        .ok_or_else(invalid)?;
    if located.profile_home != home.join(".grok") {
        return Err(invalid());
    }
    let path = located
        .executable
        .as_ref()
        .filter(|p| p.is_absolute() && p.is_file())
        .ok_or_else(invalid)?;
    if cfg!(windows) && path.extension().and_then(|s| s.to_str()) != Some("exe") {
        return Err(invalid());
    }
    Ok(path.clone())
}
pub async fn probe(
    located: LocatedClient,
    connection: String,
    route: String,
) -> Result<ConnectionMetadata> {
    let executable = supported_executable(&located)?;
    let temp = tempfile::tempdir().map_err(|_| invalid())?;
    let output = capture(
        LaunchSpec {
            executable: executable.clone(),
            args: vec!["models".into()],
            cwd: temp.path().into(),
            env: BTreeMap::new(),
        },
        Duration::from_secs(8),
    )
    .await?;
    let (auth, billing) = parse_status(&output);
    if auth.fresh_value() != Some(&AuthState::Authenticated) {
        return Ok(ConnectionMetadata {
            connection_id: connection,
            route_id: route,
            version: Observation::unknown("NOT_PROBED"),
            auth,
            billing,
            plan: Observation::unknown("PLAN_NOT_REPORTED"),
            account: None,
            models: vec![],
            additional_routes: vec![],
            providers: vec![],
            quota_pools: vec![],
            included_usage_allowed: Observation::unknown("NOT_REPORTED"),
            diagnostics: vec!["NATIVE_AUTH_REQUIRED_OR_UNKNOWN".into()],
        });
    }
    let spec = LaunchSpec {
        executable,
        args: crate::native_clients::launch::GROK_TEXT_ONLY_ARGS
            .iter()
            .map(std::ffi::OsString::from)
            .chain([
                "--leader-socket".into(),
                temp.path().join("leader.sock").into(),
                "agent".into(),
                "stdio".into(),
            ])
            .collect(),
        cwd: temp.path().into(),
        env: BTreeMap::new(),
    };
    metadata::probe(
        spec,
        "grok",
        &connection,
        &route,
        AccountEvidence {
            auth,
            billing,
            plan: Observation::unknown("PLAN_NOT_REPORTED_BY_NATIVE_PROTOCOL"),
        },
    )
    .await
}
