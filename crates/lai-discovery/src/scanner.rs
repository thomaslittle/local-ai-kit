use crate::environment::valid_root;
use crate::{HostPlatform, LocatedClient, ScanEnvironment, ScanResult, Scanner, definitions};
use lai_contracts::*;
use lai_platform::{EntryKind, LocalFilesystem};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};
fn path_key(path: &Path) -> String {
    format!("{:x}", Sha256::digest(path.as_os_str().as_encoded_bytes()))
}
fn cancelled(flag: &AtomicBool) -> Result<()> {
    if flag.load(Ordering::Relaxed) {
        Err(DomainError::new(
            ErrorCode::Cancelled,
            "Discovery cancelled",
        ))
    } else {
        Ok(())
    }
}
impl<F: LocalFilesystem> Scanner<F> {
    pub fn scan(
        &self,
        env: &ScanEnvironment,
        cancel: &AtomicBool,
        mut progress: impl FnMut(&Snapshot),
    ) -> Result<ScanResult> {
        cancelled(cancel)?;
        env.validate()?;
        let environment_id = stable_id(
            "environment",
            &[&path_key(&env.home), &format!("{:?}", env.platform)],
        );
        let mut result = ScanResult {
            snapshot: Snapshot::empty(DataMode::Live),
            locations: BTreeMap::new(),
            metadata_probes: 0,
        };
        let mut directories = env
            .search_path
            .iter()
            .filter(|p| valid_root(p))
            .cloned()
            .collect::<BTreeSet<_>>();
        directories.extend([
            env.home.join(".local/bin"),
            env.home.join(".cargo/bin"),
            env.home.join(".bun/bin"),
        ]);
        if env.platform == HostPlatform::Windows {
            directories.insert(env.app_data.join("npm"));
        }
        let children = self.filesystem.children(&env.home, 128);
        let extra = match children {
            Ok(listing) => {
                if listing.truncated {
                    result
                        .snapshot
                        .diagnostics
                        .push("PROFILE_ENUMERATION_PARTIAL".into());
                }
                listing.entries
            }
            Err(_) => {
                result
                    .snapshot
                    .diagnostics
                    .push("PROFILE_ENUMERATION_UNAVAILABLE".into());
                vec![]
            }
        };
        let mut installations = BTreeSet::new();
        for definition in definitions() {
            cancelled(cancel)?;
            result.snapshot.providers.push(Provider {
                id: definition.id.into(),
                name: definition.name.into(),
                kind: "local_client".into(),
            });
            let mut binaries = BTreeSet::new();
            for directory in &directories {
                for command in definition.commands {
                    for suffix in if env.platform == HostPlatform::Windows {
                        &[".exe", ".cmd", ".bat", ""][..]
                    } else {
                        &[""][..]
                    } {
                        cancelled(cancel)?;
                        let path = directory.join(format!("{command}{suffix}"));
                        result.metadata_probes += 1;
                        match self.filesystem.probe(&path) {
                            Ok(EntryKind::File) => {
                                binaries.insert(path);
                            }
                            Ok(EntryKind::Symlink) => result
                                .snapshot
                                .diagnostics
                                .push(format!("{}_EXECUTABLE_SYMLINK_NOT_FOLLOWED", definition.id)),
                            Err(_) => result
                                .snapshot
                                .diagnostics
                                .push(format!("{}_EXECUTABLE_METADATA_UNAVAILABLE", definition.id)),
                            _ => {}
                        }
                    }
                }
            }
            if binaries.len() > 8 {
                result
                    .snapshot
                    .diagnostics
                    .push(format!("{}_INSTALLATION_LIST_PARTIAL", definition.id));
            }
            let mut roots = definition.roots(env);
            if !definition.has_override(env) {
                if let Some(prefix) = definition.profile_prefix {
                    roots.extend(
                        extra
                            .iter()
                            .filter(|p| {
                                p.file_name()
                                    .is_some_and(|n| n.to_string_lossy().starts_with(prefix))
                            })
                            .cloned(),
                    );
                }
            }
            let default_root = roots.first().cloned();
            roots.sort();
            roots.dedup();
            if roots.len() > 16 {
                roots.truncate(16);
                result
                    .snapshot
                    .diagnostics
                    .push(format!("{}_PROFILE_LIST_PARTIAL", definition.id));
            }
            for (index, root) in roots.into_iter().enumerate() {
                cancelled(cancel)?;
                result.metadata_probes += 1;
                let config_present = match self.filesystem.probe(&root) {
                    Ok(EntryKind::Directory) => true,
                    Ok(EntryKind::Missing) => false,
                    Ok(_) => {
                        result
                            .snapshot
                            .diagnostics
                            .push(format!("{}_CONFIG_ROOT_UNSUPPORTED", definition.id));
                        false
                    }
                    Err(_) => {
                        result
                            .snapshot
                            .diagnostics
                            .push(format!("{}_CONFIG_METADATA_UNAVAILABLE", definition.id));
                        false
                    }
                };
                if !config_present && (binaries.is_empty() || Some(&root) != default_root.as_ref())
                {
                    continue;
                }
                let executable_options: Vec<_> = if binaries.is_empty() {
                    vec![None]
                } else {
                    binaries.iter().take(8).map(|p| Some(p.clone())).collect()
                };
                for executable in executable_options {
                    let binary_key = executable
                        .as_ref()
                        .map(|p| path_key(p))
                        .unwrap_or_else(|| format!("configuration-only-{}", definition.id));
                    let installation_id = stable_id(
                        "installation",
                        &[&environment_id, definition.id, &binary_key],
                    );
                    let profile_id = stable_id("profile", &[&installation_id, &path_key(&root)]);
                    let connection_id = stable_id("connection", &[&profile_id]);
                    if installations.insert(installation_id.clone()) {
                        result.snapshot.installations.push(Installation {
                            id: installation_id.clone(),
                            provider_id: definition.id.into(),
                            environment_id: environment_id.clone(),
                            location_hint: if executable.is_some() {
                                "Executable candidate present; not launched".into()
                            } else {
                                "Configuration only; no executable found".into()
                            },
                            version: Observation::unknown("VERSION_NOT_PROBED"),
                        });
                    }
                    let label = if Some(&root) == default_root.as_ref() {
                        "Default profile".into()
                    } else {
                        format!("Additional profile {}", index + 1)
                    };
                    result.snapshot.profiles.push(Profile {
                        id: profile_id.clone(),
                        installation_id: installation_id.clone(),
                        label: label.clone(),
                    });
                    result.snapshot.connections.push(Connection {
                        id: connection_id.clone(),
                        provider_id: definition.id.into(),
                        installation_id,
                        profile_id,
                        display_name: format!("{} Â· {label}", definition.name),
                        account_id: None,
                        auth: Observation::unknown("AUTH_NOT_CHECKED_PASSIVE_SCAN"),
                        plan: Observation::unknown("PLAN_NOT_REPORTED"),
                        status: if executable.is_some() {
                            "detected".into()
                        } else {
                            "configuration_only".into()
                        },
                        diagnostics: vec![if config_present {
                            "CONFIGURATION_DIRECTORY_PRESENT".into()
                        } else {
                            "NO_CONFIGURATION_DIRECTORY".into()
                        }],
                    });
                    result.snapshot.routes.push(AccessRoute {
                        id: stable_id("route", &[&connection_id]),
                        connection_id: connection_id.clone(),
                        provider_id: definition.id.into(),
                        display_name: format!("{} Â· {label}", definition.name),
                        billing: Observation::unknown("BILLING_NOT_CHECKED"),
                        capabilities: vec![],
                        catalog_revision: "unprobed".into(),
                    });
                    result.locations.insert(
                        connection_id,
                        LocatedClient {
                            provider_id: definition.id.into(),
                            profile_home: root.clone(),
                            executable,
                        },
                    );
                }
            }
            result.snapshot.diagnostics.sort();
            result.snapshot.diagnostics.dedup();
            result.snapshot.revision = stable_id(
                "inventory",
                &[&result
                    .snapshot
                    .connections
                    .iter()
                    .map(|c| c.id.as_str())
                    .collect::<Vec<_>>()
                    .join("|")],
            );
            progress(&result.snapshot);
        }
        Ok(result)
    }
}
