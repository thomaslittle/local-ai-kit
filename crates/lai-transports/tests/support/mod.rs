#[path = "../../../lai-core/tests/common/mod.rs"]
mod common;
use lai_contracts::*;
use lai_core::ClientPolicy;
use lai_discovery::{LocatedClient, ScanResult};
use lai_runtime::{Agent, BoxFuture, TurnContext};
use lai_service::*;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
    time::Duration,
};
struct Echo;
impl Agent for Echo {
    fn turn<'a>(
        &'a mut self,
        _: String,
        _: BTreeMap<String, Value>,
        cx: TurnContext,
    ) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            cx.emit(EventPayload::TextDelta {
                text: "transport-event".into(),
            })?;
            Ok(())
        })
    }
    fn close(&mut self) -> BoxFuture<'_, ()> {
        Box::pin(async { Ok(()) })
    }
}
pub fn grant() -> ExecutionGrant {
    ExecutionGrant {
        connection_ids: BTreeSet::from(["personal".into()]),
        workspaces: BTreeMap::from([("scratch".into(), std::env::temp_dir())]),
        ..Default::default()
    }
}
pub fn create() -> Value {
    json!({"selection":common::selection(),"workspaceId":"scratch"})
}
pub fn service() -> Arc<NativeService> {
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
            Arc::new(|_| Box::pin(async { Ok(Box::new(Echo) as Box<dyn Agent>) })),
        ),
    );
    s.register_client("owner", ClientPolicy::passive()).unwrap();
    s.register_execution("owner", grant()).unwrap();
    s
}
