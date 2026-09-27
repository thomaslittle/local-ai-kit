use lai_adapters::opencode::catalog;
use lai_contracts::*;
use serde_json::{Value, json};
use std::collections::BTreeMap;
fn fixture() -> Value {
    json!({"connected":["a","b"],"default":{"a":"native","b":"native"},"all":[
    {"id":"a","name":"Provider A","key":"secret-sentinel","models":{"native":{"id":"native","providerID":"a","name":"Native model","capabilities":{"input":{"text":true,"image":true},"output":{"text":true},"reasoning":true,"toolcall":true},"limit":{"context":8192},"variants":{"adaptive":{},"future-effort":{}}}}},
    {"id":"b","name":"Provider B","options":{"apiKey":"secret-sentinel"},"models":{"native":{"id":"native","providerID":"b","name":"Native model","capabilities":{"output":{"text":true}},"limit":{}}}},
    {"id":"unconnected","name":"Not signed in","models":{"absent":{"id":"absent"}}}
    ]})
}
#[test]
fn gateway_models_preserve_distinct_vendor_routes() {
    let m = catalog("connection", "root", &fixture(), &BTreeMap::new()).unwrap();
    assert_eq!(m.auth.fresh_value(), Some(&AuthState::Authenticated));
    assert!(matches!(
        &m.auth,
        Observation::Known { source, .. }
            if source.reference.as_deref() == Some("OpenCode /provider connected list")
    ));
    assert!(
        m.diagnostics
            .iter()
            .any(|d| d == "UPSTREAM_ACCESS_NOT_INDEPENDENTLY_VALIDATED")
    );
    assert_eq!(m.models.len(), 2);
    assert_eq!(m.additional_routes.len(), 2);
    assert_ne!(m.models[0].id, m.models[1].id);
    assert_ne!(m.models[0].route_id, m.models[1].route_id);
    assert_eq!(m.models[0].native_id, "native");
    assert_eq!(
        m.additional_routes[0].id,
        stable_id("route", &["root", "a"])
    );
    assert_eq!(m.additional_routes[0].provider_id, "a");
    assert!(
        m.additional_routes
            .iter()
            .all(|r| r.billing.value().is_none())
    );
    assert!(
        !serde_json::to_string(&m)
            .unwrap()
            .contains("secret-sentinel")
    );
}

#[test]
fn an_opencode_gateway_without_connected_providers_is_not_authenticated() {
    let mut raw = fixture();
    raw["connected"] = json!([]);
    let metadata = catalog("connection", "root", &raw, &BTreeMap::new()).unwrap();
    assert!(metadata.auth.fresh_value().is_none());
    assert!(matches!(
        metadata.auth,
        Observation::Unknown { ref reason, .. } if reason == "OPENCODE_NO_CONNECTED_PROVIDERS"
    ));
}
#[test]
fn variants_and_context_are_native_and_unknown_quota_is_not_zero() {
    let m = catalog("c", "r", &fixture(), &BTreeMap::new()).unwrap();
    assert_eq!(m.models[0].context_window.value().unwrap(), "8192");
    assert!(
        m.models[0].options[0]
            .validate(&json!("future-effort"))
            .is_ok()
    );
    assert!(m.models[1].context_window.value().is_none());
    assert!(m.quota_pools.is_empty());
    assert!(m.plan.value().is_none());
}
#[test]
fn billing_declarations_are_per_provider_and_labeled_as_user_supplied() {
    let m = catalog(
        "c",
        "r",
        &fixture(),
        &BTreeMap::from([
            ("a".into(), BillingMode::Subscription),
            ("b".into(), BillingMode::ApiMetered),
        ]),
    )
    .unwrap();
    assert_eq!(
        m.additional_routes[0].billing.value(),
        Some(&BillingMode::Subscription)
    );
    assert!(
        serde_json::to_string(&m.additional_routes[0].billing)
            .unwrap()
            .contains("user_supplied")
    );
    assert_eq!(
        m.additional_routes[1].billing.value(),
        Some(&BillingMode::ApiMetered)
    );
}
#[test]
fn mismatched_provider_ids_and_duplicate_sources_are_rejected() {
    let mut f = fixture();
    f["all"][0]["models"]["native"]["providerID"] = json!("b");
    assert!(catalog("c", "r", &f, &BTreeMap::new()).is_err());
    let mut f = fixture();
    f["all"][1]["id"] = json!("a");
    assert!(catalog("c", "r", &f, &BTreeMap::new()).is_err());
}
#[test]
fn large_official_provider_directory_can_still_filter_to_connected_accounts() {
    let all = (0..224)
        .map(|index| json!({"id":format!("provider_{index}"),"name":format!("Provider {index}"),"models":{}}))
        .collect::<Vec<_>>();
    let raw = json!({"connected":["provider_223"],"default":{},"all":all});
    let metadata = catalog("c", "r", &raw, &BTreeMap::new()).unwrap();
    assert!(metadata.models.is_empty());
    assert_eq!(metadata.providers.len(), 1);
    assert_eq!(metadata.providers[0].id, "provider_223");
}
