use lai_contracts::{DomainError, ErrorCode, Result};
use std::{ffi::OsString, path::PathBuf};
#[derive(Debug)]
pub enum Action {
    Help,
    Version,
    Scan,
    Serve,
}
pub struct Args {
    pub action: Action,
    pub home: Option<PathBuf>,
    pub allow_passive: bool,
    pub allow_codex_metadata: bool,
}
impl Args {
    pub fn parse(input: impl IntoIterator<Item = OsString>) -> Result<Self> {
        let mut input = input.into_iter();
        let command = input.next().unwrap_or_else(|| "--help".into());
        let action = match command.to_str() {
            Some("--help" | "-h") => Action::Help,
            Some("--version" | "-V") => Action::Version,
            Some("scan") => Action::Scan,
            Some("serve") => Action::Serve,
            _ => return Err(invalid()),
        };
        let mut home = None;
        let mut allow_passive = false;
        let mut allow_codex_metadata = false;
        let mut json = false;
        let mut stdio = false;
        while let Some(arg) = input.next() {
            match arg.to_str() {
                Some("--json") if matches!(action, Action::Scan) && !json => json = true,
                Some("--stdio") if matches!(action, Action::Serve) && !stdio => stdio = true,
                Some("--allow-passive-discovery")
                    if matches!(action, Action::Serve) && !allow_passive =>
                {
                    allow_passive = true
                }
                Some("--allow-codex-metadata")
                    if matches!(action, Action::Serve) && !allow_codex_metadata =>
                {
                    allow_codex_metadata = true;
                }
                Some("--home")
                    if matches!(action, Action::Scan | Action::Serve) && home.is_none() =>
                {
                    let path = PathBuf::from(input.next().ok_or_else(invalid)?);
                    let text = path.to_string_lossy();
                    if !path.is_absolute()
                        || text.starts_with(r"\\")
                        || text.starts_with("//")
                        || text.chars().any(char::is_control)
                    {
                        return Err(invalid());
                    }
                    home = Some(path);
                }
                _ => return Err(invalid()),
            }
        }
        if matches!(action, Action::Scan) && !json || matches!(action, Action::Serve) && !stdio {
            return Err(invalid());
        }
        Ok(Self {
            action,
            home,
            allow_passive,
            allow_codex_metadata,
        })
    }
}
fn invalid() -> DomainError {
    DomainError::new(
        ErrorCode::InvalidRequest,
        "Invalid command arguments. Run local-ai-kit --help.",
    )
}
