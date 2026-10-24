//! Freebuff model ids from the local client catalog.
//!
//! Account files and session tokens are never opened. The catalog is the
//! `model:"vendor/name"` table embedded in the Freebuff executable, plus the
//! model id the live instance or settings file already selected.
use lai_contracts::*;
use lai_discovery::LocatedClient;
use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

const MAX_EXECUTABLE_BYTES: u64 = 256 * 1024 * 1024;
const MAX_CONFIG_BYTES: u64 = 16 * 1024;
const MAX_MODELS: usize = 64;

fn unavailable(message: &str) -> DomainError {
    DomainError::new(ErrorCode::ProviderUnavailable, message)
}

fn evidence() -> EvidenceSource {
    EvidenceSource::new(
        EvidenceKind::LocalConfig,
        "freebuff",
        "Freebuff local model catalog; account sign-in is not required",
    )
}

fn valid_model_id(value: &str) -> bool {
    let Some((vendor, name)) = value.split_once('/') else {
        return false;
    };
    !vendor.is_empty()
        && !name.is_empty()
        && vendor.len() <= 48
        && name.len() <= 80
        && value.len() <= 129
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '.' | '_' | '-' | '/'))
        && value.matches('/').count() == 1
}

/// Model ids quoted as `model:"vendor/name"` in a Freebuff catalog blob.
pub fn model_ids_in_catalog(text: &str) -> Vec<String> {
    let needle = "model:\"";
    let mut found = BTreeSet::new();
    let mut start = 0;
    while let Some(offset) = text[start..].find(needle) {
        let value_at = start + offset + needle.len();
        let Some(end) = text[value_at..].find('"') else {
            break;
        };
        let id = &text[value_at..value_at + end];
        if valid_model_id(id) {
            found.insert(id.to_string());
        }
        start = value_at + end + 1;
        if found.len() > MAX_MODELS {
            break;
        }
    }
    found.into_iter().take(MAX_MODELS).collect()
}

fn json_string_field(path: &Path, field: &str) -> Option<String> {
    let info = path.symlink_metadata().ok()?;
    if !info.is_file() || info.file_type().is_symlink() || info.len() > MAX_CONFIG_BYTES {
        return None;
    }
    let text = fs::read_to_string(path).ok()?;
    let value: Value = serde_json::from_str(&text).ok()?;
    let field = value.get(field)?.as_str()?.trim();
    (!field.is_empty() && field.len() <= 128 && !field.chars().any(char::is_control))
        .then(|| field.to_string())
}

fn executable(home: &Path) -> Option<PathBuf> {
    ["freebuff.exe", "freebuff"]
        .into_iter()
        .map(|name| home.join(name))
        .find(|path| {
            path.symlink_metadata().is_ok_and(|info| {
                info.is_file() && !info.file_type().is_symlink() && info.len() <= MAX_EXECUTABLE_BYTES
            })
        })
}

fn catalog_models(home: &Path) -> (Vec<String>, bool) {
    let Some(path) = executable(home) else {
        return (Vec::new(), false);
    };
    let bytes = fs::read(&path).unwrap_or_default();
    if bytes.len() as u64 > MAX_EXECUTABLE_BYTES {
        return (Vec::new(), true);
    }
    let text = String::from_utf8_lossy(&bytes);
    let models = model_ids_in_catalog(&text);
    let truncated = models.len() == MAX_MODELS;
    (models, truncated)
}

fn selected_model(home: &Path) -> Option<String> {
    let mut selected = None;
    if let Ok(entries) = fs::read_dir(home) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with("freebuff-live-") && name.ends_with(".json") {
                if let Some(model) = json_string_field(&entry.path(), "model") {
                    if valid_model_id(&model) {
                        selected = Some(model);
                    }
                }
            }
        }
    }
    selected
}

fn default_marker(home: &Path) -> Option<String> {
    json_string_field(&home.join("settings.json"), "freebuffModelDefaultMigration")
}

fn version(home: &Path) -> Observation<String> {
    json_string_field(&home.join("freebuff-metadata.json"), "version")
        .map(|value| Observation::reported(value, evidence()))
        .unwrap_or_else(|| Observation::unknown("FREEBUFF_VERSION_NOT_REPORTED"))
}

pub fn read_catalog(
    located: &LocatedClient,
    connection: &str,
    route: &str,
) -> Result<ConnectionMetadata> {
    if located.provider_id != "freebuff" || !located.profile_home.is_absolute() {
        return Err(unavailable("Freebuff profile is unavailable"));
    }
    let home = &located.profile_home;
    let (mut models, truncated) = catalog_models(home);
    if let Some(selected) = selected_model(home) {
        if !models.iter().any(|id| id == &selected) {
            models.push(selected);
        }
    }
    models.truncate(MAX_MODELS);
    if models.is_empty() {
        return Err(unavailable(
            "Freebuff model catalog is not in this profile",
        ));
    }
    let marker = default_marker(home);
    let selected = selected_model(home);
    let mut diagnostics = vec![
        "FREEBUFF_MODEL_IDS_FROM_LOCAL_CATALOG".into(),
        "FREEBUFF_CREDENTIALS_NOT_READ".into(),
        "ACCOUNT_AUTH_NOT_REQUIRED".into(),
    ];
    if truncated {
        diagnostics.push("FREEBUFF_MODEL_LIST_PARTIAL".into());
    }
    if executable(home).is_none() {
        diagnostics.push("FREEBUFF_EXECUTABLE_CATALOG_ABSENT".into());
    }
    diagnostics.sort();
    let models = models
        .into_iter()
        .map(|native| {
            let is_default = selected.as_deref() == Some(native.as_str())
                || marker.as_deref().is_some_and(|marker| {
                    native == marker || native.ends_with(&format!("/{marker}"))
                });
            let revision = stable_id("freebuff-model", &[route, &native]);
            Model {
                id: route_model_key(route, &native),
                route_id: route.into(),
                native_id: native.clone(),
                name: native,
                revision,
                is_default,
                capabilities: vec![],
                options: vec![],
                context_window: Observation::unknown("FREEBUFF_CONTEXT_NOT_REPORTED"),
            }
        })
        .collect();
    Ok(ConnectionMetadata {
        connection_id: connection.into(),
        route_id: route.into(),
        version: version(home),
        auth: Observation::reported(AuthState::Unchecked, evidence()),
        plan: Observation::unknown("FREEBUFF_PLAN_NOT_REPORTED"),
        billing: Observation::unknown("FREEBUFF_BILLING_NOT_REPORTED"),
        account: None,
        models,
        additional_routes: vec![],
        providers: vec![],
        quota_pools: vec![],
        included_usage_allowed: Observation::unknown("FREEBUFF_QUOTA_NOT_REPORTED"),
        diagnostics,
    })
}
