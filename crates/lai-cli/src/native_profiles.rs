//! Explicit host-owned native-client registration. The file contains paths and labels, never tokens.
use lai_adapters::native_clients::config::NativeRegistry;
use lai_contracts::*;
use std::{
    io::Read,
    path::{Path, PathBuf},
};
pub fn load(path: Option<&Path>) -> Result<NativeRegistry> {
    let Some(path) = path else {
        return Ok(NativeRegistry::default());
    };
    let file = std::fs::File::open(path).map_err(|_| invalid())?;
    if !file.metadata().map_err(|_| invalid())?.is_file() {
        return Err(invalid());
    }
    let mut bytes = Vec::new();
    file.take(65537)
        .read_to_end(&mut bytes)
        .map_err(|_| invalid())?;
    if bytes.len() > 65536 {
        return Err(invalid());
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| invalid())?;
    let scope = stable_id("native-profile-source", &[&path.to_string_lossy()]);
    NativeRegistry::parse(text, &scope)
}
pub fn packaged_browser_blocker() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let parent = exe.parent()?;
    let name = if cfg!(windows) {
        "local-ai-kit-auth-blocker.exe"
    } else {
        "local-ai-kit-auth-blocker"
    };
    let candidate = parent.join(name);
    candidate
        .metadata()
        .ok()
        .filter(|m| m.is_file())
        .map(|_| candidate)
}
fn invalid() -> DomainError {
    DomainError::new(
        ErrorCode::InvalidRequest,
        "Native profile configuration could not be loaded",
    )
}
