use lai_contracts::*;
use lai_core::ClientPolicy;
use lai_service::NativeService;
use lai_transports::local_ipc::Broker;
use std::{process::Command, sync::Arc, time::Duration};

fn endpoint() -> String {
    if cfg!(windows) {
        format!(
            "lai-control-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        )
    } else {
        std::env::temp_dir()
            .join(format!(
                "lai-control-{}-{}.sock",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ))
            .to_string_lossy()
            .into_owned()
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn the_control_center_uses_only_the_public_authenticated_api() {
    let directory = tempfile::tempdir().unwrap();
    let address = endpoint();
    let service = Arc::new(NativeService::new(
        Arc::new(|_| {
            Ok(lai_discovery::ScanResult {
                snapshot: Snapshot::empty(DataMode::Live),
                locations: Default::default(),
                metadata_probes: 0,
            })
        }),
        Duration::from_secs(1),
    ));
    let broker = Broker::start(service, &address).await.unwrap();
    let token = broker
        .authorize_shared(
            ClientPolicy {
                admin: true,
                ..ClientPolicy::passive()
            },
            None,
        )
        .unwrap();
    let token_file = directory.path().join("broker-token.json");
    let descriptor = serde_json::json!({
        "protocolVersion":"1.0",
        "address":address,
        "token":token,
        "expiresInSeconds":0,
        "independent":true,
    });
    std::fs::write(&token_file, serde_json::to_vec(&descriptor).unwrap()).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&token_file, std::fs::Permissions::from_mode(0o600)).unwrap();
    }
    let output = Command::new(env!("CARGO_BIN_EXE_local-ai-kit-control"))
        .arg("--token-file")
        .arg(&token_file)
        .arg("--once")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Revision"));
    assert!(stdout.contains("No supported AI installation"));
    assert!(!stdout.contains(&token));
    assert_eq!(broker.active_connections(), 0);
    broker.close().await;
}
