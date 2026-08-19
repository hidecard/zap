use crate::Value;

/// Evaluate a standard-library unary operation after its argument is parsed.
pub(crate) fn unary(name: &str, value: Value) -> Result<Option<Value>, String> {
    let result = match name {
        "sqrt" => match value {
            Value::Number(x) if x >= 0 => Value::Number((x as f64).sqrt().round() as i64),
            _ => return Err("sqrt expects a non-negative number".into()),
        },
        "abs" => match value {
            Value::Number(x) => Value::Number(x.abs()),
            _ => return Err("abs expects a number".into()),
        },
        "upper" => match value {
            Value::Text(x) => Value::Text(x.to_uppercase()),
            _ => return Err("upper expects text".into()),
        },
        "lower" => match value {
            Value::Text(x) => Value::Text(x.to_lowercase()),
            _ => return Err("lower expects text".into()),
        },
        "trim" => match value {
            Value::Text(x) => Value::Text(x.trim().into()),
            _ => return Err("trim expects text".into()),
        },
        _ => return Ok(None),
    };
    Ok(Some(result))
}

/// Evaluate a standard-library binary operation after both arguments are parsed.
pub(crate) fn binary(name: &str, left: Value, right: Value) -> Result<Option<Value>, String> {
    let result = match name {
        "pow" => match (left, right) {
            (Value::Number(a), Value::Number(b)) if b >= 0 => Value::Number(
                a.checked_pow(b as u32)
                    .ok_or("pow result overflow".to_string())?,
            ),
            _ => return Err("pow expects a number and a non-negative exponent".into()),
        },
        "min" => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a.min(b)),
            _ => return Err("min expects numbers".into()),
        },
        "max" => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a.max(b)),
            _ => return Err("max expects numbers".into()),
        },
        "split" => match (left, right) {
            (Value::Text(text), Value::Text(separator)) => Value::List(
                text.split(&separator)
                    .map(|part| Value::Text(part.into()))
                    .collect(),
            ),
            _ => return Err("split expects text and separator".into()),
        },
        _ => return Ok(None),
    };
    Ok(Some(result))
}
