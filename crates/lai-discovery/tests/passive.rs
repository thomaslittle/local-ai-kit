use lai_contracts::*;
use lai_discovery::*;
use lai_platform::*;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
struct MetadataOnly {
    reads: AtomicUsize,
}
impl LocalFilesystem for MetadataOnly {
    fn probe(&self, p: &Path) -> Result<EntryKind> {
        NativeFilesystem.probe(p)
    }
    fn children(&self, p: &Path, n: usize) -> Result<DirectoryListing> {
        NativeFilesystem.children(p, n)
    }
    fn read_chunk(&self, _: &Path, _: u64, _: usize) -> Result<ReadChunk> {
        self.reads.fetch_add(1, Ordering::SeqCst);
        panic!("passive scan read file content")
    }
}
fn scan(env: &ScanEnvironment) -> ScanResult {
    Scanner::new(MetadataOnly {
        reads: AtomicUsize::new(0),
    })
    .scan(env, &AtomicBool::new(false), |_| {})
    .unwrap()
}
fn environment(root: &Path, platform: HostPlatform) -> ScanEnvironment {
    ScanEnvironment::for_home(root.into(), platform)
}
#[test]
fn empty_home_has_no_fabricated_connections_or_models() {
    let tmp = tempfile::tempdir().unwrap();
    let s = scan(&environment(tmp.path(), HostPlatform::Linux));
    assert!(s.snapshot.connections.is_empty());
    assert!(s.snapshot.models.is_empty());
    assert_eq!(s.snapshot.mode, DataMode::Live);
}
#[test]
fn credential_presence_is_unchecked_not_authentication_and_is_never_read() {
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir(tmp.path().join(".codex")).unwrap();
    fs::write(tmp.path().join(".codex/auth.json"), b"DO_NOT_READ_SECRET").unwrap();
    let s = scan(&environment(tmp.path(), HostPlatform::Linux));
    let c = s
        .snapshot
        .connections
        .iter()
        .find(|c| c.provider_id == "codex")
        .unwrap();
    assert_ne!(c.auth.fresh_value(), Some(&AuthState::Authenticated));
    assert!(c.plan.value().is_none());
    assert!(s.snapshot.models.is_empty());
    assert!(
        !serde_json::to_string(&s.snapshot)
            .unwrap()
            .contains("DO_NOT_READ_SECRET")
    );
}
#[test]
fn user_profile_paths_do_not_escape_the_normalized_snapshot() {
    let tmp = tempfile::tempdir().unwrap();
    let home = tmp.path().join("private-personal-name");
    fs::create_dir_all(home.join(".claude")).unwrap();
    let s = scan(&environment(&home, HostPlatform::Linux));
    assert!(
        !serde_json::to_string(&s.snapshot)
            .unwrap()
            .contains("private-personal-name")
    );
    assert_eq!(s.locations.len(), 1);
}
#[test]
fn custom_home_override_is_respected_without_mutating_either_directory() {
    let tmp = tempfile::tempdir().unwrap();
    let custom = tmp.path().join("work 世界");
    fs::create_dir(&custom).unwrap();
    let mut env = environment(tmp.path(), HostPlatform::Linux);
    env.overrides.insert("CODEX_HOME".into(), custom.clone());
    let s = scan(&env);
    assert!(
        s.locations
            .values()
            .any(|l| l.provider_id == "codex" && l.profile_home == custom)
    );
    assert!(!tmp.path().join(".codex").exists());
}
#[test]
fn gemini_and_antigravity_are_distinct_routes() {
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir_all(tmp.path().join(".gemini/antigravity")).unwrap();
    let s = scan(&environment(tmp.path(), HostPlatform::Linux));
    let ids: Vec<_> = s
        .snapshot
        .connections
        .iter()
        .map(|c| c.provider_id.as_str())
        .collect();
    assert!(ids.contains(&"gemini_cli"));
    assert!(ids.contains(&"antigravity"));
}
#[test]
fn windows_shims_are_detected_as_candidates_without_execution() {
    let tmp = tempfile::tempdir().unwrap();
    let bin = tmp.path().join("bin with spaces");
    fs::create_dir(&bin).unwrap();
    fs::write(bin.join("codex.cmd"), b"SHOULD_NOT_EXECUTE").unwrap();
    let mut env = environment(tmp.path(), HostPlatform::Windows);
    env.search_path = vec![bin.clone(), bin];
    let s = scan(&env);
    assert_eq!(
        s.snapshot
            .connections
            .iter()
            .filter(|c| c.provider_id == "codex")
            .count(),
        1
    );
    assert!(s.locations.values().any(|l| {
        l.executable
            .as_ref()
            .is_some_and(|p| p.ends_with("codex.cmd"))
    }));
}
#[test]
fn same_client_has_distinct_work_and_personal_profiles() {
    let tmp = tempfile::tempdir().unwrap();
    fs::create_dir(tmp.path().join(".codex")).unwrap();
    fs::create_dir(tmp.path().join(".codex-work")).unwrap();
    let s = scan(&environment(tmp.path(), HostPlatform::Linux));
    let profiles: Vec<_> = s
        .snapshot
        .connections
        .iter()
        .filter(|c| c.provider_id == "codex")
        .collect();
    assert_eq!(profiles.len(), 2);
    assert_ne!(profiles[0].id, profiles[1].id);
    assert!(profiles.iter().all(|c| c.account_id.is_none()));
}
#[test]
fn cancellation_before_scan_causes_no_successful_inventory() {
    let tmp = tempfile::tempdir().unwrap();
    let result = Scanner::new(MetadataOnly {
        reads: AtomicUsize::new(0),
    })
    .scan(
        &environment(tmp.path(), HostPlatform::Linux),
        &AtomicBool::new(true),
        |_| {},
    );
    assert_eq!(result.err().unwrap().code, ErrorCode::Cancelled);
}
#[test]
fn partial_results_are_emitted_progressively_and_ids_stay_stable() {
    let tmp = tempfile::tempdir().unwrap();
    for p in [".codex", ".claude", ".grok"] {
        fs::create_dir(tmp.path().join(p)).unwrap();
    }
    let env = environment(tmp.path(), HostPlatform::Linux);
    let mut progress = Vec::new();
    let s = Scanner::new(NativeFilesystem)
        .scan(&env, &AtomicBool::new(false), |snapshot| {
            progress.push(snapshot.connections.len())
        })
        .unwrap();
    assert!(
        progress
            .iter()
            .any(|n| *n > 0 && *n < s.snapshot.connections.len())
    );
    assert_eq!(
        s.snapshot
            .connections
            .iter()
            .map(|c| &c.id)
            .collect::<Vec<_>>(),
        scan(&env)
            .snapshot
            .connections
            .iter()
            .map(|c| &c.id)
            .collect::<Vec<_>>()
    );
}
#[test]
fn relative_and_network_profile_roots_are_rejected() {
    let mut env = ScanEnvironment::for_home(PathBuf::from("relative"), HostPlatform::Linux);
    assert!(
        Scanner::new(NativeFilesystem)
            .scan(&env, &AtomicBool::new(false), |_| {})
            .is_err()
    );
    let tmp = tempfile::tempdir().unwrap();
    env = environment(tmp.path(), HostPlatform::Windows);
    env.overrides
        .insert("CODEX_HOME".into(), PathBuf::from(r"\\server\private"));
    assert!(
        Scanner::new(NativeFilesystem)
            .scan(&env, &AtomicBool::new(false), |_| {})
            .is_err()
    );
}

#[test]
fn another_providers_agent_binary_is_not_misidentified_as_cursor() {
    let tmp = tempfile::tempdir().unwrap();
    let bin = tmp.path().join(".grok/bin");
    fs::create_dir_all(&bin).unwrap();
    fs::write(bin.join("agent.exe"), b"DO_NOT_EXECUTE").unwrap();
    let mut env = environment(tmp.path(), HostPlatform::Windows);
    fs::create_dir_all(env.app_data.join("Cursor")).unwrap();
    env.search_path = vec![bin];
    let result = scan(&env);
    let cursor = result
        .snapshot
        .connections
        .iter()
        .find(|c| c.provider_id == "cursor")
        .unwrap();
    assert_eq!(cursor.status, "configuration_only");
    assert!(result.locations[&cursor.id].executable.is_none());
}
#[test]
fn cursor_named_binary_remains_a_passive_candidate() {
    let tmp = tempfile::tempdir().unwrap();
    let bin = tmp.path().join(".local/bin");
    fs::create_dir_all(&bin).unwrap();
    fs::write(bin.join("cursor-agent"), b"DO_NOT_EXECUTE").unwrap();
    let result = scan(&environment(tmp.path(), HostPlatform::Linux));
    assert!(
        result
            .locations
            .values()
            .any(|l| l.provider_id == "cursor" && l.executable.is_some())
    );
}
