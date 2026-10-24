//! Canonical public contracts. Functions are initially test scaffolds.
use serde_json::Value;
pub fn validate_observation(_: Value) -> Result<Value, String> { unimplemented!() }
pub fn validate_option(_: Value, _: Value) -> Result<(), String> { unimplemented!() }
pub fn normalize_usage(_: Value) -> Result<Value, String> { unimplemented!() }
pub fn route_model_key(_: &str, _: &str) -> String { unimplemented!() }
pub fn schema_document() -> Value { unimplemented!() }
