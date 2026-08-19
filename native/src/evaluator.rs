use std::{collections::HashMap, path::Path, rc::Rc};

use std::cell::Cell;

use crate::ast::{BinaryOp, Expr, Literal, Program, Stmt, UnaryOp};
use crate::lexer::{tokenize, Token};
use crate::ExprParser;
use crate::{
    parse_signature, read_limited_text, resolve_module, Function, Param, Value, MODULE_CACHE,
    MODULE_LOADING,
};

const MAX_EXECUTION_DEPTH: usize = 256;
const MAX_SOURCE_LINES: usize = 100_000;
const MAX_LOOP_ITERATIONS: usize = 100_000;

thread_local! {
    static EXECUTION_DEPTH: Cell<usize> = const { Cell::new(0) };
}

struct ExecutionGuard;

impl Drop for ExecutionGuard {
    fn drop(&mut self) {
        EXECUTION_DEPTH.with(|depth| depth.set(depth.get().saturating_sub(1)));
    }
}

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
                .map(ast_expr_source)
                .collect::<Vec<_>>()
                .join(", ")
        ),
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
        } => {
            let params = params
                .iter()
                .map(|(name, annotation)| {
                    format!(
                        "{name}{}",
                        annotation
                            .as_ref()
                            .map_or(String::new(), |ty| format!(": {ty}"))
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
                    && else_branch
                        .as_ref()
                        .map_or(true, |branch| ast_program_compatible(branch))
            }
            Stmt::While { body, .. } | Stmt::For { body, .. } => ast_program_compatible(body),
            _ => true,
        })
}

fn register_ast_function(
    name: &str,
    params: &[(String, Option<String>)],
    return_type: &Option<String>,
    body: &Program,
    vars: &HashMap<String, Value>,
    funcs: &mut HashMap<String, Rc<Function>>,
) {
    funcs.insert(
        name.to_string(),
        Rc::new(Function {
            params: params
                .iter()
                .map(|(name, annotation)| Param {
                    name: name.clone(),
                    annotation: annotation.clone(),
                })
                .collect(),
            return_annotation: return_type.clone(),
            body: Vec::new(),
            ast_body: Some(body.clone()),
            closure: vars.clone(),
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
            params: Vec::new(),
            return_annotation: None,
            body: Vec::new(),
            ast_body: None,
            closure: vars.clone(),
        }),
    );
    if let Some(parent) = base {
        if !funcs.contains_key(&format!("{parent}.__class__")) {
            return Err(format!("unknown parent class: {parent}"));
        }
        funcs.insert(
            format!("{name}.__parent__"),
            Rc::new(Function {
                params: Vec::new(),
                return_annotation: None,
                body: vec![parent.clone()],
                ast_body: None,
                closure: vars.clone(),
            }),
        );
    }
    for statement in &body.statements {
        if let Stmt::Function {
            name: method,
            params,
            return_type,
            body,
        } = &statement.node
        {
            if method == "init" {
                funcs.insert(
                    format!("{name}.__own_init__"),
                    Rc::new(Function {
                        params: Vec::new(),
                        return_annotation: None,
                        body: Vec::new(),
                        ast_body: None,
                        closure: vars.clone(),
                    }),
                );
            }
            let mut method_params = params
                .iter()
                .map(|(name, annotation)| Param {
                    name: name.clone(),
                    annotation: annotation.clone(),
                })
                .collect::<Vec<_>>();
            if method_params.first().map(|param| param.name.as_str()) != Some("self") {
                method_params.insert(
                    0,
                    Param {
                        name: "self".into(),
                        annotation: None,
                    },
                );
            }
            funcs.insert(
                format!("{name}.{method}"),
                Rc::new(Function {
                    params: method_params,
                    return_annotation: return_type.clone(),
                    body: Vec::new(),
                    ast_body: Some(body.clone()),
                    closure: vars.clone(),
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
                let _ = expression(&ast_expr_source(value), vars, funcs)?;
                Flow::Continue
            }
            Stmt::Assignment { name, value } => {
                let evaluated = expression(&ast_expr_source(value), vars, funcs)?;
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
                let evaluated = expression(&ast_expr_source(value), vars, funcs)?;
                if let Some(annotation) = annotation {
                    check_annotation(name, annotation, &evaluated)?;
                }
                vars.insert(name.clone(), evaluated);
                Flow::Continue
            }
            Stmt::Say(value) => {
                println!(
                    "{}",
                    expression(&ast_expr_source(value), vars, funcs)?.show()
                );
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
                if expression(&ast_expr_source(condition), vars, funcs)?.truthy() {
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
                    if !expression(&ast_expr_source(condition), vars, funcs)?.truthy() {
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
                let value = expression(&ast_expr_source(iterable), vars, funcs)?;
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
            } => {
                register_ast_function(name, params, return_type, body, vars, funcs);
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
                    params: Vec::new(),
                    return_annotation: None,
                    body: Vec::new(),
                    ast_body: None,
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
                        ast_body: None,
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
                                ast_body: None,
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
                            ast_body: None,
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
                    ast_body: None,
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
                        ast_body: None,
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
            "fn add(a: number, b: number) -> number:\n    return a + b\nlet result: number = add(3, 3)\n",
        )
        .expect("valid declaration AST program");
        let mut vars = HashMap::<String, Value>::new();
        let mut funcs = HashMap::<String, Rc<Function>>::new();
        execute_ast_program(&program, &mut vars, &mut funcs, Path::new("."))
            .expect("native AST declarations should execute");
        assert!(funcs
            .get("add")
            .is_some_and(|function| function.ast_body.is_some()));
        assert_eq!(vars.get("result"), Some(&Value::Number(6)));
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
