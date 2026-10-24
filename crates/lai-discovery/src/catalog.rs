//! Every pinned Tokscale client that does not already have a hand-tuned adapter.
//!
//! The table is the committed snapshot, parsed once. A released build does not
//! fetch Tokscale. Clients already represented by Codex, Claude, Gemini,
//! Antigravity, OpenCode, Cursor, Copilot, Grok, or LM Studio are skipped so
//! those adapters keep their own roots. Tokscale's Cursor and Antigravity
//! caches stay skipped with them.
use crate::registry::ProviderDefinition;
use crate::{HostPlatform, ScanEnvironment, join_relative};
use serde_json::Value;
use std::{path::PathBuf, sync::OnceLock};

const SNAPSHOT: &str = include_str!("../../../upstream/tokscale/snapshot.json");

/// Snapshot clients whose profile roots are implemented on the existing adapters.
const HANDLED: &[&str] = &[
    "opencode",
    "claude",
    "codex",
    "cursor",
    "gemini",
    "copilot",
    "grok",
    "lmstudio",
    "antigravity",
    "antigravity-cli",
    "antigravity-extension",
];

const SESSION_TAILS: &[&str] = &[
    "agent",
    "agents",
    "cli",
    "conversations",
    "otel",
    "projects",
    "sessions",
    "stats",
    "tasks",
    "threads",
    "tmp",
    "usage",
];

#[derive(Clone, Copy)]
enum RootKind {
    Home,
    XdgData,
    Config,
    AppData,
    Reasonix,
    Env,
}

struct Client {
    id: &'static str,
    name: &'static str,
    kind: RootKind,
    env_var: &'static str,
    fallback: &'static str,
    /// Path under the root base. Empty means the base itself is the profile.
    profile: &'static str,
    session: &'static str,
}

struct Table {
    clients: Vec<Client>,
    env_vars: Vec<&'static str>,
}

fn leak(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

fn is_file_segment(segment: &str) -> bool {
    segment.contains('.') && !segment.starts_with('.')
}

/// Split a Tokscale session relative into the directory we probe and the
/// session leaf under it. The probed directory is an ancestor of the session
/// path, so a client is detected when its home exists.
fn split_relative(kind: &str, relative: &str) -> (String, String) {
    if kind == "env" || kind == "reasonix" {
        return (String::new(), relative.to_string());
    }
    if kind == "xdg-data" || kind == "app-data" {
        let first = relative.split('/').next().unwrap_or("");
        let session = relative
            .strip_prefix(first)
            .unwrap_or("")
            .trim_start_matches('/');
        return (first.to_string(), session.to_string());
    }
    let mut parts: Vec<&str> = relative.split('/').filter(|part| !part.is_empty()).collect();
    if parts.last().is_some_and(|part| is_file_segment(part)) {
        parts.pop();
    }
    while parts.len() > 1 && SESSION_TAILS.contains(&parts[parts.len() - 1]) {
        parts.pop();
    }
    if parts.len() >= 3 && parts[0] == "Library" && parts[1] == "Application Support" {
        parts.truncate(3);
    }
    let profile = parts.join("/");
    let session = if relative == profile {
        String::new()
    } else {
        relative
            .strip_prefix(&profile)
            .unwrap_or("")
            .trim_start_matches('/')
            .to_string()
    };
    (profile, session)
}

fn display_name(id: &str, enum_name: &str) -> String {
    match id {
        "zcode" => "z.ai".to_string(),
        "micode" => "MiMo Code".to_string(),
        "micode-desktop" => "MiMo Desktop".to_string(),
        _ => {
            let mut name = String::new();
            for (index, ch) in enum_name.chars().enumerate() {
                if index > 0 && ch.is_uppercase() {
                    let previous = enum_name.chars().nth(index - 1);
                    if previous.is_some_and(|item| item.is_lowercase()) {
                        name.push(' ');
                    }
                }
                name.push(ch);
            }
            name
        }
    }
}

fn table() -> &'static Table {
    static TABLE: OnceLock<Table> = OnceLock::new();
    TABLE.get_or_init(|| {
        let snapshot: Value = serde_json::from_str(SNAPSHOT).expect("pinned Tokscale snapshot");
        let mut clients = Vec::new();
        let mut env_vars = Vec::new();
        for client in snapshot["clients"].as_array().expect("snapshot clients") {
            let id = client["id"].as_str().expect("client id");
            if HANDLED.contains(&id) {
                continue;
            }
            let kind_name = client["root"]["kind"].as_str().expect("root kind");
            let relative = client["relative"].as_str().unwrap_or("");
            let (profile, session) = split_relative(kind_name, relative);
            let kind = match kind_name {
                "home" => RootKind::Home,
                "xdg-data" => RootKind::XdgData,
                "config" => RootKind::Config,
                "app-data" => RootKind::AppData,
                "reasonix" => RootKind::Reasonix,
                "env" => RootKind::Env,
                other => panic!("unsupported Tokscale root kind {other}"),
            };
            let env_var = client["root"]["var"].as_str().unwrap_or("");
            let fallback = client["root"]["fallback"].as_str().unwrap_or("");
            if kind_name == "env" && !env_var.is_empty() {
                env_vars.push(leak(env_var.to_string()));
            }
            let enum_name = client["enum"].as_str().unwrap_or(id);
            clients.push(Client {
                id: leak(id.to_string()),
                name: leak(display_name(id, enum_name)),
                kind,
                env_var: leak(env_var.to_string()),
                fallback: leak(fallback.to_string()),
                profile: leak(profile),
                session: leak(session),
            });
        }
        env_vars.sort_unstable();
        env_vars.dedup();
        Table { clients, env_vars }
    })
}

pub fn env_vars() -> &'static [&'static str] {
    table().env_vars.as_slice()
}

pub fn provider_definitions() -> Vec<ProviderDefinition> {
    table()
        .clients
        .iter()
        .map(|client| ProviderDefinition {
            id: client.id,
            name: client.name,
            commands: &[],
            profile_prefix: None,
            emit_when_absent: false,
        })
        .collect()
}

pub fn discovery_ids() -> Vec<(&'static str, &'static str)> {
    table()
        .clients
        .iter()
        .map(|client| (client.id, client.name))
        .collect()
}

fn find(provider_id: &str) -> Option<&'static Client> {
    table().clients.iter().find(|client| client.id == provider_id)
}

pub fn has_override(provider_id: &str, env: &ScanEnvironment) -> bool {
    find(provider_id).is_some_and(|client| {
        !client.env_var.is_empty() && env.overrides.contains_key(client.env_var)
    })
}

pub fn profile_roots(provider_id: &str, env: &ScanEnvironment) -> Vec<PathBuf> {
    let Some(client) = find(provider_id) else {
        return Vec::new();
    };
    let base = match client.kind {
        RootKind::Home => env.home.clone(),
        RootKind::XdgData => env.data_home.clone(),
        RootKind::Config => join_relative(&env.home, ".config/tokscale"),
        RootKind::AppData => env.app_data.clone(),
        RootKind::Reasonix => join_relative(&env.home, ".reasonix"),
        RootKind::Env => {
            if let Some(path) = env.overrides.get(client.env_var) {
                path.clone()
            } else {
                join_relative(&env.home, client.fallback)
            }
        }
    };
    let mut roots = vec![if client.profile.is_empty() {
        base
    } else {
        join_relative(&base, client.profile)
    }];
    if env.platform == HostPlatform::Windows {
        match client.id {
            "cline" => roots.push(join_relative(
                &env.app_data,
                "Code/User/globalStorage/saoudrizwan.claude-dev",
            )),
            "zed" => roots.push(join_relative(&env.local_app_data, "Zed")),
            "crush" => roots.push(join_relative(&env.local_app_data, "crush")),
            "hermes" => roots.push(join_relative(&env.local_app_data, "hermes")),
            "devin-desktop" => roots.push(join_relative(&env.app_data, "Devin")),
            "devin-cli" => roots.push(join_relative(&env.app_data, "devin")),
            "kiro" => roots.push(join_relative(
                &env.app_data,
                "Kiro/User/globalStorage/kiro.kiroagent",
            )),
            _ => {}
        }
    }
    if client.id == "devin-desktop" {
        roots.push(join_relative(&env.home, ".config/Devin"));
    }
    roots
}

pub fn session_stores(provider_id: &str, profile: &std::path::Path) -> Vec<PathBuf> {
    find(provider_id)
        .filter(|client| !client.session.is_empty())
        .map(|client| vec![join_relative(profile, client.session)])
        .unwrap_or_default()
}
