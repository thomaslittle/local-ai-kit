use lai_contracts::*;
use lai_core::ClientPolicy;
use lai_discovery::ScanResult;
use lai_service::NativeService;
use serde_json::json;
use std::{collections::BTreeMap, sync::Arc, time::Duration};
fn service(path: &std::path::Path) -> NativeService {
    let service = NativeService::new(
        Arc::new(|_| {
            Ok(ScanResult {
                snapshot: Snapshot::empty(DataMode::Synthetic),
                locations: BTreeMap::new(),
                metadata_probes: 0,
            })
        }),
        Duration::from_secs(60),
    )
    .with_cost_prices(path.to_owned());
    service
        .register_client("app", ClientPolicy::passive())
        .unwrap();
    let mut admin = ClientPolicy::passive();
    admin.admin = true;
    service.register_client("admin", admin).unwrap();
    service
}
#[tokio::test]
async fn prices_require_admin_and_survive_a_host_restart() {
    let directory = tempfile::tempdir().unwrap();
    let host = service(directory.path());
    let report = host.handle("app", "costs.read", json!({})).await.unwrap();
    assert_eq!(report["records"], json!([]));
    assert_eq!(report["canEditPrices"], false);
    let params = json!({"prices": [{"providerId":"fixture", "nativeModelId":"exact-model", "inputUsdPerMillion":"0.3", "outputUsdPerMillion":"1.2", "reviewedAt":"2026-10-28", "source":"local rate"}]});
    assert!(
        host.handle("app", "costs.setPrices", params.clone())
            .await
            .is_err()
    );
    host.handle("admin", "costs.setPrices", params.clone())
        .await
        .unwrap();
    host.handle("admin", "costs.setPrices", params.clone())
        .await
        .unwrap();
    let restarted = service(directory.path());
    let report = restarted
        .handle("admin", "costs.read", json!({}))
        .await
        .unwrap();
    assert_eq!(report["prices"], params["prices"]);
    assert_eq!(report["pricesPersisted"], true);
}
#[tokio::test]
async fn invalid_saved_prices_do_not_silently_restore_defaults() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("cost-prices.json"), "invalid").unwrap();
    let host = service(directory.path());
    let report = host.handle("admin", "costs.read", json!({})).await.unwrap();
    assert_eq!(report["prices"], json!([]));
    assert!(report["storageError"].is_string());
    assert!(host.handle("admin", "costs.setPrices", json!({"prices":[{"providerId":"fixture", "nativeModelId":"exact-model", "inputUsdPerMillion":"-1", "outputUsdPerMillion":"1", "reviewedAt":"2026-10-28", "source":"invalid"}]})).await.is_err());
}
