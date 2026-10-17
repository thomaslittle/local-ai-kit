use lai_adapters::grok::{parse_status, probe_environment};
use lai_adapters::native_clients::launch::apply_grok_probe_environment;
use lai_contracts::*;
use std::{
    collections::BTreeMap,
    ffi::{OsStr, OsString},
};
#[test]
fn native_grok_login_line_reports_auth_but_not_unpublished_plan_or_costs() {
    let (auth, billing) =
        parse_status("You are logged in with grok.com.\nAvailable models:\n * native-model\n");
    assert_eq!(auth.fresh_value(), Some(&AuthState::Authenticated));
    assert!(billing.value().is_none());
}
#[test]
fn absent_status_and_auth_failure_are_not_active_accounts() {
    let (auth, _) = parse_status("not authenticated");
    assert_eq!(auth.fresh_value(), Some(&AuthState::Unauthenticated));
    let (auth, _) = parse_status("Default model: example\nAvailable models:\n * native");
    assert!(auth.value().is_none());
}
#[test]
fn health_probes_disable_the_autoupdater_even_when_the_caller_left_it_off() {
    let mut env = BTreeMap::new();
    env.insert(
        OsString::from("GROK_DISABLE_AUTOUPDATER"),
        OsString::from("0"),
    );
    apply_grok_probe_environment(&mut env);
    assert_eq!(
        env.get(OsStr::new("GROK_DISABLE_AUTOUPDATER"))
            .and_then(|value| value.to_str()),
        Some("1")
    );
    assert_eq!(
        probe_environment()
            .get(OsStr::new("GROK_DISABLE_AUTOUPDATER"))
            .and_then(|value| value.to_str()),
        Some("1")
    );
}
#[test]
fn unrelated_output_is_not_preserved() {
    let (auth, _) = parse_status("secret-sentinel");
    assert!(
        !serde_json::to_string(&auth)
            .unwrap()
            .contains("secret-sentinel")
    );
}
