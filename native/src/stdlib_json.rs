use crate::value::Value;

/// JSON operations standard library module
pub(crate) fn json_encode(value: &Value) -> Result<Value, String> {
    let json_string = serde_json::to_string_pretty(value)
        .map_err(|e| format!("failed to encode JSON: {}", e))?;
    Ok(Value::Text(json_string))
}

pub(crate) fn json_decode(json_string: &str) -> Result<Value, String> {
    let value: Value = serde_json::from_str(json_string)
        .map_err(|e| format!("failed to decode JSON: {}", e))?;
    Ok(value)
}

pub(crate) fn json_parse(json_string: &str) -> Result<Value, String> {
    json_decode(json_string)
}

pub(crate) fn json_stringify(value: &Value) -> Result<Value, String> {
    json_encode(value)
}