use lai_contracts::{DecimalString, ModelPrice, estimate_cost};
fn decimal(value: &str) -> DecimalString {
    value.to_string().try_into().unwrap()
}
fn price(input: &str, output: &str) -> ModelPrice {
    ModelPrice {
        provider_id: "fixture".into(),
        native_model_id: "exact-model".into(),
        input_usd_per_million: decimal(input),
        output_usd_per_million: decimal(output),
        reviewed_at: "2026-10-28".into(),
        source: "fixture".into(),
    }
}
#[test]
fn small_costs_use_decimal_arithmetic_without_rounding_to_zero() {
    assert_eq!(
        estimate_cost(&decimal("1"), &decimal("1"), &price("0.1", "0.2"))
            .unwrap()
            .as_str(),
        "0.0000003"
    );
    assert_eq!(
        estimate_cost(&decimal("1800"), &decimal("400"), &price("3", "15"))
            .unwrap()
            .as_str(),
        "0.0114"
    );
}
#[test]
fn zero_price_is_explicit_and_oversized_results_are_unknown() {
    assert_eq!(
        estimate_cost(&decimal("100"), &decimal("200"), &price("0", "0"))
            .unwrap()
            .as_str(),
        "0"
    );
    assert!(
        estimate_cost(
            &decimal(&"9".repeat(128)),
            &decimal("0"),
            &price(&"9".repeat(24), "0")
        )
        .is_none()
    );
}
