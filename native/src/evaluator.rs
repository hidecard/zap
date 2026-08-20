use std::{collections::HashMap, path::Path, rc::Rc};

use std::cell::{Cell, RefCell};

use crate::ast::{BinaryOp, CallArg, Expr, Literal, Program, Stmt, UnaryOp};
use crate::lexer::{tokenize, Token};
use crate::ExprParser;
use crate::{
    parse_signature, read_limited_text, resolve_module, write_limited_text, Function, Param, Value,
    MODULE_CACHE, MODULE_LOADING,
};

const MAX_EXECUTION_DEPTH: usize = 256;
const MAX_SOURCE_LINES: usize = 100_000;
const MAX_LOOP_ITERATIONS: usize = 100_000;
const MAX_JSON_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone, Debug)]
pub(crate) struct CallArgument {
    pub(crate) name: Option<String>,
    pub(crate) value: Value,
}

thread_local! {
    static EXECUTION_DEPTH: Cell<usize> = const { Cell::new(0) };
}

struct ExecutionGuard;

impl Drop for ExecutionGuard {
    fn drop(&mut self) {
        EXECUTION_DEPTH.with(|depth| depth.set(depth.get().saturating_sub(1)));
    }
}

#[allow(clippy::manual_is_multiple_of)]
fn validate_indentation(lines: &[String]) -> Result<(), String> {
    let mut style: Option<&'static str> = None;
    for (index, line) in lines.iter().enumerate() {
        let prefix: String = line
            .chars()
            .take_while(|ch| *ch == ' ' || *ch == '\t')
            .collect();
        if prefix.is_empty() {
            continue;
        }
        let has_spaces = prefix.contains(' ');
        let has_tabs = prefix.contains('\t');
        if has_spaces && has_tabs {
            return Err(format!(
                "mixed indentation at line {}: use spaces or tabs, not both",
                index + 1
            ));
        }
        if has_spaces && prefix.chars().count() % 4 != 0 {
            return Err(format!(
                "invalid indentation at line {}: spaces must be groups of four",
                index + 1
            ));
        }
        let current = if has_tabs { "tabs" } else { "spaces" };
        if let Some(previous) = style {
            if previous != current {
                return Err(format!(
                    "mixed indentation at line {}: file uses both tabs and spaces",
                    index + 1
                ));
            }
        } else {
            style = Some(current);
        }
    }
    Ok(())
}

pub(crate) fn validate_source_layout(source: &str) -> Result<(), String> {
    let lines = source.lines().map(str::to_string).collect::<Vec<_>>();
    validate_indentation(&lines)?;
    if lines.len() > MAX_SOURCE_LINES {
        return Err(format!(
            "source line limit exceeded: maximum is {MAX_SOURCE_LINES}"
        ));
    }
    Ok(())
}

fn enter_execution(lines: &[String]) -> Result<ExecutionGuard, String> {
    validate_indentation(lines)?;
    if lines.len() > MAX_SOURCE_LINES {
        return Err(format!(
            "source line limit exceeded: maximum is {MAX_SOURCE_LINES}"
        ));
    }
    EXECUTION_DEPTH.with(|depth| {
        if depth.get() >= MAX_EXECUTION_DEPTH {
            Err(format!(
                "execution depth limit exceeded: maximum is {MAX_EXECUTION_DEPTH}"
            ))
        } else {
            depth.set(depth.get() + 1);
            Ok(ExecutionGuard)
        }
    })
}

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
            Value::ResultOk(value) | Value::OptionSome(value) => Ok(EvalOutcome::Value(*value)),
            Value::ResultErr(error) => Ok(EvalOutcome::Propagate(Value::ResultErr(error))),
            Value::OptionNone => Ok(EvalOutcome::Propagate(Value::OptionNone)),
            _ => Err("? expects a Result or Option value".into()),
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
pub(crate) fn json_to_value(v: serde_json::Value) -> Result<Value, String> {
    match v {
        serde_json::Value::Null => Ok(Value::None),
        serde_json::Value::Bool(x) => Ok(Value::Bool(x)),
        serde_json::Value::Number(x) => x
            .as_i64()
            .map(Value::Number)
            .ok_or_else(|| "JSON number is outside Zap's integer range".to_string()),
        serde_json::Value::String(x) => Ok(Value::Text(x)),
        serde_json::Value::Array(xs) => xs
            .into_iter()
            .map(json_to_value)
            .collect::<Result<Vec<_>, _>>()
            .map(Value::List),
        serde_json::Value::Object(mut m) => match m.remove("__zap_variant") {
            Some(serde_json::Value::String(tag)) => match tag.as_str() {
                "ok" | "err" | "some" => {
                    let value = m
                        .remove("value")
                        .ok_or_else(|| format!("JSON {tag} variant is missing its value"))?;
                    let value = json_to_value(value)?;
                    Ok(match tag.as_str() {
                        "ok" => Value::ResultOk(Box::new(value)),
                        "err" => Value::ResultErr(Box::new(value)),
                        _ => Value::OptionSome(Box::new(value)),
                    })
                }
                "none" => Ok(Value::OptionNone),
                _ => Err(format!("unknown Zap JSON variant: {tag}")),
            },
            Some(other) => Err(format!("Zap JSON variant must be text, got {other}")),
            None => m
                .into_iter()
                .map(|(k, value)| json_to_value(value).map(|value| (k, value)))
                .collect::<Result<HashMap<_, _>, _>>()
                .map(Value::Map),
        },
    }
}

pub(crate) fn expression(
    raw: &str,
    vars: &HashMap<String, Value>,
    funcs: &HashMap<String, Rc<Function>>,
) -> Result<Value, String> {
    let tokens = tokenize(raw)?;
    ExprParser::new(&tokens, vars, funcs).parse_complete()
}

pub(crate) fn direct_builtin(name: &str, args: Vec<Value>) -> Result<Option<Value>, String> {
    let expect = |count: usize| {
        if args.len() == count {
            Ok(())
        } else {
            Err(format!(
                "{name} expects {count} arguments, got {}",
                args.len()
            ))
        }
    };
    match name {
        "json" => {
            expect(1)?;
            let encoded = serde_json::to_string(&value_to_json(&args[0]))
                .map_err(|error| format!("json encode failed: {error}"))?;
            if encoded.len() > MAX_JSON_BYTES {
                return Err(format!(
                    "json encode failed: output exceeds the {MAX_JSON_BYTES} byte limit"
                ));
            }
            Ok(Some(Value::Text(encoded)))
        }
        "from_json" => {
            expect(1)?;
            let Value::Text(text) = &args[0] else {
                return Err("from_json expects text".into());
            };
            if text.len() > MAX_JSON_BYTES {
                return Err(format!(
                    "from_json failed: input exceeds the {MAX_JSON_BYTES} byte limit"
                ));
            }
            let parsed =
                serde_json::from_str(text).map_err(|error| format!("from_json failed: {error}"))?;
            Ok(Some(json_to_value(parsed)?))
        }
        "len" => {
            expect(1)?;
            let length = match &args[0] {
                Value::Text(value) => value.chars().count(),
                Value::List(value) => value.len(),
                Value::Map(value) => value.len(),
                _ => return Err("len expects text, list, or map".into()),
            };
            Ok(Some(Value::Number(length as i64)))
        }
        "str" => {
            expect(1)?;
            Ok(Some(Value::Text(args[0].show())))
        }
        "type" => {
            expect(1)?;
            let type_name = match args[0] {
                Value::None => "none",
                Value::Bool(_) => "bool",
                Value::Number(_) => "number",
                Value::Text(_) => "text",
                Value::List(_) => "list",
                Value::Map(_) => "map",
                Value::Object { .. } => "object",
                Value::ResultOk(_) | Value::ResultErr(_) => "result",
                Value::OptionSome(_) | Value::OptionNone => "option",
            };
            Ok(Some(Value::Text(type_name.into())))
        }
        "keys" => {
            expect(1)?;
            match &args[0] {
                Value::Map(values) => Ok(Some(Value::List(
                    values.keys().cloned().map(Value::Text).collect(),
                ))),
                _ => Err("keys expects a map".into()),
            }
        }
        "contains" => {
            expect(2)?;
            match (&args[0], &args[1]) {
                (Value::Text(value), Value::Text(part)) => {
                    Ok(Some(Value::Bool(value.contains(part))))
                }
                (Value::List(values), item) => Ok(Some(Value::Bool(values.contains(item)))),
                _ => Err("contains expects text/text or list/value".into()),
            }
        }
        "is_empty" => {
            expect(1)?;
            let empty = match &args[0] {
                Value::Text(value) => value.is_empty(),
                Value::List(value) => value.is_empty(),
                Value::Map(value) => value.is_empty(),
                _ => return Err("is_empty expects text, list, or map".into()),
            };
            Ok(Some(Value::Bool(empty)))
        }
        "split" => {
            expect(2)?;
            match (&args[0], &args[1]) {
                (Value::Text(value), Value::Text(separator)) => Ok(Some(Value::List(
                    value
                        .split(separator)
                        .map(|part| Value::Text(part.into()))
                        .collect(),
                ))),
                _ => Err("split expects text and text separator".into()),
            }
        }
        "join" => {
            expect(2)?;
            let (Value::List(values), Value::Text(separator)) = (&args[0], &args[1]) else {
                return Err("join expects a list of text and a separator".into());
            };
            let mut output = String::new();
            for (index, value) in values.iter().enumerate() {
                let Value::Text(value) = value else {
                    return Err("join expects a list of text and a separator".into());
                };
                if index > 0 {
                    output.push_str(separator);
                }
                output.push_str(value);
            }
            Ok(Some(Value::Text(output)))
        }
        "trim" | "lower" | "upper" => {
            expect(1)?;
            let Value::Text(value) = &args[0] else {
                return Err(format!("{name} expects text"));
            };
            let output = match name {
                "trim" => value.trim().to_string(),
                "lower" => value.to_lowercase(),
                _ => value.to_uppercase(),
            };
            Ok(Some(Value::Text(output)))
        }
        "replace" => {
            expect(3)?;
            let (Value::Text(value), Value::Text(from), Value::Text(to)) =
                (&args[0], &args[1], &args[2])
            else {
                return Err("replace expects text, text, and text".into());
            };
            Ok(Some(Value::Text(value.replace(from, to))))
        }
        "starts_with" | "ends_with" => {
            expect(2)?;
            let (Value::Text(value), Value::Text(part)) = (&args[0], &args[1]) else {
                return Err(format!("{name} expects text and text"));
            };
            let matched = if name == "starts_with" {
                value.starts_with(part)
            } else {
                value.ends_with(part)
            };
            Ok(Some(Value::Bool(matched)))
        }
        "abs" => {
            expect(1)?;
            let Value::Number(value) = args[0] else {
                return Err("abs expects a number".into());
            };
            value
                .checked_abs()
                .map(Value::Number)
                .map(Some)
                .ok_or_else(|| "integer overflow".into())
        }
        "min" | "max" => {
            expect(2)?;
            let (Value::Number(left), Value::Number(right)) = (&args[0], &args[1]) else {
                return Err(format!("{name} expects two numbers"));
            };
            Ok(Some(Value::Number(if name == "min" {
                (*left).min(*right)
            } else {
                (*left).max(*right)
            })))
        }
        "pow" => {
            expect(2)?;
            let (Value::Number(base), Value::Number(exponent)) = (&args[0], &args[1]) else {
                return Err("pow expects two numbers".into());
            };
            if *exponent < 0 {
                return Err("pow expects a non-negative exponent".into());
            }
            let mut result = 1_i64;
            for _ in 0..(*exponent as u64) {
                result = result.checked_mul(*base).ok_or("integer overflow")?;
            }
            Ok(Some(Value::Number(result)))
        }
        "count" => {
            expect(2)?;
            let (Value::List(values), item) = (&args[0], &args[1]) else {
                return Err("count expects a list and a value".into());
            };
            Ok(Some(Value::Number(
                values.iter().filter(|value| *value == item).count() as i64,
            )))
        }
        "sum" => {
            expect(1)?;
            let Value::List(values) = &args[0] else {
                return Err("sum expects a list".into());
            };
            let mut total = 0_i64;
            for value in values {
                let Value::Number(value) = value else {
                    return Err("sum expects a list of numbers".into());
                };
                total = total.checked_add(*value).ok_or("integer overflow")?;
            }
            Ok(Some(Value::Number(total)))
        }
        "reverse" => {
            expect(1)?;
            let Value::List(values) = &args[0] else {
                return Err("reverse expects a list".into());
            };
            let mut values = values.clone();
            values.reverse();
            Ok(Some(Value::List(values)))
        }
        "range" => {
            if args.len() != 1 && args.len() != 2 {
                return Err(format!(
                    "range expects one or two arguments, got {}",
                    args.len()
                ));
            }
            let (start, end) = match args.as_slice() {
                [Value::Number(end)] => (0, *end),
                [Value::Number(start), Value::Number(end)] => (*start, *end),
                _ => return Err("range expects numeric arguments".into()),
            };
            Ok(Some(Value::List((start..end).map(Value::Number).collect())))
        }
        "ok" | "err" | "some" => {
            expect(1)?;
            Ok(Some(match name {
                "ok" => Value::ResultOk(Box::new(args[0].clone())),
                "err" => Value::ResultErr(Box::new(args[0].clone())),
                _ => Value::OptionSome(Box::new(args[0].clone())),
            }))
        }
        "option_none" => {
            expect(0)?;
            Ok(Some(Value::OptionNone))
        }
        "is_ok" | "is_err" | "is_some" | "is_option_none" => {
            expect(1)?;
            let value = &args[0];
            let result = match name {
                "is_ok" => matches!(value, Value::ResultOk(_)),
                "is_err" => matches!(value, Value::ResultErr(_)),
                "is_some" => matches!(value, Value::OptionSome(_)),
                _ => matches!(value, Value::OptionNone),
            };
            Ok(Some(Value::Bool(result)))
        }
        _ => Ok(None),
    }
}

fn direct_io_builtin(name: &str, args: &[Value]) -> Result<Option<Value>, String> {
    match name {
        "read_text" => {
            if args.len() != 1 {
                return Err(format!("read_text expects 1 argument, got {}", args.len()));
            }
            let Value::Text(path) = &args[0] else {
                return Err("read_text expects a text path".into());
            };
            Ok(Some(Value::Text(read_limited_text(
                Path::new(path),
                "read_text",
            )?)))
        }
        "write_text" => {
            if args.len() != 2 {
                return Err(format!(
                    "write_text expects 2 arguments, got {}",
                    args.len()
                ));
            }
            let (Value::Text(path), Value::Text(content)) = (&args[0], &args[1]) else {
                return Err("write_text expects text path and content".into());
            };
            write_limited_text(Path::new(path), content, "write_text")?;
            Ok(Some(Value::None))
        }
        "read_lines" => {
            if args.len() != 1 {
                return Err(format!("read_lines expects 1 argument, got {}", args.len()));
            }
            let Value::Text(path) = &args[0] else {
                return Err("read_lines expects a text path".into());
            };
            Ok(Some(Value::List(
                read_limited_text(Path::new(path), "read_lines")?
                    .lines()
                    .map(|line| Value::Text(line.to_string()))
                    .collect(),
            )))
        }
        "write_lines" => {
            if args.len() != 2 {
                return Err(format!(
                    "write_lines expects 2 arguments, got {}",
                    args.len()
                ));
            }
            let (Value::Text(path), Value::List(lines)) = (&args[0], &args[1]) else {
                return Err("write_lines expects a text path and list".into());
            };
            let mut output = String::new();
            for (index, value) in lines.iter().enumerate() {
                let Value::Text(line) = value else {
                    return Err("write_lines expects a list of text".into());
                };
                if index > 0 {
                    output.push('\n');
                }
                output.push_str(line);
            }
            write_limited_text(Path::new(path), &output, "write_lines")?;
            Ok(Some(Value::None))
        }
        _ => Ok(None),
    }
}

fn direct_system_builtin(name: &str, args: &[Value]) -> Result<Option<Value>, String> {
    match name {
        "now" => {
            if !args.is_empty() {
                return Err(format!("now expects 0 arguments, got {}", args.len()));
            }
            let seconds = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|_| "system clock is before Unix epoch".to_string())?
                .as_secs() as i64;
            Ok(Some(Value::Number(seconds)))
        }
        "sleep" => {
            if args.len() != 1 {
                return Err(format!("sleep expects 1 argument, got {}", args.len()));
            }
            let Value::Number(milliseconds) = args[0] else {
                return Err("sleep expects a non-negative number of milliseconds".into());
            };
            if milliseconds < 0 {
                return Err("sleep expects a non-negative number of milliseconds".into());
            }
            std::thread::sleep(std::time::Duration::from_millis(milliseconds as u64));
            Ok(Some(Value::None))
        }
        "env" | "has_env" => {
            if args.len() != 1 {
                return Err(format!("{name} expects 1 argument, got {}", args.len()));
            }
            let Value::Text(key) = &args[0] else {
                return Err(format!("{name} expects a text key"));
            };
            if name == "env" {
                Ok(Some(Value::Text(std::env::var(key).unwrap_or_default())))
            } else {
                Ok(Some(Value::Bool(std::env::var_os(key).is_some())))
            }
        }
        "exists" => {
            if args.len() != 1 {
                return Err(format!("exists expects 1 argument, got {}", args.len()));
            }
            let Value::Text(path) = &args[0] else {
                return Err("exists expects a text path".into());
            };
            Ok(Some(Value::Bool(Path::new(path).exists())))
        }
        "path_join" => {
            let mut path = std::path::PathBuf::new();
            for value in args {
                let Value::Text(part) = value else {
                    return Err("path_join expects text parts".into());
                };
                path.push(part);
            }
            Ok(Some(Value::Text(path.to_string_lossy().into())))
        }
        "basename" | "dirname" => {
            if args.len() != 1 {
                return Err(format!("{name} expects 1 argument, got {}", args.len()));
            }
            let Value::Text(path) = &args[0] else {
                return Err(format!("{name} expects a text path"));
            };
            let value = if name == "basename" {
                Path::new(path)
                    .file_name()
                    .and_then(|part| part.to_str())
                    .unwrap_or("")
            } else {
                Path::new(path)
                    .parent()
                    .and_then(|part| part.to_str())
                    .unwrap_or("")
            };
            Ok(Some(Value::Text(value.into())))
        }
        _ => Ok(None),
    }
}

fn is_same_or_subclass(current: &str, target: &str, funcs: &HashMap<String, Rc<Function>>) -> bool {
    let mut class = current.to_string();
    let mut visited = std::collections::HashSet::new();
    loop {
        if class == target {
            return true;
        }
        if !visited.insert(class.clone()) {
            return false;
        }
        let Some(parent) = funcs.get(&format!("{class}.__parent__")) else {
            return false;
        };
        let Some(Value::Text(parent)) = parent.body.first().map(|value| Value::Text(value.clone()))
        else {
            return false;
        };
        class = parent;
    }
}

fn check_method_visibility(
    function: &Function,
    dispatch_class: &str,
    vars: &HashMap<String, Value>,
    funcs: &HashMap<String, Rc<Function>>,
) -> Result<(), String> {
    if function.visibility == "public" {
        return Ok(());
    }
    let caller = match vars.get("__zap_owner_class") {
        Some(Value::Text(class)) => class.as_str(),
        _ => {
            return Err(format!(
                "{} method is not accessible from this context",
                function.visibility
            ))
        }
    };
    let allowed = if function.visibility == "private" {
        caller == dispatch_class
    } else {
        is_same_or_subclass(caller, dispatch_class, funcs)
    };
    if allowed {
        Ok(())
    } else {
        Err(format!(
            "{} method is not accessible from {caller}",
            function.visibility
        ))
    }
}

fn ast_expression(
    node: &crate::ast::Spanned<Expr>,
    vars: &HashMap<String, Value>,
    funcs: &HashMap<String, Rc<Function>>,
) -> Result<Value, String> {
    match &node.node {
        Expr::Literal(Literal::Number(value)) => Ok(Value::Number(*value)),
        Expr::Literal(Literal::Text(value)) => Ok(Value::Text(value.clone())),
        Expr::Literal(Literal::Bool(value)) => Ok(Value::Bool(*value)),
        Expr::Literal(Literal::None) => Ok(Value::None),
        Expr::Name(name) => vars
            .get(name)
            .cloned()
            .or_else(|| funcs.get(name).map(|_| Value::None))
            .ok_or_else(|| format!("undefined variable: {name}")),
        Expr::List(items) => items
            .iter()
            .map(|item| ast_expression(item, vars, funcs))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::List),
        Expr::Map(items) => {
            let mut map = HashMap::new();
            for (key, value) in items {
                let key = ast_expression(key, vars, funcs)?;
                let Value::Text(key) = key else {
                    return Err("map keys must be text".into());
                };
                map.insert(key, ast_expression(value, vars, funcs)?);
            }
            Ok(Value::Map(map))
        }
        Expr::Unary { op, value } => {
            let value = ast_expression(value, vars, funcs)?;
            match (op, value) {
                (UnaryOp::Negate, Value::Number(value)) => value
                    .checked_neg()
                    .map(Value::Number)
                    .ok_or_else(|| "integer overflow".into()),
                (UnaryOp::Not, value) => Ok(Value::Bool(!value.truthy())),
                (UnaryOp::Negate, _) => Err("unary '-' expects a number".into()),
            }
        }
        Expr::Binary { left, op, right } => {
            let left = ast_expression(left, vars, funcs)?;
            let right = ast_expression(right, vars, funcs)?;
            let token = match op {
                BinaryOp::Add => Token::Plus,
                BinaryOp::Subtract => Token::Minus,
                BinaryOp::Multiply => Token::Star,
                BinaryOp::Divide => Token::Slash,
                BinaryOp::Remainder => Token::Percent,
                BinaryOp::Equal => Token::EqEq,
                BinaryOp::NotEqual => Token::NotEq,
                BinaryOp::Less => Token::Less,
                BinaryOp::Greater => Token::Greater,
                BinaryOp::LessEqual => Token::LessEq,
                BinaryOp::GreaterEqual => Token::GreaterEq,
                BinaryOp::And => Token::And,
                BinaryOp::Or => Token::Or,
            };
            operate(left, token, right)
        }
        Expr::Call { callee, args } => {
            let values = args
                .iter()
                .map(|arg| match arg {
                    CallArg::Positional(value) => Ok(CallArgument {
                        name: None,
                        value: ast_expression(value, vars, funcs)?,
                    }),
                    CallArg::Named { name, value } => Ok(CallArgument {
                        name: Some(name.clone()),
                        value: ast_expression(value, vars, funcs)?,
                    }),
                })
                .collect::<Result<Vec<_>, String>>()?;
            match &callee.node {
                Expr::Name(name) => {
                    if let Some(function) = funcs.get(name) {
                        return call_function_with_arguments(function, values, funcs);
                    } else if values.iter().any(|argument| argument.name.is_some()) {
                        return Err(format!(
                            "named arguments are not supported for built-in function: {name}"
                        ));
                    }
                    let positional = values
                        .iter()
                        .map(|argument| argument.value.clone())
                        .collect::<Vec<_>>();
                    if let Some(value) = direct_builtin(name, positional.clone())? {
                        Ok(value)
                    } else if let Some(value) = direct_io_builtin(name, &positional)? {
                        Ok(value)
                    } else if let Some(value) = direct_system_builtin(name, &positional)? {
                        Ok(value)
                    } else {
                        expression(&ast_expr_source(node), vars, funcs)
                    }
                }
                Expr::Member { target, member } => {
                    let (dispatch_class, receiver) = if let Expr::Name(name) = &target.node {
                        if name == "super" {
                            let parent = match vars.get("super") {
                                Some(Value::Text(parent)) => parent.clone(),
                                _ => return Err("super is only available inside a method".into()),
                            };
                            let receiver = vars
                                .get("self")
                                .cloned()
                                .ok_or_else(|| "super requires self".to_string())?;
                            (parent, receiver)
                        } else {
                            let receiver = ast_expression(target, vars, funcs)?;
                            let Value::Object { class_name, .. } = &receiver else {
                                return Err("methods can only be called on objects".into());
                            };
                            (class_name.clone(), receiver)
                        }
                    } else {
                        let receiver = ast_expression(target, vars, funcs)?;
                        let Value::Object { class_name, .. } = &receiver else {
                            return Err("methods can only be called on objects".into());
                        };
                        (class_name.clone(), receiver)
                    };
                    let function = funcs
                        .get(&format!("{dispatch_class}.{}", member))
                        .ok_or_else(|| format!("undefined method: {dispatch_class}.{member}"))?
                        .clone();
                    check_method_visibility(&function, &dispatch_class, vars, funcs)?;
                    call_method_with_arguments(&function, values, receiver, funcs)
                }
                _ => expression(&ast_expr_source(node), vars, funcs),
            }
        }
        Expr::Member { target, member } => {
            let value = ast_expression(target, vars, funcs)?;
            match value {
                Value::Object { fields, .. } => fields
                    .borrow()
                    .get(member)
                    .cloned()
                    .ok_or_else(|| format!("property not found: {member}")),
                Value::Map(values) => values
                    .get(member)
                    .cloned()
                    .ok_or_else(|| format!("key not found: {member}")),
                _ => Err("property access expects an object or map".into()),
            }
        }
        Expr::Index { target, index } => {
            let target = ast_expression(target, vars, funcs)?;
            let index = ast_expression(index, vars, funcs)?;
            match (target, index) {
                (Value::List(values), Value::Number(index)) if index >= 0 => values
                    .get(index as usize)
                    .cloned()
                    .ok_or_else(|| "index out of range".to_string()),
                (Value::Map(values), Value::Text(key)) => values
                    .get(&key)
                    .cloned()
                    .ok_or_else(|| "key not found".to_string()),
                _ => Err("invalid index operation".into()),
            }
        }
    }
}

pub(crate) fn call_function(
    f: &Function,
    args: Vec<Value>,
    funcs: &HashMap<String, Rc<Function>>,
) -> Result<Value, String> {
    call_function_with_arguments(
        f,
        args.into_iter()
            .map(|value| CallArgument { name: None, value })
            .collect(),
        funcs,
    )
}

fn call_function_with_arguments(
    f: &Function,
    args: Vec<CallArgument>,
    funcs: &HashMap<String, Rc<Function>>,
) -> Result<Value, String> {
    let required = f
        .params
        .iter()
        .filter(|param| param.default.is_none())
        .count();
    if args.len() < required || args.len() > f.params.len() {
        return Err(format!(
            "function expects {} to {} arguments, got {}",
            required,
            f.params.len(),
            args.len()
        ));
    }
    let mut local = f.closure.borrow().clone();
    let captured_keys = local.keys().cloned().collect::<Vec<_>>();
    let mut positional_index = 0usize;
    let mut named = HashMap::new();
    let mut saw_named = false;
    for argument in args {
        if let Some(name) = argument.name {
            saw_named = true;
            if named.insert(name.clone(), argument.value).is_some() {
                return Err(format!("duplicate named argument: {name}"));
            }
        } else {
            if saw_named {
                return Err("positional argument cannot follow a named argument".into());
            }
            if positional_index >= f.params.len() {
                return Err(format!(
                    "function expects at most {} arguments",
                    f.params.len()
                ));
            }
            let parameter = &f.params[positional_index];
            named.insert(parameter.name.clone(), argument.value);
            positional_index += 1;
        }
    }
    for name in named.keys() {
        if !f.params.iter().any(|param| param.name == *name) {
            return Err(format!("unknown named argument: {name}"));
        }
    }
    for param in &f.params {
        let v = if let Some(value) = named.remove(&param.name) {
            value
        } else if let Some(default) = &param.default {
            expression(default, &local, funcs)?
        } else {
            return Err(format!("missing required argument: {}", param.name));
        };
        if let Some(annotation) = &param.annotation {
            check_annotation(&param.name, annotation, &v)?;
        }
        local.insert(param.name.clone(), v);
    }
    let mut local_funcs = funcs.clone();
    let value = match if let Some(body) = &f.ast_body {
        execute_ast_program(body, &mut local, &mut local_funcs, Path::new("."))
    } else {
        execute_lines(&f.body, &mut local, &mut local_funcs, Path::new("."))
    }? {
        Flow::Return(v) => v,
        Flow::Continue => Value::None,
        Flow::Break | Flow::LoopContinue => {
            return Err("break/continue cannot be used outside a loop".into())
        }
    };
    {
        let mut captured = f.closure.borrow_mut();
        for key in &captured_keys {
            if let Some(value) = local.get(key) {
                captured.insert(key.clone(), value.clone());
            }
        }
    }
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
    call_method_with_arguments(
        f,
        args.into_iter()
            .map(|value| CallArgument { name: None, value })
            .collect(),
        self_value,
        funcs,
    )
}

fn call_method_with_arguments(
    f: &Function,
    args: Vec<CallArgument>,
    self_value: Value,
    funcs: &HashMap<String, Rc<Function>>,
) -> Result<Value, String> {
    let callable_params = f.params.iter().skip(1).collect::<Vec<_>>();
    let required = callable_params
        .iter()
        .filter(|param| param.default.is_none())
        .count();
    if args.len() < required || args.len() > callable_params.len() {
        return Err(format!(
            "method expects {} to {} arguments after self, got {}",
            required,
            callable_params.len(),
            args.len()
        ));
    }
    let mut local = f.closure.borrow().clone();
    let captured_keys = local.keys().cloned().collect::<Vec<_>>();
    local.insert("self".into(), self_value);
    if let Some(Value::Text(owner_class)) = local.get("__zap_owner_class").cloned() {
        if let Some(Value::Text(parent_class)) = funcs
            .get(&format!("{owner_class}.__parent__"))
            .and_then(|parent| parent.body.first())
            .cloned()
            .map(Value::Text)
        {
            local.insert("super".into(), Value::Text(parent_class));
        }
    }
    let mut positional_index = 0usize;
    let mut named = HashMap::new();
    let mut saw_named = false;
    for argument in args {
        if let Some(name) = argument.name {
            saw_named = true;
            if named.insert(name.clone(), argument.value).is_some() {
                return Err(format!("duplicate named argument: {name}"));
            }
        } else {
            if saw_named {
                return Err("positional argument cannot follow a named argument".into());
            }
            if positional_index >= callable_params.len() {
                return Err(format!(
                    "method expects at most {} arguments after self",
                    callable_params.len()
                ));
            }
            let parameter = &callable_params[positional_index];
            named.insert(parameter.name.clone(), argument.value);
            positional_index += 1;
        }
    }
    for name in named.keys() {
        if !callable_params.iter().any(|param| param.name == *name) {
            return Err(format!("unknown named argument: {name}"));
        }
    }
    for param in callable_params {
        let v = if let Some(value) = named.remove(&param.name) {
            value
        } else if let Some(default) = &param.default {
            expression(default, &local, funcs)?
        } else {
            return Err(format!("missing required argument: {}", param.name));
        };
        if let Some(annotation) = &param.annotation {
            check_annotation(&param.name, annotation, &v)?;
        }
        local.insert(param.name.clone(), v);
    }
    let mut local_funcs = funcs.clone();
    let value = match if let Some(body) = &f.ast_body {
        execute_ast_program(body, &mut local, &mut local_funcs, Path::new("."))
    } else {
        execute_lines(&f.body, &mut local, &mut local_funcs, Path::new("."))
    }? {
        Flow::Return(v) => v,
        Flow::Continue => Value::None,
        Flow::Break | Flow::LoopContinue => {
            return Err("break/continue cannot be used outside a loop".into())
        }
    };
    {
        let mut captured = f.closure.borrow_mut();
        for key in &captured_keys {
            if key != "self" {
                if let Some(value) = local.get(key) {
                    captured.insert(key.clone(), value.clone());
                }
            }
        }
    }
    if let Some(annotation) = &f.return_annotation {
        check_annotation("return", annotation, &value)?;
    }
    Ok(value)
}

pub(crate) fn indented(lines: &[String], start: usize) -> (Vec<String>, usize) {
    let mut i = start;
    let mut body = Vec::new();
    while i < lines.len() {
        let line = &lines[i];
        if line.trim().is_empty() {
            body.push(String::new());
            i += 1;
            continue;
        }
        if !(line.starts_with(' ') || line.starts_with('\t')) {
            if line.trim_start().starts_with('#') {
                body.push(line.trim().to_string());
                i += 1;
                continue;
            }
            break;
        }
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

fn split_generic_args(inner: &str) -> Result<Vec<&str>, String> {
    let mut args = Vec::new();
    let mut start = 0;
    let mut depth = 0usize;
    for (index, character) in inner.char_indices() {
        match character {
            '<' => depth += 1,
            '>' => {
                depth = depth
                    .checked_sub(1)
                    .ok_or_else(|| "unbalanced type annotation".to_string())?
            }
            ',' if depth == 0 => {
                let argument = inner[start..index].trim();
                if argument.is_empty() {
                    return Err("generic type arguments cannot be empty".to_string());
                }
                args.push(argument);
                start = index + character.len_utf8();
            }
            _ => {}
        }
    }
    if depth != 0 {
        return Err("unbalanced type annotation".to_string());
    }
    let argument = inner[start..].trim();
    if argument.is_empty() {
        return Err("generic type arguments cannot be empty".to_string());
    }
    args.push(argument);
    Ok(args)
}

fn generic_annotation(annotation: &str) -> Option<(&str, &str)> {
    let open = annotation.find('<')?;
    if !annotation.ends_with('>') || open == 0 {
        return None;
    }
    Some((
        &annotation[..open],
        &annotation[open + 1..annotation.len() - 1],
    ))
}

fn matches_annotation(annotation: &str, value: &Value) -> Result<bool, String> {
    let expected = annotation.trim();
    if expected.is_empty() || expected == "any" {
        return Ok(true);
    }
    if let Some((base, inner)) = generic_annotation(expected) {
        let args = split_generic_args(inner)?;
        return match (base.trim(), value) {
            ("list", Value::List(items)) if args.len() == 1 => {
                items.iter().try_fold(true, |valid, item| {
                    Ok(valid && matches_annotation(args[0], item)?)
                })
            }
            ("map", Value::Map(entries)) if args.len() == 2 => {
                if args[0].trim() != "text" && args[0].trim() != "any" {
                    return Ok(false);
                }
                entries.values().try_fold(true, |valid, item| {
                    Ok(valid && matches_annotation(args[1], item)?)
                })
            }
            ("result", Value::ResultOk(item) | Value::ResultErr(item)) if args.len() == 1 => {
                matches_annotation(args[0], item)
            }
            ("option", Value::OptionSome(item)) if args.len() == 1 => {
                matches_annotation(args[0], item)
            }
            ("option", Value::OptionNone) if args.len() == 1 => Ok(true),
            ("list" | "map" | "result" | "option", _) => Ok(false),
            _ => Err(format!(
                "unknown or invalid generic type annotation: {expected}"
            )),
        };
    }
    Ok(matches!(
        (expected, value_type(value)),
        ("text", "text")
            | ("number", "number")
            | ("bool", "bool")
            | ("list", "list")
            | ("map", "map")
            | ("object", "object")
            | ("result", "result")
            | ("option", "option")
            | ("none", "none")
    ))
}

pub(crate) fn check_annotation(name: &str, annotation: &str, value: &Value) -> Result<(), String> {
    let expected = annotation.trim();
    if expected.is_empty() || expected == "any" {
        return Ok(());
    }
    match matches_annotation(expected, value) {
        Ok(true) => Ok(()),
        Ok(false) => Err(format!(
            "type mismatch for {name}: expected {expected}, got {}",
            value_type(value)
        )),
        Err(error) => Err(format!("invalid type annotation for {name}: {error}")),
    }
}

fn ast_expr_source(expression: &crate::ast::Spanned<Expr>) -> String {
    match &expression.node {
        Expr::Literal(Literal::Number(value)) => value.to_string(),
        Expr::Literal(Literal::Text(value)) => format!(
            "\"{}\"",
            value
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t")
        ),
        Expr::Literal(Literal::Bool(value)) => value.to_string(),
        Expr::Literal(Literal::None) => "none".into(),
        Expr::Name(name) => name.clone(),
        Expr::List(items) => format!(
            "[{}]",
            items
                .iter()
                .map(ast_expr_source)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Expr::Map(items) => format!(
            "{{{}}}",
            items
                .iter()
                .map(|(key, value)| format!("{}: {}", ast_expr_source(key), ast_expr_source(value)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Expr::Unary {
            op: UnaryOp::Negate,
            value,
        } => format!("(-{})", ast_expr_source(value)),
        Expr::Unary {
            op: UnaryOp::Not,
            value,
        } => format!("(not {})", ast_expr_source(value)),
        Expr::Binary { left, op, right } => {
            let operator = match op {
                BinaryOp::Add => "+",
                BinaryOp::Subtract => "-",
                BinaryOp::Multiply => "*",
                BinaryOp::Divide => "/",
                BinaryOp::Remainder => "%",
                BinaryOp::Equal => "==",
                BinaryOp::NotEqual => "!=",
                BinaryOp::Less => "<",
                BinaryOp::Greater => ">",
                BinaryOp::LessEqual => "<=",
                BinaryOp::GreaterEqual => ">=",
                BinaryOp::And => "and",
                BinaryOp::Or => "or",
            };
            format!(
                "({} {} {})",
                ast_expr_source(left),
                operator,
                ast_expr_source(right)
            )
        }
        Expr::Call { callee, args } => format!(
            "{}({})",
            ast_expr_source(callee),
            args.iter()
                .map(|argument| match argument {
                    CallArg::Positional(value) => ast_expr_source(value),
                    CallArg::Named { name, value } =>
                        format!("{} = {}", name, ast_expr_source(value)),
                })
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Expr::Member { target, member } => {
            format!("{}.{}", ast_expr_source(target), member)
        }
        Expr::Index { target, index } => {
            format!("{}[{}]", ast_expr_source(target), ast_expr_source(index))
        }
    }
}

fn ast_stmt_lines(statement: &crate::ast::Spanned<Stmt>, indent: usize, out: &mut Vec<String>) {
    let prefix = "    ".repeat(indent);
    match &statement.node {
        Stmt::Expression(value) => out.push(format!("{prefix}{}", ast_expr_source(value))),
        Stmt::Assignment { name, value } => {
            out.push(format!("{prefix}{name} = {}", ast_expr_source(value)))
        }
        Stmt::Declaration {
            name,
            annotation,
            value,
        } => out.push(format!(
            "{prefix}let {name}{} = {}",
            annotation
                .as_ref()
                .map_or(String::new(), |ty| format!(": {ty}")),
            ast_expr_source(value)
        )),
        Stmt::Say(value) => out.push(format!("{prefix}say {}", ast_expr_source(value))),
        Stmt::Import { path, explicit } => out.push(format!(
            "{prefix}{} \"{path}\"",
            if *explicit { "import" } else { "use" }
        )),
        Stmt::Return(value) => out.push(format!(
            "{prefix}return{}",
            value.as_ref().map_or(String::new(), |value| format!(
                " {}",
                ast_expr_source(value)
            ))
        )),
        Stmt::Break => out.push(format!("{prefix}break")),
        Stmt::Continue => out.push(format!("{prefix}continue")),
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            out.push(format!("{prefix}if {}:", ast_expr_source(condition)));
            ast_program_lines(then_branch, indent + 1, out);
            if let Some(branch) = else_branch {
                out.push(format!("{prefix}else:"));
                ast_program_lines(branch, indent + 1, out);
            }
        }
        Stmt::While { condition, body } => {
            out.push(format!("{prefix}while {}:", ast_expr_source(condition)));
            ast_program_lines(body, indent + 1, out);
        }
        Stmt::For {
            binding,
            iterable,
            body,
        } => {
            out.push(format!(
                "{prefix}for {binding} in {}:",
                ast_expr_source(iterable)
            ));
            ast_program_lines(body, indent + 1, out);
        }
        Stmt::Function {
            name,
            params,
            return_type,
            body,
            ..
        } => {
            let params = params
                .iter()
                .map(|(name, annotation, default)| {
                    format!(
                        "{name}{}{}",
                        annotation
                            .as_ref()
                            .map_or(String::new(), |ty| format!(": {ty}")),
                        default
                            .as_ref()
                            .map_or(String::new(), |value| format!(" = {value}"))
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            out.push(format!(
                "{prefix}fn {name}({params}){}:",
                return_type
                    .as_ref()
                    .map_or(String::new(), |ty| format!(" -> {ty}"))
            ));
            ast_program_lines(body, indent + 1, out);
        }
        Stmt::Class { name, base, body } => {
            out.push(format!(
                "{prefix}class {name}{}:",
                base.as_ref()
                    .map_or(String::new(), |base| format!(" extends {base}"))
            ));
            ast_program_lines(body, indent + 1, out);
        }
    }
}

fn ast_program_lines(program: &Program, indent: usize, out: &mut Vec<String>) {
    for statement in &program.statements {
        ast_stmt_lines(statement, indent, out);
    }
}

pub(crate) fn ast_program_compatible(program: &Program) -> bool {
    program
        .statements
        .iter()
        .all(|statement| match &statement.node {
            Stmt::Function { .. } | Stmt::Class { .. } | Stmt::Import { .. } => true,
            Stmt::If {
                then_branch,
                else_branch,
                ..
            } => {
                ast_program_compatible(then_branch)
                    && match else_branch.as_ref() {
                        Some(branch) => ast_program_compatible(branch),
                        None => true,
                    }
            }
            Stmt::While { body, .. } | Stmt::For { body, .. } => ast_program_compatible(body),
            _ => true,
        })
}

fn register_ast_function(
    name: &str,
    params: &[(String, Option<String>, Option<String>)],
    visibility: &str,
    return_type: &Option<String>,
    body: &Program,
    vars: &HashMap<String, Value>,
    funcs: &mut HashMap<String, Rc<Function>>,
) {
    funcs.insert(
        name.to_string(),
        Rc::new(Function {
            visibility: visibility.to_string(),
            params: params
                .iter()
                .map(|(name, annotation, default)| Param {
                    name: name.clone(),
                    annotation: annotation.clone(),
                    default: default.clone(),
                })
                .collect(),
            return_annotation: return_type.clone(),
            body: Vec::new(),
            ast_body: Some(body.clone()),
            closure: Rc::new(RefCell::new(vars.clone())),
        }),
    );
}

fn register_ast_class(
    name: &str,
    base: &Option<String>,
    body: &Program,
    vars: &HashMap<String, Value>,
    funcs: &mut HashMap<String, Rc<Function>>,
) -> Result<(), String> {
    funcs.insert(
        format!("{name}.__class__"),
        Rc::new(Function {
            visibility: "public".into(),
            params: Vec::new(),
            return_annotation: None,
            body: Vec::new(),
            ast_body: None,
            closure: Rc::new(RefCell::new(vars.clone())),
        }),
    );
    if let Some(parent) = base {
        if !funcs.contains_key(&format!("{parent}.__class__")) {
            return Err(format!("unknown parent class: {parent}"));
        }
        funcs.insert(
            format!("{name}.__parent__"),
            Rc::new(Function {
                visibility: "public".into(),
                params: Vec::new(),
                return_annotation: None,
                body: vec![parent.clone()],
                ast_body: None,
                closure: Rc::new(RefCell::new(vars.clone())),
            }),
        );
    }
    for statement in &body.statements {
        if let Stmt::Function {
            name: method,
            params,
            return_type,
            body,
            visibility,
        } = &statement.node
        {
            if method == "init" {
                funcs.insert(
                    format!("{name}.__own_init__"),
                    Rc::new(Function {
                        visibility: "public".into(),
                        params: Vec::new(),
                        return_annotation: None,
                        body: Vec::new(),
                        ast_body: None,
                        closure: Rc::new(RefCell::new(vars.clone())),
                    }),
                );
            }
            let mut method_params = params
                .iter()
                .map(|(name, annotation, default)| Param {
                    name: name.clone(),
                    annotation: annotation.clone(),
                    default: default.clone(),
                })
                .collect::<Vec<_>>();
            if method_params.first().map(|param| param.name.as_str()) != Some("self") {
                method_params.insert(
                    0,
                    Param {
                        name: "self".into(),
                        annotation: None,
                        default: None,
                    },
                );
            }
            let mut method_closure = vars.clone();
            method_closure.insert("__zap_owner_class".into(), Value::Text(name.to_string()));
            funcs.insert(
                format!("{name}.{method}"),
                Rc::new(Function {
                    visibility: visibility.clone(),
                    params: method_params,
                    return_annotation: return_type.clone(),
                    body: Vec::new(),
                    ast_body: Some(body.clone()),
                    closure: Rc::new(RefCell::new(method_closure)),
                }),
            );
        }
    }
    if let Some(parent) = base {
        let prefix = format!("{parent}.");
        let inherited = funcs
            .iter()
            .filter(|(key, _)| key.starts_with(&prefix))
            .map(|(key, function)| {
                (
                    key.trim_start_matches(&prefix).to_string(),
                    function.clone(),
                )
            })
            .collect::<Vec<_>>();
        for (method, function) in inherited {
            funcs.entry(format!("{name}.{method}")).or_insert(function);
        }
    }
    Ok(())
}

pub(crate) fn execute_ast_program(
    program: &Program,
    vars: &mut HashMap<String, Value>,
    funcs: &mut HashMap<String, Rc<Function>>,
    base: &Path,
) -> Result<Flow, String> {
    if !ast_program_compatible(program) {
        let mut lines = Vec::new();
        ast_program_lines(program, 0, &mut lines);
        return execute_lines(&lines, vars, funcs, base);
    }
    let _guard = enter_execution(
        &program
            .statements
            .iter()
            .map(|_| String::new())
            .collect::<Vec<_>>(),
    )?;
    for statement in &program.statements {
        let flow = match &statement.node {
            Stmt::Expression(value) => {
                let _ = ast_expression(value, vars, funcs)?;
                Flow::Continue
            }
            Stmt::Assignment { name, value } => {
                let evaluated = ast_expression(value, vars, funcs)?;
                if let Some((object_name, field)) = name.split_once('.') {
                    let object = vars
                        .get_mut(object_name)
                        .ok_or(format!("undefined variable: {object_name}"))?;
                    match object {
                        Value::Object { fields, .. } => {
                            fields.borrow_mut().insert(field.into(), evaluated);
                        }
                        _ => return Err("property assignment expects an object".into()),
                    }
                } else {
                    vars.insert(name.clone(), evaluated);
                }
                Flow::Continue
            }
            Stmt::Declaration {
                name,
                annotation,
                value,
            } => {
                let evaluated = ast_expression(value, vars, funcs)?;
                if let Some(annotation) = annotation {
                    check_annotation(name, annotation, &evaluated)?;
                }
                vars.insert(name.clone(), evaluated);
                Flow::Continue
            }
            Stmt::Say(value) => {
                println!("{}", ast_expression(value, vars, funcs)?.show());
                Flow::Continue
            }
            Stmt::Return(value) => {
                Flow::Return(value.as_ref().map_or(Ok(Value::None), |value| {
                    expression(&ast_expr_source(value), vars, funcs)
                })?)
            }
            Stmt::Break => Flow::Break,
            Stmt::Continue => Flow::LoopContinue,
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if ast_expression(condition, vars, funcs)?.truthy() {
                    execute_ast_program(then_branch, vars, funcs, base)?
                } else if let Some(branch) = else_branch {
                    execute_ast_program(branch, vars, funcs, base)?
                } else {
                    Flow::Continue
                }
            }
            Stmt::While { condition, body } => {
                let mut iterations = 0;
                loop {
                    if !ast_expression(condition, vars, funcs)?.truthy() {
                        break Flow::Continue;
                    }
                    match execute_ast_program(body, vars, funcs, base)? {
                        Flow::Continue | Flow::LoopContinue => {}
                        Flow::Break => break Flow::Continue,
                        flow @ Flow::Return(_) => break flow,
                    }
                    iterations += 1;
                    if iterations >= MAX_LOOP_ITERATIONS {
                        return Err(format!(
                            "loop limit exceeded: maximum is {MAX_LOOP_ITERATIONS}"
                        ));
                    }
                }
            }
            Stmt::For {
                binding,
                iterable,
                body,
            } => {
                let value = ast_expression(iterable, vars, funcs)?;
                let items = match value {
                    Value::List(items) => items,
                    _ => return Err("for expects a list".into()),
                };
                if items.len() > MAX_LOOP_ITERATIONS {
                    return Err(format!(
                        "loop limit exceeded: maximum is {MAX_LOOP_ITERATIONS}"
                    ));
                }
                let mut outcome = Flow::Continue;
                for item in items {
                    vars.insert(binding.clone(), item);
                    match execute_ast_program(body, vars, funcs, base)? {
                        Flow::Continue | Flow::LoopContinue => {}
                        Flow::Break => break,
                        flow @ Flow::Return(_) => {
                            outcome = flow;
                            break;
                        }
                    }
                }
                outcome
            }
            Stmt::Function {
                name,
                params,
                return_type,
                body,
                visibility,
            } => {
                register_ast_function(name, params, visibility, return_type, body, vars, funcs);
                Flow::Continue
            }
            Stmt::Class { name, base, body } => {
                register_ast_class(name, base, body, vars, funcs)?;
                Flow::Continue
            }
            Stmt::Import { path, explicit } => load_module(path, vars, funcs, base, *explicit)?,
        };
        match flow {
            Flow::Continue => {}
            flow => return Ok(flow),
        }
    }
    Ok(Flow::Continue)
}

pub(crate) fn load_module(
    raw: &str,
    vars: &mut HashMap<String, Value>,
    funcs: &mut HashMap<String, Rc<Function>>,
    base: &Path,
    explicit: bool,
) -> Result<Flow, String> {
    let spec = raw.trim();
    let spec = spec.strip_prefix("import ").unwrap_or(spec).trim();
    let spec = spec.strip_suffix(";").unwrap_or(spec).trim();
    let spec = spec.strip_suffix(" as").unwrap_or(spec).trim();
    let raw_path = spec.trim_matches('"');
    if raw_path.is_empty() {
        return Err("import expects a module path".into());
    }
    let requested_path = Path::new(raw_path);
    if requested_path.is_absolute() {
        return Err("absolute module paths are not allowed".into());
    }
    if requested_path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err("module paths may not traverse parent directories".into());
    }
    let candidate = if requested_path.extension().is_some() {
        raw_path.to_string()
    } else {
        format!("{raw_path}.zp")
    };
    let path = if Path::new(&candidate).is_absolute() {
        Path::new(&candidate).to_path_buf()
    } else {
        resolve_module(base, raw_path).ok_or(format!("module not found: {raw_path}"))?
    };
    let canonical = path
        .canonicalize()
        .map_err(|e| format!("cannot resolve module {}: {e}", path.display()))?;
    if !canonical.is_file() {
        return Err(format!("module is not a file: {}", canonical.display()));
    }
    let cached = MODULE_CACHE.with(|cache| cache.borrow().get(&canonical).cloned());
    if let Some((module_vars, module_funcs)) = cached {
        if explicit {
            let exported_vars = module_vars
                .keys()
                .filter_map(|key| key.strip_prefix("__zap_export_var__:").map(str::to_string))
                .collect::<Vec<_>>();
            for name in exported_vars {
                if let Some(value) = module_vars.get(&name).cloned() {
                    vars.insert(name, value);
                }
            }
            let exported_funcs = module_funcs
                .keys()
                .filter_map(|key| key.strip_prefix("__zap_export_fn__:").map(str::to_string))
                .collect::<Vec<_>>();
            for name in exported_funcs {
                if let Some(function) = module_funcs.get(&name).cloned() {
                    funcs.insert(name, function);
                }
            }
        } else {
            for (key, value) in module_vars {
                if !key.starts_with("__zap_export_var__:") {
                    vars.insert(key, value);
                }
            }
            for (key, function) in module_funcs {
                if !key.starts_with("__zap_export_fn__:") {
                    funcs.insert(key, function);
                }
            }
        }
        return Ok(Flow::Continue);
    }
    let cycle = MODULE_LOADING.with(|stack| {
        let stack = stack.borrow();
        stack.iter().position(|item| item == &canonical)
    });
    if let Some(start) = cycle {
        let chain = MODULE_LOADING.with(|stack| {
            let stack = stack.borrow();
            stack[start..]
                .iter()
                .chain(std::iter::once(&canonical))
                .map(|item| item.display().to_string())
                .collect::<Vec<_>>()
                .join(" -> ")
        });
        return Err(format!("circular import detected: {chain}"));
    }
    MODULE_LOADING.with(|stack| stack.borrow_mut().push(canonical.clone()));
    let imported_result = read_limited_text(&canonical, "module import");
    let imported = match imported_result {
        Ok(value) => value,
        Err(error) => {
            MODULE_LOADING.with(|stack| {
                stack.borrow_mut().pop();
            });
            return Err(error);
        }
    };
    let imported_lines = imported.lines().map(str::to_string).collect::<Vec<_>>();
    let mut module_vars = HashMap::new();
    let mut module_funcs = HashMap::new();
    let flow_result = execute_lines(
        &imported_lines,
        &mut module_vars,
        &mut module_funcs,
        canonical.parent().unwrap_or(base),
    );
    MODULE_LOADING.with(|stack| {
        stack.borrow_mut().pop();
    });
    let flow = flow_result?;
    if !matches!(flow, Flow::Continue) {
        return Ok(flow);
    }
    MODULE_CACHE.with(|cache| {
        cache.borrow_mut().insert(
            canonical.clone(),
            (module_vars.clone(), module_funcs.clone()),
        );
    });
    if explicit {
        let exported_vars = module_vars
            .keys()
            .filter_map(|key| key.strip_prefix("__zap_export_var__:").map(str::to_string))
            .collect::<Vec<_>>();
        for name in exported_vars {
            if let Some(value) = module_vars.get(&name).cloned() {
                vars.insert(name, value);
            }
        }
        let exported_funcs = module_funcs
            .keys()
            .filter_map(|key| key.strip_prefix("__zap_export_fn__:").map(str::to_string))
            .collect::<Vec<_>>();
        for name in exported_funcs {
            if let Some(function) = module_funcs.get(&name).cloned() {
                funcs.insert(name, function);
            }
        }
    } else {
        for (key, value) in module_vars {
            if !key.starts_with("__zap_export_var__:") {
                vars.insert(key, value);
            }
        }
        for (key, function) in module_funcs {
            if !key.starts_with("__zap_export_fn__:") {
                funcs.insert(key, function);
            }
        }
    }
    Ok(Flow::Continue)
}
pub(crate) fn execute_lines(
    lines: &[String],
    vars: &mut HashMap<String, Value>,
    funcs: &mut HashMap<String, Rc<Function>>,
    base: &Path,
) -> Result<Flow, String> {
    let _execution_guard = enter_execution(lines)?;
    let mut i = 0;
    while i < lines.len() {
        let raw_line = lines[i].trim();
        let is_export = raw_line.starts_with("export ");
        let line = if is_export {
            raw_line.strip_prefix("export ").unwrap_or(raw_line).trim()
        } else {
            raw_line
        };
        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }
        if let Some(rest) = line.strip_prefix("class ") {
            let head = rest.trim_end_matches(':').trim();
            let mut parts = head.split_whitespace();
            let class_name = parts
                .next()
                .ok_or("class syntax: class Name:".to_string())?
                .to_string();
            let parent = if parts.next() == Some("extends") {
                parts.next().map(str::to_string)
            } else {
                None
            };
            funcs.insert(
                format!("{class_name}.__class__"),
                Rc::new(Function {
                    visibility: "public".into(),
                    params: Vec::new(),
                    return_annotation: None,
                    body: Vec::new(),
                    ast_body: None,
                    closure: Rc::new(RefCell::new(vars.clone())),
                }),
            );
            if let Some(parent_name) = parent.clone() {
                if !funcs.contains_key(&format!("{parent_name}.__class__")) {
                    return Err(format!("unknown parent class: {parent_name}"));
                }
                funcs.insert(
                    format!("{class_name}.__parent__"),
                    Rc::new(Function {
                        visibility: "public".into(),
                        params: Vec::new(),
                        return_annotation: None,
                        body: vec![parent_name],
                        ast_body: None,
                        closure: Rc::new(RefCell::new(vars.clone())),
                    }),
                );
            }
            let (body, end) = indented(lines, i + 1);
            let mut j = 0;
            while j < body.len() {
                let method_line = body[j].trim();
                if let Some(method_rest) = method_line
                    .strip_prefix("fn ")
                    .or_else(|| method_line.strip_prefix("def "))
                {
                    let method_head = method_rest.trim_end_matches(':');
                    let (method_name, args) = method_head
                        .split_once('(')
                        .ok_or("method syntax: fn name(self):".to_string())?;
                    if method_name.trim() == "init" {
                        funcs.insert(
                            format!("{class_name}.__own_init__"),
                            Rc::new(Function {
                                visibility: "public".into(),
                                params: Vec::new(),
                                return_annotation: None,
                                body: Vec::new(),
                                ast_body: None,
                                closure: Rc::new(RefCell::new(vars.clone())),
                            }),
                        );
                    }
                    let (signature_params, return_annotation) = parse_signature(args)?;
                    let mut params = signature_params;
                    if params.first().map(|x| x.name.as_str()) != Some("self") {
                        params.insert(
                            0,
                            Param {
                                name: "self".into(),
                                annotation: None,
                                default: None,
                            },
                        );
                    }
                    let (method_body, method_end) = indented(&body, j + 1);
                    let mut method_closure = vars.clone();
                    method_closure
                        .insert("__zap_owner_class".into(), Value::Text(class_name.clone()));
                    funcs.insert(
                        format!("{class_name}.{}", method_name.trim()),
                        Rc::new(Function {
                            visibility: "public".into(),
                            params,
                            return_annotation,
                            body: method_body,
                            ast_body: None,
                            closure: Rc::new(RefCell::new(method_closure)),
                        }),
                    );
                    j = method_end;
                } else {
                    j += 1;
                }
            }
            if let Some(parent_name) = parent {
                let prefix = format!("{parent_name}.");
                let inherited = funcs
                    .iter()
                    .filter(|(name, _)| name.starts_with(&prefix))
                    .map(|(name, function)| {
                        (
                            name.trim_start_matches(&prefix).to_string(),
                            function.clone(),
                        )
                    })
                    .collect::<Vec<_>>();
                for (name, function) in inherited {
                    let child_name = format!("{class_name}.{name}");
                    funcs.entry(child_name).or_insert(function);
                }
            }
            i = end;
            continue;
        }
        if let Some(rest) = line
            .strip_prefix("fn ")
            .or_else(|| line.strip_prefix("def "))
        {
            let head = rest.trim_end_matches(':');
            let (name, args) = head
                .split_once('(')
                .ok_or("function syntax: fn name(a, b):".to_string())?;
            let (args, return_annotation) = parse_signature(args)?;
            let name = name.trim().to_string();
            let (body, end) = indented(lines, i + 1);
            funcs.insert(
                name.clone(),
                Rc::new(Function {
                    visibility: "public".into(),
                    params: args,
                    return_annotation,
                    body,
                    ast_body: None,
                    closure: Rc::new(RefCell::new(vars.clone())),
                }),
            );
            if is_export {
                funcs.insert(
                    format!("__zap_export_fn__:{name}"),
                    Rc::new(Function {
                        visibility: "public".into(),
                        params: Vec::new(),
                        return_annotation: None,
                        body: Vec::new(),
                        ast_body: None,
                        closure: Rc::new(RefCell::new(HashMap::new())),
                    }),
                );
            }
            i = end;
            continue;
        }
        if let Some(rest) = line.strip_prefix("return") {
            let outcome = if rest.trim().is_empty() {
                EvalOutcome::Value(Value::None)
            } else {
                evaluate_with_propagation(rest.trim(), vars, funcs)?
            };
            return Ok(Flow::Return(match outcome {
                EvalOutcome::Value(value) | EvalOutcome::Propagate(value) => value,
            }));
        }
        if line == "break" {
            return Ok(Flow::Break);
        }
        if line == "continue" {
            return Ok(Flow::LoopContinue);
        }
        if let Some(c) = line.strip_prefix("while ") {
            let condition = c.trim_end_matches(':').trim();
            let (body, end) = indented(lines, i + 1);
            let mut guard = 0;
            while expression(condition, vars, funcs)?.truthy() {
                match execute_lines(&body, vars, funcs, base)? {
                    Flow::Return(v) => return Ok(Flow::Return(v)),
                    Flow::Break => break,
                    Flow::LoopContinue => {}
                    Flow::Continue => {}
                }
                guard += 1;
                if guard >= MAX_LOOP_ITERATIONS {
                    return Err(format!(
                        "loop limit exceeded: maximum is {MAX_LOOP_ITERATIONS}"
                    ));
                }
            }
            i = end;
            continue;
        }
        if let Some(rest) = line.strip_prefix("for ") {
            let (name, src) = rest
                .trim_end_matches(':')
                .split_once(" in ")
                .ok_or("for syntax: for item in list:".to_string())?;
            let value = expression(src.trim(), vars, funcs)?;
            let (body, end) = indented(lines, i + 1);
            match value {
                Value::List(items) => {
                    if items.len() > MAX_LOOP_ITERATIONS {
                        return Err(format!(
                            "loop limit exceeded: maximum is {MAX_LOOP_ITERATIONS}"
                        ));
                    }
                    for (iteration, item) in items.into_iter().enumerate() {
                        if iteration >= MAX_LOOP_ITERATIONS {
                            return Err(format!(
                                "loop limit exceeded: maximum is {MAX_LOOP_ITERATIONS}"
                            ));
                        }
                        vars.insert(name.trim().into(), item);
                        match execute_lines(&body, vars, funcs, base)? {
                            Flow::Return(v) => return Ok(Flow::Return(v)),
                            Flow::Break => break,
                            Flow::LoopContinue => continue,
                            Flow::Continue => {}
                        }
                    }
                }
                _ => return Err("for expects a list".into()),
            }
            i = end;
            continue;
        }
        if let Some(c) = line.strip_prefix("if ") {
            let take = expression(c.trim_end_matches(':').trim(), vars, funcs)?.truthy();
            let (body, mut end) = indented(lines, i + 1);
            if take {
                match execute_lines(&body, vars, funcs, base)? {
                    Flow::Continue => {}
                    flow => return Ok(flow),
                }
            }
            if end < lines.len() && lines[end].trim() == "else:" {
                let (else_body, e) = indented(lines, end + 1);
                if !take {
                    match execute_lines(&else_body, vars, funcs, base)? {
                        Flow::Continue => {}
                        flow => return Ok(flow),
                    }
                }
                end = e;
            }
            i = end;
            continue;
        }
        if let Some(x) = line.strip_prefix("say ") {
            println!("{}", expression(x, vars, funcs)?.show());
            i += 1;
            continue;
        }
        if let Some(x) = line.strip_prefix("import ") {
            match load_module(x, vars, funcs, base, true)? {
                Flow::Continue => {}
                Flow::Return(_) => return Err("return is not allowed at module top level".into()),
                flow => return Ok(flow),
            }
            i += 1;
            continue;
        }
        if let Some(x) = line.strip_prefix("use ") {
            let spec = x.trim();
            if spec.starts_with('"') || spec.contains('/') || spec.ends_with(".zp") {
                match load_module(spec, vars, funcs, base, false)? {
                    Flow::Continue => {}
                    Flow::Return(_) => {
                        return Err("return is not allowed at module top level".into())
                    }
                    flow => return Ok(flow),
                }
            } else {
                println!("[Zap native] module declared: {spec}");
            }
            i += 1;
            continue;
        }
        if let Some(x) = line.strip_prefix("let ") {
            let (n, v) = x
                .split_once('=')
                .ok_or(format!("line {}: expected =", i + 1))?;
            let (name, annotation) = n
                .trim()
                .split_once(':')
                .map(|(name, ty)| (name.trim(), Some(ty.trim())))
                .unwrap_or((n.trim(), None));
            let value = match evaluate_with_propagation(v, vars, funcs)? {
                EvalOutcome::Value(value) => value,
                EvalOutcome::Propagate(value) => return Ok(Flow::Return(value)),
            };
            if let Some(ty) = annotation {
                check_annotation(name, ty, &value).map_err(|e| format!("line {}: {e}", i + 1))?;
            }
            let name = name.to_string();
            vars.insert(name.clone(), value);
            if is_export {
                vars.insert(format!("__zap_export_var__:{name}"), Value::None);
            }
            i += 1;
            continue;
        }
        if !line.contains("==")
            && !line.contains("!=")
            && !line.contains("<=")
            && !line.contains(">=")
        {
            if let Some((n, v)) = line.split_once('=') {
                let target = n.trim();
                let value = match evaluate_with_propagation(v, vars, funcs)? {
                    EvalOutcome::Value(value) => value,
                    EvalOutcome::Propagate(value) => return Ok(Flow::Return(value)),
                };
                if let Some((object_name, field)) = target.split_once('.') {
                    let object = vars
                        .get_mut(object_name)
                        .ok_or(format!("undefined variable: {object_name}"))?;
                    match object {
                        Value::Object { fields, .. } => {
                            fields.borrow_mut().insert(field.trim().into(), value);
                        }
                        _ => return Err("property assignment expects an object".into()),
                    }
                } else {
                    vars.insert(target.into(), value);
                }
                i += 1;
                continue;
            }
        }
        let _ = expression(line, vars, funcs)?;
        i += 1;
        continue;
    }
    Ok(Flow::Continue)
}

#[cfg(test)]
mod tests {
    use super::{execute_ast_program, execute_lines};
    use crate::ast::parse_program;
    use crate::{Function, Value};
    use std::{collections::HashMap, path::Path, rc::Rc};

    #[test]
    fn executes_ast_compatible_statements() {
        let program =
            parse_program("let total: number = 1\nif total > 0:\n    total = total + 5\n")
                .expect("valid AST program");
        let mut vars = HashMap::<String, Value>::new();
        let mut funcs = HashMap::<String, Rc<Function>>::new();
        let flow = execute_ast_program(&program, &mut vars, &mut funcs, Path::new("."))
            .expect("AST execution should succeed");
        assert!(matches!(flow, super::Flow::Continue));
        assert_eq!(vars.get("total"), Some(&Value::Number(6)));
    }

    #[test]
    fn executes_function_and_method_bodies_from_native_ast() {
        let program = parse_program(
            "fn add(a: number, b: number) -> number:\n    return a + b\nfn twice(value: number) -> number:\n    return add(value, value)\nlet result: number = twice(3)\n",
        )
        .expect("valid declaration AST program");
        let mut vars = HashMap::<String, Value>::new();
        let mut funcs = HashMap::<String, Rc<Function>>::new();
        execute_ast_program(&program, &mut vars, &mut funcs, Path::new("."))
            .expect("native AST declarations should execute");
        assert!(funcs
            .get("add")
            .is_some_and(|function| function.ast_body.is_some()));
        assert!(funcs
            .get("twice")
            .is_some_and(|function| function.ast_body.is_some()));
        assert_eq!(vars.get("result"), Some(&Value::Number(6)));
    }

    #[test]
    fn evaluates_pure_builtins_from_native_ast() {
        let program = parse_program(
            "let count: number = len(range(0, 3))\nlet total: number = sum(range(1, 4))\nlet joined: text = join(split(\"a,b\", \",\"), \"-\")\nlet present: bool = is_some(some(1))\nlet value: number = unwrap(ok(7))\n",
        )
        .expect("valid built-in AST program");
        let mut vars = HashMap::<String, Value>::new();
        let mut funcs = HashMap::<String, Rc<Function>>::new();
        execute_ast_program(&program, &mut vars, &mut funcs, Path::new("."))
            .expect("direct built-in AST calls should execute");
        assert_eq!(vars.get("count"), Some(&Value::Number(3)));
        assert_eq!(vars.get("total"), Some(&Value::Number(6)));
        assert_eq!(vars.get("joined"), Some(&Value::Text("a-b".into())));
        assert_eq!(vars.get("present"), Some(&Value::Bool(true)));
        assert_eq!(vars.get("value"), Some(&Value::Number(7)));
    }

    #[test]
    fn evaluates_json_builtins_from_native_ast() {
        let program = parse_program(
            "let encoded: text = json(range(1, 3))\nlet decoded = from_json(\"{\\\"name\\\":\\\"Zap\\\",\\\"version\\\":1}\")\nlet name: text = decoded[\"name\"]\n",
        )
        .expect("valid JSON built-in AST program");
        let mut vars = HashMap::<String, Value>::new();
        let mut funcs = HashMap::<String, Rc<Function>>::new();
        execute_ast_program(&program, &mut vars, &mut funcs, Path::new("."))
            .expect("direct JSON built-ins should execute");
        assert_eq!(vars.get("encoded"), Some(&Value::Text("[1,2]".into())));
        assert_eq!(vars.get("name"), Some(&Value::Text("Zap".into())));

        let invalid = parse_program("let value = from_json(\"{invalid}\")\n")
            .expect("invalid JSON remains syntactically valid Zap");
        let result = execute_ast_program(
            &invalid,
            &mut HashMap::<String, Value>::new(),
            &mut HashMap::<String, Rc<Function>>::new(),
            Path::new("."),
        );
        match result {
            Err(error) => assert!(error.contains("from_json failed:")),
            Ok(_) => panic!("malformed JSON should fail at runtime"),
        }
    }

    #[test]
    fn evaluates_file_builtins_from_native_ast() {
        let path = std::env::temp_dir().join(format!("zap-direct-io-{}.txt", std::process::id()));
        let path_text = path.to_string_lossy().replace('\\', "\\\\");
        let source = format!(
            "write_text(\"{path_text}\", \"hello\")\nlet content: text = read_text(\"{path_text}\")\nwrite_lines(\"{path_text}\", split(\"one,two\", \",\"))\nlet lines = read_lines(\"{path_text}\")\n",
        );
        let program = parse_program(&source).expect("valid file built-in AST program");
        let mut vars = HashMap::<String, Value>::new();
        let mut funcs = HashMap::<String, Rc<Function>>::new();
        execute_ast_program(&program, &mut vars, &mut funcs, Path::new("."))
            .expect("direct file built-ins should execute");
        assert_eq!(vars.get("content"), Some(&Value::Text("hello".into())));
        assert_eq!(
            vars.get("lines"),
            Some(&Value::List(vec![
                Value::Text("one".into()),
                Value::Text("two".into())
            ]))
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn evaluates_system_builtins_from_native_ast() {
        let program = parse_program(
            "let present: bool = has_env(\"PATH\")\nlet joined: text = path_join(\"tmp\", \"zap\", \"main.zp\")\nlet base: text = basename(joined)\nlet parent: text = dirname(joined)\nlet available: bool = exists(\".\")\nlet timestamp: number = now()\nsleep(0)\n",
        )
        .expect("valid system built-in AST program");
        let mut vars = HashMap::<String, Value>::new();
        let mut funcs = HashMap::<String, Rc<Function>>::new();
        execute_ast_program(&program, &mut vars, &mut funcs, Path::new("."))
            .expect("direct system built-ins should execute");
        assert!(matches!(vars.get("present"), Some(Value::Bool(_))));
        assert_eq!(
            vars.get("joined"),
            Some(&Value::Text(
                std::path::Path::new("tmp/zap/main.zp")
                    .to_string_lossy()
                    .into()
            ))
        );
        assert_eq!(vars.get("base"), Some(&Value::Text("main.zp".into())));
        assert!(matches!(vars.get("parent"), Some(Value::Text(_))));
        assert_eq!(vars.get("available"), Some(&Value::Bool(true)));
        assert!(matches!(vars.get("timestamp"), Some(Value::Number(value)) if *value > 0));
    }

    #[test]
    fn evaluates_list_indexing_from_native_ast() {
        let program = parse_program("let selected: number = range(0, 3)[1]\n")
            .expect("valid indexed AST program");
        let mut vars = HashMap::<String, Value>::new();
        let mut funcs = HashMap::<String, Rc<Function>>::new();
        execute_ast_program(&program, &mut vars, &mut funcs, Path::new("."))
            .expect("native AST indexing should execute");
        assert_eq!(vars.get("selected"), Some(&Value::Number(1)));
    }

    #[test]
    fn rejects_oversized_source_blocks() {
        let lines = vec![String::new(); 100_001];
        let result = execute_lines(
            &lines,
            &mut HashMap::<String, Value>::new(),
            &mut HashMap::<String, Rc<Function>>::new(),
            Path::new("."),
        );
        match result {
            Err(error) => assert!(error.contains("source line limit exceeded")),
            Ok(_) => panic!("source limit should reject oversized input"),
        }
    }

    #[test]
    fn rejects_unbounded_loop_iterations() {
        let lines = vec!["while true:".into(), "    continue".into()];
        let result = execute_lines(
            &lines,
            &mut HashMap::<String, Value>::new(),
            &mut HashMap::<String, Rc<Function>>::new(),
            Path::new("."),
        );
        match result {
            Err(error) => assert!(error.contains("loop limit exceeded")),
            Ok(_) => panic!("loop limit should reject an unbounded loop"),
        }
    }
}
