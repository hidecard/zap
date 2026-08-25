use std::{collections::BTreeMap, path::Path};

use serde_json::{json, Value as JsonValue};

use crate::ast::{BinaryOp, CallArg, Expr, Literal, Program, Spanned, Stmt, UnaryOp};
use crate::lexer::{tokenize_with_spans, SpannedToken, Token};
use crate::{read_limited_text, ZapResult};

pub(crate) const TOKEN_SCHEMA_VERSION: u32 = 1;
pub(crate) const DIAGNOSTIC_SCHEMA_VERSION: u32 = 1;
pub(crate) const AST_SCHEMA_VERSION: u32 = 1;
pub(crate) const TYPED_IR_SCHEMA_VERSION: u32 = 1;

pub(crate) fn status_json() -> String {
    let mut schemas = BTreeMap::new();
    schemas.insert("ast", 1);
    schemas.insert("diagnostic", DIAGNOSTIC_SCHEMA_VERSION);
    schemas.insert("lockfile", 1);
    schemas.insert("manifest", 1);
    schemas.insert("token", TOKEN_SCHEMA_VERSION);
    schemas.insert("typed_ir", TYPED_IR_SCHEMA_VERSION);
    json!({
        "bootstrap_stage": "B0",
        "compiler_version": env!("CARGO_PKG_VERSION"),
        "language_version": "2.10.1",
        "reference_owner": "native Rust implementation",
        "schemas": schemas,
        "self_hosted": false,
        "stdlib_version": "2.10.1"
    })
    .to_string()
}

pub(crate) fn tokens_json(path: &Path) -> ZapResult<String> {
    let source = read_limited_text(path, "bootstrap source read")
        .map_err(crate::diagnostics::ZapError::from_message)?;
    let tokens =
        tokenize_with_spans(&source).map_err(crate::diagnostics::ZapError::from_message)?;
    let mut artifact = BTreeMap::new();
    artifact.insert("kind", json!("zap.token_stream"));
    artifact.insert("schema_version", json!(TOKEN_SCHEMA_VERSION));
    artifact.insert("source_name", json!(path.to_string_lossy()));
    artifact.insert(
        "tokens",
        JsonValue::Array(tokens.iter().map(token_json).collect()),
    );
    serde_json::to_string(&artifact).map_err(|error| {
        crate::diagnostics::ZapError::from_message(format!(
            "bootstrap token encoding failed: {error}"
        ))
    })
}

pub(crate) fn ast_json(path: &Path) -> ZapResult<String> {
    let source = read_limited_text(path, "bootstrap source read")
        .map_err(crate::diagnostics::ZapError::from_message)?;
    let program =
        crate::ast::parse_program(&source).map_err(crate::diagnostics::ZapError::from_message)?;
    let mut artifact = BTreeMap::new();
    artifact.insert("ast", program_json(&program));
    artifact.insert("kind", json!("zap.ast"));
    artifact.insert("schema_version", json!(AST_SCHEMA_VERSION));
    artifact.insert("source_name", json!(path.to_string_lossy()));
    serde_json::to_string(&artifact).map_err(|error| {
        crate::diagnostics::ZapError::from_message(format!(
            "bootstrap AST encoding failed: {error}"
        ))
    })
}

pub(crate) fn typed_ir_json(path: &Path) -> ZapResult<String> {
    let source = read_limited_text(path, "bootstrap source read")
        .map_err(crate::diagnostics::ZapError::from_message)?;
    let program =
        crate::ast::parse_program(&source).map_err(crate::diagnostics::ZapError::from_message)?;
    let mut artifact = BTreeMap::new();
    artifact.insert("ir", typed_ir_program(&program));
    artifact.insert("kind", json!("zap.typed_ir"));
    artifact.insert("reference_only", json!(true));
    artifact.insert("schema_version", json!(TYPED_IR_SCHEMA_VERSION));
    artifact.insert("source_name", json!(path.to_string_lossy()));
    serde_json::to_string(&artifact).map_err(|error| {
        crate::diagnostics::ZapError::from_message(format!(
            "bootstrap typed IR encoding failed: {error}"
        ))
    })
}

fn typed_ir_program(program: &Program) -> JsonValue {
    obj(vec![(
        "nodes",
        JsonValue::Array(program.statements.iter().map(typed_ir_statement).collect()),
    )])
}

fn typed_ir_statement(statement: &Spanned<Stmt>) -> JsonValue {
    let mut fields = match stmt_json(statement) {
        JsonValue::Object(fields) => fields,
        _ => unreachable!("statement serializer must return an object"),
    };
    if let Stmt::Declaration {
        annotation, value, ..
    } = &statement.node
    {
        fields.insert(
            "inferred_type".into(),
            json!(annotation
                .clone()
                .unwrap_or_else(|| inferred_expr_type(value).to_string())),
        );
    }
    fields.insert("ir_schema_version".into(), json!(TYPED_IR_SCHEMA_VERSION));
    JsonValue::Object(fields)
}

fn inferred_expr_type(expression: &Spanned<Expr>) -> &'static str {
    match &expression.node {
        Expr::Literal(Literal::Number(_)) => "number",
        Expr::Literal(Literal::Text(_)) => "text",
        Expr::Literal(Literal::Bool(_)) => "bool",
        Expr::Literal(Literal::None) => "none",
        Expr::List(_) => "list<any>",
        Expr::Map(_) => "map<any, any>",
        Expr::Name(_) => "unknown",
        Expr::Unary { op, value } => match op {
            UnaryOp::Not => "bool",
            UnaryOp::Negate if inferred_expr_type(value) == "number" => "number",
            UnaryOp::Negate => "unknown",
        },
        Expr::Binary { left, op, right } => match op {
            BinaryOp::Equal
            | BinaryOp::NotEqual
            | BinaryOp::Less
            | BinaryOp::Greater
            | BinaryOp::LessEqual
            | BinaryOp::GreaterEqual
            | BinaryOp::And
            | BinaryOp::Or => "bool",
            BinaryOp::Add
            | BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Divide
            | BinaryOp::Remainder
                if inferred_expr_type(left) == "number"
                    && inferred_expr_type(right) == "number" =>
            {
                "number"
            }
            BinaryOp::Add
                if inferred_expr_type(left) == "text" && inferred_expr_type(right) == "text" =>
            {
                "text"
            }
            BinaryOp::Add
            | BinaryOp::Subtract
            | BinaryOp::Multiply
            | BinaryOp::Divide
            | BinaryOp::Remainder => "unknown",
        },
        Expr::Conditional { .. } => "unknown",
        Expr::Call { .. } | Expr::Await(_) => "unknown",
        Expr::Propagate(_) => "unknown",
        Expr::Member { .. } | Expr::Index { .. } => "unknown",
    }
}

pub(crate) fn diagnostics_json(path: &Path) -> String {
    let result = read_limited_text(path, "bootstrap source read").and_then(|source| {
        tokenize_with_spans(&source)?;
        crate::ast::parse_program(&source)
    });
    let diagnostics = match result {
        Ok(_) => Vec::new(),
        Err(message) => vec![diagnostic_json(path, &message)],
    };
    let mut artifact = BTreeMap::new();
    artifact.insert("diagnostics", JsonValue::Array(diagnostics));
    artifact.insert("kind", json!("zap.diagnostics"));
    artifact.insert("schema_version", json!(DIAGNOSTIC_SCHEMA_VERSION));
    artifact.insert("source_name", json!(path.to_string_lossy()));
    serde_json::to_string(&artifact).unwrap_or_else(|error| {
        format!(
            "{{\"diagnostics\":[{{\"code\":\"ZAP-DIAG-ENCODE-001\",\"message\":{}}}],\"kind\":\"zap.diagnostics\",\"schema_version\":{DIAGNOSTIC_SCHEMA_VERSION}}}",
            serde_json::to_string(&error.to_string()).unwrap_or_else(|_| "\"encoding failed\"".into())
        )
    })
}

fn token_json(spanned: &SpannedToken) -> JsonValue {
    let (kind, value) = match &spanned.token {
        Token::Name(value) => ("name", json!(value)),
        Token::Number(value) => ("number", json!(value)),
        Token::Text(value) => ("text", json!(value)),
        Token::Plus => ("plus", JsonValue::Null),
        Token::Minus => ("minus", JsonValue::Null),
        Token::Star => ("star", JsonValue::Null),
        Token::Slash => ("slash", JsonValue::Null),
        Token::Percent => ("percent", JsonValue::Null),
        Token::Equal => ("equal", JsonValue::Null),
        Token::EqEq => ("equal_equal", JsonValue::Null),
        Token::NotEq => ("not_equal", JsonValue::Null),
        Token::Less => ("less", JsonValue::Null),
        Token::Greater => ("greater", JsonValue::Null),
        Token::LessEq => ("less_equal", JsonValue::Null),
        Token::GreaterEq => ("greater_equal", JsonValue::Null),
        Token::And => ("and", JsonValue::Null),
        Token::Or => ("or", JsonValue::Null),
        Token::LParen => ("left_paren", JsonValue::Null),
        Token::RParen => ("right_paren", JsonValue::Null),
        Token::LBracket => ("left_bracket", JsonValue::Null),
        Token::RBracket => ("right_bracket", JsonValue::Null),
        Token::LBrace => ("left_brace", JsonValue::Null),
        Token::RBrace => ("right_brace", JsonValue::Null),
        Token::Colon => ("colon", JsonValue::Null),
        Token::Comma => ("comma", JsonValue::Null),
        Token::Dot => ("dot", JsonValue::Null),
        Token::Question => ("question", JsonValue::Null),
        Token::End => ("end", JsonValue::Null),
    };
    let mut span = BTreeMap::new();
    span.insert("column", json!(spanned.span.column));
    span.insert("length", json!(spanned.span.length));
    span.insert("line", json!(spanned.span.line));
    let mut token = BTreeMap::new();
    token.insert("kind", json!(kind));
    token.insert("span", json!(span));
    token.insert("value", value);
    json!(token)
}

fn program_json(program: &Program) -> JsonValue {
    obj(vec![(
        "statements",
        JsonValue::Array(program.statements.iter().map(stmt_json).collect()),
    )])
}

fn stmt_json(statement: &Spanned<Stmt>) -> JsonValue {
    let mut fields = BTreeMap::new();
    fields.insert("span", span_json(&statement.span));
    match &statement.node {
        Stmt::Expression(value) => {
            fields.insert("kind", json!("expression"));
            fields.insert("value", expr_json(value));
        }
        Stmt::Assignment { name, value } => {
            fields.insert("kind", json!("assignment"));
            fields.insert("name", json!(name));
            fields.insert("value", expr_json(value));
        }
        Stmt::Declaration {
            name,
            annotation,
            value,
            exported,
        } => {
            fields.insert("annotation", option_json(annotation));
            fields.insert("exported", json!(exported));
            fields.insert("kind", json!("declaration"));
            fields.insert("name", json!(name));
            fields.insert("value", expr_json(value));
        }
        Stmt::Field {
            name,
            annotation,
            value,
            visibility,
        } => {
            fields.insert("annotation", option_json(annotation));
            fields.insert("kind", json!("field"));
            fields.insert("name", json!(name));
            fields.insert("value", expr_json(value));
            fields.insert("visibility", json!(visibility));
        }
        Stmt::Say(value) => {
            fields.insert("kind", json!("say"));
            fields.insert("value", expr_json(value));
        }
        Stmt::Raise(value) => {
            fields.insert("kind", json!("raise"));
            fields.insert("value", expr_json(value));
        }
        Stmt::TryCatch {
            body,
            binding,
            catch_body,
        } => {
            fields.insert("body", program_json(body));
            fields.insert("binding", json!(binding));
            fields.insert("catch_body", program_json(catch_body));
            fields.insert("kind", json!("try_catch"));
        }
        Stmt::Module { name } => {
            fields.insert("kind", json!("module"));
            fields.insert("name", json!(name));
        }
        Stmt::Import {
            path,
            explicit,
            alias,
        } => {
            fields.insert("alias", option_json(alias));
            fields.insert("explicit", json!(explicit));
            fields.insert("kind", json!("import"));
            fields.insert("path", json!(path));
        }
        Stmt::Return(value) => {
            fields.insert("kind", json!("return"));
            fields.insert(
                "value",
                value.as_ref().map(expr_json).unwrap_or(JsonValue::Null),
            );
        }
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            fields.insert("condition", expr_json(condition));
            fields.insert(
                "else_branch",
                else_branch
                    .as_ref()
                    .map(program_json)
                    .unwrap_or(JsonValue::Null),
            );
            fields.insert("kind", json!("if"));
            fields.insert("then_branch", program_json(then_branch));
        }
        Stmt::While { condition, body } => {
            fields.insert("body", program_json(body));
            fields.insert("condition", expr_json(condition));
            fields.insert("kind", json!("while"));
        }
        Stmt::For {
            binding,
            iterable,
            body,
        } => {
            fields.insert("binding", json!(binding));
            fields.insert("body", program_json(body));
            fields.insert("iterable", expr_json(iterable));
            fields.insert("kind", json!("for"));
        }
        Stmt::Function {
            name,
            type_params,
            params,
            return_type,
            body,
            visibility,
            is_async,
            exported,
        } => {
            fields.insert("body", program_json(body));
            fields.insert("exported", json!(exported));
            fields.insert("is_async", json!(is_async));
            fields.insert("kind", json!("function"));
            fields.insert("name", json!(name));
            fields.insert("type_params", json!(type_params));
            fields.insert(
                "params",
                JsonValue::Array(
                    params
                        .iter()
                        .map(|(name, annotation, default)| {
                            obj(vec![
                                ("annotation", option_json(annotation)),
                                ("default", option_json(default)),
                                ("name", json!(name)),
                            ])
                        })
                        .collect(),
                ),
            );
            fields.insert("return_type", option_json(return_type));
            fields.insert("visibility", json!(visibility));
        }
        Stmt::Class { name, base, body } => {
            fields.insert("base", option_json(base));
            fields.insert("body", program_json(body));
            fields.insert("kind", json!("class"));
            fields.insert("name", json!(name));
        }
        Stmt::Break => {
            fields.insert("kind", json!("break"));
        }
        Stmt::Continue => {
            fields.insert("kind", json!("continue"));
        }
    }
    json!(fields)
}

fn expr_json(expression: &Spanned<Expr>) -> JsonValue {
    let mut fields = BTreeMap::new();
    fields.insert("span", span_json(&expression.span));
    match &expression.node {
        Expr::Literal(value) => {
            fields.insert("kind", json!("literal"));
            match value {
                Literal::Number(value) => {
                    fields.insert("literal_kind", json!("number"));
                    fields.insert("value", json!(value));
                }
                Literal::Text(value) => {
                    fields.insert("literal_kind", json!("text"));
                    fields.insert("value", json!(value));
                }
                Literal::Bool(value) => {
                    fields.insert("literal_kind", json!("bool"));
                    fields.insert("value", json!(value));
                }
                Literal::None => {
                    fields.insert("literal_kind", json!("none"));
                    fields.insert("value", JsonValue::Null);
                }
            }
        }
        Expr::Name(name) => {
            fields.insert("kind", json!("name"));
            fields.insert("name", json!(name));
        }
        Expr::List(values) => {
            fields.insert(
                "elements",
                JsonValue::Array(values.iter().map(expr_json).collect()),
            );
            fields.insert("kind", json!("list"));
        }
        Expr::Map(entries) => {
            fields.insert(
                "entries",
                JsonValue::Array(
                    entries
                        .iter()
                        .map(|(key, value)| {
                            obj(vec![("key", expr_json(key)), ("value", expr_json(value))])
                        })
                        .collect(),
                ),
            );
            fields.insert("kind", json!("map"));
        }
        Expr::Unary { op, value } => {
            fields.insert("kind", json!("unary"));
            fields.insert("op", json!(unary_name(*op)));
            fields.insert("value", expr_json(value));
        }
        Expr::Binary { left, op, right } => {
            fields.insert("kind", json!("binary"));
            fields.insert("left", expr_json(left));
            fields.insert("op", json!(binary_name(*op)));
            fields.insert("right", expr_json(right));
        }
        Expr::Conditional {
            condition,
            then_branch,
            else_branch,
        } => {
            fields.insert("condition", expr_json(condition));
            fields.insert("else_branch", expr_json(else_branch));
            fields.insert("kind", json!("conditional"));
            fields.insert("then_branch", expr_json(then_branch));
        }
        Expr::Call { callee, args } => {
            fields.insert(
                "args",
                JsonValue::Array(args.iter().map(call_arg_json).collect()),
            );
            fields.insert("callee", expr_json(callee));
            fields.insert("kind", json!("call"));
        }
        Expr::Await(value) => {
            fields.insert("kind", json!("await"));
            fields.insert("value", expr_json(value));
        }
        Expr::Propagate(value) => {
            fields.insert("kind", json!("propagate"));
            fields.insert("value", expr_json(value));
        }
        Expr::Member { target, member } => {
            fields.insert("kind", json!("member"));
            fields.insert("member", json!(member));
            fields.insert("target", expr_json(target));
        }
        Expr::Index { target, index } => {
            fields.insert("index", expr_json(index));
            fields.insert("kind", json!("index"));
            fields.insert("target", expr_json(target));
        }
    }
    json!(fields)
}

fn call_arg_json(argument: &CallArg) -> JsonValue {
    match argument {
        CallArg::Positional(value) => obj(vec![
            ("kind", json!("positional")),
            ("value", expr_json(value)),
        ]),
        CallArg::Named { name, value } => obj(vec![
            ("kind", json!("named")),
            ("name", json!(name)),
            ("value", expr_json(value)),
        ]),
    }
}

fn span_json(span: &crate::lexer::SourceSpan) -> JsonValue {
    obj(vec![
        ("column", json!(span.column)),
        ("length", json!(span.length)),
        ("line", json!(span.line)),
    ])
}

fn option_json(value: &Option<String>) -> JsonValue {
    value.as_ref().map_or(JsonValue::Null, |value| json!(value))
}

fn obj(entries: Vec<(&str, JsonValue)>) -> JsonValue {
    let mut object = BTreeMap::new();
    for (key, value) in entries {
        object.insert(key, value);
    }
    json!(object)
}

fn unary_name(op: UnaryOp) -> &'static str {
    match op {
        UnaryOp::Negate => "negate",
        UnaryOp::Not => "not",
    }
}

fn binary_name(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Add => "add",
        BinaryOp::Subtract => "subtract",
        BinaryOp::Multiply => "multiply",
        BinaryOp::Divide => "divide",
        BinaryOp::Remainder => "remainder",
        BinaryOp::Equal => "equal",
        BinaryOp::NotEqual => "not_equal",
        BinaryOp::Less => "less",
        BinaryOp::Greater => "greater",
        BinaryOp::LessEqual => "less_equal",
        BinaryOp::GreaterEqual => "greater_equal",
        BinaryOp::And => "and",
        BinaryOp::Or => "or",
    }
}

fn diagnostic_json(path: &Path, message: &str) -> JsonValue {
    let (code, line, column) = classify_error(message);
    let help = if code.starts_with("ZAP-SYNTAX") {
        "check the surrounding syntax and delimiters"
    } else {
        "check the source token and its spelling"
    };
    json!({
        "code": code,
        "column": column,
        "help": help,
        "line": line,
        "message": message,
        "severity": "error",
        "source_name": path.to_string_lossy()
    })
}

fn classify_error(message: &str) -> (&'static str, Option<usize>, Option<usize>) {
    let code = if message.starts_with("invalid integer literal") {
        "ZAP-LEX-INT-001"
    } else if message.starts_with("unterminated string") {
        "ZAP-LEX-STR-001"
    } else if message.starts_with("unexpected character") {
        "ZAP-LEX-CHAR-001"
    } else if message.starts_with("expected ")
        || message.starts_with("unexpected token")
        || message.starts_with("invalid ")
        || message.contains("expects an expression")
        || message.contains("expects '")
    {
        "ZAP-SYNTAX-001"
    } else {
        "ZAP-LEX-001"
    };
    let (line, column) = message
        .split_once(" at ")
        .and_then(|(_, location)| location.split_once(':'))
        .map(|(line, column)| {
            (
                line.parse::<usize>().ok(),
                column
                    .split(':')
                    .next()
                    .and_then(|value| value.parse::<usize>().ok()),
            )
        })
        .unwrap_or((None, None));
    (code, line, column)
}

#[cfg(test)]
mod tests {
    use super::{classify_error, status_json, token_json, tokens_json, typed_ir_program};
    use crate::lexer::{tokenize_with_spans, Token};
    use std::path::Path;

    #[test]
    fn status_is_explicitly_b0_and_not_self_hosted() {
        let status = status_json();
        assert!(status.contains("\"bootstrap_stage\":\"B0\""));
        assert!(status.contains("\"self_hosted\":false"));
    }

    #[test]
    fn token_artifact_is_deterministic_and_has_end_token() {
        let tokens = tokenize_with_spans("say 1").expect("tokenize");
        let first = token_json(&tokens[0]).to_string();
        let second = token_json(&tokens[0]).to_string();
        assert_eq!(first, second);
        assert!(matches!(
            tokens.last().map(|item| &item.token),
            Some(Token::End)
        ));
        let _ = tokens_json(Path::new("bootstrap/fixtures/lexer/basic.zp"));
    }

    #[test]
    fn typed_ir_snapshot_has_stable_schema_and_inferred_type() {
        let program = crate::ast::parse_program("let answer = 1 + 2\n").expect("parse");
        let artifact = typed_ir_program(&program);
        assert_eq!(artifact["nodes"][0]["ir_schema_version"], 1);
        assert_eq!(artifact["nodes"][0]["inferred_type"], "number");
        assert_eq!(artifact["nodes"][0]["kind"], "declaration");
    }

    #[test]
    fn lexer_errors_get_stable_codes_and_locations() {
        assert_eq!(
            classify_error("unexpected character at 3:7: @"),
            ("ZAP-LEX-CHAR-001", Some(3), Some(7))
        );
        assert_eq!(
            classify_error("unterminated string at 2:4"),
            ("ZAP-LEX-STR-001", Some(2), Some(4))
        );
    }
}
