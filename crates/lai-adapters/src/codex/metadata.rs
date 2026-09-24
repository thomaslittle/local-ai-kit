use super::{invalid, models, text, usage};
use lai_contracts::*;
use serde_json::Value;

pub fn parse_metadata(
    connection: &str,
    route: &str,
    initialize: &Value,
    account_response: &Value,
    native_models: &[Value],
    rate_limits: Option<&Value>,
) -> Result<ConnectionMetadata> {
    if connection.is_empty() || route.is_empty() || native_models.len() > 2048 {
        return Err(invalid());
    }
    let mut source = EvidenceSource::new(
        EvidenceKind::OfficialProtocol,
        "codex",
        "https://developers.openai.com/codex/app-server/",
    );
    let version = text(&initialize["userAgent"])
        .and_then(|agent| {
            agent
                .split_once('/')
                .map(|(_, v)| v.split_whitespace().next().unwrap_or(""))
        })
        .filter(|v| {
            !v.is_empty()
                && v.len() < 64
                && v.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b".-+".contains(&b))
        });
    source.client_version = version.map(str::to_owned);
    let mut report = ConnectionMetadata {
        connection_id: connection.into(),
        route_id: route.into(),
        version: version
            .map(|v| Observation::reported(v.to_owned(), source.clone()))
            .unwrap_or_else(|| Observation::unknown("CLIENT_VERSION_NOT_REPORTED")),
        auth: Observation::unknown("AUTH_NOT_REPORTED"),
        plan: Observation::unknown("PLAN_NOT_REPORTED"),
        billing: Observation::unknown("BILLING_NOT_REPORTED"),
        account: None,
        models: Vec::new(),
        quota_pools: Vec::new(),
        included_usage_allowed: Observation::unknown("INCLUDED_USAGE_PERMISSION_NOT_REPORTED"),
        diagnostics: Vec::new(),
    };
    if !account_response["requiresOpenaiAuth"].is_boolean() {
        return Err(invalid());
    }
    let account = &account_response["account"];
    if account.is_null() {
        report.auth = if account_response["requiresOpenaiAuth"] == true {
            Observation::reported(AuthState::Unauthenticated, source)
        } else {
            Observation::unknown("NATIVE_AUTH_CONTEXT_UNAVAILABLE")
        };
        return Ok(report);
    }
    let kind = text(&account["type"]).ok_or_else(invalid)?;
    let billing = match kind {
        "chatgpt" => BillingMode::Subscription,
        "apiKey" | "amazonBedrock" => BillingMode::ApiMetered,
        _ => return Err(invalid()),
    };
    report.auth = Observation::reported(AuthState::Authenticated, source.clone());
    report.billing = Observation::reported(billing, source.clone());
    let email = text(&account["email"]);
    // This is a connection-scoped identity observation, not cross-client account deduplication.
    let identity = stable_id(
        "account",
        &[connection, kind, email.unwrap_or("not-reported")],
    );
    if let Some(email) = email {
        report.account = Some(Account {
            id: identity.clone(),
            provider_id: "openai".into(),
            display_label: Observation::reported(email.into(), source.clone()),
            identity_source: Observation::reported(
                "Connection-scoped native account/read; no cross-client linking".into(),
                source.clone(),
            ),
        });
    }
    if kind == "chatgpt" {
        if let Some(plan) = text(&account["planType"]).filter(|plan| *plan != "unknown") {
            report.plan = Observation::reported(
                Plan {
                    native_id: plan.into(),
                    label: plan.into(),
                },
                source.clone(),
            );
        }
    }
    report.models = models::parse_models(route, &identity, native_models, &source)?;
    if let Some(rates) = rate_limits {
        report.quota_pools = usage::parse_usage(
            connection,
            route,
            report.account.as_ref().map(|a| a.id.clone()),
            rates,
            &source,
        )?;
        if let Some(allowed) = rates["ordinaryUsageAllowed"].as_bool() {
            report.included_usage_allowed = Observation::reported(allowed, source.clone());
        }
        if rates.get("rateLimitResetCredits").is_some() {
            report
                .diagnostics
                .push("RESET_CREDITS_NOT_ACTIONABLE_BY_TOOLKIT".into());
        }
    } else {
        report.diagnostics.push("USAGE_UNAVAILABLE".into());
    }
    Ok(report)
}
