use lai_contracts::{DataMode, Snapshot};
use lai_core::ClientPolicy;
use lai_discovery::ScanResult;
use lai_service::NativeService;
use lai_transports::http::*;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
async fn setup() -> (HttpServer, Arc<AtomicUsize>) {
    let count = Arc::new(AtomicUsize::new(0));
    let reads = count.clone();
    let service = Arc::new(NativeService::new(
        Arc::new(move |_| {
            reads.fetch_add(1, Ordering::SeqCst);
            Ok(ScanResult {
                snapshot: Snapshot::empty(DataMode::Live),
                locations: BTreeMap::new(),
                metadata_probes: 0,
            })
        }),
        Duration::from_secs(30),
    ));
    (
        start(
            service,
            HttpOptions {
                port: 0,
                policy: ClientPolicy::passive(),
                execution: None,
                assets: None,
            },
        )
        .await
        .unwrap(),
        count,
    )
}
fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap()
}
async fn pair(server: &HttpServer) -> String {
    let reply: Value = client()
        .post(format!("{}/v1/pair", server.url))
        .json(&json!({"code":server.pairing_code}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    reply["token"].as_str().unwrap().into()
}
async fn rpc(s: &HttpServer, token: &str, method: &str, params: Value) -> Value {
    client()
        .post(format!("{}/v1/rpc", s.url))
        .bearer_auth(token)
        .json(&json!({"jsonrpc":"2.0","id":1,"method":method,"params":params}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
#[tokio::test]
async fn no_bootstrap_token_or_automatic_scan() {
    let (s, count) = setup().await;
    let result = client()
        .get(format!("{}/runtime/info", s.url))
        .send()
        .await
        .unwrap();
    let body = result.text().await.unwrap();
    assert!(!body.contains(&s.pairing_code));
    assert!(!body.contains("token"));
    assert_eq!(count.load(Ordering::SeqCst), 0);
    assert_eq!(
        client()
            .get(format!("{}/v1/snapshot", s.url))
            .send()
            .await
            .unwrap()
            .status(),
        401
    );
    s.close().await;
}
#[tokio::test]
async fn single_use_pairing_and_native_rpc() {
    let (s, count) = setup().await;
    let token = pair(&s).await;
    let denied = client()
        .post(format!("{}/v1/pair", s.url))
        .json(&json!({"code":s.pairing_code}))
        .send()
        .await
        .unwrap();
    assert_eq!(denied.status(), 403);
    let h = rpc(
        &s,
        &token,
        "runtime.handshake",
        json!({"protocolVersion":"1.0"}),
    )
    .await;
    assert_eq!(h["result"]["protocolVersion"], "1.0");
    rpc(&s, &token, "discovery.refresh", json!({"level":"passive"})).await;
    let snapshot = rpc(&s, &token, "snapshot.get", json!({})).await;
    assert_eq!(snapshot["result"]["mode"], "live");
    assert_eq!(count.load(Ordering::SeqCst), 1);
    s.close().await;
}
#[tokio::test]
async fn host_origin_and_query_credentials_are_rejected() {
    let (s, _) = setup().await;
    let token = pair(&s).await;
    for (header, value) in [("origin", "https://evil.example"), ("host", "evil.example")] {
        let r = client()
            .get(format!("{}/v1/snapshot", s.url))
            .bearer_auth(&token)
            .header(header, value)
            .send()
            .await
            .unwrap();
        assert_eq!(r.status(), 403);
    }
    let r = client()
        .get(format!("{}/v1/snapshot?token={token}", s.url))
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), 400);
    s.close().await;
}
#[tokio::test]
async fn pairing_cannot_upgrade_grants() {
    let (s, count) = setup().await;
    let bad = client()
        .post(format!("{}/v1/pair", s.url))
        .json(&json!({"code":s.pairing_code,"execution":true}))
        .send()
        .await
        .unwrap();
    assert_eq!(bad.status(), 400);
    assert_eq!(count.load(Ordering::SeqCst), 0);
    s.close().await;
}
#[tokio::test]
async fn foreign_origin_cannot_redeem_a_valid_code() {
    let (s, _) = setup().await;
    let result = client()
        .post(format!("{}/v1/pair", s.url))
        .header("Origin", "https://evil.example")
        .json(&json!({"code":s.pairing_code}))
        .send()
        .await
        .unwrap();
    assert_eq!(result.status(), 403);
    assert!(!pair(&s).await.is_empty());
    s.close().await;
}
#[tokio::test]
async fn malformed_requests_and_body_limits_are_enforced() {
    let (s, _) = setup().await;
    let token = pair(&s).await;
    let bad = client()
        .post(format!("{}/v1/rpc", s.url))
        .bearer_auth(&token)
        .header("Content-Type", "application/json")
        .body("x".repeat(1_048_577))
        .send()
        .await
        .unwrap();
    assert_eq!(bad.status(), 413);
    let malformed = rpc(&s, &token, "runtime.handshake", json!([])).await;
    assert!(malformed.get("error").is_some());
    s.close().await;
}

#[tokio::test]
async fn repeated_pair_and_revoke_does_not_exhaust_broker_client_slots() {
    let (s, _) = setup().await;
    for _ in 0..80 {
        let code = s.issue_pairing_code().unwrap();
        let response: Value = client()
            .post(format!("{}/v1/pair", s.url))
            .json(&json!({"code":code}))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        let token = response["token"]
            .as_str()
            .expect("closed clients must release their registry slot");
        rpc(
            &s,
            token,
            "runtime.handshake",
            json!({"protocolVersion":"1.0"}),
        )
        .await;
        assert_eq!(
            rpc(&s, token, "grants.revoke", json!({})).await["result"]["revoked"],
            true
        );
    }
    s.close().await;
}
