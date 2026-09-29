use crate::ScanEnvironment;
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
            id: "github_copilot",
            name: "GitHub Copilot",
            commands: &["copilot"],
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
        crate::knowledge::profile_roots(self.id, env)
    }
    pub(crate) fn has_override(&self, env: &ScanEnvironment) -> bool {
        crate::knowledge::has_profile_override(self.id, env)
    }
}
