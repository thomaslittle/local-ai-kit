use lai_adapters::freebuff::{model_ids_in_catalog, read_catalog};
use lai_contracts::AuthState;
use lai_discovery::LocatedClient;
use std::fs;

#[test]
fn catalog_text_keeps_vendor_model_ids_only() {
    let text = r#"
        tokenKey:"do-not-copy-this-secret"
        model:"z-ai/glm-5.3-flash"
        model:"not a model"
        model:"openai/gpt-5.6-luna"
        credentials.json
    "#;
    assert_eq!(
        model_ids_in_catalog(text),
        vec![
            "openai/gpt-5.6-luna".to_string(),
            "z-ai/glm-5.3-flash".to_string()
        ]
    );
}

#[test]
fn profile_catalog_reports_models_without_reading_the_session_token() {
    let root = tempfile::tempdir().unwrap();
    let home = root.path().join("manicode");
    fs::create_dir_all(&home).unwrap();
    fs::write(
        home.join("freebuff.exe"),
        br#"model:"z-ai/glm-5.3-flash" model:"openai/gpt-5.6-luna""#,
    )
    .unwrap();
    fs::write(
        home.join("freebuff-metadata.json"),
        br#"{"version":"0.1.4","target":"win32-x64"}"#,
    )
    .unwrap();
    fs::write(
        home.join("settings.json"),
        br#"{"freebuffModelDefaultMigration":"glm-5.3-flash"}"#,
    )
    .unwrap();
    fs::write(
        home.join("freebuff-live-1.json"),
        br#"{"model":"z-ai/glm-5.3-flash","tokenKey":"do-not-copy-this-secret","ownerPid":1}"#,
    )
    .unwrap();
    fs::write(home.join("credentials.json"), br#"{"token":"do-not-copy-this-secret"}"#).unwrap();
    let report = read_catalog(
        &LocatedClient {
            provider_id: "freebuff".into(),
            profile_home: home,
            executable: None,
        },
        "connection",
        "route",
    )
    .unwrap();
    assert_eq!(report.auth.fresh_value(), Some(&AuthState::Unchecked));
    assert!(report.account.is_none());
    let ids: Vec<_> = report
        .models
        .iter()
        .map(|model| model.native_id.as_str())
        .collect();
    assert_eq!(ids, vec!["openai/gpt-5.6-luna", "z-ai/glm-5.3-flash"]);
    assert!(report.models.iter().any(|model| {
        model.native_id == "z-ai/glm-5.3-flash" && model.is_default
    }));
    assert!(!report.models.iter().any(|model| model.native_id == "openai/gpt-5.6-luna" && model.is_default));
    let encoded = serde_json::to_string(&report).unwrap();
    assert!(!encoded.contains("do-not-copy-this-secret"));
    assert!(encoded.contains("account sign-in is not required"));
    assert_eq!(report.version.fresh_value().map(String::as_str), Some("0.1.4"));
}
