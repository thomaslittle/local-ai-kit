use serde_json::{Value, json};
use std::{
    process::{Command, Stdio},
    time::{Duration, Instant},
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

fn endpoint() -> String {
    if cfg!(windows) {
        format!(
            "lai-cli-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        )
    } else {
        std::env::temp_dir()
            .join(format!(
                "lai-cli-{}-{}.sock",
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

async fn connect(name: &str) -> BufReader<lai_transports::local_ipc::IpcStream> {
    let stream = lai_transports::local_ipc::connect(name).await.unwrap();
    BufReader::new(stream)
}

async fn call(
    stream: &mut BufReader<lai_transports::local_ipc::IpcStream>,
    id: u64,
    method: &str,
    params: Value,
) -> Value {
    stream
        .get_mut()
        .write_all(
            format!(
                "{}\n",
                json!({"jsonrpc":"2.0","id":id,"method":method,"params":params})
            )
            .as_bytes(),
        )
        .await
        .unwrap();
    let mut line = String::new();
    tokio::time::timeout(Duration::from_secs(5), stream.read_line(&mut line))
        .await
        .unwrap()
        .unwrap();
    serde_json::from_str(&line).unwrap()
}

async fn authenticated(name: &str, token: &str) -> BufReader<lai_transports::local_ipc::IpcStream> {
    let mut stream = connect(name).await;
    let result = call(
        &mut stream,
        1,
        "broker.authenticate",
        json!({"token":token}),
    )
    .await;
    assert_eq!(result["result"]["authenticated"], true);
    let handshake = call(
        &mut stream,
        2,
        "runtime.handshake",
        json!({"protocolVersion":"1.0","clientName":"cli-e2e"}),
    )
    .await;
    assert_eq!(handshake["result"]["protocolVersion"], "1.0");
    stream
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn an_independent_broker_survives_consumer_disconnects_and_converges() {
    let directory = tempfile::tempdir().unwrap();
    let home = directory.path().to_path_buf();
    let address = endpoint();
    let token_file = directory.path().join("broker-token.json");
    let mut child = Command::new(env!("CARGO_BIN_EXE_local-ai-kit"))
        .args([
            "serve",
            "--ipc",
            &address,
            "--independent",
            "--token-file",
            token_file.to_str().unwrap(),
            "--allow-passive-discovery",
            "--idle-timeout",
            "5",
            "--home",
            home.to_str().unwrap(),
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(15);
    let descriptor: Value = loop {
        if let Ok(bytes) = std::fs::read(&token_file) {
            if let Ok(value) = serde_json::from_slice(&bytes) {
                break value;
            }
        }
        assert!(
            Instant::now() < deadline,
            "broker did not publish a token file"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    };
    let token = descriptor["token"].as_str().unwrap().to_owned();
    assert_eq!(descriptor["address"], address);
    let mut first = authenticated(&address, &token).await;
    assert!(
        call(&mut first, 3, "snapshot.get", json!({}))
            .await
            .get("result")
            .is_some()
    );
    drop(first);
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert!(
        child.try_wait().unwrap().is_none(),
        "closing one consumer must not stop the independent broker"
    );
    let mut second = authenticated(&address, &token).await;
    assert!(
        call(&mut second, 3, "runtime.health", json!({}))
            .await
            .get("result")
            .is_some()
    );
    drop(second);

    // A concurrent launch must fail to take over the live endpoint.
    let mut duplicate = Command::new(env!("CARGO_BIN_EXE_local-ai-kit"))
        .args([
            "serve",
            "--ipc",
            &address,
            "--independent",
            "--token-file",
            token_file.to_str().unwrap(),
            "--allow-passive-discovery",
            "--home",
            home.to_str().unwrap(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let status = tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            if let Some(status) = duplicate.try_wait().unwrap() {
                break status;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("duplicate broker must exit");
    assert!(!status.success());

    // After the idle timeout the broker releases the endpoint and token file.
    let status = tokio::time::timeout(Duration::from_secs(20), async {
        loop {
            if let Some(status) = child.try_wait().unwrap() {
                break status;
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
    })
    .await
    .expect("idle broker must stop");
    assert!(status.success());
    assert!(
        !token_file.exists(),
        "a stopped broker removes its owner token"
    );
}
