use crate::{HostPlatform, ScanEnvironment};
use std::path::PathBuf;
#[derive(Debug, Clone, Copy)]
pub struct ProviderDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub commands: &'static [&'static str],
    pub profile_prefix: Option<&'static str>,
}
pub fn definitions() -> &'static [ProviderDefinition] {
    &[
        ProviderDefinition {
            id: "codex",
            name: "Codex",
            commands: &["codex"],
            profile_prefix: Some(".codex-"),
        },
        ProviderDefinition {
            id: "claude_code",
            name: "Claude Code",
            commands: &["claude"],
            profile_prefix: Some(".claude-"),
        },
        ProviderDefinition {
            id: "gemini_cli",
            name: "Gemini CLI",
            commands: &["gemini"],
            profile_prefix: None,
        },
        ProviderDefinition {
            id: "antigravity",
            name: "Antigravity",
            commands: &["antigravity", "agy"],
            profile_prefix: None,
        },
        ProviderDefinition {
            id: "grok",
            name: "Grok",
            commands: &["grok"],
            profile_prefix: None,
        },
        ProviderDefinition {
            id: "opencode",
            name: "OpenCode",
            commands: &["opencode"],
            profile_prefix: None,
        },
        ProviderDefinition {
            id: "cursor",
            name: "Cursor",
            commands: &["cursor-agent"],
            profile_prefix: None,
        },
        ProviderDefinition {
            id: "ollama",
            name: "Ollama",
            commands: &["ollama"],
            profile_prefix: None,
        },
        ProviderDefinition {
            id: "lmstudio",
            name: "LM Studio",
            commands: &["lms"],
            profile_prefix: None,
        },
    ]
}
impl ProviderDefinition {
    pub(crate) fn roots(&self, env: &ScanEnvironment) -> Vec<PathBuf> {
        let overrides: &[&str] = match self.id {
            "codex" => &["CODEX_HOME"],
            "claude_code" => &["CLAUDE_SECURESTORAGE_CONFIG_DIR", "CLAUDE_CONFIG_DIR"],
            "opencode" => &["OPENCODE_CONFIG_DIR"],
            _ => &[],
        };
        if let Some(root) = overrides.iter().find_map(|key| env.overrides.get(*key)) {
            return vec![root.clone()];
        }
        match self.id {
            "codex" => vec![env.home.join(".codex")],
            "claude_code" => vec![env.home.join(".claude")],
            "gemini_cli" => vec![env.home.join(".gemini")],
            "antigravity" => vec![env.home.join(".gemini/antigravity")],
            "grok" => vec![env.home.join(".grok")],
            "opencode" => vec![
                env.config_home.join("opencode"),
                env.data_home.join("opencode"),
            ],
            "cursor" => vec![match env.platform {
                HostPlatform::Windows => env.app_data.join("Cursor"),
                HostPlatform::Macos => env.home.join("Library/Application Support/Cursor"),
                HostPlatform::Linux => env.config_home.join("Cursor"),
            }],
            "ollama" => vec![env.home.join(".ollama")],
            "lmstudio" => vec![env.home.join(".lmstudio")],
            _ => vec![],
        }
    }
    pub(crate) fn has_override(&self, env: &ScanEnvironment) -> bool {
        match self.id {
            "codex" => env.overrides.contains_key("CODEX_HOME"),
            "claude_code" => {
                env.overrides.contains_key("CLAUDE_CONFIG_DIR")
                    || env
                        .overrides
                        .contains_key("CLAUDE_SECURESTORAGE_CONFIG_DIR")
            }
            _ => false,
        }
    }
}
