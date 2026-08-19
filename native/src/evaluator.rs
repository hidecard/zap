use std::{collections::HashMap, path::Path, rc::Rc};

use crate::lexer::{tokenize, Token};
use crate::{execute_lines, ExprParser, Function, Value};

pub(crate) enum Flow {
    Continue,
    Break,
    LoopContinue,
    Return(Value),
}
pub(crate) enum EvalOutcome {
    Value(Value),
    Propagate(Value),
}
pub(crate) fn evaluate_with_propagation(
    raw: &str,
    vars: &HashMap<String, Value>,
    funcs: &HashMap<String, Rc<Function>>,
) -> Result<EvalOutcome, String> {
    let trimmed = raw.trim();
    if let Some(inner) = trimmed.strip_suffix('?') {
        let value = expression(inner.trim(), vars, funcs)?;
        match value {
            Value::ResultOk(value) => Ok(EvalOutcome::Value(*value)),
            Value::ResultErr(error) => Ok(EvalOutcome::Propagate(Value::ResultErr(error))),
            _ => Err("? expects a Result value".into()),
        }
    } else {
        Ok(EvalOutcome::Value(expression(trimmed, vars, funcs)?))
    }
}

pub(crate) fn operate(a: Value, op: Token, b: Value) -> Result<Value, String> {
    match (a, op, b) {
        (Value::Number(x), Token::Plus, Value::Number(y)) => x
            .checked_add(y)
            .map(Value::Number)
            .ok_or("integer overflow".into()),
        (Value::Number(x), Token::Minus, Value::Number(y)) => x
            .checked_sub(y)
            .map(Value::Number)
            .ok_or("integer overflow".into()),
        (Value::Number(x), Token::Star, Value::Number(y)) => x
            .checked_mul(y)
            .map(Value::Number)
            .ok_or("integer overflow".into()),
        (Value::Number(_), Token::Slash, Value::Number(0)) => Err("division by zero".into()),
        (Value::Number(i64::MIN), Token::Slash, Value::Number(-1)) => {
            Err("integer overflow".into())
        }
        (Value::Number(x), Token::Slash, Value::Number(y)) => Ok(Value::Number(x / y)),
        (Value::Number(_), Token::Percent, Value::Number(0)) => Err("division by zero".into()),
        (Value::Number(i64::MIN), Token::Percent, Value::Number(-1)) => {
            Err("integer overflow".into())
        }
        (Value::Number(x), Token::Percent, Value::Number(y)) => Ok(Value::Number(x % y)),
        (Value::Text(x), Token::Plus, Value::Text(y)) => Ok(Value::Text(x + &y)),
        (Value::Bool(x), Token::And, Value::Bool(y)) => Ok(Value::Bool(x && y)),
        (Value::Bool(x), Token::Or, Value::Bool(y)) => Ok(Value::Bool(x || y)),
        (x, Token::EqEq, y) => Ok(Value::Bool(x == y)),
        (x, Token::NotEq, y) => Ok(Value::Bool(x != y)),
        (Value::Number(x), Token::Less, Value::Number(y)) => Ok(Value::Bool(x < y)),
        (Value::Number(x), Token::Greater, Value::Number(y)) => Ok(Value::Bool(x > y)),
        (Value::Number(x), Token::LessEq, Value::Number(y)) => Ok(Value::Bool(x <= y)),
        (Value::Number(x), Token::GreaterEq, Value::Number(y)) => Ok(Value::Bool(x >= y)),
        _ => Err("invalid operation".into()),
    }
}
pub(crate) fn value_to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::None => serde_json::Value::Null,
        Value::Bool(x) => serde_json::Value::Bool(*x),
        Value::Number(x) => serde_json::Value::Number((*x).into()),
        Value::Text(x) => serde_json::Value::String(x.clone()),
        Value::List(xs) => serde_json::Value::Array(xs.iter().map(value_to_json).collect()),
        Value::Map(m) => serde_json::Value::Object(
            m.iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect(),
        ),
        Value::Object { class_name, fields } => {
            let mut object = serde_json::Map::new();
            object.insert(
                "__class".into(),
                serde_json::Value::String(class_name.clone()),
            );
            for (k, v) in fields.borrow().iter() {
                object.insert(k.clone(), value_to_json(v));
            }
            serde_json::Value::Object(object)
        }
        Value::ResultOk(x) => serde_json::json!({"__zap_variant":"ok","value":value_to_json(x)}),
        Value::ResultErr(x) => serde_json::json!({"__zap_variant":"err","value":value_to_json(x)}),
        Value::OptionSome(x) => {
            serde_json::json!({"__zap_variant":"some","value":value_to_json(x)})
        }
        Value::OptionNone => serde_json::json!({"__zap_variant":"none"}),
    }
}
pub(crate) fn json_to_value(v: serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::None,
        serde_json::Value::Bool(x) => Value::Bool(x),
        serde_json::Value::Number(x) => Value::Number(x.as_i64().unwrap_or(0)),
        serde_json::Value::String(x) => Value::Text(x),
        serde_json::Value::Array(xs) => Value::List(xs.into_iter().map(json_to_value).collect()),
        serde_json::Value::Object(mut m) => match m.remove("__zap_variant") {
            Some(serde_json::Value::String(tag)) => match tag.as_str() {
                "ok" => Value::ResultOk(Box::new(json_to_value(
                    m.remove("value").unwrap_or(serde_json::Value::Null),
                ))),
                "err" => Value::ResultErr(Box::new(json_to_value(
                    m.remove("value").unwrap_or(serde_json::Value::Null),
                ))),
                "some" => Value::OptionSome(Box::new(json_to_value(
                    m.remove("value").unwrap_or(serde_json::Value::Null),
                ))),
                "none" => Value::OptionNone,
                _ => Value::Map(m.into_iter().map(|(k, v)| (k, json_to_value(v))).collect()),
            },
            _ => Value::Map(m.into_iter().map(|(k, v)| (k, json_to_value(v))).collect()),
        },
    }
}

pub(crate) fn expression(
    raw: &str,
    vars: &HashMap<String, Value>,
    funcs: &HashMap<String, Rc<Function>>,
) -> Result<Value, String> {
    ExprParser::new(&tokenize(raw)?, vars, funcs).parse(0)
}

pub(crate) fn call_function(
    f: &Function,
    args: Vec<Value>,
    funcs: &HashMap<String, Rc<Function>>,
) -> Result<Value, String> {
    if args.len() != f.params.len() {
        return Err(format!(
            "function expects {} arguments, got {}",
            f.params.len(),
            args.len()
        ));
    }
    let mut local = f.closure.clone();
    for (param, v) in f.params.iter().zip(args) {
        if let Some(annotation) = &param.annotation {
            check_annotation(&param.name, annotation, &v)?;
        }
        local.insert(param.name.clone(), v);
    }
    let mut local_funcs = funcs.clone();
    let value = match execute_lines(&f.body, &mut local, &mut local_funcs, Path::new("."))? {
        Flow::Return(v) => v,
        Flow::Continue => Value::None,
        Flow::Break | Flow::LoopContinue => {
            return Err("break/continue cannot be used outside a loop".into())
        }
    };
    if let Some(annotation) = &f.return_annotation {
        check_annotation("return", annotation, &value)?;
    }
    Ok(value)
}
pub(crate) fn call_method(
    f: &Function,
    args: Vec<Value>,
    self_value: Value,
    funcs: &HashMap<String, Rc<Function>>,
) -> Result<Value, String> {
    if args.len() + 1 != f.params.len() {
        return Err(format!(
            "method expects {} arguments after self, got {}",
            f.params.len().saturating_sub(1),
            args.len()
        ));
    }
    let mut local = f.closure.clone();
    local.insert("self".into(), self_value);
    for (param, v) in f.params.iter().skip(1).zip(args) {
        if let Some(annotation) = &param.annotation {
            check_annotation(&param.name, annotation, &v)?;
        }
        local.insert(param.name.clone(), v);
    }
    let mut local_funcs = funcs.clone();
    let value = match execute_lines(&f.body, &mut local, &mut local_funcs, Path::new("."))? {
        Flow::Return(v) => v,
        Flow::Continue => Value::None,
        Flow::Break | Flow::LoopContinue => {
            return Err("break/continue cannot be used outside a loop".into())
        }
    };
    if let Some(annotation) = &f.return_annotation {
        check_annotation("return", annotation, &value)?;
    }
    Ok(value)
}

pub(crate) fn indented(lines: &[String], start: usize) -> (Vec<String>, usize) {
    let mut i = start;
    let mut body = Vec::new();
    while i < lines.len() && (lines[i].starts_with(' ') || lines[i].starts_with('\t')) {
        let line = &lines[i];
        let normalized = if let Some(stripped) = line.strip_prefix('\t') {
            stripped.to_string()
        } else {
            line.strip_prefix("    ").unwrap_or(line).to_string()
        };
        body.push(normalized);
        i += 1;
    }
    (body, i)
}
fn value_type(v: &Value) -> &'static str {
    match v {
        Value::Text(_) => "text",
        Value::Number(_) => "number",
        Value::Bool(_) => "bool",
        Value::List(_) => "list",
        Value::Map(_) => "map",
        Value::Object { .. } => "object",
        Value::ResultOk(_) | Value::ResultErr(_) => "result",
        Value::OptionSome(_) | Value::OptionNone => "option",
        Value::None => "none",
    }
}
pub(crate) fn check_annotation(name: &str, annotation: &str, value: &Value) -> Result<(), String> {
    let expected = annotation.trim();
    if expected.is_empty() {
        return Ok(());
    }
    let valid = matches!(
        (expected, value_type(value)),
        ("text", "text")
            | ("number", "number")
            | ("bool", "bool")
            | ("list", "list")
            | ("map", "map")
            | ("result", "result")
            | ("option", "option")
            | ("none", "none")
            | ("any", _)
    );
    if valid {
        Ok(())
    } else {
        Err(format!(
            "type mismatch for {name}: expected {expected}, got {}",
            value_type(value)
        ))
    }
}
