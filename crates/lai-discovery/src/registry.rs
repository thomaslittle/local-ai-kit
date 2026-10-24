use crate::ScanEnvironment;
use std::path::PathBuf;
#[derive(Debug, Clone, Copy)]
pub struct ProviderDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub commands: &'static [&'static str],
    pub profile_prefix: Option<&'static str>,
    /// Core adapters stay in the catalog even when nothing is installed.
    /// Pinned Tokscale clients appear only after a profile directory is found.
    pub emit_when_absent: bool,
}
pub fn definitions() -> &'static [ProviderDefinition] {
    use std::sync::OnceLock;
    static ALL: OnceLock<Vec<ProviderDefinition>> = OnceLock::new();
    ALL.get_or_init(|| {
        let mut all = core_definitions().to_vec();
        all.extend(crate::catalog::provider_definitions());
        all
    })
    .as_slice()
}
fn core_definitions() -> &'static [ProviderDefinition] {
    &[
        ProviderDefinition {
            id: "codex",
            name: "Codex",
            commands: &["codex"],
            profile_prefix: Some(".codex-"),
            emit_when_absent: true,
        },
        ProviderDefinition {
            id: "claude_code",
            name: "Claude Code",
            commands: &["claude"],
            profile_prefix: Some(".claude-"),
            emit_when_absent: true,
        },
        ProviderDefinition {
            id: "gemini_cli",
            name: "Gemini CLI",
            commands: &["gemini"],
            profile_prefix: None,
            emit_when_absent: true,
        },
        ProviderDefinition {
            id: "antigravity",
            name: "Antigravity",
            commands: &["antigravity", "agy"],
            profile_prefix: None,
            emit_when_absent: true,
        },
        ProviderDefinition {
            id: "grok",
            name: "Grok",
            commands: &["grok"],
            profile_prefix: None,
            emit_when_absent: true,
        },
        ProviderDefinition {
            id: "opencode",
            name: "OpenCode",
            commands: &["opencode"],
            profile_prefix: None,
            emit_when_absent: true,
        },
        ProviderDefinition {
            id: "cursor",
            name: "Cursor",
            commands: &["cursor-agent"],
            profile_prefix: None,
            emit_when_absent: true,
        },
        ProviderDefinition {
            id: "github_copilot",
            name: "GitHub Copilot",
            commands: &["copilot"],
            profile_prefix: None,
            emit_when_absent: true,
        },
        ProviderDefinition {
            id: "ollama",
            name: "Ollama",
            commands: &["ollama"],
            profile_prefix: None,
            emit_when_absent: true,
        },
        ProviderDefinition {
            id: "lmstudio",
            name: "LM Studio",
            commands: &["lms"],
            profile_prefix: None,
            emit_when_absent: true,
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
