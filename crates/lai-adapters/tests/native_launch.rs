use lai_adapters::native_clients::{config::*, launch::prepare};
use lai_contracts::*;
use std::{fs, path::Path};
fn file(path: &Path) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, b"synthetic fixture; never execute").unwrap();
}
fn profile(root: &Path, provider: NativeProvider, exe: &str) -> NativeProfile {
    NativeProfile {
        id: "test".into(),
        label: "Test client".into(),
        provider,
        executable: root.join(exe),
        profile_home: root.join(".gemini/antigravity-acp"),
        auth_method: None,
        billing: None,
    }
}
#[test]
fn ide_launcher_is_not_a_google_agent_runtime() {
    let root = tempfile::tempdir().unwrap();
    let p = profile(root.path(), NativeProvider::Antigravity, "Antigravity.exe");
    file(&p.executable);
    assert_eq!(
        prepare(&p, root.path(), None).err().unwrap().code,
        ErrorCode::RuntimeMissing
    );
}
#[test]
fn google_acp_requires_native_harness_and_blocked_interactive_browser() {
    let root = tempfile::tempdir().unwrap();
    let exe = if cfg!(windows) {
        "agy_acp_server.exe"
    } else {
        "agy_acp_server.par"
    };
    let p = profile(root.path(), NativeProvider::Antigravity, exe);
    file(&p.executable);
    assert!(prepare(&p, root.path(), None).is_err());
    file(&root.path().join(if cfg!(windows) {
        "localharness_external.exe"
    } else {
        "localharness_external"
    }));
    let blocker = root.path().join(if cfg!(windows) {
        "local-ai-kit-auth-blocker.exe"
    } else {
        "local-ai-kit-auth-blocker"
    });
    file(&blocker);
    let launch = prepare(&p, root.path(), Some(&blocker)).unwrap();
    assert!(
        launch
            .spec
            .env
            .contains_key(std::ffi::OsStr::new("ANTIGRAVITY_HARNESS_PATH"))
    );
    assert!(
        launch
            .spec
            .env
            .contains_key(std::ffi::OsStr::new("BROWSER"))
    );
    assert!(
        !launch
            .spec
            .env
            .contains_key(std::ffi::OsStr::new("GEMINI_API_KEY"))
    );
    assert_eq!(launch.auth_method.as_deref(), Some("oauth-personal"));
    assert!(launch.scratch.path().is_dir());
}
#[test]
fn custom_auth_is_explicit_and_not_a_token_export() {
    let root = tempfile::tempdir().unwrap();
    let mut p = profile(
        root.path(),
        NativeProvider::Antigravity,
        if cfg!(windows) {
            "agy_acp_server.exe"
        } else {
            "agy_acp_server.par"
        },
    );
    file(&p.executable);
    file(&root.path().join(if cfg!(windows) {
        "localharness_external.exe"
    } else {
        "localharness_external"
    }));
    let blocker = root.path().join(if cfg!(windows) {
        "local-ai-kit-auth-blocker.exe"
    } else {
        "local-ai-kit-auth-blocker"
    });
    file(&blocker);
    p.auth_method = Some("oauth-business".into());
    let l = prepare(&p, root.path(), Some(&blocker)).unwrap();
    assert_eq!(l.auth_method.as_deref(), Some("oauth-business"));
    assert!(
        !l.spec
            .env
            .keys()
            .any(|k| k.to_string_lossy().contains("TOKEN")
                || k.to_string_lossy().contains("API_KEY"))
    );
}
#[test]
fn unsupported_shell_wrappers_are_never_executed() {
    let root = tempfile::tempdir().unwrap();
    let p = profile(root.path(), NativeProvider::GeminiCli, "arbitrary.cmd");
    file(&p.executable);
    assert!(prepare(&p, root.path(), None).is_err());
}
#[test]
fn grok_custom_profile_is_not_silently_replaced_with_another_account() {
    let root = tempfile::tempdir().unwrap();
    let p = profile(
        root.path(),
        NativeProvider::Grok,
        if cfg!(windows) { "grok.exe" } else { "grok" },
    );
    file(&p.executable);
    assert_eq!(
        prepare(&p, root.path(), None).err().unwrap().code,
        ErrorCode::IntegrationRestricted
    );
}
