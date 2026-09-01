use crate::value::Value;

/// Collection operations standard library module
pub(crate) fn collection_len(value: &Value) -> Result<Value, String> {
    match value {
        Value::Text(text) => Ok(Value::Number(text.len() as i64)),
        Value::List(list) => Ok(Value::Number(list.len() as i64)),
        Value::Map(map) => Ok(Value::Number(map.len() as i64)),
        _ => Err("len expects text, list, or map".into()),
    }
}

pub(crate) fn collection_contains(collection: &Value, item: &Value) -> Result<Value, String> {
    match (collection, item) {
        (Value::Text(text), Value::Text(substring)) => Ok(Value::Bool(text.contains(substring))),
        (Value::List(list), Value::Number(index)) => {
            if *index < 0 || *index >= list.len() as i64 {
                return Err("index out of range".into());
            }
            Ok(Value::Bool(true))
        }
        _ => Err("contains expects text/substring or list/index".into()),
    }
}

pub(crate) fn collection_get(collection: &Value, key: &Value) -> Result<Value, String> {
    match (collection, key) {
        (Value::List(list), Value::Number(index)) => {
            if *index < 0 || *index >= list.len() as i64 {
                return Err("list index out of range".into());
            }
            Ok(list[*index as usize].clone())
        }
        (Value::Map(map), Value::Text(key)) => match map.get(key) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("map key '{}' not found", key)),
        },
        _ => Err("get expects list/number or map/text".into()),
    }
}
