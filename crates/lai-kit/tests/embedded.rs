use lai_discovery::ScanResult;
use lai_kit::*;
use std::{
    fs,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
#[tokio::test]
async fn building_and_reading_do_not_scan_or_probe() {
    let calls = Arc::new(AtomicUsize::new(0));
    let count = calls.clone();
    let kit = Kit::builder()
        .inventory(Arc::new(move |_| {
            count.fetch_add(1, Ordering::SeqCst);
            Ok(ScanResult {
                snapshot: Snapshot::empty(DataMode::Live),
                locations: Default::default(),
                metadata_probes: 0,
            })
        }))
        .build()
        .unwrap();
    let app = kit.connect("my-app", ClientPolicy::passive()).unwrap();
    assert!(app.snapshot().await.unwrap().connections.is_empty());
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    app.scan().await.unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
#[tokio::test]
async fn one_import_provides_real_passive_discovery() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir(temp.path().join(".codex")).unwrap();
    let kit = Kit::builder().home(temp.path().into()).build().unwrap();
    let app = kit.connect("demo", ClientPolicy::passive()).unwrap();
    let snapshot = app.scan().await.unwrap();
    assert_eq!(snapshot.mode, DataMode::Live);
    assert_eq!(snapshot.connections.len(), 1);
    assert!(snapshot.models.is_empty());
    assert!(snapshot.connections[0].plan.value().is_none());
}
#[tokio::test]
async fn denied_and_same_label_clients_are_isolated() {
    let tmp = tempfile::tempdir().unwrap();
    let kit = Kit::builder().home(tmp.path().into()).build().unwrap();
    let allowed = kit.connect("same-label", ClientPolicy::passive()).unwrap();
    let denied = kit.connect("same-label", ClientPolicy::denied()).unwrap();
    assert!(denied.scan().await.is_err());
    denied.close();
    assert!(allowed.scan().await.is_ok());
    allowed.close();
    assert!(allowed.snapshot().await.is_err());
}
#[test]
fn relative_home_is_rejected_without_creating_a_directory() {
    assert!(Kit::builder().home("relative-home".into()).build().is_err());
}
