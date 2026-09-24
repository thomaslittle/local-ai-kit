#[path = "../../lai-core/tests/common/mod.rs"]
mod common;
use lai_contracts::*;
use lai_core::ClientPolicy;
use lai_discovery::{LocatedClient, ScanResult};
use lai_runtime::{Agent, BoxFuture, TurnContext};
use lai_service::*;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
struct TestAgent;
impl Agent for TestAgent {
    fn turn<'a>(
        &'a mut self,
        _: String,
        _: BTreeMap<String, Value>,
        cx: TurnContext,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            cx.emit(EventPayload::TextDelta {
                text: "service-native-fixture".into(),
            })?;
            Ok(())
        })
    }
    fn close(&mut self) -> BoxFuture<'_, ()> {
        Box::pin(async { Ok(()) })
    }
}
fn setup(granted: bool) -> (Arc<NativeService>, Arc<AtomicUsize>) {
    let count = Arc::new(AtomicUsize::new(0));
    let calls = count.clone();
    let service = Arc::new(
        NativeService::new(
            Arc::new(|_| {
                let s = common::fixture();
                let locations = s
                    .connections
                    .iter()
                    .map(|c| {
                        (
                            c.id.clone(),
                            LocatedClient {
                                provider_id: "test".into(),
                                profile_home: std::env::temp_dir(),
                                executable: None,
                            },
                        )
                    })
                    .collect();
                Ok(ScanResult {
                    snapshot: s,
                    locations,
                    metadata_probes: 0,
                })
            }),
            Duration::from_secs(60),
        )
        .with_execution(
            BTreeSet::from(["test".into()]),
            Arc::new(move |_| {
                calls.fetch_add(1, Ordering::SeqCst);
                Box::pin(async { Ok(Box::new(TestAgent) as Box<dyn Agent>) })
            }),
        ),
    );
    service
        .register_client("a", ClientPolicy::passive())
        .unwrap();
    service
        .register_client("b", ClientPolicy::denied())
        .unwrap();
    if granted {
        service
            .register_execution(
                "a",
                ExecutionGrant {
                    connection_ids: BTreeSet::from(["personal".into()]),
                    workspaces: BTreeMap::from([("scratch".into(), std::env::temp_dir())]),
                    ..Default::default()
                },
            )
            .unwrap();
    }
    (service, count)
}
fn create() -> Value {
    json!({"selection":common::selection(),"workspaceId":"scratch"})
}
#[tokio::test]
async fn discovery_permission_does_not_authorize_execution() {
    let (s, c) = setup(false);
    s.handle("a", "discovery.refresh", json!({})).await.unwrap();
    assert_eq!(
        s.handle("a", "sessions.create", create())
            .await
            .unwrap_err()
            .code,
        ErrorCode::LocalAccessDenied
    );
    assert_eq!(c.load(Ordering::SeqCst), 0);
}
#[tokio::test]
async fn unknown_workspaces_and_permission_upgrades_are_rejected() {
    let (s, c) = setup(true);
    s.handle("a", "discovery.refresh", json!({})).await.unwrap();
    for extra in [
        json!({"workspaceId":"other"}),
        json!({"permissionMode":"workspace_write"}),
        json!({"reviewRequests":true}),
        json!({"allowPaid":true}),
    ] {
        let mut p = create();
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        assert!(s.handle("a", "sessions.create", p).await.is_err());
    }
    assert_eq!(c.load(Ordering::SeqCst), 0);
}
#[tokio::test]
async fn session_factory_and_events_go_through_public_service() {
    let (s, c) = setup(true);
    s.handle("a", "discovery.refresh", json!({})).await.unwrap();
    let session = s.handle("a", "sessions.create", create()).await.unwrap();
    assert_eq!(c.load(Ordering::SeqCst), 1);
    assert!(
        s.handle("b", "sessions.get", json!({"sessionId":session["id"]}))
            .await
            .is_err()
    );
    s.handle(
        "a",
        "turns.start",
        json!({"sessionId":session["id"],"prompt":"hi","requestId":"one"}),
    )
    .await
    .unwrap();
    let data = tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let e = s.handle("a", "events.read", json!({})).await.unwrap();
            if e.to_string().contains("service-native-fixture") {
                break e;
            }
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
    })
    .await
    .unwrap();
    assert!(!data["events"].as_array().unwrap().is_empty());
    assert!(
        s.handle("b", "events.read", json!({})).await.unwrap()["events"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    s.handle("a", "grants.revoke", json!({})).await.unwrap();
    assert!(
        s.handle(
            "a",
            "turns.start",
            json!({"sessionId":session["id"],"prompt":"hi","requestId":"two"})
        )
        .await
        .is_err()
    );
}
#[tokio::test]
async fn selection_smuggling_does_not_reach_factory() {
    let (s, c) = setup(true);
    s.handle("a", "discovery.refresh", json!({})).await.unwrap();
    let mut p = create();
    p["selection"]["routeId"] = json!("route-work");
    assert!(s.handle("a", "sessions.create", p).await.is_err());
    assert_eq!(c.load(Ordering::SeqCst), 0);
}
#[tokio::test]
async fn request_cannot_supply_native_arguments_or_paths() {
    let (s, c) = setup(true);
    s.handle("a", "discovery.refresh", json!({})).await.unwrap();
    let mut p = create();
    p["executable"] = json!("arbitrary");
    assert!(s.handle("a", "sessions.create", p).await.is_err());
    assert_eq!(c.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn artifacts_are_read_through_the_public_service_and_never_by_path() {
    let (s, _) = setup(true);
    s.handle("a", "discovery.refresh", json!({})).await.unwrap();
    let session = s.handle("a", "sessions.create", create()).await.unwrap();
    s.handle(
        "a",
        "turns.start",
        json!({"sessionId":session["id"],"prompt":"hello","requestId":"artifact-check"}),
    )
    .await
    .unwrap();
    let id = tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let e = s.handle("a", "events.read", json!({})).await.unwrap();
            if let Some(e) = e["events"]
                .as_array()
                .unwrap()
                .iter()
                .find(|e| e["payload"]["type"] == "artifact")
            {
                break e["payload"]["artifactId"].clone();
            }
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
    })
    .await
    .unwrap();
    let p = json!({"sessionId":session["id"],"artifactId":id,"offset":"0","maxBytes":3});
    let c = s.handle("a", "artifacts.read", p.clone()).await.unwrap();
    assert_eq!(c["dataBase64"], "c2Vy");
    assert_eq!(c["nextOffset"], "3");
    assert!(s.handle("b", "artifacts.read", p.clone()).await.is_err());
    let mut bad = p.clone();
    bad["path"] = json!("private-sentinel");
    assert!(s.handle("a", "artifacts.read", bad).await.is_err());
    s.handle("a", "grants.revoke", json!({})).await.unwrap();
    assert!(s.handle("a", "artifacts.read", p).await.is_err());
}
