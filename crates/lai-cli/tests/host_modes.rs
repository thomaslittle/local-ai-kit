use std::io::Write;
use std::process::{Command, Stdio};
#[test]
fn connect_mode_does_not_start_a_missing_broker_or_scan() {
    let out = Command::new(env!("CARGO_BIN_EXE_local-ai-kit"))
        .args([
            "connect",
            "--ipc",
            if cfg!(windows) {
                "missing-lai-test-broker"
            } else {
                "/tmp/missing-lai-test-broker.sock"
            },
        ])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("RuntimeMissing"));
    assert!(out.stdout.is_empty());
}
#[test]
fn native_host_without_registration_rejects_origin() {
    let out = Command::new(env!("CARGO_BIN_EXE_local-ai-kit-native-host"))
        .arg("chrome-extension://abcdefghijklmnopabcdefghijklmnop/")
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(out.stdout.is_empty());
}
#[test]
fn broker_endpoint_rejects_tokens_in_argv() {
    let out = Command::new(env!("CARGO_BIN_EXE_local-ai-kit"))
        .args(["connect", "--ipc", "any", "--token", "secret-sentinel"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(!String::from_utf8_lossy(&out.stderr).contains("secret-sentinel"));
}
#[test]
fn native_host_untrusted_origin_cannot_use_json_as_authorization() {
    let mut p = Command::new(env!("CARGO_BIN_EXE_local-ai-kit-native-host"))
        .arg("https://evil.invalid")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let _ = p.stdin.take().unwrap().write_all(b"{\"grant\":true}");
    assert!(!p.wait_with_output().unwrap().status.success());
}
