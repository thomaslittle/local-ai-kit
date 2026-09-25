use lai_contracts::{DomainError, ErrorCode, Result};
use std::{collections::BTreeSet, ffi::OsString, path::PathBuf};
#[derive(Debug)]
pub enum Action {
    Help,
    Version,
    Scan,
    Serve,
    Connect,
}
pub struct Args {
    pub action: Action,
    pub home: Option<PathBuf>,
    pub endpoints: Option<PathBuf>,
    pub allow_passive: bool,
    pub allow_codex_metadata: bool,
    pub http: bool,
    pub ipc: Option<String>,
    pub port: u16,
    pub assets: Option<PathBuf>,
    pub allow_execution: bool,
    pub workspace: Option<PathBuf>,
    pub allow_writes: bool,
    pub review_requests: bool,
    pub allow_paid: bool,
    pub metadata_providers: BTreeSet<String>,
    pub execution_providers: BTreeSet<String>,
}
fn absolute(value: OsString) -> Result<PathBuf> {
    let path = PathBuf::from(value);
    let text = path.to_string_lossy();
    if !path.is_absolute()
        || text.starts_with(r"\\")
        || text.starts_with("//")
        || text.chars().any(char::is_control)
    {
        Err(invalid())
    } else {
        Ok(path)
    }
}
impl Args {
    pub fn parse(input: impl IntoIterator<Item = OsString>) -> Result<Self> {
        let mut input = input.into_iter();
        let command = input.next().unwrap_or_else(|| "--help".into());
        if command == "connect" {
            let rest: Vec<_> = input.collect();
            if rest.len() != 2 || rest[0] != "--ipc" {
                return Err(invalid());
            }
            let mut args = Self::parse([OsString::from("serve"), OsString::from("--stdio")])?;
            args.action = Action::Connect;
            args.ipc = Some(rest[1].clone().into_string().map_err(|_| invalid())?);
            return Ok(args);
        }
        let action = match command.to_str() {
            Some("--help" | "-h") => Action::Help,
            Some("--version" | "-V") => Action::Version,
            Some("scan") => Action::Scan,
            Some("serve") => Action::Serve,
            _ => return Err(invalid()),
        };
        let mut args = Self {
            action,
            home: None,
            endpoints: None,
            allow_passive: false,
            allow_codex_metadata: false,
            http: false,
            ipc: None,
            port: 4499,
            assets: None,
            allow_execution: false,
            workspace: None,
            allow_writes: false,
            review_requests: false,
            allow_paid: false,
            metadata_providers: BTreeSet::new(),
            execution_providers: BTreeSet::new(),
        };
        let mut seen = BTreeSet::new();
        let (mut json, mut stdio) = (false, false);
        while let Some(flag) = input.next() {
            let flag = flag.to_str().ok_or_else(invalid)?;
            if !seen.insert(flag.to_owned())
                && !["--allow-metadata", "--allow-execution"].contains(&flag)
            {
                return Err(invalid());
            }
            match flag {
                "--json" if matches!(args.action, Action::Scan) => json = true,
                "--home" if matches!(args.action, Action::Scan | Action::Serve) => {
                    args.home = Some(absolute(input.next().ok_or_else(invalid)?)?)
                }
                "--endpoints" if matches!(args.action, Action::Serve | Action::Scan) => {
                    args.endpoints = Some(absolute(input.next().ok_or_else(invalid)?)?)
                }
                "--stdio" if matches!(args.action, Action::Serve) => stdio = true,
                "--http" if matches!(args.action, Action::Serve) => args.http = true,
                "--ipc" if matches!(args.action, Action::Serve) => {
                    args.ipc = Some(
                        input
                            .next()
                            .and_then(|s| s.into_string().ok())
                            .ok_or_else(invalid)?,
                    )
                }
                "--allow-passive-discovery" if matches!(args.action, Action::Serve) => {
                    args.allow_passive = true
                }
                "--allow-metadata" | "--allow-execution"
                    if matches!(args.action, Action::Serve) =>
                {
                    let provider = input
                        .next()
                        .and_then(|v| v.into_string().ok())
                        .filter(|v| {
                            [
                                "codex",
                                "claude_code",
                                "ollama",
                                "lmstudio",
                                "openai_api",
                                "anthropic_api",
                                "gemini_api",
                                "xai_api",
                                "compatible_api",
                                "opencode",
                            ]
                            .contains(&v.as_str())
                                || (flag == "--allow-metadata"
                                    && ["grok", "cursor"].contains(&v.as_str()))
                        })
                        .ok_or_else(invalid)?;
                    if flag == "--allow-metadata" {
                        args.metadata_providers.insert(provider);
                    } else {
                        args.execution_providers.insert(provider);
                        args.allow_execution = true;
                    }
                }
                "--allow-codex-metadata" if matches!(args.action, Action::Serve) => {
                    args.allow_codex_metadata = true
                }
                "--allow-codex-execution" if matches!(args.action, Action::Serve) => {
                    args.allow_execution = true
                }
                "--workspace" if matches!(args.action, Action::Serve) => {
                    args.workspace = Some(absolute(input.next().ok_or_else(invalid)?)?)
                }
                "--assets" if matches!(args.action, Action::Serve) => {
                    args.assets = Some(absolute(input.next().ok_or_else(invalid)?)?)
                }
                "--port" if matches!(args.action, Action::Serve) => {
                    args.port = input
                        .next()
                        .and_then(|s| s.to_str().and_then(|s| s.parse().ok()))
                        .ok_or_else(invalid)?
                }
                "--allow-writes" if matches!(args.action, Action::Serve) => {
                    args.allow_writes = true
                }
                "--review-native-requests" if matches!(args.action, Action::Serve) => {
                    args.review_requests = true
                }
                "--allow-paid-options" if matches!(args.action, Action::Serve) => {
                    args.allow_paid = true
                }
                _ => return Err(invalid()),
            }
        }
        if matches!(args.action, Action::Scan) && !json
            || matches!(args.action, Action::Serve)
                && (u8::from(stdio) + u8::from(args.http) + u8::from(args.ipc.is_some()) != 1)
        {
            return Err(invalid());
        }
        if args.allow_codex_metadata {
            args.metadata_providers.insert("codex".into());
        }
        if seen.contains("--allow-codex-execution") {
            args.execution_providers.insert("codex".into());
        }
        if (!args.http && (args.assets.is_some() || seen.contains("--port")))
            || (args.allow_execution
                && (args.workspace.is_none()
                    || !args.execution_providers.is_subset(&args.metadata_providers)))
            || (!args.allow_execution
                && (args.allow_writes
                    || args.review_requests
                    || args.allow_paid
                    || args.workspace.is_some()))
        {
            return Err(invalid());
        }
        Ok(args)
    }
}
fn invalid() -> DomainError {
    DomainError::new(
        ErrorCode::InvalidRequest,
        "Invalid command arguments. Run local-ai-kit --help.",
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    fn parse(args: &[&str]) -> Result<Args> {
        Args::parse(args.iter().map(OsString::from))
    }
    #[test]
    fn explicit_http_serving_and_assets_are_supported() {
        let directory = std::env::temp_dir();
        let path = directory.to_str().unwrap();
        assert!(
            parse(&[
                "serve",
                "--http",
                "--port",
                "0",
                "--assets",
                path,
                "--allow-passive-discovery"
            ])
            .is_ok()
        );
    }
    #[test]
    fn execution_needs_explicit_workspace_and_metadata_permission() {
        let directory = std::env::temp_dir();
        let path = directory.to_str().unwrap();
        assert!(
            parse(&[
                "serve",
                "--stdio",
                "--allow-codex-metadata",
                "--allow-codex-execution",
                "--workspace",
                path
            ])
            .is_ok()
        );
        assert!(parse(&["serve", "--stdio", "--allow-codex-execution"]).is_err());
    }
    #[test]
    fn unsafe_or_conflicting_flags_are_rejected() {
        for args in [
            vec!["serve", "--http", "--stdio"],
            vec!["serve", "--http", "--port", "99999"],
            vec!["serve", "--stdio", "--allow-writes"],
            vec!["serve", "--http", "--host", "0.0.0.0"],
        ] {
            assert!(parse(&args).is_err());
        }
    }
}

#[cfg(test)]
mod additional_provider_tests {
    use super::*;
    #[test]
    fn grok_metadata_is_explicit_and_does_not_imply_execution() {
        let args = Args::parse(
            ["serve", "--stdio", "--allow-metadata", "grok"]
                .into_iter()
                .map(OsString::from),
        )
        .unwrap();
        assert!(args.metadata_providers.contains("grok"));
        assert!(!args.allow_execution);
    }
}
