#![allow(clippy::missing_const_for_thread_local, clippy::type_complexity)]

mod diagnostics;
mod lexer;

use diagnostics::ZapError;
use lexer::Token;
mod value;

use value::{Function, Param, StaticSignature, Value};
mod project;

use project::{resolve_module, run_zap_tests, validate_project};
mod parser;

use parser::{
    annotation_matches, generic_type, is_allowed_annotation, matching_paren, parse_signature,
    split_static_args, static_literal_type,
};
mod evaluator;

use evaluator::{
    evaluate_with_propagation, expression, json_to_value, operate, value_to_json, EvalOutcome, Flow,
};

use std::{
    cell::RefCell,
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
    process,
    rc::Rc,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
thread_local! { static MODULE_LOADING: RefCell<Vec<PathBuf>> = const { RefCell::new(Vec::new()) }; }
thread_local! { static MODULE_CACHE: RefCell<HashMap<PathBuf, (HashMap<String,Value>, HashMap<String,Rc<Function>>)>> = RefCell::new(HashMap::new()); }

struct ExprParser<'a> {
    tokens: &'a [Token],
    pos: usize,
    vars: &'a HashMap<String, Value>,
    funcs: &'a HashMap<String, Rc<Function>>,
}
impl<'a> ExprParser<'a> {
    fn new(
        t: &'a [Token],
        v: &'a HashMap<String, Value>,
        f: &'a HashMap<String, Rc<Function>>,
    ) -> Self {
        Self {
            tokens: t,
            pos: 0,
            vars: v,
            funcs: f,
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
        &self.tokens[self.pos]
    }
    fn take(&mut self) -> Token {
        let x = self.tokens[self.pos].clone();
        self.pos += 1;
        x
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
            Token::Name(n) if n == "sleep" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after sleep".into());
                }
                match v {
                    Value::Number(ms) if ms >= 0 => {
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
            Token::Name(n) if n == "pow" && *self.peek() == Token::LParen => {
                self.take();
                let base = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in pow".into());
                }
                let exponent = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after pow".into());
                }
                match (base, exponent) {
                    (Value::Number(a), Value::Number(b)) if b >= 0 => Value::Number(
                        a.checked_pow(b as u32)
                            .ok_or("pow result overflow".to_string())?,
                    ),
                    _ => return Err("pow expects a number and a non-negative exponent".into()),
                }
            }
            Token::Name(n) if n == "sqrt" && *self.peek() == Token::LParen => {
                self.take();
                let value = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after sqrt".into());
                }
                match value {
                    Value::Number(x) if x >= 0 => Value::Number((x as f64).sqrt().round() as i64),
                    _ => return Err("sqrt expects a non-negative number".into()),
                }
            }
            Token::Name(n) if n == "abs" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after abs".into());
                }
                match v {
                    Value::Number(x) => Value::Number(x.abs()),
                    _ => return Err("abs expects a number".into()),
                }
            }
            Token::Name(n) if n == "min" && *self.peek() == Token::LParen => {
                self.take();
                let a = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in min".into());
                }
                let b = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after min".into());
                }
                match (a, b) {
                    (Value::Number(x), Value::Number(y)) => Value::Number(x.min(y)),
                    _ => return Err("min expects numbers".into()),
                }
            }
            Token::Name(n) if n == "max" && *self.peek() == Token::LParen => {
                self.take();
                let a = self.parse(0)?;
                if self.take() != Token::Comma {
                    return Err("expected comma in max".into());
                }
                let b = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after max".into());
                }
                match (a, b) {
                    (Value::Number(x), Value::Number(y)) => Value::Number(x.max(y)),
                    _ => return Err("max expects numbers".into()),
                }
            }
            Token::Name(n) if n == "upper" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after upper".into());
                }
                match v {
                    Value::Text(x) => Value::Text(x.to_uppercase()),
                    _ => return Err("upper expects text".into()),
                }
            }
            Token::Name(n) if n == "lower" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after lower".into());
                }
                match v {
                    Value::Text(x) => Value::Text(x.to_lowercase()),
                    _ => return Err("lower expects text".into()),
                }
            }
            Token::Name(n) if n == "trim" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after trim".into());
                }
                match v {
                    Value::Text(x) => Value::Text(x.trim().into()),
                    _ => return Err("trim expects text".into()),
                }
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
                        Value::ResultOk(_) | Value::ResultErr(_) => "result",
                        Value::OptionSome(_) | Value::OptionNone => "option",
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
                    return Err(message.show());
                }
            }
            Token::Name(n) if n == "json" && *self.peek() == Token::LParen => {
                self.take();
                let v = self.parse(0)?;
                if self.take() != Token::RParen {
                    return Err("expected ) after json".into());
                }
                Value::Text(
                    serde_json::to_string(&value_to_json(&v))
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
                    ),
                    _ => return Err("from_json expects text".into()),
                }
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
                        let object = Value::Object {
                            class_name: class_name.clone(),
                            fields: Rc::new(RefCell::new(fields)),
                        };
                        if self
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
                                        call_method(
                                            &parent_init,
                                            ctor_args.clone(),
                                            object.clone(),
                                            self.funcs,
                                        )?;
                                    }
                                }
                            }
                        }
                        if let Some(init) = self.funcs.get(&format!("{class_name}.init")).cloned() {
                            call_method(&init, ctor_args, object.clone(), self.funcs)?;
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
                    Value::Text(p) => Value::Text(
                        fs::read_to_string(p).map_err(|e| format!("read_text failed: {e}"))?,
                    ),
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
                        fs::write(p, c).map_err(|e| format!("write_text failed: {e}"))?;
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
                        fs::read_to_string(p)
                            .map_err(|e| format!("read_lines failed: {e}"))?
                            .lines()
                            .map(|line| Value::Text(line.into()))
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
                        fs::write(p, out).map_err(|e| format!("write_lines failed: {e}"))?;
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
                let f = self
                    .funcs
                    .get(&n)
                    .ok_or(format!("undefined function: {n}"))?
                    .clone();
                call_function(&f, args, self.funcs)?
            }
            Token::Name(n) => self
                .vars
                .get(&n)
                .cloned()
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
                    let class_name = match &left {
                        Value::Object { class_name, .. } => class_name.clone(),
                        _ => return Err("methods can only be called on objects".into()),
                    };
                    let f = self
                        .funcs
                        .get(&format!("{class_name}.{member}"))
                        .ok_or(format!("undefined method: {class_name}.{member}"))?
                        .clone();
                    left = call_method(&f, args, left, self.funcs)?;
                } else {
                    left = match left {
                        Value::Object { fields, .. } => fields
                            .borrow()
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
fn indented(lines: &[String], start: usize) -> (Vec<String>, usize) {
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
fn call_function(
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
fn call_method(
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
fn check_annotation(name: &str, annotation: &str, value: &Value) -> Result<(), String> {
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
fn load_module(
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
    if Path::new(raw_path).is_absolute() {
        return Err("absolute module paths are not allowed".into());
    }
    let candidate = if Path::new(raw_path).extension().is_some() {
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
    let imported_result = fs::read_to_string(&canonical)
        .map_err(|e| format!("cannot import {}: {e}", canonical.display()));
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
fn execute_lines(
    lines: &[String],
    vars: &mut HashMap<String, Value>,
    funcs: &mut HashMap<String, Rc<Function>>,
    base: &Path,
) -> Result<Flow, String> {
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
                    params: Vec::new(),
                    return_annotation: None,
                    body: Vec::new(),
                    closure: vars.clone(),
                }),
            );
            if let Some(parent_name) = parent.clone() {
                if !funcs.contains_key(&format!("{parent_name}.__class__")) {
                    return Err(format!("unknown parent class: {parent_name}"));
                }
                funcs.insert(
                    format!("{class_name}.__parent__"),
                    Rc::new(Function {
                        params: Vec::new(),
                        return_annotation: None,
                        body: vec![parent_name],
                        closure: vars.clone(),
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
                                params: Vec::new(),
                                return_annotation: None,
                                body: Vec::new(),
                                closure: vars.clone(),
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
                            },
                        );
                    }
                    let (method_body, method_end) = indented(&body, j + 1);
                    funcs.insert(
                        format!("{class_name}.{}", method_name.trim()),
                        Rc::new(Function {
                            params,
                            return_annotation,
                            body: method_body,
                            closure: vars.clone(),
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
                    params: args,
                    return_annotation,
                    body,
                    closure: vars.clone(),
                }),
            );
            if is_export {
                funcs.insert(
                    format!("__zap_export_fn__:{name}"),
                    Rc::new(Function {
                        params: Vec::new(),
                        return_annotation: None,
                        body: Vec::new(),
                        closure: HashMap::new(),
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
                if guard > 100000 {
                    return Err("loop limit exceeded".into());
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
                    for item in items {
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

fn run(source: &str, base: &Path) -> Result<(), String> {
    MODULE_LOADING.with(|stack| stack.borrow_mut().clear());
    MODULE_CACHE.with(|cache| cache.borrow_mut().clear());
    let mut vars = HashMap::new();
    let mut funcs = HashMap::new();
    let lines = source.lines().map(str::to_string).collect::<Vec<_>>();
    match execute_lines(&lines, &mut vars, &mut funcs, base)? {
        Flow::Continue | Flow::Return(_) => Ok(()),
        Flow::Break | Flow::LoopContinue => Err("break/continue must be inside a loop".into()),
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
        "text", "number", "bool", "list", "map", "object", "none", "any",
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
fn static_expr_type(
    raw: &str,
    vars: &HashMap<String, String>,
    signatures: &HashMap<String, StaticSignature>,
) -> Option<String> {
    let value = raw.trim();
    if let Some(inner) = value.strip_suffix('?') {
        let result_type = static_expr_type(inner.trim(), vars, signatures)?;
        return result_type
            .strip_prefix("result<")
            .and_then(|rest| rest.strip_suffix('>'))
            .map(str::to_string);
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
    if args.len() != params.len() {
        return Err(format!(
            "TypeError at {}:{}:{}: function '{}' expects {} arguments, got {}",
            file.display(),
            line,
            column,
            name,
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
    for (line_index, line) in source.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("fn ") || trimmed.starts_with("def ") {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("let ") {
            if let Some((left, right)) = rest.split_once('=') {
                let (name, annotation) = left
                    .trim()
                    .split_once(':')
                    .map(|(n, ty)| (n.trim(), Some(ty.trim())))
                    .unwrap_or((left.trim(), None));
                if let Some(kind) = static_expr_type(right, &vars, &signatures) {
                    if let Some(expected) = annotation {
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
                if !name.contains(' ') && !name.contains('(') {
                    if let Some(kind) = static_expr_type(right, &vars, &signatures) {
                        vars.insert(name.trim().to_string(), kind);
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
            let (_, file, line, column) = diagnostic.parts();
            let message = diagnostic.message();
            println!(
                "{{\"ok\":false,\"kind\":\"{}\",\"file\":\"{}\",\"line\":{},\"column\":{},\"message\":\"{}\",\"error\":\"{}\"}}",
                diagnostic.kind(),
                json_escape(file),
                line,
                column,
                json_escape(message),
                json_escape(&diagnostic.to_string())
            );
            process::exit(1);
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && (args[1] == "--version" || args[1] == "-V") {
        println!("zap 0.9.1 (native)");
        return;
    }
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h") {
        println!("Zap native runtime\n\nUsage:\n  zap <file.zp>       Run a Zap source file\n  zap run <file.zp>   Run a Zap source file explicitly\n  zap fmt <file.zp>   Format a Zap source file\n  zap check [dir]      Validate zap.toml and the project entry file\n  zap test [dir]       Run *_test.zp files in a tests directory
  zap lint <file.zp>   Check formatting and style warnings
  zap check --json     Validate a project with JSON diagnostics\n  zap build [dir]      Validate and prepare a Zap project\n  zap init <dir>       Create a new Zap project\n  zap --version        Show the version\n  zap --help           Show this help");
        return;
    }
    if args.len() == 3 && args[1] == "init" {
        let dir = Path::new(&args[2]);
        if dir.exists() {
            eprintln!("cannot initialize existing path: {}", dir.display());
            process::exit(1);
        }
        fs::create_dir_all(dir).unwrap_or_else(|e| {
            eprintln!("cannot create {}: {e}", dir.display());
            process::exit(1);
        });
        fs::write(
            dir.join("zap.toml"),
            "[package]\nname = \"hello-zap\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
        )
        .unwrap_or_else(|e| {
            eprintln!("cannot write manifest: {e}");
            process::exit(1);
        });
        fs::write(
            dir.join("main.zp"),
            "fn main():\n    say \"Hello from Zap\"\n\nmain()\n",
        )
        .unwrap_or_else(|e| {
            eprintln!("cannot write entry file: {e}");
            process::exit(1);
        });
        fs::create_dir_all(dir.join("tests")).unwrap_or_else(|e| {
            eprintln!("cannot create test directory: {e}");
            process::exit(1);
        });
        fs::write(dir.join("tests").join("smoke_test.zp"),"let total = 2 + 3\nassert(total == 5, \"basic arithmetic failed\")\nsay \"smoke test passed\"\n").unwrap_or_else(|e|{eprintln!("cannot write starter test: {e}");process::exit(1);});
        println!("Created Zap project: {}", dir.display());
        return;
    }
    if args.len() == 2 && args[1] == "check" {
        let dir = Path::new(".");
        match validate_project(dir) {
            Ok(info) => println!("valid Zap project: {info}"),
            Err(e) => {
                eprintln!("Zap check error: {e}");
                process::exit(1);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "check" {
        if args[2] == "--json" {
            print_project_json(Path::new("."));
            return;
        }
        let dir = Path::new(&args[2]);
        match validate_project(dir) {
            Ok(info) => println!("valid Zap project: {info}"),
            Err(e) => {
                eprintln!("Zap check error: {e}");
                process::exit(1);
            }
        }
        return;
    }
    if args.len() == 4 && args[1] == "check" && args[2] == "--json" {
        print_project_json(Path::new(&args[3]));
        return;
    }
    if args.len() == 2 && args[1] == "test" {
        if let Err(e) = run_zap_tests(Path::new("tests")) {
            eprintln!("Zap test error: {e}");
            process::exit(1);
        }
        return;
    }
    if args.len() == 2 && args[1] == "build" {
        match validate_project(Path::new(".")) {
            Ok(info) => println!("built Zap project: {info}"),
            Err(e) => {
                eprintln!("Zap build error: {e}");
                process::exit(1);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "build" {
        let dir = Path::new(&args[2]);
        match validate_project(dir) {
            Ok(info) => println!("built Zap project: {info}"),
            Err(e) => {
                eprintln!("Zap build error: {e}");
                process::exit(1);
            }
        }
        return;
    }
    if args.len() == 3 && args[1] == "test" {
        if let Err(e) = run_zap_tests(Path::new(&args[2])) {
            eprintln!("Zap test error: {e}");
            process::exit(1);
        }
        return;
    }
    if args.len() == 3 && args[1] == "run" {
        let source = fs::read_to_string(&args[2]).unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(1);
        });
        let base = Path::new(&args[2]).parent().unwrap_or(Path::new("."));
        if let Err(e) = run(&source, base) {
            eprintln!("Zap error: {}", ZapError::from_message(e));
            process::exit(1);
        }
        return;
    }
    if args.len() == 3 && args[1] == "lint" {
        let path = Path::new(&args[2]);
        let source = fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("cannot read {}: {e}", path.display());
            process::exit(1);
        });
        let issues = lint_source(&source);
        if issues.is_empty() {
            println!("lint ok: {}", path.display());
        } else {
            for issue in issues {
                println!("{}: {}", path.display(), issue);
            }
            process::exit(1);
        }
        return;
    }
    if args.len() == 3 && args[1] == "fmt" {
        let path = Path::new(&args[2]);
        let source = fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("cannot read {}: {e}", path.display());
            process::exit(1);
        });
        fs::write(path, format_source(&source)).unwrap_or_else(|e| {
            eprintln!("cannot write {}: {e}", path.display());
            process::exit(1);
        });
        return;
    }
    if args.len() != 2 {
        eprintln!("Usage: zap <file.zp>\n       zap run <file.zp>\n       zap fmt <file.zp>\n       zap lint <file.zp>\n       zap check [dir]\n       zap check --json [dir]\n       zap test [dir]\n       zap build [dir]\n       zap init <dir>\n       zap --version");
        process::exit(2);
    }
    let source = fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });
    let base = Path::new(&args[1]).parent().unwrap_or(Path::new("."));
    if let Err(e) = run(&source, base) {
        eprintln!("Zap error: {}", ZapError::from_message(e));
        process::exit(1);
    }
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
}
