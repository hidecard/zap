#![allow(clippy::type_complexity)]

mod ast;
mod async_runtime;
#[cfg(test)]
mod corpus;
mod diagnostics;
mod lexer;

use diagnostics::ZapError;
use lexer::Token;
mod value;

use value::{EnvFrame, Function, Param, StaticSignature, Value};
mod project;
mod registry;

use project::{resolve_module, run_zap_tests, validate_project};
mod parser;

use parser::{
    annotation_matches, generic_type, is_allowed_annotation, matching_paren, parse_signature,
    split_static_args, static_literal_type,
};
mod cli;
mod evaluator;
mod lsp;
mod runtime_state;
mod stdlib;
mod stdlib_catalog;

use evaluator::{
    call_function_with_context, call_method_with_context, check_method_visibility,
    constructor_delegates_to_parent, direct_builtin_with_context, duration_value,
    execute_ast_program_with_context, initialize_object_fields, json_to_value, operate,
    utc_now_value, validate_source_layout, value_to_json, value_type_name, Flow,
};

use runtime_state::ExecutionContext;

use std::{
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    process,
    rc::Rc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
pub(crate) const MAX_FILE_BYTES: u64 = 8 * 1024 * 1024;
pub(crate) type ZapResult<T> = Result<T, ZapError>;

pub(crate) fn run_checked(source: &str, base: &Path) -> ZapResult<()> {
    run(source, base).map_err(ZapError::from_message)
}

pub(crate) fn read_limited_text(path: &Path, operation: &str) -> Result<String, String> {
    let metadata = fs::metadata(path).map_err(|e| format!("{operation} failed: {e}"))?;
    if metadata.len() > MAX_FILE_BYTES {
        return Err(format!(
            "{operation} failed: file exceeds the {} byte limit",
            MAX_FILE_BYTES
        ));
    }
    fs::read_to_string(path).map_err(|e| format!("{operation} failed: {e}"))
}

pub(crate) fn write_limited_text(
    path: &Path,
    content: &str,
    operation: &str,
) -> Result<(), String> {
    if content.len() as u64 > MAX_FILE_BYTES {
        return Err(format!(
            "{operation} failed: content exceeds the {} byte limit",
            MAX_FILE_BYTES
        ));
    }
    fs::write(path, content).map_err(|e| format!("{operation} failed: {e}"))
}

struct ExprParser<'a> {
    tokens: &'a [Token],
    pos: usize,
    vars: &'a HashMap<String, Value>,
    funcs: &'a HashMap<String, Rc<Function>>,
    context: &'a mut ExecutionContext,
}
impl<'a> ExprParser<'a> {
    fn new(
        t: &'a [Token],
        v: &'a HashMap<String, Value>,
        f: &'a HashMap<String, Rc<Function>>,
        context: &'a mut ExecutionContext,
    ) -> Self {
        Self {
            tokens: t,
            pos: 0,
            vars: v,
            funcs: f,
            context,
        }
    }
    fn call_args(&mut self) -> Result<Vec<Value>, String> {
        let mut args = Vec::new();
        if *self.peek() != Token::RParen {
            loop {
                args.push(self.parse(0)?);
                if *self.peek() == Token::RParen {
                    break;
                }
                if self.take() != Token::Comma {
                    return Err("expected comma in call".into());
                }
            }
        }
        if self.take() != Token::RParen {
            return Err("expected ) after call".into());
        }
        Ok(args)
    }
    fn peek(&self) -> &Token {
        let last = self.tokens.len().saturating_sub(1);
        &self.tokens[self.pos.min(last)]
    }
    fn take(&mut self) -> Token {
        let token = self.peek().clone();
        self.pos = self.pos.saturating_add(1);
        token
    }
    fn parse_complete(&mut self) -> Result<Value, String> {
        let value = self.parse(0)?;
        if *self.peek() != Token::End {
            return Err(format!(
                "unexpected token after expression: {:?}",
                self.peek()
            ));
        }
        Ok(value)
    }
    fn parse(&mut self, min: u8) -> Result<Value, String> {
        let mut left = match self.take() {
            Token::Number(n) => Value::Number(n),
            Token::Text(s) => Value::Text(s),
            Token::Minus => match self.parse(4)? {
                Value::Number(n) => Value::Number(-n),
                _ => return Err("unary - expects a number".into()),
            },
            Token::Name(n) if n == "true" => Value::Bool(true),
            Token::Name(n) if n == "false" => Value::Bool(false),
            Token::Name(n) if n == "not" => Value::Bool(!self.parse(4)?.truthy()),
            Token::Name(n) if n == "none" => Value::None,
            Token::Name(n) if n == "len" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after len".into());
                }
                match v {
                    Value::List(x) => Value::Number(x.len() as i64),
                    Value::Text(x) => Value::Number(x.chars().count() as i64),
                    _ => return Err("len expects a list or text".into()),
                }
            }
            Token::Name(n) if n == "str" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after str".into());
                }
                Value::Text(v.show())
            }
            Token::Name(n) if n == "now" && *self.peek() == Token::LParen => {
                self.take();
                if self.take() != Token::RParen {
                    return Err("expected ) after now".into());
                }
                Value::Number(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .map_err(|_| "system clock is before Unix epoch".to_string())?
                        .as_secs() as i64,
                )
            }
            Token::Name(n)
                if (n == "log_record" || n == "log_json") && *self.peek() == Token::LParen =>
            {
                self.take();
                let level = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err(format!("expected comma in {n}"));
                }
                let message = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err(format!("expected second comma in {n}"));
                }
                let fields = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err(format!("expected ) after {n}"));
                }
                direct_builtin_with_context(
                    n.as_str(),
                    vec![level, message, fields],
                    Some(&mut *self.context),
                )?
                .ok_or_else(|| format!("unknown standard function: {n}"))?
            }
            Token::Name(n) if n == "task_join_timeout" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err(format!("expected comma in {n}"));
                }
                let poll_budget = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err(format!("expected ) after {n}"));
                }
                direct_builtin_with_context(
                    n.as_str(),
                    vec![value, poll_budget],
                    Some(&mut *self.context),
                )?
                .ok_or_else(|| format!("unknown standard function: {n}"))?
            }
            Token::Name(n)
                if (n == "spawn"
                    || n == "task_join"
                    || n == "task_is_ready"
                    || n == "task_cancel")
                    && *self.peek() == Token::LParen =>
            {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err(format!("expected ) after {n}"));
                }
                direct_builtin_with_context(n.as_str(), vec![value], Some(&mut *self.context))?
                    .ok_or_else(|| format!("unknown standard function: {n}"))?
            }
            Token::Name(n) if n == "sleep" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after sleep".into());
                }
                match v {
                    Value::Number(ms) if ms >= 0 => {
                        if ms > stdlib::MAX_SLEEP_MILLISECONDS {
                            return Err(format!(
                                "sleep exceeds the {} millisecond limit",
                                stdlib::MAX_SLEEP_MILLISECONDS
                            ));
                        }
                        thread::sleep(Duration::from_millis(ms as u64));
                        Value::None
                    }
                    _ => return Err("sleep expects a non-negative number of milliseconds".into()),
                }
            }
            Token::Name(n) if n == "env" && *self.peek() == Token::LParen => {
                self.take();
                let key = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after env".into());
                }
                match key {
                    Value::Text(k) => Value::Text(env::var(k).unwrap_or_default()),
                    _ => return Err("env expects a text key".into()),
                }
            }
            Token::Name(n) if n == "has_env" && *self.peek() == Token::LParen => {
                self.take();
                let key = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after has_env".into());
                }
                match key {
                    Value::Text(k) => Value::Bool(env::var_os(k).is_some()),
                    _ => return Err("has_env expects a text key".into()),
                }
            }
            Token::Name(n) if n == "env_get" && *self.peek() == Token::LParen => {
                self.take();
                let key = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in env_get".into());
                }
                let default = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after env_get".into());
                }
                match (key, default) {
                    (Value::Text(key), Value::Text(default)) => {
                        Value::Text(env::var(key).unwrap_or(default))
                    }
                    _ => return Err("env_get expects two text arguments".into()),
                }
            }
            Token::Name(n) if n == "config_dir" && *self.peek() == Token::LParen => {
                self.take();
                if self.take() != Token::RParen {
                    return Err("expected ) after config_dir".into());
                }
                Value::Text(legacy_configuration_directory())
            }
            Token::Name(n) if n == "config_path" && *self.peek() == Token::LParen => {
                self.take();
                let name = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after config_path".into());
                }
                let Value::Text(name) = name else {
                    return Err("config_path expects a text file name".into());
                };
                Value::Text(legacy_configuration_path(&name)?)
            }
            Token::Name(n) if n == "exists" && *self.peek() == Token::LParen => {
                self.take();
                let path = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after exists".into());
                }
                match path {
                    Value::Text(p) => Value::Bool(Path::new(&p).exists()),
                    _ => return Err("exists expects a text path".into()),
                }
            }
            Token::Name(n) if n == "path_join" && *self.peek() == Token::LParen => {
                self.take();
                let mut parts = Vec::new();
                if *self.peek() != Token::RParen {
                    loop {
                        parts.push(match self.parse(0)? {
                            Value::Text(p) => p,
                            _ => return Err("path_join expects text parts".into()),
                        });
                        if *self.peek() == Token::RParen {
                            break;
                        }
                        if self.take() != Token::Comma {
                            return Err("expected comma in path_join".into());
                        }
                    }
                }
                self.take();
                Value::Text(
                    parts
                        .into_iter()
                        .fold(PathBuf::new(), |mut path, part| {
                            path.push(part);
                            path
                        })
                        .to_string_lossy()
                        .into(),
                )
            }
            Token::Name(n) if n == "basename" && *self.peek() == Token::LParen => {
                self.take();
                let path = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after basename".into());
                }
                match path {
                    Value::Text(p) => Value::Text(
                        Path::new(&p)
                            .file_name()
                            .and_then(|x| x.to_str())
                            .unwrap_or("")
                            .into(),
                    ),
                    _ => return Err("basename expects a text path".into()),
                }
            }
            Token::Name(n) if n == "dirname" && *self.peek() == Token::LParen => {
                self.take();
                let path = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after dirname".into());
                }
                match path {
                    Value::Text(p) => Value::Text(
                        Path::new(&p)
                            .parent()
                            .and_then(|x| x.to_str())
                            .unwrap_or("")
                            .into(),
                    ),
                    _ => return Err("dirname expects a text path".into()),
                }
            }
            Token::Name(n)
                if ["pow", "min", "max"].contains(&n.as_str()) && *self.peek() == Token::LParen =>
            {
                self.take();
                let left = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err(format!("expected comma in {n}"));
                }
                let right = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err(format!("expected ) after {n}"));
                }
                stdlib::binary(&n, left, right)?
                    .ok_or_else(|| format!("unknown standard function: {n}"))?
            }
            Token::Name(n)
                if ["sqrt", "abs", "upper", "lower", "trim"].contains(&n.as_str())
                    && *self.peek() == Token::LParen =>
            {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err(format!("expected ) after {n}"));
                }
                stdlib::unary(&n, value)?
                    .ok_or_else(|| format!("unknown standard function: {n}"))?
            }
            Token::Name(n) if n == "split" && *self.peek() == Token::LParen => {
                self.take();
                let text = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in split".into());
                }
                let separator = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after split".into());
                }
                match (text, separator) {
                    (Value::Text(x), Value::Text(sep)) => {
                        Value::List(x.split(&sep).map(|part| Value::Text(part.into())).collect())
                    }
                    _ => return Err("split expects text and separator".into()),
                }
            }
            Token::Name(n) if n == "utc_now" && *self.peek() == Token::LParen => {
                self.take();
                if self.take() != Token::RParen {
                    return Err("utc_now expects no arguments".into());
                }
                utc_now_value()?
            }
            Token::Name(n) if n == "duration_parts" && *self.peek() == Token::LParen => {
                self.take();
                let milliseconds = match self.parse(0)? {
                    Value::Number(value) => value,
                    _ => return Err("duration_parts expects milliseconds as a number".into()),
                };
                if self.take() != Token::RParen {
                    return Err("expected ) after duration_parts".into());
                }
                duration_value(milliseconds)?
            }
            Token::Name(n) if n == "duration_between" && *self.peek() == Token::LParen => {
                self.take();
                let start = match self.parse(0)? {
                    Value::Number(value) => value,
                    _ => return Err("duration_between expects two millisecond numbers".into()),
                };
                if self.take() != Token::Comma {
                    return Err("expected comma in duration_between".into());
                }
                let end = match self.parse(0)? {
                    Value::Number(value) => value,
                    _ => return Err("duration_between expects two millisecond numbers".into()),
                };
                if self.take() != Token::RParen {
                    return Err("expected ) after duration_between".into());
                }
                let milliseconds = end
                    .checked_sub(start)
                    .ok_or_else(|| "duration_between integer overflow".to_string())?;
                duration_value(milliseconds)?
            }
            Token::Name(n) if n == "type" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after type".into());
                }
                Value::Text(
                    match v {
                        Value::Text(_) => "text",
                        Value::Number(_) => "number",
                        Value::Bool(_) => "bool",
                        Value::List(_) => "list",
                        Value::Map(_) => "map",
                        Value::Object { .. } => "object",
                        Value::Callable(_) => "function",
                        Value::ResultOk(_) | Value::ResultErr(_) => "result",
                        Value::OptionSome(_) | Value::OptionNone => "option",
                        Value::Future(_) | Value::ScheduledFuture(_) => "future",
                        Value::None => "none",
                    }
                    .into(),
                )
            }
            Token::Name(n) if n == "keys" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after keys".into());
                }
                match v {
                    Value::Map(m) => Value::List(m.keys().cloned().map(Value::Text).collect()),
                    _ => return Err("keys expects a map".into()),
                }
            }
            Token::Name(n) if n == "entries" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after entries".into());
                }
                match v {
                    Value::Map(m) => {
                        let mut keys = m.keys().cloned().collect::<Vec<_>>();
                        keys.sort();
                        Value::List(
                            keys.into_iter()
                                .map(|key| {
                                    let mut entry = HashMap::new();
                                    entry.insert("key".into(), Value::Text(key.clone()));
                                    entry.insert(
                                        "value".into(),
                                        m.get(&key).cloned().unwrap_or(Value::None),
                                    );
                                    Value::Map(entry)
                                })
                                .collect(),
                        )
                    }
                    _ => return Err("entries expects a map".into()),
                }
            }
            Token::Name(n) if n == "enumerate" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after enumerate".into());
                }
                match v {
                    Value::List(values) => Value::List(
                        values
                            .into_iter()
                            .enumerate()
                            .map(|(index, value)| {
                                let mut entry = HashMap::new();
                                entry.insert("index".into(), Value::Number(index as i64));
                                entry.insert("value".into(), value);
                                Value::Map(entry)
                            })
                            .collect(),
                    ),
                    _ => return Err("enumerate expects a list".into()),
                }
            }
            Token::Name(n) if n == "contains" && *self.peek() == Token::LParen => {
                self.take();
                let collection = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in contains".into());
                }
                let item = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after contains".into());
                }
                Value::Bool(match collection {
                    Value::List(xs) => xs.contains(&item),
                    Value::Text(s) => match item {
                        Value::Text(q) => s.contains(&q),
                        _ => false,
                    },
                    Value::Map(m) => match item {
                        Value::Text(k) => m.contains_key(&k),
                        _ => false,
                    },
                    _ => false,
                })
            }
            Token::Name(n) if n == "join" && *self.peek() == Token::LParen => {
                self.take();
                let values = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in join".into());
                }
                let separator = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after join".into());
                }
                match (values, separator) {
                    (Value::List(xs), Value::Text(sep)) => {
                        Value::Text(xs.iter().map(Value::show).collect::<Vec<_>>().join(&sep))
                    }
                    _ => return Err("join expects a list and text separator".into()),
                }
            }
            Token::Name(n) if n == "is_empty" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after is_empty".into());
                }
                Value::Bool(match v {
                    Value::Text(x) => x.is_empty(),
                    Value::List(x) => x.is_empty(),
                    Value::Map(x) => x.is_empty(),
                    Value::None => true,
                    _ => false,
                })
            }
            Token::Name(n) if n == "sum" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after sum".into());
                }
                match v {
                    Value::List(xs) => {
                        let mut total = 0i64;
                        for item in xs {
                            if let Value::Number(n) = item {
                                total = total.checked_add(n).ok_or("sum overflow".to_string())?;
                            } else {
                                return Err("sum expects a list of numbers".into());
                            }
                        }
                        Value::Number(total)
                    }
                    _ => return Err("sum expects a list".into()),
                }
            }
            Token::Name(n) if n == "reverse" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after reverse".into());
                }
                match v {
                    Value::Text(x) => Value::Text(x.chars().rev().collect()),
                    Value::List(mut x) => {
                        x.reverse();
                        Value::List(x)
                    }
                    _ => return Err("reverse expects text or list".into()),
                }
            }
            Token::Name(n) if n == "sort" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after sort".into());
                }
                match v {
                    Value::List(mut x) => {
                        if x.iter().all(|v| matches!(v, Value::Number(_))) {
                            x.sort_by_key(|v| if let Value::Number(n) = v { *n } else { 0 });
                        } else if x.iter().all(|v| matches!(v, Value::Text(_))) {
                            x.sort_by_key(|v| {
                                if let Value::Text(s) = v {
                                    s.clone()
                                } else {
                                    String::new()
                                }
                            });
                        } else {
                            return Err("sort expects a list of numbers or text".into());
                        }
                        Value::List(x)
                    }
                    _ => return Err("sort expects a list".into()),
                }
            }
            Token::Name(n) if n == "get" && *self.peek() == Token::LParen => {
                self.take();
                let map = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in get".into());
                }
                let key = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected default value in get".into());
                }
                let default = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after get".into());
                }
                match (map, key) {
                    (Value::Map(m), Value::Text(k)) => m.get(&k).cloned().unwrap_or(default),
                    _ => return Err("get expects a map, text key, and default value".into()),
                }
            }
            Token::Name(n) if n == "assert" && *self.peek() == Token::LParen => {
                self.take();
                let condition = self.parse(0)?;
                let message = if *self.peek() == Token::Comma {
                    self.take();
                    self.parse(0)?
                } else {
                    Value::Text("assertion failed".into())
                };
                if self.take() != Token::RParen {
                    return Err("expected ) after assert".into());
                }
                if condition.truthy() {
                    Value::None
                } else {
                    return Err(format!(
                        "{}: expected true, got {}",
                        message.show(),
                        condition.show()
                    ));
                }
            }
            Token::Name(n) if n == "json" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after json".into());
                }
                Value::Text(
                    serde_json::to_string(&value_to_json(&v)?)
                        .map_err(|e| format!("json encode failed: {e}"))?,
                )
            }
            Token::Name(n) if n == "from_json" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after from_json".into());
                }
                match v {
                    Value::Text(s) => json_to_value(
                        serde_json::from_str(&s).map_err(|e| format!("from_json failed: {e}"))?,
                    )?,
                    _ => return Err("from_json expects text".into()),
                }
            }
            Token::Name(n) if n == "from_json_typed" && *self.peek() == Token::LParen => {
                self.take();
                let source = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in from_json_typed".into());
                }
                let expected = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after from_json_typed".into());
                }
                let (Value::Text(source), Value::Text(expected)) = (source, expected) else {
                    return Err("from_json_typed expects text and type name".into());
                };
                let value = json_to_value(
                    serde_json::from_str(&source)
                        .map_err(|error| format!("from_json_typed failed: {error}"))?,
                )?;
                let actual = value_type_name(&value);
                if actual != expected {
                    return Err(format!(
                        "from_json_typed failed: expected {expected}, got {actual}"
                    ));
                }
                value
            }
            Token::Name(n) if n == "char_at" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in char_at".into());
                }
                let index = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after char_at".into());
                }
                let (Value::Text(value), Value::Number(index)) = (value, index) else {
                    return Err("char_at expects text and non-negative index".into());
                };
                let index = usize::try_from(index)
                    .map_err(|_| "char_at expects a non-negative index".to_string())?;
                value
                    .chars()
                    .nth(index)
                    .map(|character| Value::Text(character.to_string()))
                    .ok_or_else(|| "char_at index out of range".to_string())?
            }
            Token::Name(n) if n == "substring" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected first index in substring".into());
                }
                let start = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected second index in substring".into());
                }
                let end = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after substring".into());
                }
                let (Value::Text(value), Value::Number(start), Value::Number(end)) =
                    (value, start, end)
                else {
                    return Err("substring expects text and non-negative start/end indices".into());
                };
                let start = usize::try_from(start)
                    .map_err(|_| "substring expects non-negative indices".to_string())?;
                let end = usize::try_from(end)
                    .map_err(|_| "substring expects non-negative indices".to_string())?;
                if start > end {
                    return Err("substring start must not exceed end".into());
                }
                Value::Text(value.chars().skip(start).take(end - start).collect())
            }
            Token::Name(n) if n == "codepoints" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after codepoints".into());
                }
                let Value::Text(value) = value else {
                    return Err("codepoints expects text".into());
                };
                Value::List(
                    value
                        .chars()
                        .map(|character| Value::Number(i64::from(u32::from(character))))
                        .collect(),
                )
            }
            Token::Name(n) if n == "new" && *self.peek() == Token::LParen => {
                self.take();
                let class = self.parse(0)?;
                let mut ctor_args = Vec::new();
                let mut fields = HashMap::new();
                while *self.peek() == Token::Comma {
                    self.take();
                    let value = self.parse(0)?;
                    if ctor_args.is_empty() {
                        if let Value::Map(m) = value {
                            fields = m;
                            continue;
                        }
                    }
                    ctor_args.push(value);
                }
                if self.take() != Token::RParen {
                    return Err("expected ) after new".into());
                }
                match class {
                    Value::Text(class_name) => {
                        if !self.funcs.contains_key(&format!("{class_name}.__class__")) {
                            return Err(format!("unknown class: {class_name}"));
                        }
                        let explicit_fields = fields;
                        self.context
                            .state_mut()
                            .memory_budget_mut()
                            .reserve_object(class_name.len(), explicit_fields.len())?;
                        let object = Value::object_with_store(
                            class_name.clone(),
                            Some(self.context.state().object_store().clone()),
                        );
                        initialize_object_fields(
                            &class_name,
                            &object,
                            self.vars,
                            self.funcs,
                            self.context,
                        )?;
                        if let Value::Object { fields, .. } = &object {
                            fields.try_borrow_mut()?.extend(explicit_fields);
                        }
                        if self
                            .funcs
                            .get(&format!("{class_name}.init"))
                            .is_some_and(|init| !constructor_delegates_to_parent(init))
                            && self
                                .funcs
                                .contains_key(&format!("{class_name}.__own_init__"))
                        {
                            if let Some(parent_meta) =
                                self.funcs.get(&format!("{class_name}.__parent__"))
                            {
                                if let Some(parent_name) = parent_meta.body.first() {
                                    if let Some(parent_init) =
                                        self.funcs.get(&format!("{parent_name}.init")).cloned()
                                    {
                                        check_method_visibility(
                                            &parent_init,
                                            parent_name,
                                            self.vars,
                                            self.funcs,
                                        )?;
                                        call_method_with_context(
                                            &parent_init,
                                            ctor_args.clone(),
                                            object.clone(),
                                            self.funcs,
                                            self.context,
                                        )?;
                                    }
                                }
                            }
                        }
                        if let Some(init) = self.funcs.get(&format!("{class_name}.init")).cloned() {
                            check_method_visibility(&init, &class_name, self.vars, self.funcs)?;
                            call_method_with_context(
                                &init,
                                ctor_args,
                                object.clone(),
                                self.funcs,
                                self.context,
                            )?;
                        }
                        object
                    }
                    _ => return Err("new expects a text class name".into()),
                }
            }
            Token::Name(n) if n == "range" && *self.peek() == Token::LParen => {
                self.take();
                let first = self.parse(0)?;
                let (start, end) = if *self.peek() == Token::Comma {
                    self.take();
                    (first, self.parse(0)?)
                } else {
                    (Value::Number(0), first)
                };
                if self.take() != Token::RParen {
                    return Err("expected ) after range".into());
                }
                match (start, end) {
                    (Value::Number(a), Value::Number(b)) if a <= b => {
                        Value::List((a..b).map(Value::Number).collect())
                    }
                    _ => return Err("range expects numeric bounds".into()),
                }
            }
            Token::Name(n) if n == "read_text" && *self.peek() == Token::LParen => {
                self.take();
                let path = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after read_text".into());
                }
                match path {
                    Value::Text(p) => Value::Text(read_limited_text(
                        &evaluator::confined_path(
                            Path::new(&p),
                            "read_text",
                            Some(&*self.context),
                        )?,
                        "read_text",
                    )?),
                    _ => return Err("read_text expects a text path".into()),
                }
            }
            Token::Name(n) if n == "write_text" && *self.peek() == Token::LParen => {
                self.take();
                let path = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in write_text".into());
                }
                let content = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after write_text".into());
                }
                match (path, content) {
                    (Value::Text(p), Value::Text(c)) => {
                        write_limited_text(
                            &evaluator::confined_path(
                                Path::new(&p),
                                "write_text",
                                Some(&*self.context),
                            )?,
                            &c,
                            "write_text",
                        )?;
                        Value::None
                    }
                    _ => return Err("write_text expects text path and content".into()),
                }
            }
            Token::Name(n) if n == "read_lines" && *self.peek() == Token::LParen => {
                self.take();
                let path = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after read_lines".into());
                }
                match path {
                    Value::Text(p) => Value::List(
                        read_limited_text(
                            &evaluator::confined_path(
                                Path::new(&p),
                                "read_lines",
                                Some(&*self.context),
                            )?,
                            "read_lines",
                        )?
                        .lines()
                        .map(|line| Value::Text(line.to_string()))
                        .collect(),
                    ),
                    _ => return Err("read_lines expects a text path".into()),
                }
            }
            Token::Name(n) if n == "write_lines" && *self.peek() == Token::LParen => {
                self.take();
                let path = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in write_lines".into());
                }
                let lines = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after write_lines".into());
                }
                match (path, lines) {
                    (Value::Text(p), Value::List(xs)) => {
                        let mut out = String::new();
                        for (i, v) in xs.iter().enumerate() {
                            if i > 0 {
                                out.push('\n');
                            }
                            if let Value::Text(s) = v {
                                out.push_str(s)
                            } else {
                                return Err("write_lines expects a list of text".into());
                            }
                        }
                        write_limited_text(
                            &evaluator::confined_path(
                                Path::new(&p),
                                "write_lines",
                                Some(&*self.context),
                            )?,
                            &out,
                            "write_lines",
                        )?;
                        Value::None
                    }
                    _ => return Err("write_lines expects a text path and list".into()),
                }
            }
            Token::Name(n) if n == "ok" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after ok".into());
                }
                Value::ResultOk(Box::new(value))
            }
            Token::Name(n) if n == "err" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after err".into());
                }
                Value::ResultErr(Box::new(value))
            }
            Token::Name(n) if n == "some" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after some".into());
                }
                Value::OptionSome(Box::new(value))
            }
            Token::Name(n) if n == "option_none" && *self.peek() == Token::LParen => {
                self.take();
                if self.take() != Token::RParen {
                    return Err("expected ) after option_none".into());
                }
                Value::OptionNone
            }
            Token::Name(n) if n == "is_ok" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after is_ok".into());
                }
                Value::Bool(matches!(value, Value::ResultOk(_)))
            }
            Token::Name(n) if n == "is_err" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after is_err".into());
                }
                Value::Bool(matches!(value, Value::ResultErr(_)))
            }
            Token::Name(n) if n == "is_some" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after is_some".into());
                }
                Value::Bool(matches!(value, Value::OptionSome(_)))
            }
            Token::Name(n) if n == "is_option_none" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after is_option_none".into());
                }
                Value::Bool(matches!(value, Value::OptionNone))
            }
            Token::Name(n) if n == "unwrap" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after unwrap".into());
                }
                match value {
                    Value::ResultOk(x) | Value::OptionSome(x) => *x,
                    Value::ResultErr(x) => return Err(format!("unwrap failed: {}", x.show())),
                    Value::OptionNone => return Err("unwrap failed: option is none".into()),
                    _ => return Err("unwrap expects a result or option".into()),
                }
            }
            Token::Name(n) if n == "unwrap_or" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in unwrap_or".into());
                }
                let fallback = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after unwrap_or".into());
                }
                match value {
                    Value::ResultOk(x) | Value::OptionSome(x) => *x,
                    Value::ResultErr(_) | Value::OptionNone => fallback,
                    _ => return Err("unwrap_or expects a result or option".into()),
                }
            }
            Token::Name(n) if *self.peek() == Token::LParen => {
                self.take();
                let mut args = Vec::new();
                if *self.peek() != Token::RParen {
                    loop {
                        args.push(self.parse(0)?);
                        if *self.peek() == Token::RParen {
                            break;
                        }
                        if self.take() != Token::Comma {
                            return Err("expected comma in call".into());
                        }
                    }
                }
                self.take();
                if let Some(value) =
                    direct_builtin_with_context(&n, args.clone(), Some(&mut *self.context))?
                {
                    value
                } else if let Some(value) =
                    evaluator::direct_external_builtin_with_context(&n, &args, Some(self.context))?
                {
                    value
                } else if let Some(Value::Callable(function)) = self.vars.get(&n) {
                    call_function_with_context(function, args, self.funcs, self.context)?
                } else {
                    let f = self
                        .funcs
                        .get(&n)
                        .ok_or(format!("undefined function: {n}"))?
                        .clone();
                    call_function_with_context(&f, args, self.funcs, self.context)?
                }
            }
            Token::Name(n) => self
                .vars
                .get(&n)
                .cloned()
                .or_else(|| {
                    self.funcs
                        .get(&n)
                        .map(|function| Value::Callable(function.clone()))
                })
                .ok_or(format!("undefined variable: {n}"))?,
            Token::LParen => {
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected )".into());
                }
                v
            }
            Token::LBracket => {
                let mut x = Vec::new();
                if *self.peek() != Token::RBracket {
                    loop {
                        x.push(self.parse(0)?);
                        if *self.peek() == Token::RBracket {
                            break;
                        }
                        if self.take() != Token::Comma {
                            return Err("expected comma".into());
                        }
                    }
                }
                self.take();
                Value::List(x)
            }
            Token::LBrace => {
                let mut m = HashMap::new();
                if *self.peek() != Token::RBrace {
                    loop {
                        let k = match self.take() {
                            Token::Text(s) | Token::Name(s) => s,
                            _ => return Err("map key must be text or name".into()),
                        };
                        if self.take() != Token::Colon {
                            return Err("expected : in map".into());
                        }
                        m.insert(k, self.parse(0)?);
                        if *self.peek() == Token::RBrace {
                            break;
                        }
                        if self.take() != Token::Comma {
                            return Err("expected comma in map".into());
                        }
                    }
                }
                self.take();
                Value::Map(m)
            }
            x => return Err(format!("unexpected token: {x:?}")),
        };
        loop {
            if *self.peek() == Token::Dot {
                self.take();
                let member = match self.take() {
                    Token::Name(name) => name,
                    _ => return Err("expected a property or method name after .".into()),
                };
                if *self.peek() == Token::LParen {
                    self.take();
                    let args = self.call_args()?;
                    let (dispatch_class, receiver) = match &left {
                        Value::Object { class_name, .. } => (class_name.clone(), left.clone()),
                        Value::Text(class_name)
                            if self.vars.contains_key("self")
                                && self.funcs.contains_key(&format!("{class_name}.__class__")) =>
                        {
                            let receiver = self
                                .vars
                                .get("self")
                                .cloned()
                                .ok_or("super calls require a method receiver".to_string())?;
                            (class_name.clone(), receiver)
                        }
                        _ => return Err("methods can only be called on objects".into()),
                    };
                    let f = self
                        .funcs
                        .get(&format!("{dispatch_class}.{member}"))
                        .ok_or(format!("undefined method: {dispatch_class}.{member}"))?
                        .clone();
                    left = call_method_with_context(&f, args, receiver, self.funcs, self.context)?;
                } else {
                    left = match left {
                        Value::Object { fields, .. } => fields
                            .try_borrow()?
                            .get(&member)
                            .cloned()
                            .ok_or(format!("property not found: {member}"))?,
                        Value::Map(m) => m
                            .get(&member)
                            .cloned()
                            .ok_or(format!("key not found: {member}"))?,
                        _ => return Err("property access expects an object or map".into()),
                    };
                }
                continue;
            }
            if *self.peek() == Token::LBracket {
                self.take();
                let idx = self.parse(0)?;
                if self.take() != Token::RBracket {
                    return Err("expected ]".into());
                }
                left = match (left, idx) {
                    (Value::List(x), Value::Number(n)) if n >= 0 => x
                        .get(n as usize)
                        .cloned()
                        .ok_or("index out of range".to_string())?,
                    (Value::Map(m), Value::Text(k)) => {
                        m.get(&k).cloned().ok_or("key not found".to_string())?
                    }
                    _ => return Err("invalid index operation".into()),
                };
                continue;
            }
            let op = self.peek().clone();
            let p = match op {
                Token::EqEq
                | Token::NotEq
                | Token::Less
                | Token::Greater
                | Token::LessEq
                | Token::GreaterEq
                | Token::And
                | Token::Or => 1,
                Token::Plus | Token::Minus => 2,
                Token::Star | Token::Slash | Token::Percent => 3,
                _ => 0,
            };
            if p == 0 || p < min {
                break;
            }
            self.take();
            left = operate(left, op, self.parse(p + 1)?)?;
        }
        Ok(left)
    }
}
fn run(source: &str, base: &Path) -> Result<(), String> {
    let mut context = ExecutionContext::new();
    context.reset_for_run();
    let mut vars = HashMap::new();
    vars.insert("__zap_module".into(), Value::Text("__main__".into()));
    let mut funcs = HashMap::new();
    validate_source_layout(source)?;
    let program = ast::parse_program(source).map_err(|error| format!("syntax error: {error}"))?;
    match execute_ast_program_with_context(&program, &mut vars, &mut funcs, &mut context, base)? {
        Flow::Continue | Flow::Return(_) => Ok(()),
        Flow::Break | Flow::LoopContinue => Err("break/continue must be inside a loop".into()),
        Flow::Raise(value) => Err(format!("raised error: {}", value.show())),
    }
}

fn format_source(source: &str) -> String {
    let mut out = String::new();
    for line in source.lines() {
        let normalized = line.replace('\t', "    ").trim_end().to_string();
        out.push_str(&normalized);
        out.push('\n');
    }
    out
}
#[cfg(test)]
mod stdlib_catalog_tests {
    use super::stdlib_catalog::{contains, PUBLIC_BUILTINS};

    #[test]
    fn public_builtin_catalog_is_unique_and_domain_grouped() {
        for (index, builtin) in PUBLIC_BUILTINS.iter().enumerate() {
            assert!(!builtin.name.is_empty());
            assert!(!builtin.domain.is_empty());
            assert!(!PUBLIC_BUILTINS[index + 1..]
                .iter()
                .any(|other| other.name == builtin.name));
            assert!(contains(builtin.name));
        }
    }
}

fn manifest_value(text: &str, key: &str) -> Option<String> {
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with('#') || !line.starts_with(key) {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            if k.trim() == key {
                return Some(v.trim().trim_matches('"').to_string());
            }
        }
    }
    None
}
fn validate_function_signatures(source: &str, file: &Path) -> Result<(), String> {
    let allowed = [
        "text", "number", "bool", "list", "map", "object", "none", "any", "function",
    ];
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed
            .strip_prefix("fn ")
            .or_else(|| trimmed.strip_prefix("def "))
        {
            let head = rest.trim_end_matches(':').trim();
            let (_name, args) = head.split_once('(').ok_or_else(|| {
                format!(
                    "SyntaxError at {}:{}: function signature requires parentheses",
                    file.display(),
                    index + 1
                )
            })?;
            let (params, return_annotation) = parse_signature(args).map_err(|error| {
                format!("SyntaxError at {}:{}: {error}", file.display(), index + 1)
            })?;
            for param in &params {
                if let Some(annotation) = &param.annotation {
                    if !is_allowed_annotation(annotation) {
                        return Err(format!(
                            "TypeError at {}:{}: unknown type annotation '{}'",
                            file.display(),
                            index + 1,
                            annotation
                        ));
                    }
                }
            }
            if let Some(annotation) = return_annotation {
                if !allowed.contains(&annotation.as_str()) {
                    return Err(format!(
                        "TypeError at {}:{}: unknown return type annotation '{}'",
                        file.display(),
                        index + 1,
                        annotation
                    ));
                }
            }
        }
    }
    Ok(())
}
fn validate_function_returns(source: &str, file: &Path) -> Result<(), String> {
    let mut signatures = HashMap::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed
            .strip_prefix("fn ")
            .or_else(|| trimmed.strip_prefix("def "))
        {
            let head = rest.trim_end_matches(':').trim();
            let (name, args) = head.split_once('(').ok_or_else(|| {
                format!(
                    "SyntaxError at {}: function signature requires parentheses",
                    file.display()
                )
            })?;
            let (params, return_annotation) = parse_signature(args)
                .map_err(|error| format!("SyntaxError at {}: {error}", file.display()))?;
            signatures.insert(
                name.trim().to_string(),
                StaticSignature {
                    params,
                    return_annotation,
                },
            );
        }
    }
    let mut active: Option<(usize, String, Option<String>, HashMap<String, String>)> = None;
    for (line_index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        let indent = line
            .chars()
            .take_while(|character| *character == ' ')
            .count();
        if let Some(rest) = trimmed
            .strip_prefix("fn ")
            .or_else(|| trimmed.strip_prefix("def "))
        {
            let head = rest.trim_end_matches(':').trim();
            let (name, args) = head.split_once('(').ok_or_else(|| {
                format!(
                    "SyntaxError at {}:{}: function signature requires parentheses",
                    file.display(),
                    line_index + 1
                )
            })?;
            let (params, return_annotation) = parse_signature(args).map_err(|error| {
                format!(
                    "SyntaxError at {}:{}: {error}",
                    file.display(),
                    line_index + 1
                )
            })?;
            let vars = params
                .iter()
                .filter_map(|param| {
                    param
                        .annotation
                        .as_ref()
                        .map(|annotation| (param.name.clone(), annotation.clone()))
                })
                .collect();
            active = Some((indent, name.trim().to_string(), return_annotation, vars));
            continue;
        }
        let Some((function_indent, function_name, return_annotation, vars)) = active.as_mut()
        else {
            continue;
        };
        if !trimmed.is_empty() && indent <= *function_indent {
            active = None;
            continue;
        }
        let Some(return_value) = trimmed.strip_prefix("return") else {
            continue;
        };
        let Some(expected) = return_annotation.as_deref() else {
            continue;
        };
        let expression = return_value.trim();
        let actual = if expression.is_empty() {
            "none".to_string()
        } else {
            static_expr_type(expression, vars, &signatures).unwrap_or_else(|| "any".into())
        };
        if !annotation_matches(expected, &actual) {
            return Err(format!(
                "TypeError at {}:{}: return from '{}' expects {}, got {}",
                file.display(),
                line_index + 1,
                function_name,
                expected,
                actual
            ));
        }
    }
    Ok(())
}
fn split_static_conditional(value: &str) -> Option<(&str, &str, &str)> {
    let rest = value.strip_prefix("if ")?;
    let then_index = rest.find(" then ")?;
    let after_then = &rest[then_index + 6..];
    let else_index = after_then.rfind(" else ")?;
    let condition = rest[..then_index].trim();
    let then_branch = after_then[..else_index].trim();
    let else_branch = after_then[else_index + 6..].trim();
    (!condition.is_empty() && !then_branch.is_empty() && !else_branch.is_empty()).then_some((
        condition,
        then_branch,
        else_branch,
    ))
}

fn static_expr_type(
    raw: &str,
    vars: &HashMap<String, String>,
    signatures: &HashMap<String, StaticSignature>,
) -> Option<String> {
    let value = raw.trim();
    if let Some(inner) = value.strip_suffix('?') {
        let result_type = static_expr_type(inner.trim(), vars, signatures)?;
        return ["result<", "option<"].iter().find_map(|prefix| {
            result_type
                .strip_prefix(prefix)
                .and_then(|rest| rest.strip_suffix('>'))
                .map(str::to_string)
        });
    }
    if let Some((condition, then_branch, else_branch)) = split_static_conditional(value) {
        if static_expr_type(condition, vars, signatures).as_deref() != Some("bool") {
            return None;
        }
        let then_type = static_expr_type(then_branch, vars, signatures)?;
        let else_type = static_expr_type(else_branch, vars, signatures)?;
        return (then_type == else_type).then_some(then_type);
    }
    if value.starts_with('[') && value.ends_with(']') {
        let inner = &value[1..value.len() - 1];
        let mut item_type: Option<String> = None;
        for item in split_static_args(inner) {
            let Some(kind) = static_expr_type(&item, vars, signatures) else {
                continue;
            };
            if let Some(current) = &item_type {
                if current != &kind {
                    item_type = Some("any".into());
                    break;
                }
            } else {
                item_type = Some(kind);
            }
        }
        return Some(generic_type("list", item_type.as_deref().unwrap_or("any")));
    }
    if value.starts_with('{') && value.ends_with('}') {
        let inner = &value[1..value.len() - 1];
        let mut value_type = "any".to_string();
        for entry in split_static_args(inner) {
            if let Some((_, raw_value)) = entry.split_once(':') {
                if let Some(kind) = static_expr_type(raw_value, vars, signatures) {
                    value_type = kind;
                    break;
                }
            }
        }
        return Some(format!("map<text,{value_type}>"));
    }
    for operator in ["and", "or"] {
        let terms = split_boolean_terms(value, operator);
        if terms.len() > 1
            && terms
                .iter()
                .all(|term| static_expr_type(term, vars, signatures).as_deref() == Some("bool"))
        {
            return Some("bool".into());
        }
    }
    if let Some(kind) = static_literal_type(value) {
        return Some(kind.to_string());
    }
    if let Some(kind) = vars.get(value) {
        return Some(kind.clone());
    }
    if value.starts_with('(') && value.ends_with(')') {
        return static_expr_type(&value[1..value.len() - 1], vars, signatures);
    }
    if let Some(open) = value.find('(') {
        if value.ends_with(')') {
            let name = value[..open].trim();
            let close = matching_paren(value, open)?;
            let args = split_static_args(&value[open + 1..close]);
            if name == "ok" || name == "err" {
                let payload = args
                    .first()
                    .and_then(|arg| static_expr_type(arg, vars, signatures))
                    .unwrap_or_else(|| "any".into());
                return Some(generic_type("result", &payload));
            }
            if name == "some" {
                let payload = args
                    .first()
                    .and_then(|arg| static_expr_type(arg, vars, signatures))
                    .unwrap_or_else(|| "any".into());
                return Some(generic_type("option", &payload));
            }
            if name == "option_none" {
                return Some("option<any>".into());
            }
            if let Some(signature) = signatures.get(name) {
                return signature.return_annotation.clone();
            }
        }
    }
    let mut depth = 0i32;
    let mut quote = None;
    for (index, ch) in value.char_indices() {
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            }
            continue;
        }
        if ch == '"' || ch == '\'' {
            quote = Some(ch);
            continue;
        }
        match ch {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            '+' | '-' | '*' | '/' | '%' if depth == 0 => {
                let left = static_expr_type(&value[..index], vars, signatures);
                let right = static_expr_type(&value[index + ch.len_utf8()..], vars, signatures);
                if ch == '+' && left.as_deref() == Some("text") && right.as_deref() == Some("text")
                {
                    return Some("text".into());
                }
                if left.as_deref() == Some("number") && right.as_deref() == Some("number") {
                    return Some("number".into());
                }
                return None;
            }
            _ => {}
        }
    }
    None
}

#[allow(clippy::too_many_arguments)]
fn validate_static_call(
    name: &str,
    args: &[String],
    params: &[Param],
    vars: &HashMap<String, String>,
    signatures: &HashMap<String, StaticSignature>,
    file: &Path,
    line: usize,
    column: usize,
) -> Result<(), String> {
    let required = params
        .iter()
        .filter(|param| param.default.is_none())
        .count();
    if args.len() < required || args.len() > params.len() {
        return Err(format!(
            "TypeError at {}:{}:{}: function '{}' expects {} to {} arguments, got {}",
            file.display(),
            line,
            column,
            name,
            required,
            params.len(),
            args.len()
        ));
    }
    for (param, arg) in params.iter().zip(args.iter()) {
        if let Some(expected) = param.annotation.as_deref() {
            if expected == "any" {
                continue;
            }
            if let Some(actual) = static_expr_type(arg, vars, signatures) {
                if !annotation_matches(expected, &actual) {
                    return Err(format!(
                        "TypeError at {}:{}:{}: argument '{}' for '{}' expects {}, got {}",
                        file.display(),
                        line,
                        column,
                        param.name,
                        name,
                        expected,
                        actual
                    ));
                }
            }
        }
    }
    Ok(())
}

fn split_boolean_terms(condition: &str, operator: &str) -> Vec<String> {
    let mut terms = Vec::new();
    let mut start = 0usize;
    let mut depth = 0i32;
    let mut quote = None;
    let bytes = condition.as_bytes();
    let token = format!(" {operator} ");
    let mut index = 0usize;
    while index < bytes.len() {
        let ch = bytes[index] as char;
        if let Some(q) = quote {
            if ch == q {
                quote = None;
            }
            index += 1;
            continue;
        }
        if ch == '"' || ch == '\'' {
            quote = Some(ch);
            index += 1;
            continue;
        }
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if depth == 0 && condition[index..].starts_with(&token) {
            terms.push(condition[start..index].trim().to_string());
            index += token.len();
            start = index;
        } else {
            index += 1;
        }
    }
    terms.push(condition[start..].trim().to_string());
    terms
}

fn single_narrowed_type(
    condition: &str,
    vars: &HashMap<String, String>,
) -> Option<(String, String)> {
    let condition = condition.trim();
    let condition = if condition.starts_with('(') && condition.ends_with(')') {
        &condition[1..condition.len() - 1]
    } else {
        condition
    };
    let (guard, argument) = ["is_some", "is_ok", "is_err"].iter().find_map(|guard| {
        let prefix = format!("{guard}(");
        let argument = condition.strip_prefix(&prefix)?.strip_suffix(')')?;
        Some((*guard, argument.trim()))
    })?;
    if argument.is_empty() || argument.contains('(') || argument.contains(' ') {
        return None;
    }
    let kind = vars.get(argument)?;
    let prefix = if guard == "is_some" {
        "option<"
    } else {
        "result<"
    };
    let payload = kind.strip_prefix(prefix)?.strip_suffix('>')?;
    Some((argument.to_string(), payload.to_string()))
}

fn else_narrowed_branch_type(
    condition: &str,
    vars: &HashMap<String, String>,
) -> Option<(String, String)> {
    let condition = condition.trim();
    let argument = condition
        .strip_prefix("is_option_none(")?
        .strip_suffix(')')?
        .trim();
    if argument.is_empty() || argument.contains('(') || argument.contains(' ') {
        return None;
    }
    let kind = vars.get(argument)?;
    let payload = kind.strip_prefix("option<")?.strip_suffix('>')?;
    Some((argument.to_string(), payload.to_string()))
}

fn narrowed_branch_types(condition: &str, vars: &HashMap<String, String>) -> Vec<(String, String)> {
    let condition = condition.trim();
    let conjunction = split_boolean_terms(condition, "and");
    if conjunction.len() > 1 {
        return conjunction
            .iter()
            .filter_map(|term| single_narrowed_type(term, vars))
            .collect();
    }
    if let Some(single) = single_narrowed_type(condition, vars) {
        return vec![single];
    }
    let disjunction = split_boolean_terms(condition, "or");
    if disjunction.len() > 1 {
        let candidates = disjunction
            .iter()
            .filter_map(|term| single_narrowed_type(term, vars))
            .collect::<Vec<_>>();
        if !candidates.is_empty()
            && candidates
                .iter()
                .all(|candidate| candidate == &candidates[0])
        {
            return candidates;
        }
    }
    Vec::new()
}

fn validate_function_calls(source: &str, file: &Path) -> Result<(), String> {
    let mut signatures: HashMap<String, StaticSignature> = HashMap::new();
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed
            .strip_prefix("fn ")
            .or_else(|| trimmed.strip_prefix("def "))
        {
            let head = rest.trim_end_matches(':').trim();
            let (name, args) = head.split_once('(').ok_or_else(|| {
                format!(
                    "SyntaxError at {}: function signature requires parentheses",
                    file.display()
                )
            })?;
            let (params, return_annotation) = parse_signature(args)
                .map_err(|error| format!("SyntaxError at {}: {error}", file.display()))?;
            signatures.insert(
                name.trim().to_string(),
                StaticSignature {
                    params,
                    return_annotation,
                },
            );
        }
    }
    let mut vars: HashMap<String, String> = HashMap::new();
    let mut branch_scopes: Vec<(usize, HashMap<String, String>)> = Vec::new();
    let mut pending_else: Option<(usize, String)> = None;
    for (line_index, line) in source.lines().enumerate() {
        let indent = line.chars().take_while(|ch| *ch == ' ').count();
        while branch_scopes
            .last()
            .is_some_and(|(scope_indent, _)| indent <= *scope_indent)
        {
            if let Some((_, previous)) = branch_scopes.pop() {
                vars = previous;
            }
        }
        let trimmed = line.trim();
        if trimmed == "else:" {
            if let Some((if_indent, condition)) = pending_else.take() {
                if if_indent == indent {
                    if let Some((name, kind)) = else_narrowed_branch_type(&condition, &vars) {
                        let previous = vars.clone();
                        vars.insert(name, kind);
                        branch_scopes.push((indent, previous));
                    }
                }
            }
        } else if pending_else
            .as_ref()
            .is_some_and(|(if_indent, _)| indent <= *if_indent)
        {
            pending_else = None;
        }
        if trimmed.starts_with("fn ") || trimmed.starts_with("def ") {
            continue;
        }
        if let Some(condition) = trimmed
            .strip_prefix("if ")
            .or_else(|| trimmed.strip_prefix("while "))
        {
            let condition = condition.trim_end_matches(':').trim();
            if let Some(kind) = static_expr_type(condition, &vars, &signatures) {
                if kind != "bool" {
                    return Err(format!(
                        "TypeError at {}:{}:1: control-flow condition expects bool, got {}",
                        file.display(),
                        line_index + 1,
                        kind
                    ));
                }
            }
            if trimmed.starts_with("if ") || trimmed.starts_with("while ") {
                if trimmed.starts_with("if ") {
                    pending_else = Some((indent, condition.to_string()));
                }
                let narrowed = narrowed_branch_types(condition, &vars);
                if !narrowed.is_empty() {
                    let previous = vars.clone();
                    for (name, kind) in narrowed {
                        vars.insert(name, kind);
                    }
                    branch_scopes.push((indent, previous));
                }
            }
        }
        if let Some(rest) = trimmed.strip_prefix("let ") {
            if let Some((left, right)) = rest.split_once('=') {
                let (name, annotation) = left
                    .trim()
                    .split_once(':')
                    .map(|(n, ty)| (n.trim(), Some(ty.trim())))
                    .unwrap_or((left.trim(), None));
                if let Some((condition, then_branch, else_branch)) =
                    split_static_conditional(right.trim())
                {
                    let condition_type = static_expr_type(condition, &vars, &signatures);
                    if condition_type.as_deref() != Some("bool") {
                        return Err(format!(
                            "TypeError at {}:{}:1: conditional expression expects bool condition",
                            file.display(),
                            line_index + 1
                        ));
                    }
                    let then_type = static_expr_type(then_branch, &vars, &signatures);
                    let else_type = static_expr_type(else_branch, &vars, &signatures);
                    if then_type.is_some() && else_type.is_some() && then_type != else_type {
                        return Err(format!(
                            "TypeError at {}:{}:1: conditional branches must have compatible types, got {} and {}",
                            file.display(),
                            line_index + 1,
                            then_type.unwrap_or_default(),
                            else_type.unwrap_or_default()
                        ));
                    }
                }
                if let Some(kind) = static_expr_type(right, &vars, &signatures) {
                    if let Some(expected) = annotation {
                        if !is_allowed_annotation(expected) {
                            return Err(format!(
                                "TypeError at {}:{}:1: unknown type annotation '{}'",
                                file.display(),
                                line_index + 1,
                                expected
                            ));
                        }
                        if !annotation_matches(expected, &kind) {
                            return Err(format!(
                                "TypeError at {}:{}:1: variable '{}' expects {}, got {}",
                                file.display(),
                                line_index + 1,
                                name,
                                expected,
                                kind
                            ));
                        }
                        vars.insert(name.to_string(), expected.to_string());
                    } else {
                        vars.insert(name.to_string(), kind);
                    }
                }
            }
        } else if !trimmed.contains("==")
            && !trimmed.contains("!=")
            && !trimmed.contains("<=")
            && !trimmed.contains(">=")
        {
            if let Some((name, right)) = trimmed.split_once('=') {
                let name = name.trim();
                if !name.contains(' ') && !name.contains('(') {
                    if let Some(kind) = static_expr_type(right, &vars, &signatures) {
                        if let Some(expected) = vars.get(name) {
                            if !annotation_matches(expected, &kind) {
                                return Err(format!(
                                    "TypeError at {}:{}:1: variable '{}' expects {}, got {}",
                                    file.display(),
                                    line_index + 1,
                                    name,
                                    expected,
                                    kind
                                ));
                            }
                        }
                        vars.insert(name.to_string(), kind);
                    }
                }
            }
        }
        let mut search_from = 0usize;
        while search_from < line.len() {
            let Some(relative) = line[search_from..].find('(') else {
                break;
            };
            let open = search_from + relative;
            let prefix = &line[..open];
            let name_start = prefix
                .char_indices()
                .rev()
                .find(|(_, ch)| !ch.is_ascii_alphanumeric() && *ch != '_')
                .map(|(index, _)| index + 1)
                .unwrap_or(0);
            let name = &prefix[name_start..];
            if let Some(signature) = signatures.get(name) {
                if let Some(close) = matching_paren(line, open) {
                    let args_text = &line[open + 1..close];
                    let args = if args_text.trim().is_empty() {
                        Vec::new()
                    } else {
                        split_static_args(args_text)
                    };
                    validate_static_call(
                        name,
                        &args,
                        &signature.params,
                        &vars,
                        &signatures,
                        file,
                        line_index + 1,
                        name_start + 1,
                    )?;
                }
            }
            search_from = open + 1;
        }
    }
    Ok(())
}
pub(crate) fn source_diagnostics(source: &str, file: &Path) -> Vec<String> {
    let mut diagnostics = lint_source(source);
    if let Err(error) = validate_function_calls(source, file) {
        diagnostics.push(error);
    }
    diagnostics
}
fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}
fn lint_source(source: &str) -> Vec<String> {
    let mut issues = Vec::new();
    for (index, line) in source.lines().enumerate() {
        let number = index + 1;
        if line.contains('\t') {
            issues.push(format!("line {number}: tabs are not allowed; use spaces"));
        }
        if line.trim_end() != line {
            issues.push(format!("line {number}: trailing whitespace"));
        }
        if line.len() > 120 {
            issues.push(format!("line {number}: line exceeds 120 characters"));
        }
    }
    issues
}
fn print_project_json(dir: &Path) {
    match validate_project(dir) {
        Ok(info) => println!("{{\"ok\":true,\"project\":\"{}\"}}", json_escape(&info)),
        Err(error) => {
            let diagnostic = ZapError::from_message(error);
            println!("{{\"ok\":false,{}}}", diagnostic.json_fields());
            process::exit(1);
        }
    }
}
fn legacy_configuration_directory() -> String {
    #[cfg(target_os = "windows")]
    {
        return env::var("APPDATA")
            .or_else(|_| env::var("LOCALAPPDATA"))
            .unwrap_or_else(|_| ".".into());
    }
    #[cfg(target_os = "macos")]
    {
        return env::var("HOME")
            .map(|home| format!("{home}/Library/Application Support"))
            .unwrap_or_else(|_| ".".into());
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        env::var("XDG_CONFIG_HOME")
            .or_else(|_| env::var("HOME").map(|home| format!("{home}/.config")))
            .unwrap_or_else(|_| ".".into())
    }
}

fn legacy_configuration_path(name: &str) -> Result<String, String> {
    if name.is_empty() || name.contains('/') || name.contains('\\') || name == "." || name == ".." {
        return Err("config_path expects a single relative file name".into());
    }
    Ok(Path::new(&legacy_configuration_directory())
        .join(name)
        .to_string_lossy()
        .into())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    cli::run_cli(&args);
}

#[cfg(test)]
mod zap_error_tests {
    use super::*;

    #[test]
    fn classifies_type_errors_and_preserves_location() {
        let error = ZapError::from_message("TypeError at main.zp:4:12: expected number, got text");
        assert_eq!(error.kind(), "TypeError");
        assert_eq!(error.parts().1, "main.zp");
        assert_eq!(error.parts().2, 4);
        assert_eq!(error.parts().3, 12);
        assert!(error.to_string().contains("TypeError at main.zp:4:12"));
    }

    #[test]
    fn classifies_io_and_file_errors() {
        assert_eq!(
            ZapError::from_message("cannot read missing.zp: No such file").kind(),
            "FileNotFound"
        );
        assert_eq!(
            ZapError::from_message("cannot write output.zp: permission denied").kind(),
            "PermissionError"
        );
        assert_eq!(
            ZapError::from_message("cannot read config.zp: I/O failure").kind(),
            "IOError"
        );
    }

    #[test]
    fn unknown_errors_use_project_kind_without_losing_message() {
        let error = ZapError::from_message("module import failed");
        assert_eq!(error.kind(), "ProjectError");
        assert!(error.to_string().contains("module import failed"));
    }

    #[test]
    fn rejects_oversized_user_files_before_reading() {
        let path = std::env::temp_dir().join("zap_oversized_file_test.zp");
        std::fs::write(&path, vec![b'x'; (MAX_FILE_BYTES + 1) as usize]).unwrap();
        let result = read_limited_text(&path, "test read");
        let _ = std::fs::remove_file(&path);
        let error = result.expect_err("oversized files must be rejected");
        assert!(error.contains("file exceeds"));
    }

    #[test]
    fn normal_programs_do_not_fallback_to_legacy_line_execution() {
        let error = run_checked("use core", Path::new("."))
            .expect_err("AST import semantics should reject an unresolved module");
        assert!(error.message().contains("module not found: core"));
    }

    #[test]
    fn compatibility_expression_supports_callable_aliases() {
        let mut context = ExecutionContext::new();
        let mut vars = HashMap::new();
        let mut funcs = HashMap::new();
        funcs.insert(
            "increment".into(),
            Rc::new(Function {
                visibility: "public".into(),
                params: vec![Param {
                    name: "value".into(),
                    annotation: Some("number".into()),
                    default: None,
                }],
                return_annotation: Some("number".into()),
                is_async: false,
                body: vec!["return value + 1".into()],
                ast_body: None,
                closure: EnvFrame::from_map(&HashMap::new()),
            }),
        );
        let alias_tokens = lexer::tokenize("increment").expect("alias expression should tokenize");
        let alias = ExprParser::new(&alias_tokens, &vars, &funcs, &mut context)
            .parse_complete()
            .expect("legacy function names should produce callable values");
        assert_eq!(alias.show(), "<callable>");
        vars.insert("alias".into(), alias);
        let call_tokens = lexer::tokenize("alias(4)").expect("call expression should tokenize");
        let result = ExprParser::new(&call_tokens, &vars, &funcs, &mut context)
            .parse_complete()
            .expect("callable aliases should be invokable");
        assert_eq!(result, Value::Number(5));
    }

    #[test]
    fn wraps_runtime_string_errors_as_typed_errors() {
        let error =
            run_checked("say missing_name", Path::new(".")).expect_err("unknown names must fail");
        assert_eq!(error.kind(), "NameError");
        assert!(error.message().contains("undefined variable"));
    }
}
