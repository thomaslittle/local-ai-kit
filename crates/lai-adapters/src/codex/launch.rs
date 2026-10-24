use crate::process::{LaunchSpec, codex_managed_metadata, codex_metadata};
use lai_contracts::{ConnectionMetadata, DomainError, ErrorCode, Result};
use lai_discovery::LocatedClient;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    time::Duration,
};
fn missing() -> DomainError {
    DomainError::new(
        ErrorCode::RuntimeMissing,
        "A supported native Codex executable is unavailable",
    )
}

pub fn resolve_executable(candidate: &Path) -> Result<PathBuf> {
    if !candidate.is_absolute() {
        return Err(missing());
    }
    let name = candidate.file_name().and_then(|n| n.to_str()).unwrap_or("");
    // Check provider identity on the logical command, never a versioned target's name.
    // This is an executable candidate, not publisher/signature attestation.
    #[cfg(unix)]
    if std::fs::symlink_metadata(candidate).is_ok_and(|m| m.file_type().is_symlink()) {
        if name != "codex" {
            return Err(missing());
        }
        return lai_discovery::resolve_executable(candidate)
            .map(|identity| identity.resolved)
            .map_err(|_| missing());
    }
    if !candidate.is_file() {
        return Err(missing());
    }
    if name.eq_ignore_ascii_case("codex.exe") {
        return Ok(candidate.to_owned());
    }
    let wrapper = name.eq_ignore_ascii_case("codex.cmd")
        || name.eq_ignore_ascii_case("codex.ps1")
        || (cfg!(windows) && name == "codex");
    if wrapper {
        let base = candidate.parent().ok_or_else(missing)?;
        let (package, target) = if cfg!(target_arch = "aarch64") {
            ("codex-win32-arm64", "aarch64-pc-windows-msvc")
        } else {
            ("codex-win32-x64", "x86_64-pc-windows-msvc")
        };
        let roots = [
            base.join(format!(
                "node_modules/@openai/codex/node_modules/@openai/{package}"
            )),
            base.join(format!("node_modules/@openai/{package}")),
            base.join("node_modules/@openai/codex"),
        ];
        for root in roots {
            let binary = root.join(format!("vendor/{target}/bin/codex.exe"));
            if binary.is_file() {
                return Ok(binary);
            }
        }
        return Err(missing());
    }
    if name == "codex" {
        // Do not execute a JavaScript/shell wrapper as if it were a native Windows binary.
        #[cfg(windows)]
        return Err(missing());
        #[cfg(not(windows))]
        return Ok(candidate.to_owned());
    }
    Err(missing())
}

pub async fn probe(
    client: LocatedClient,
    connection: String,
    route: String,
) -> Result<ConnectionMetadata> {
    if client.provider_id != "codex" {
        return Err(DomainError::new(
            ErrorCode::CapabilityUnsupported,
            "Client metadata adapter is unavailable",
        ));
    }
    let executable = resolve_executable(client.executable.as_deref().ok_or_else(missing)?)?;
    client.validate_executable()?;
    if client
        .executable_identity
        .as_ref()
        .is_some_and(|identity| identity.resolved != executable)
    {
        return Err(missing());
    }
    let workspace = tempfile::Builder::new()
        .prefix("lai-codex-metadata-")
        .tempdir()
        .map_err(|_| {
            DomainError::new(
                ErrorCode::LocalAccessDenied,
                "Neutral probe workspace unavailable",
            )
        })?;
    let mut args: Vec<std::ffi::OsString> = vec!["app-server".into(), "--stdio".into()];
    // Invocation-only overrides: do not modify the client's saved settings.
    for feature in [
        "hooks",
        "plugins",
        "remote_plugin",
        "apps",
        "code_mode_host",
        "shell_snapshot",
    ] {
        args.extend(["--disable".into(), feature.into()]);
    }
    let env = BTreeMap::from([("CODEX_HOME".into(), client.profile_home.into_os_string())]);
    codex_metadata(
        LaunchSpec {
            executable,
            args,
            cwd: workspace.path().into(),
            env,
        },
        &connection,
        &route,
        Duration::from_secs(20),
    )
    .await
}

pub async fn managed_metadata(
    spec: LaunchSpec,
    connection: &str,
    route: &str,
    identity: super::ManagedIdentity,
    deadline: Duration,
) -> Result<ConnectionMetadata> {
    codex_managed_metadata(spec, connection, route, identity, deadline).await
}

/// Invocation-only SIWC provider settings. No token or provider setting is
/// written to the user's normal Codex profile.
pub fn managed_launch_spec(
    candidate: &Path,
    profile_home: &Path,
    workspace: &Path,
    access_token: &str,
) -> Result<LaunchSpec> {
    if !profile_home.is_absolute()
        || !workspace.is_absolute()
        || access_token.is_empty()
        || access_token.chars().any(char::is_control)
    {
        return Err(DomainError::new(
            ErrorCode::InvalidRequest,
            "Invalid managed Codex launch",
        ));
    }
    let executable = resolve_executable(candidate)?;
    let mut args: Vec<std::ffi::OsString> =
        vec!["app-server".into(), "--listen".into(), "stdio://".into()];
    for feature in [
        "hooks",
        "plugins",
        "remote_plugin",
        "apps",
        "code_mode_host",
        "shell_snapshot",
    ] {
        args.extend(["--disable".into(), feature.into()]);
    }
    for setting in [
        "model_provider=\"openai_chatgpt_plan\"",
        "model_providers.openai_chatgpt_plan.name=\"ChatGPT plan\"",
        "model_providers.openai_chatgpt_plan.base_url=\"https://api.openai.com/v1\"",
        "model_providers.openai_chatgpt_plan.env_key=\"ACCESS_TOKEN\"",
        "model_providers.openai_chatgpt_plan.wire_api=\"responses\"",
        "model_providers.openai_chatgpt_plan.requires_openai_auth=false",
        "model_providers.openai_chatgpt_plan.supports_websockets=false",
    ] {
        args.extend(["-c".into(), setting.into()]);
    }
    Ok(LaunchSpec {
        executable,
        args,
        cwd: workspace.into(),
        env: BTreeMap::from([
            ("CODEX_HOME".into(), profile_home.as_os_str().to_owned()),
            ("ACCESS_TOKEN".into(), access_token.into()),
        ]),
    })
}
