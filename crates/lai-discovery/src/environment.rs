use lai_contracts::{DomainError, ErrorCode, Result};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostPlatform {
    Windows,
    Macos,
    Linux,
}
#[derive(Debug, Clone)]
pub struct ScanEnvironment {
    pub home: PathBuf,
    pub config_home: PathBuf,
    pub data_home: PathBuf,
    pub app_data: PathBuf,
    pub local_app_data: PathBuf,
    pub search_path: Vec<PathBuf>,
    pub overrides: BTreeMap<String, PathBuf>,
    pub platform: HostPlatform,
}
impl ScanEnvironment {
    pub fn for_home(home: PathBuf, platform: HostPlatform) -> Self {
        Self {
            config_home: home.join(".config"),
            data_home: home.join(".local/share"),
            app_data: home.join("AppData/Roaming"),
            local_app_data: home.join("AppData/Local"),
            home,
            search_path: vec![],
            overrides: BTreeMap::new(),
            platform,
        }
    }
    /// Only called by an explicit scan command. This does not read provider credential variables.
    pub fn from_process() -> Result<Self> {
        let platform = if cfg!(windows) {
            HostPlatform::Windows
        } else if cfg!(target_os = "macos") {
            HostPlatform::Macos
        } else {
            HostPlatform::Linux
        };
        let key = if platform == HostPlatform::Windows {
            "USERPROFILE"
        } else {
            "HOME"
        };
        let home = std::env::var_os(key).map(PathBuf::from).ok_or_else(|| {
            DomainError::new(
                ErrorCode::LocalAccessDenied,
                "Current user home is unavailable",
            )
        })?;
        let mut env = Self::for_home(home, platform);
        for (key, destination) in [
            ("XDG_CONFIG_HOME", &mut env.config_home),
            ("XDG_DATA_HOME", &mut env.data_home),
            ("APPDATA", &mut env.app_data),
            ("LOCALAPPDATA", &mut env.local_app_data),
        ] {
            if let Some(value) = std::env::var_os(key).filter(|v| !v.is_empty()) {
                *destination = expand(&env.home, PathBuf::from(value));
            }
        }
        if let Some(path) = std::env::var_os("PATH") {
            env.search_path = std::env::split_paths(&path)
                .take(128)
                .filter(|p| valid_root(p))
                .collect();
        }
        for key in [
            "CODEX_HOME",
            "CLAUDE_SECURESTORAGE_CONFIG_DIR",
            "CLAUDE_CONFIG_DIR",
            "OPENCODE_CONFIG_DIR",
        ] {
            if let Some(value) = std::env::var_os(key).filter(|v| !v.is_empty()) {
                env.overrides
                    .insert(key.into(), expand(&env.home, PathBuf::from(value)));
            }
        }
        env.validate()?;
        Ok(env)
    }
    pub fn validate(&self) -> Result<()> {
        if [
            &self.home,
            &self.config_home,
            &self.data_home,
            &self.app_data,
            &self.local_app_data,
        ]
        .into_iter()
        .chain(self.overrides.values())
        .any(|p| !valid_root(p))
        {
            return Err(DomainError::new(
                ErrorCode::InvalidRequest,
                "Scan roots must be absolute local paths",
            ));
        }
        if self.overrides.len() > 32 || self.search_path.len() > 128 {
            return Err(DomainError::new(
                ErrorCode::InvalidRequest,
                "Scan environment exceeds configured bounds",
            ));
        }
        Ok(())
    }
}
fn expand(home: &Path, path: PathBuf) -> PathBuf {
    if path == Path::new("~") {
        home.into()
    } else if let Ok(tail) = path.strip_prefix("~") {
        home.join(tail)
    } else {
        path
    }
}
pub(crate) fn valid_root(path: &Path) -> bool {
    let text = path.as_os_str().to_string_lossy();
    path.is_absolute()
        && !text.starts_with("//")
        && !text.starts_with(r"\\")
        && !text.chars().any(char::is_control)
}
