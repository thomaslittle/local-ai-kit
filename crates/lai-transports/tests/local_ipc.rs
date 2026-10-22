use interprocess::local_socket::{
    GenericFilePath, GenericNamespaced,
    tokio::{Stream, prelude::*},
};
use lai_contracts::*;
use lai_core::ClientPolicy;
use lai_service::NativeService;
use lai_transports::local_ipc::{Broker, BrokerOptions};
use serde_json::{Value, json};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
static ENDPOINT_SEQUENCE: AtomicUsize = AtomicUsize::new(0);
fn endpoint() -> String {
    // SystemTime may return the same tick to concurrently starting Windows tests.
    let sequence = ENDPOINT_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    if cfg!(windows) {
        format!(
            "lai-test-{}-{}-{sequence}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        )
    } else {
        std::env::temp_dir()
            .join(format!(
                "lai-{}-{}-{sequence}.sock",
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
fn service() -> (Arc<NativeService>, Arc<AtomicUsize>) {
    let n = Arc::new(AtomicUsize::new(0));
    let c = n.clone();
    (
        Arc::new(NativeService::new(
            Arc::new(move |_| {
                c.fetch_add(1, Ordering::SeqCst);
                Ok(lai_discovery::ScanResult {
                    snapshot: Snapshot::empty(DataMode::Live),
                    locations: Default::default(),
                    metadata_probes: 0,
                })
            }),
            Duration::from_secs(60),
        )),
        n,
    )
}
async fn connect(name: &str) -> BufReader<Stream> {
    let name = if cfg!(windows) {
        name.to_ns_name::<GenericNamespaced>().unwrap()
    } else {
        name.to_fs_name::<GenericFilePath>().unwrap()
    };
    BufReader::new(Stream::connect(name).await.unwrap())
}
async fn call(c: &mut BufReader<Stream>, m: &str, p: Value) -> Value {
    c.get_mut()
        .write_all(
            format!(
                "{}\n",
                json!({"jsonrpc":"2.0","id":1,"method":m,"params":p})
            )
            .as_bytes(),
        )
        .await
        .unwrap();
    let mut line = String::new();
    tokio::time::timeout(Duration::from_secs(2), c.read_line(&mut line))
        .await
        .unwrap()
        .unwrap();
    serde_json::from_str(&line).unwrap()
}
async fn auth(name: &str, key: String) -> BufReader<Stream> {
    let mut c = connect(name).await;
    assert!(
        call(&mut c, "broker.authenticate", json!({"token":key}))
            .await
            .get("result")
            .is_some()
    );
    assert_eq!(
        call(&mut c, "runtime.handshake", json!({})).await["result"]["protocolVersion"],
        "1.0"
    );
    c
}
#[tokio::test]
async fn two_clients_share_inventory_and_closing_one_does_not_stop_broker() {
    let (s, n) = service();
    let name = endpoint();
    let b = Broker::start(s, &name).await.unwrap();
    let mut a = auth(&name, b.authorize(ClientPolicy::passive(), None).unwrap()).await;
    let mut c = auth(&name, b.authorize(ClientPolicy::passive(), None).unwrap()).await;
    call(&mut a, "discovery.refresh", json!({})).await;
    call(&mut c, "discovery.refresh", json!({})).await;
    assert_eq!(n.load(Ordering::SeqCst), 1);
    drop(a);
    assert_eq!(
        call(&mut c, "runtime.health", json!({})).await["result"]["status"],
        "ok"
    );
    b.close().await;
}
#[tokio::test]
async fn unauthorized_connections_cannot_query_and_keys_are_single_use() {
    let (s, n) = service();
    let name = endpoint();
    let b = Broker::start(s, &name).await.unwrap();
    let mut bad = connect(&name).await;
    let r = call(&mut bad, "snapshot.get", json!({})).await;
    assert_eq!(r["error"]["data"]["code"], "LOCAL_ACCESS_DENIED");
    let key = b.authorize(ClientPolicy::passive(), None).unwrap();
    let _a = auth(&name, key.clone()).await;
    let mut reuse = connect(&name).await;
    assert!(
        call(&mut reuse, "broker.authenticate", json!({"token":key}))
            .await
            .get("error")
            .is_some()
    );
    assert_eq!(n.load(Ordering::SeqCst), 0);
    b.close().await;
}
#[tokio::test]
async fn another_broker_cannot_take_over_a_live_endpoint() {
    let (s, _) = service();
    let name = endpoint();
    let b = Broker::start(s.clone(), &name).await.unwrap();
    assert!(Broker::start(s, &name).await.is_err());
    let mut a = auth(&name, b.authorize(ClientPolicy::passive(), None).unwrap()).await;
    assert!(
        call(&mut a, "snapshot.get", json!({}))
            .await
            .get("result")
            .is_some()
    );
    b.close().await;
}
#[tokio::test]
async fn permissions_are_per_connection_and_revocation_is_local() {
    let (s, n) = service();
    let name = endpoint();
    let b = Broker::start(s, &name).await.unwrap();
    let mut a = auth(&name, b.authorize(ClientPolicy::denied(), None).unwrap()).await;
    let mut c = auth(&name, b.authorize(ClientPolicy::passive(), None).unwrap()).await;
    assert_eq!(
        call(&mut a, "discovery.refresh", json!({})).await["error"]["data"]["code"],
        "LOCAL_ACCESS_DENIED"
    );
    call(&mut c, "grants.revoke", json!({})).await;
    assert!(
        call(&mut c, "discovery.refresh", json!({}))
            .await
            .get("error")
            .is_some()
    );
    assert_eq!(n.load(Ordering::SeqCst), 0);
    b.close().await;
}
#[tokio::test]
async fn endpoint_rejects_network_names_and_ambiguous_addresses() {
    let (s, _) = service();
    for bad in [
        "",
        "relative/path",
        "\\\\remote\\pipe\\steal",
        "http://127.0.0.1:1000",
        "a\nb",
    ] {
        assert!(Broker::start(s.clone(), bad).await.is_err());
    }
}

#[cfg(unix)]
#[tokio::test]
async fn endpoint_is_owner_only() {
    use std::os::unix::fs::PermissionsExt;
    let (s, _) = service();
    let name = endpoint();
    let b = Broker::start(s, &name).await.unwrap();
    let mode = std::fs::metadata(&name).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "broker endpoint must be owner-only");
    b.close().await;
}

#[tokio::test]
async fn an_independent_owner_credential_can_open_more_than_one_connection() {
    let (s, _) = service();
    let name = endpoint();
    let b = Broker::start(s, &name).await.unwrap();
    let token = b.authorize_shared(ClientPolicy::passive(), None).unwrap();
    let _first = auth(&name, token.clone()).await;
    let _second = auth(&name, token).await;
    assert_eq!(b.active_connections(), 2);
    b.close().await;
}

#[tokio::test]
async fn an_idle_independent_broker_releases_its_endpoint() {
    let (s, _) = service();
    let name = endpoint();
    let b = Broker::start_with(
        s.clone(),
        &name,
        BrokerOptions {
            idle_timeout: Some(Duration::from_millis(300)),
        },
    )
    .await
    .unwrap();
    tokio::time::sleep(Duration::from_millis(900)).await;
    assert_eq!(b.active_connections(), 0);
    b.close().await;
    let second = Broker::start(s, &name).await;
    assert!(second.is_ok());
    second.unwrap().close().await;
}
