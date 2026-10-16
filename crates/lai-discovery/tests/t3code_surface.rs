use lai_discovery::*;
use lai_platform::*;
use std::{fs, path::Path, sync::atomic::AtomicBool};

struct MetadataOnly;
impl LocalFilesystem for MetadataOnly {
    fn probe(&self, path: &Path) -> lai_contracts::Result<EntryKind> {
        NativeFilesystem.probe(path)
    }
    fn children(&self, path: &Path, limit: usize) -> lai_contracts::Result<DirectoryListing> {
        NativeFilesystem.children(path, limit)
    }
    fn read_chunk(&self, _: &Path, _: u64, _: usize) -> lai_contracts::Result<ReadChunk> {
        panic!("passive scan read file content")
    }
}

#[test]
fn pi_profile_follows_the_t3code_snapshot() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../upstream/t3code/snapshot.json");
    let snapshot: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    assert_eq!(snapshot["source"]["commit"], T3CODE_COMMIT);
    assert_eq!(snapshot["source"]["repository"], T3CODE_REPOSITORY);

    let pi = definitions().iter().find(|item| item.id == "pi").unwrap();
    let commands: Vec<_> = snapshot["adopted"]["pi"]["commands"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item.as_str().unwrap())
        .collect();
    assert_eq!(pi.commands, commands.as_slice());

    let home = Path::new("/home/user");
    let env = ScanEnvironment::for_home(home.into(), HostPlatform::Linux);
    let profile = &profile_roots("pi", &env)[0];
    assert_eq!(
        profile,
        &join_relative(
            home,
            snapshot["adopted"]["pi"]["profileRelative"]
                .as_str()
                .unwrap()
        )
    );
    assert_eq!(
        session_stores("pi", profile)[0],
        join_relative(
            profile,
            snapshot["adopted"]["pi"]["sessionRelative"]
                .as_str()
                .unwrap()
        )
    );

    let custom = Path::new("/work/pi-agent");
    let mut overridden = env;
    overridden
        .overrides
        .insert("PI_CODING_AGENT_DIR".into(), custom.into());
    assert_eq!(profile_roots("pi", &overridden), vec![custom.to_path_buf()]);
    assert!(
        discovery_rules()
            .iter()
            .any(|rule| rule.id == "pi-agent" && rule.provider_id == "pi")
    );
}

#[test]
fn pi_configuration_and_off_path_opencode_are_inventory_only() {
    let root = tempfile::tempdir().unwrap();
    let home = root.path().join("home");
    fs::create_dir_all(home.join(".pi/agent/sessions")).unwrap();
    fs::create_dir_all(home.join(".opencode/bin")).unwrap();
    let binary = home.join(".opencode/bin/opencode");
    fs::write(&binary, b"DO_NOT_EXECUTE").unwrap();
    let env = ScanEnvironment::for_home(home, HostPlatform::Linux);
    let result = Scanner::new(MetadataOnly)
        .scan(&env, &AtomicBool::new(false), |_| {})
        .unwrap();

    let pi = result
        .snapshot
        .connections
        .iter()
        .find(|connection| connection.provider_id == "pi")
        .unwrap();
    assert_eq!(pi.status, "configuration_only");
    assert!(
        result
            .locations
            .get(&pi.id)
            .is_some_and(|location| location.executable.is_none())
    );

    let opencode = result
        .locations
        .values()
        .find(|location| location.provider_id == "opencode")
        .unwrap();
    assert_eq!(opencode.executable.as_deref(), Some(binary.as_path()));
    assert_eq!(fs::read_to_string(&binary).unwrap(), "DO_NOT_EXECUTE");
}
