use lai_adapters::{acp::client, process::LaunchSpec};
use lai_contracts::*;
use std::{collections::BTreeMap, path::PathBuf};
fn spec(mode: &str) -> LaunchSpec {
    LaunchSpec {
        executable: std::env::var_os("LAI_TEST_PYTHON")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                PathBuf::from(if cfg!(windows) {
                    "C:/Python313/python.exe"
                } else {
                    "/usr/bin/python3"
                })
            }),
        args: vec![
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/support/acp_peer.py")
                .into(),
            mode.into(),
        ],
        cwd: std::env::temp_dir(),
        env: BTreeMap::new(),
    }
}
#[tokio::test]
async fn advertised_load_session_can_suspend_without_closing_and_resume_same_native_id() {
    let catalog = client::inspect(spec("normal"), "fixture", "r")
        .await
        .unwrap();
    let model = catalog.models[0].clone();
    let mut first = client::open(
        spec("normal"),
        "fixture".into(),
        "r".into(),
        model.clone(),
        false,
    )
    .await
    .unwrap();
    let native = first.suspend().await.unwrap();
    assert_eq!(native, "session-native");
    let resumed = client::open_resume(
        spec("resume"),
        "fixture".into(),
        "r".into(),
        model,
        false,
        None,
        &native,
    )
    .await
    .unwrap();
    assert!(resumed.native_resumption());
}
#[tokio::test]
async fn missing_load_session_capability_fails_before_exposing_a_resume_id() {
    let catalog = client::inspect(spec("no_resume"), "fixture", "r")
        .await
        .unwrap();
    let model = catalog.models[0].clone();
    let mut agent = client::open(
        spec("no_resume"),
        "fixture".into(),
        "r".into(),
        model,
        false,
    )
    .await
    .unwrap();
    assert_eq!(
        agent.suspend().await.unwrap_err().code,
        ErrorCode::CapabilityUnsupported
    );
}
#[tokio::test]
async fn wrong_or_unavailable_native_session_is_not_recreated_as_a_new_chat() {
    let catalog = client::inspect(spec("normal"), "fixture", "r")
        .await
        .unwrap();
    let model = catalog.models[0].clone();
    let error = match client::open_resume(
        spec("resume"),
        "fixture".into(),
        "r".into(),
        model,
        false,
        None,
        "other-session",
    )
    .await
    {
        Ok(_) => panic!("unexpected resume success"),
        Err(error) => error,
    };
    assert!(matches!(
        error.code,
        ErrorCode::NotFound | ErrorCode::ProviderUnavailable
    ));
}
