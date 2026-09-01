use crate::evaluator::{json_to_value, value_to_json};
use crate::value::Value;

/// JSON operations standard library module.
pub(crate) fn json_encode(value: &Value) -> Result<Value, String> {
    let json = value_to_json(value)?;
    serde_json::to_string_pretty(&json)
        .map(Value::Text)
        .map_err(|error| format!("failed to encode JSON: {error}"))
}

pub(crate) fn json_decode(json_string: &str) -> Result<Value, String> {
    let json = serde_json::from_str(json_string)
        .map_err(|error| format!("failed to decode JSON: {error}"))?;
    json_to_value(json)
}

pub(crate) fn json_parse(json_string: &str) -> Result<Value, String> {
    json_decode(json_string)
}

pub(crate) fn json_stringify(value: &Value) -> Result<Value, String> {
    json_encode(value)
}
