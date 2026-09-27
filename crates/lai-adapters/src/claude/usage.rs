use super::{invalid, text};
use lai_contracts::*;
use serde_json::Value;
pub(super) fn parse(
    connection: &str,
    route: &str,
    account: Option<String>,
    usage: &Value,
    source: &EvidenceSource,
) -> Result<Vec<QuotaPool>> {
    if usage["rate_limits_available"] != true {
        return Ok(vec![]);
    }
    let rates = &usage["rate_limits"];
    if !rates.is_object() {
        return Err(invalid());
    }
    let mut rows: Vec<(String, String, Value)> = vec![];
    if let Some(limits) = rates["limits"].as_array() {
        if limits.len() > 128 {
            return Err(invalid());
        }
        for (index, limit) in limits.iter().enumerate() {
            let kind = text(&limit["kind"]).ok_or_else(invalid)?;
            let scope = text(&limit["scope"]["model"]["display_name"]);
            rows.push((
                format!("{kind}-{index}"),
                scope
                    .map(|s| format!("{kind} · {s}"))
                    .unwrap_or_else(|| kind.into()),
                serde_json::json!({"percent":limit["percent"],"resetsAt":limit["resets_at"]}),
            ));
        }
    } else {
        for (key, value) in rates.as_object().ok_or_else(invalid)? {
            if value.is_object() && !value["utilization"].is_null() {
                rows.push((key.clone(),key.clone(),serde_json::json!({"percent":value["utilization"],"resetsAt":value["resets_at"]})));
            }
        }
    }
    let mut windows = vec![];
    for (id, label, row) in rows {
        let used = row["percent"]
            .as_f64()
            .filter(|n| n.is_finite() && *n >= 0.0 && *n <= 1000.0)
            .ok_or_else(invalid)?;
        let reading = UsageReading::try_from(UsageInput {
            unit: "percent".into(),
            used: DecimalString::try_from(used.to_string()).map_err(|_| invalid())?,
            limit: Some(DecimalString::try_from("100".to_owned()).map_err(|_| invalid())?),
        })
        .map_err(|_| invalid())?;
        let reset = text(&row["resetsAt"])
            .map(|s| Timestamp::try_from(s.to_owned()).map_err(|_| invalid()))
            .transpose()?;
        windows.push(UsageWindow {
            id,
            label,
            reading: Observation::reported(reading, source.clone()),
            resets_at: reset,
            duration_seconds: None,
        });
    }
    if windows.is_empty() {
        return Ok(vec![]);
    }
    Ok(vec![QuotaPool {
        id: stable_id("quota", &[connection, "claude-native-usage"]),
        account_id: account,
        connection_ids: vec![connection.into()],
        route_ids: vec![route.into()],
        sharing: "unknown".into(),
        windows,
    }])
}
