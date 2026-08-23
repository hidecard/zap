use crate::value::collect_bounded_values;
use crate::Value;

pub(crate) const MAX_SLEEP_MILLISECONDS: i64 = 60_000;
pub(crate) const MAX_POW_EXPONENT: i64 = 1_000_000;

pub(crate) fn checked_integer_pow(base: i64, exponent: i64) -> Result<i64, String> {
    if exponent < 0 {
        return Err("pow expects a non-negative exponent".into());
    }
    if exponent > MAX_POW_EXPONENT {
        return Err(format!("pow exponent exceeds the {MAX_POW_EXPONENT} limit"));
    }
    let mut exponent = exponent as u64;
    let mut factor = base;
    let mut result = 1_i64;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result
                .checked_mul(factor)
                .ok_or_else(|| "pow result overflow".to_string())?;
        }
        exponent >>= 1;
        if exponent > 0 {
            factor = factor
                .checked_mul(factor)
                .ok_or_else(|| "pow result overflow".to_string())?;
        }
    }
    Ok(result)
}

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
            (Value::Number(a), Value::Number(b)) => Value::Number(checked_integer_pow(a, b)?),
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
            (Value::Text(text), Value::Text(separator)) => Value::List(collect_bounded_values(
                text.split(&separator).map(|part| Value::Text(part.into())),
                "split",
            )?),
            _ => return Err("split expects text and separator".into()),
        },
        _ => return Ok(None),
    };
    Ok(Some(result))
}
