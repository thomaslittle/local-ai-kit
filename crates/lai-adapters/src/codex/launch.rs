use crate::process::{LaunchSpec, codex_metadata};
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
    if !candidate.is_absolute() || !candidate.is_file() {
        return Err(missing());
    }
    let name = candidate.file_name().and_then(|n| n.to_str()).unwrap_or("");
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
