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
    sync::{Arc, Mutex},
    time::Duration,
};
struct ResumeAgent;
impl Agent for ResumeAgent {
    fn turn<'a>(
        &'a mut self,
        _: String,
        _: BTreeMap<String, Value>,
        _: TurnContext,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async { Ok(()) })
    }
    fn close(&mut self) -> BoxFuture<'_, ()> {
        Box::pin(async { Ok(()) })
    }
    fn native_resumption(&self) -> bool {
        true
    }
    fn suspend(&mut self) -> BoxFuture<'_, String> {
        Box::pin(async { Ok("native-secret-session".into()) })
    }
}
fn setup() -> (Arc<NativeService>, Arc<Mutex<Vec<Option<String>>>>) {
    let opens = Arc::new(Mutex::new(vec![]));
    let seen = opens.clone();
    let s = Arc::new(
        NativeService::new(
            Arc::new(|_| {
                let snapshot = common::fixture();
                let locations = snapshot
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
                    snapshot,
                    locations,
                    metadata_probes: 0,
                })
            }),
            Duration::from_secs(60),
        )
        .with_execution(
            BTreeSet::from(["test".into()]),
            Arc::new(move |input| {
                seen.lock().unwrap().push(input.native_resume.clone());
                Box::pin(async move {
                    if input
                        .native_resume
                        .as_deref()
                        .is_some_and(|id| id != "native-secret-session")
                    {
                        return Err(DomainError::new(
                            ErrorCode::NotFound,
                            "native session missing",
                        ));
                    }
                    Ok(Box::new(ResumeAgent) as Box<dyn Agent>)
                })
            }),
        ),
    );
    for id in ["a", "b"] {
        s.register_client(id, ClientPolicy::passive()).unwrap();
        s.register_execution(
            id,
            ExecutionGrant {
                connection_ids: BTreeSet::from(["personal".into()]),
                workspaces: BTreeMap::from([("scratch".into(), std::env::temp_dir())]),
                ..Default::default()
            },
        )
        .unwrap();
    }
    (s, opens)
}
fn create() -> Value {
    json!({"selection":common::selection(),"workspaceId":"scratch"})
}
#[tokio::test]
async fn resume_tokens_are_opaque_owner_scoped_and_one_time() {
    let (s, opens) = setup();
    s.handle("a", "discovery.refresh", json!({})).await.unwrap();
    let session = s.handle("a", "sessions.create", create()).await.unwrap();
    assert_eq!(session["nativeResumption"], true);
    let suspended = s
        .handle("a", "sessions.suspend", json!({"sessionId":session["id"]}))
        .await
        .unwrap();
    let token = suspended["resumeToken"].as_str().unwrap();
    assert_eq!(token.len(), 64);
    assert!(!suspended.to_string().contains("native-secret-session"));
    assert!(
        s.handle("a", "sessions.get", json!({"sessionId":session["id"]}))
            .await
            .is_err()
    );
    assert_eq!(
        s.handle("b", "sessions.resume", json!({"resumeToken":token}))
            .await
            .unwrap_err()
            .code,
        ErrorCode::NotFound
    );
    let resumed = s
        .handle("a", "sessions.resume", json!({"resumeToken":token}))
        .await
        .unwrap();
    assert_ne!(resumed["id"], session["id"]);
    assert_eq!(resumed["nativeResumption"], true);
    assert_eq!(
        s.handle("a", "sessions.resume", json!({"resumeToken":token}))
            .await
            .unwrap_err()
            .code,
        ErrorCode::NotFound
    );
    assert_eq!(
        *opens.lock().unwrap(),
        vec![None, Some("native-secret-session".into())]
    );
}
#[tokio::test]
async fn revocation_invalidates_unconsumed_resume_tokens() {
    let (s, _) = setup();
    s.handle("a", "discovery.refresh", json!({})).await.unwrap();
    let session = s.handle("a", "sessions.create", create()).await.unwrap();
    let token = s
        .handle("a", "sessions.suspend", json!({"sessionId":session["id"]}))
        .await
        .unwrap()["resumeToken"]
        .as_str()
        .unwrap()
        .to_owned();
    s.handle("a", "grants.revoke", json!({})).await.unwrap();
    assert_eq!(
        s.handle("a", "sessions.resume", json!({"resumeToken":token}))
            .await
            .unwrap_err()
            .code,
        ErrorCode::NotFound
    );
}
