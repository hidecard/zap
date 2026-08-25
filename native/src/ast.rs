#![allow(dead_code)]

use std::collections::HashSet;

use crate::lexer::SourceSpan;
use crate::parser::{
    is_allowed_annotation_with_type_params, parse_function_name_and_type_parameters,
};

/// A source-aware expression node used by future parser and tooling phases.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Spanned<T> {
    pub(crate) node: T,
    pub(crate) span: SourceSpan,
}

impl<T> Spanned<T> {
    pub(crate) fn new(node: T, span: SourceSpan) -> Self {
        Self { node, span }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Literal {
    Number(i64),
    Text(String),
    Bool(bool),
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UnaryOp {
    Negate,
    Not,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CallArg {
    Positional(Spanned<Expr>),
    Named { name: String, value: Spanned<Expr> },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Expr {
    Literal(Literal),
    Name(String),
    List(Vec<Spanned<Expr>>),
    Map(Vec<(Spanned<Expr>, Spanned<Expr>)>),
    Unary {
        op: UnaryOp,
        value: Box<Spanned<Expr>>,
    },
    Binary {
        left: Box<Spanned<Expr>>,
        op: BinaryOp,
        right: Box<Spanned<Expr>>,
    },
    Conditional {
        condition: Box<Spanned<Expr>>,
        then_branch: Box<Spanned<Expr>>,
        else_branch: Box<Spanned<Expr>>,
    },
    Call {
        callee: Box<Spanned<Expr>>,
        args: Vec<CallArg>,
    },
    Await(Box<Spanned<Expr>>),
    Propagate(Box<Spanned<Expr>>),
    Member {
        target: Box<Spanned<Expr>>,
        member: String,
    },
    Index {
        target: Box<Spanned<Expr>>,
        index: Box<Spanned<Expr>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Stmt {
    Expression(Spanned<Expr>),
    Assignment {
        name: String,
        value: Spanned<Expr>,
    },
    Declaration {
        name: String,
        annotation: Option<String>,
        value: Spanned<Expr>,
        exported: bool,
    },
    Field {
        name: String,
        annotation: Option<String>,
        value: Spanned<Expr>,
        visibility: String,
    },
    Say(Spanned<Expr>),
    Raise(Spanned<Expr>),
    TryCatch {
        body: Program,
        binding: String,
        catch_body: Program,
    },
    Module {
        name: String,
    },
    Import {
        path: String,
        explicit: bool,
        alias: Option<String>,
    },
    Return(Option<Spanned<Expr>>),
    If {
        condition: Spanned<Expr>,
        then_branch: Program,
        else_branch: Option<Program>,
    },
    While {
        condition: Spanned<Expr>,
        body: Program,
    },
    For {
        binding: String,
        iterable: Spanned<Expr>,
        body: Program,
    },
    Function {
        name: String,
        type_params: Vec<String>,
        params: Vec<(String, Option<String>, Option<String>)>,
        return_type: Option<String>,
        body: Program,
        visibility: String,
        is_async: bool,
        exported: bool,
    },
    Class {
        name: String,
        base: Option<String>,
        body: Program,
    },
    Break,
    Continue,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Program {
    pub(crate) statements: Vec<Spanned<Stmt>>,
}

struct AstParser {
    tokens: Vec<crate::lexer::SpannedToken>,
    cursor: usize,
}

impl AstParser {
    fn new(tokens: Vec<crate::lexer::SpannedToken>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn current(&self) -> &crate::lexer::SpannedToken {
        &self.tokens[self.cursor]
    }

    fn advance(&mut self) -> crate::lexer::SpannedToken {
        let token = self.tokens[self.cursor].clone();
        self.cursor += 1;
        token
    }

    fn parse_complete(&mut self) -> Result<Spanned<Expr>, String> {
        let expression = self.parse_expression(0)?;
        if !matches!(self.current().token, crate::lexer::Token::End) {
            return Err(format!(
                "unexpected token after expression at {}:{}",
                self.current().span.line,
                self.current().span.column
            ));
        }
        Ok(expression)
    }

    fn parse_expression(&mut self, min_precedence: u8) -> Result<Spanned<Expr>, String> {
        let mut left = self.parse_prefix()?;
        loop {
            let (operator, precedence) = match self.current().token {
                crate::lexer::Token::Or => (BinaryOp::Or, 1),
                crate::lexer::Token::And => (BinaryOp::And, 2),
                crate::lexer::Token::EqEq => (BinaryOp::Equal, 3),
                crate::lexer::Token::NotEq => (BinaryOp::NotEqual, 3),
                crate::lexer::Token::Less => (BinaryOp::Less, 4),
                crate::lexer::Token::Greater => (BinaryOp::Greater, 4),
                crate::lexer::Token::LessEq => (BinaryOp::LessEqual, 4),
                crate::lexer::Token::GreaterEq => (BinaryOp::GreaterEqual, 4),
                crate::lexer::Token::Plus => (BinaryOp::Add, 5),
                crate::lexer::Token::Minus => (BinaryOp::Subtract, 5),
                crate::lexer::Token::Star => (BinaryOp::Multiply, 6),
                crate::lexer::Token::Slash => (BinaryOp::Divide, 6),
                crate::lexer::Token::Percent => (BinaryOp::Remainder, 6),
                _ => break,
            };
            if precedence < min_precedence {
                break;
            }
            self.advance();
            let right = self.parse_expression(precedence + 1)?;
            let span = SourceSpan {
                line: left.span.line,
                column: left.span.column,
                length: right.span.column + right.span.length - left.span.column,
            };
            left = Spanned::new(
                Expr::Binary {
                    left: Box::new(left),
                    op: operator,
                    right: Box::new(right),
                },
                span,
            );
        }
        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Spanned<Expr>, String> {
        let token = self.advance();
        match token.token {
            crate::lexer::Token::Minus => {
                let value = self.parse_expression(7)?;
                let span = SourceSpan {
                    line: token.span.line,
                    column: token.span.column,
                    length: value.span.column + value.span.length - token.span.column,
                };
                Ok(Spanned::new(
                    Expr::Unary {
                        op: UnaryOp::Negate,
                        value: Box::new(value),
                    },
                    span,
                ))
            }
            crate::lexer::Token::Name(name) if name == "await" || name == "not" => {
                let value = self.parse_expression(7)?;
                let span = SourceSpan {
                    line: token.span.line,
                    column: token.span.column,
                    length: value.span.column + value.span.length - token.span.column,
                };
                if name == "await" {
                    Ok(Spanned::new(Expr::Await(Box::new(value)), span))
                } else {
                    Ok(Spanned::new(
                        Expr::Unary {
                            op: UnaryOp::Not,
                            value: Box::new(value),
                        },
                        span,
                    ))
                }
            }
            crate::lexer::Token::Name(name) if name == "if" => {
                let condition = self.parse_expression(0)?;
                let then_token = self.advance();
                if !matches!(then_token.token, crate::lexer::Token::Name(ref keyword) if keyword == "then")
                {
                    return Err(format!(
                        "expected 'then' in conditional expression at {}:{}",
                        then_token.span.line, then_token.span.column
                    ));
                }
                let then_branch = self.parse_expression(0)?;
                let else_token = self.advance();
                if !matches!(else_token.token, crate::lexer::Token::Name(ref keyword) if keyword == "else")
                {
                    return Err(format!(
                        "expected 'else' in conditional expression at {}:{}",
                        else_token.span.line, else_token.span.column
                    ));
                }
                let else_branch = self.parse_expression(0)?;
                let span = SourceSpan {
                    line: token.span.line,
                    column: token.span.column,
                    length: else_branch.span.column + else_branch.span.length - token.span.column,
                };
                Ok(Spanned::new(
                    Expr::Conditional {
                        condition: Box::new(condition),
                        then_branch: Box::new(then_branch),
                        else_branch: Box::new(else_branch),
                    },
                    span,
                ))
            }
            crate::lexer::Token::Name(name) => {
                let literal = match name.as_str() {
                    "true" => Some(Literal::Bool(true)),
                    "false" => Some(Literal::Bool(false)),
                    "none" => Some(Literal::None),
                    _ => None,
                };
                let expression = literal.map(Expr::Literal).unwrap_or(Expr::Name(name));
                self.parse_postfix(Spanned::new(expression, token.span))
            }
            crate::lexer::Token::Number(value) => self.parse_postfix(Spanned::new(
                Expr::Literal(Literal::Number(value)),
                token.span,
            )),
            crate::lexer::Token::Text(value) => self.parse_postfix(Spanned::new(
                Expr::Literal(Literal::Text(value)),
                token.span,
            )),
            crate::lexer::Token::LParen => {
                let expression = self.parse_expression(0)?;
                let close = self.advance();
                if !matches!(close.token, crate::lexer::Token::RParen) {
                    return Err(format!(
                        "expected ')' at {}:{}",
                        close.span.line, close.span.column
                    ));
                }
                self.parse_postfix(expression)
            }
            crate::lexer::Token::LBracket => {
                let mut items = Vec::new();
                while !matches!(self.current().token, crate::lexer::Token::RBracket) {
                    items.push(self.parse_expression(0)?);
                    if !matches!(self.current().token, crate::lexer::Token::Comma) {
                        break;
                    }
                    self.advance();
                }
                let close = self.advance();
                if !matches!(close.token, crate::lexer::Token::RBracket) {
                    return Err(format!(
                        "expected ']' at {}:{}",
                        close.span.line, close.span.column
                    ));
                }
                self.parse_postfix(Spanned::new(
                    Expr::List(items),
                    SourceSpan {
                        line: token.span.line,
                        column: token.span.column,
                        length: close.span.column + close.span.length - token.span.column,
                    },
                ))
            }
            crate::lexer::Token::LBrace => {
                let mut items = Vec::new();
                while !matches!(self.current().token, crate::lexer::Token::RBrace) {
                    let key_token = self.advance();
                    let key = match key_token.token {
                        crate::lexer::Token::Text(value) => {
                            Spanned::new(Expr::Literal(Literal::Text(value)), key_token.span)
                        }
                        crate::lexer::Token::Name(value) => {
                            Spanned::new(Expr::Literal(Literal::Text(value)), key_token.span)
                        }
                        other => return Err(format!("invalid map key: {other:?}")),
                    };
                    if !matches!(self.current().token, crate::lexer::Token::Colon) {
                        return Err("expected ':' after map key".into());
                    }
                    self.advance();
                    let value = self.parse_expression(0)?;
                    items.push((key, value));
                    if !matches!(self.current().token, crate::lexer::Token::Comma) {
                        break;
                    }
                    self.advance();
                }
                let close = self.advance();
                if !matches!(close.token, crate::lexer::Token::RBrace) {
                    return Err(format!(
                        "expected '}}' at {}:{}",
                        close.span.line, close.span.column
                    ));
                }
                self.parse_postfix(Spanned::new(
                    Expr::Map(items),
                    SourceSpan {
                        line: token.span.line,
                        column: token.span.column,
                        length: close.span.column + close.span.length - token.span.column,
                    },
                ))
            }
            other => Err(format!(
                "expected expression, got {other:?} at {}:{}",
                token.span.line, token.span.column
            )),
        }
    }

    fn parse_postfix(&mut self, mut expression: Spanned<Expr>) -> Result<Spanned<Expr>, String> {
        loop {
            match self.current().token {
                crate::lexer::Token::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    if !matches!(self.current().token, crate::lexer::Token::RParen) {
                        loop {
                            let argument = if let crate::lexer::Token::Name(name) =
                                self.current().token.clone()
                            {
                                if matches!(
                                    self.tokens.get(self.cursor + 1).map(|token| &token.token),
                                    Some(crate::lexer::Token::Equal)
                                ) {
                                    self.advance();
                                    self.advance();
                                    CallArg::Named {
                                        name,
                                        value: self.parse_expression(0)?,
                                    }
                                } else {
                                    CallArg::Positional(self.parse_expression(0)?)
                                }
                            } else {
                                CallArg::Positional(self.parse_expression(0)?)
                            };
                            args.push(argument);
                            if !matches!(self.current().token, crate::lexer::Token::Comma) {
                                break;
                            }
                            self.advance();
                        }
                    }
                    let close = self.advance();
                    if !matches!(close.token, crate::lexer::Token::RParen) {
                        return Err(format!(
                            "expected ')' at {}:{}",
                            close.span.line, close.span.column
                        ));
                    }
                    let end = close.span.column + close.span.length;
                    let callee_span = expression.span.clone();
                    let callee = Spanned::new(
                        std::mem::replace(&mut expression.node, Expr::Literal(Literal::None)),
                        callee_span,
                    );
                    expression.span.length = end.saturating_sub(expression.span.column);
                    expression.node = Expr::Call {
                        callee: Box::new(callee),
                        args,
                    };
                }
                crate::lexer::Token::Dot => {
                    self.advance();
                    let member = match self.advance().token {
                        crate::lexer::Token::Name(name) => name,
                        other => {
                            return Err(format!("expected member name after '.', got {other:?}"))
                        }
                    };
                    let target_span = expression.span.clone();
                    let target = Spanned::new(
                        std::mem::replace(&mut expression.node, Expr::Literal(Literal::None)),
                        target_span,
                    );
                    expression.node = Expr::Member {
                        target: Box::new(target),
                        member,
                    };
                }
                crate::lexer::Token::Question => {
                    self.advance();
                    let value_span = expression.span.clone();
                    let value = Spanned::new(
                        std::mem::replace(&mut expression.node, Expr::Literal(Literal::None)),
                        value_span.clone(),
                    );
                    expression.span.length = expression.span.length.saturating_add(1);
                    expression.node = Expr::Propagate(Box::new(value));
                }
                crate::lexer::Token::LBracket => {
                    self.advance();
                    let index = self.parse_expression(0)?;
                    let close = self.advance();
                    if !matches!(close.token, crate::lexer::Token::RBracket) {
                        return Err(format!(
                            "expected ']' at {}:{}",
                            close.span.line, close.span.column
                        ));
                    }
                    let end = close.span.column + close.span.length;
                    let target_span = expression.span.clone();
                    let target = Spanned::new(
                        std::mem::replace(&mut expression.node, Expr::Literal(Literal::None)),
                        target_span,
                    );
                    expression.span.length = end.saturating_sub(expression.span.column);
                    expression.node = Expr::Index {
                        target: Box::new(target),
                        index: Box::new(index),
                    };
                }
                _ => break,
            }
        }
        Ok(expression)
    }
}

pub(crate) fn parse_expression(source: &str) -> Result<Spanned<Expr>, String> {
    let tokens = crate::lexer::tokenize_with_spans(source)?;
    AstParser::new(tokens).parse_complete()
}

pub(crate) fn parse_statement(source: &str) -> Result<Spanned<Stmt>, String> {
    let original = source.trim();
    if original.is_empty() || original.starts_with('#') {
        return Err("empty or comment-only statement".to_string());
    }
    let exported = original.starts_with("export ");
    let trimmed = if exported {
        original.strip_prefix("export ").unwrap_or(original).trim()
    } else {
        original
    };
    let leading = source.len() - source.trim_start().len();
    let span = |length: usize| SourceSpan {
        line: 1,
        column: leading + 1,
        length: length.max(1),
    };

    if trimmed == "break" {
        return Ok(Spanned::new(Stmt::Break, span(trimmed.len())));
    }
    if trimmed == "continue" {
        return Ok(Spanned::new(Stmt::Continue, span(trimmed.len())));
    }
    if let Some(rest) = trimmed.strip_prefix("say ") {
        return Ok(Spanned::new(
            Stmt::Say(parse_expression(rest.trim())?),
            span(trimmed.len()),
        ));
    }
    if trimmed == "raise" {
        return Err("raise expects an expression".to_string());
    }
    if let Some(rest) = trimmed.strip_prefix("raise ") {
        let value = rest.trim();
        if value.is_empty() {
            return Err("raise expects an expression".to_string());
        }
        return Ok(Spanned::new(
            Stmt::Raise(parse_expression(value)?),
            span(trimmed.len()),
        ));
    }
    if let Some(rest) = trimmed.strip_prefix("module ") {
        let name = rest.trim().trim_end_matches(';').trim();
        let valid = !name.is_empty()
            && name.split('.').all(|part| {
                !part.is_empty()
                    && part.chars().enumerate().all(|(index, character)| {
                        character == '_'
                            || character.is_ascii_alphanumeric()
                                && (index > 0 || character.is_ascii_alphabetic())
                    })
            });
        if !valid {
            return Err("module expects a dotted identifier path".to_string());
        }
        return Ok(Spanned::new(
            Stmt::Module {
                name: name.to_string(),
            },
            span(trimmed.len()),
        ));
    }
    for (keyword, explicit) in [("import ", true), ("use ", false)] {
        if let Some(rest) = trimmed.strip_prefix(keyword) {
            let clause = rest.trim().trim_end_matches(';').trim();
            let (path_text, alias) = clause
                .split_once(" as ")
                .map_or((clause, None), |(path, alias)| {
                    (path.trim(), Some(alias.trim()))
                });
            let path = path_text.trim_matches('"').trim();
            if path.is_empty() {
                return Err(format!("{}expects a module path", keyword.trim()));
            }
            if let Some(alias) = alias {
                let valid_alias = !alias.is_empty()
                    && alias.chars().enumerate().all(|(index, character)| {
                        character == '_'
                            || character.is_ascii_alphanumeric()
                                && (index > 0 || character.is_ascii_alphabetic())
                    });
                if !valid_alias {
                    return Err("import alias must be an identifier".to_string());
                }
            }
            return Ok(Spanned::new(
                Stmt::Import {
                    path: path.to_string(),
                    explicit,
                    alias: alias.map(str::to_string),
                },
                span(trimmed.len()),
            ));
        }
    }
    if let Some(rest) = trimmed.strip_prefix("return") {
        if rest.is_empty() || rest.chars().next().is_some_and(char::is_whitespace) {
            let value = rest.trim();
            let statement = if value.is_empty() {
                Stmt::Return(None)
            } else {
                Stmt::Return(Some(parse_expression(value)?))
            };
            return Ok(Spanned::new(statement, span(trimmed.len())));
        }
    }

    for (visibility, prefix) in [
        ("public", "public let "),
        ("private", "private let "),
        ("protected", "protected let "),
    ] {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            let (target, value) = rest
                .split_once('=')
                .ok_or_else(|| "field declaration expects '='".to_string())?;
            let (name, annotation) = target
                .trim()
                .split_once(':')
                .map_or((target.trim(), None), |(name, ty)| {
                    (name.trim(), Some(ty.trim().to_string()))
                });
            if let Some(annotation) = annotation.as_deref() {
                validate_annotation_syntax(annotation)?;
            }
            if name.is_empty() || value.trim().is_empty() {
                return Err("field declaration requires a name and value".to_string());
            }
            return Ok(Spanned::new(
                Stmt::Field {
                    name: name.to_string(),
                    annotation,
                    value: parse_expression(value.trim())?,
                    visibility: visibility.to_string(),
                },
                span(trimmed.len()),
            ));
        }
    }

    if let Some(rest) = trimmed.strip_prefix("let ") {
        let (target, value) = rest
            .split_once('=')
            .ok_or_else(|| "declaration expects '='".to_string())?;
        let (name, annotation) = target
            .trim()
            .split_once(':')
            .map_or((target.trim(), None), |(name, ty)| {
                (name.trim(), Some(ty.trim().to_string()))
            });
        if let Some(annotation) = annotation.as_deref() {
            validate_annotation_syntax(annotation)?;
        }
        if name.is_empty() || value.trim().is_empty() {
            return Err("declaration requires a name and value".to_string());
        }
        return Ok(Spanned::new(
            Stmt::Declaration {
                name: name.to_string(),
                annotation,
                value: parse_expression(value.trim())?,
                exported,
            },
            span(trimmed.len()),
        ));
    }

    if let Some(equal) = trimmed.find('=') {
        let is_comparison = trimmed.as_bytes().get(equal + 1) == Some(&b'=')
            || (equal > 0 && trimmed.as_bytes()[equal - 1] == b'=');
        if !is_comparison {
            let name = trimmed[..equal].trim();
            let valid_target = !name.is_empty()
                && name.split('.').all(|part| {
                    !part.is_empty()
                        && part.chars().enumerate().all(|(index, character)| {
                            character == '_'
                                || character.is_ascii_alphanumeric()
                                    && (index > 0 || character.is_ascii_alphabetic())
                        })
                });
            if !valid_target {
                return Err(format!("invalid assignment target at 1:{}", leading + 1));
            }
            let value = trimmed[equal + 1..].trim();
            if value.is_empty() {
                return Err(format!(
                    "missing assignment value at 1:{}",
                    leading + equal + 2
                ));
            }
            return Ok(Spanned::new(
                Stmt::Assignment {
                    name: name.to_string(),
                    value: parse_expression(value)?,
                },
                span(trimmed.len()),
            ));
        }
    }

    Ok(Spanned::new(
        Stmt::Expression(parse_expression(trimmed)?),
        span(trimmed.len()),
    ))
}

#[derive(Clone, Debug)]
struct SourceLine {
    number: usize,
    indent: usize,
    text: String,
}

fn parse_function_header(
    text: &str,
) -> Option<
    Result<
        (
            String,
            Vec<String>,
            Vec<(String, Option<String>, Option<String>)>,
            Option<String>,
            String,
            bool,
            bool,
        ),
        String,
    >,
> {
    let header = text.strip_suffix(':')?;
    let (exported, header) = if let Some(rest) = header.strip_prefix("export ") {
        (true, rest)
    } else {
        (false, header)
    };
    let (visibility, signature, is_async) =
        if let Some(rest) = header.strip_prefix("public async fn ") {
            ("public", rest, true)
        } else if let Some(rest) = header.strip_prefix("private async fn ") {
            ("private", rest, true)
        } else if let Some(rest) = header.strip_prefix("protected async fn ") {
            ("protected", rest, true)
        } else if let Some(rest) = header.strip_prefix("async fn ") {
            ("public", rest, true)
        } else if let Some(rest) = header.strip_prefix("public fn ") {
            ("public", rest, false)
        } else if let Some(rest) = header.strip_prefix("private fn ") {
            ("private", rest, false)
        } else if let Some(rest) = header.strip_prefix("protected fn ") {
            ("protected", rest, false)
        } else if let Some(rest) = header.strip_prefix("fn ") {
            ("public", rest, false)
        } else {
            let rest = header.strip_prefix("def ")?;
            ("public", rest, false)
        };
    let open = signature.find('(')?;
    let close = signature.rfind(')')?;
    if close < open {
        return Some(Err("function parameter list is malformed".to_string()));
    }
    let raw_name = signature[..open].trim();
    let (name, type_params) = match parse_function_name_and_type_parameters(raw_name) {
        Ok(value) => value,
        Err(error) => return Some(Err(error)),
    };
    if name.is_empty() {
        return Some(Err("function name is missing".to_string()));
    }
    let valid_name = name.chars().enumerate().all(|(index, character)| {
        character == '_'
            || character.is_ascii_alphanumeric() && (index > 0 || character.is_ascii_alphabetic())
    });
    if !valid_name {
        return Some(Err(format!("invalid function name '{name}'")));
    }
    let params_text = &signature[open + 1..close];
    let mut params = Vec::new();
    let mut parameter_names = HashSet::new();
    if !params_text.trim().is_empty() {
        for parameter in params_text.split(',') {
            let parameter = parameter.trim();
            let (parameter, default) = parameter
                .split_once('=')
                .map_or((parameter, None), |(left, right)| {
                    (left.trim(), Some(right.trim()))
                });
            if default.is_some_and(str::is_empty) {
                return Some(Err(
                    "parameter default expression cannot be empty".to_string()
                ));
            }
            let (parameter_name, annotation) = parameter
                .split_once(':')
                .map_or((parameter, None), |(name, annotation)| {
                    (name.trim(), Some(annotation.trim()))
                });
            if parameter_name.is_empty() || parameter_name == "self" && annotation.is_some() {
                return Some(Err("invalid function parameter".to_string()));
            }
            let valid_parameter = parameter_name
                .chars()
                .enumerate()
                .all(|(index, character)| {
                    character == '_'
                        || character.is_ascii_alphanumeric()
                            && (index > 0 || character.is_ascii_alphabetic())
                });
            if !valid_parameter {
                return Some(Err(format!(
                    "invalid function parameter '{parameter_name}'"
                )));
            }
            if !parameter_names.insert(parameter_name) {
                return Some(Err(format!("duplicate parameter name: {parameter_name}")));
            }
            let annotation = annotation
                .filter(|value| !value.is_empty())
                .map(str::to_string);
            if let Some(annotation) = annotation.as_deref() {
                if !is_allowed_annotation_with_type_params(annotation, &type_params) {
                    if let Err(error) = validate_annotation_syntax(annotation) {
                        return Some(Err(format!(
                            "invalid annotation for parameter {parameter_name}: {error}"
                        )));
                    }
                }
            }
            params.push((
                parameter_name.to_string(),
                annotation,
                default.map(str::to_string),
            ));
        }
    }
    let suffix = signature[close + 1..].trim();
    let return_type = suffix
        .strip_prefix("->")
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    if let Some(annotation) = return_type.as_deref() {
        if !is_allowed_annotation_with_type_params(annotation, &type_params) {
            if let Err(error) = validate_annotation_syntax(annotation) {
                return Some(Err(format!("invalid return annotation: {error}")));
            }
        }
    }
    if !suffix.is_empty() && return_type.is_none() {
        return Some(Err("invalid function return annotation".to_string()));
    }
    Some(Ok((
        name.to_string(),
        type_params,
        params,
        return_type,
        visibility.to_string(),
        is_async,
        exported,
    )))
}

fn validate_annotation_syntax(annotation: &str) -> Result<(), String> {
    let value = annotation.trim();
    if value.is_empty() {
        return Err("annotation cannot be empty".to_string());
    }
    let (base, args) = match value.split_once('<') {
        Some((base, rest)) => {
            if !rest.ends_with('>') {
                return Err(format!("malformed type annotation '{value}'"));
            }
            (
                base.trim().to_ascii_lowercase(),
                Some(&rest[..rest.len() - 1]),
            )
        }
        None => (value.to_ascii_lowercase(), None),
    };
    let known = [
        "any", "text", "number", "bool", "list", "map", "object", "result", "option", "function",
        "none",
    ];
    if !known.contains(&base.as_str()) {
        return Err(format!("unknown type annotation '{value}'"));
    }
    match (base.as_str(), args) {
        ("list" | "result" | "option", Some(args)) if args.split(',').count() == 1 => {
            validate_annotation_syntax(args.trim())
        }
        ("map", Some(args)) if args.split(',').count() == 2 => {
            for arg in args.split(',') {
                validate_annotation_syntax(arg.trim())?;
            }
            Ok(())
        }
        (_, None) => Ok(()),
        _ => Err(format!("invalid generic type annotation '{value}'")),
    }
}

fn parse_class_header(text: &str) -> Option<Result<(String, Option<String>), String>> {
    let header = text.strip_suffix(':')?;
    let declaration = header.strip_prefix("class ")?;
    let declaration = declaration.strip_suffix(':').unwrap_or(declaration).trim();
    let (name, base) = if let Some(rest) = declaration.strip_prefix(' ') {
        (rest.trim(), None)
    } else if let Some((name, parent)) = declaration.split_once(" extends ") {
        (name.trim(), Some(parent.trim().to_string()))
    } else if let Some(open) = declaration.find('(') {
        let close = declaration.rfind(')')?;
        if close <= open || !declaration[close + 1..].trim().is_empty() {
            return Some(Err("class inheritance header is malformed".to_string()));
        }
        (
            declaration[..open].trim(),
            Some(declaration[open + 1..close].trim().to_string()),
        )
    } else {
        (declaration.trim(), None)
    };
    if name.is_empty() {
        return Some(Err("class name is missing".to_string()));
    }
    Some(Ok((
        name.to_string(),
        base.filter(|value| !value.is_empty()),
    )))
}

fn parse_control_header(text: &str) -> Option<(&str, &str)> {
    let header = text.strip_suffix(':')?;
    if let Some(condition) = header.strip_prefix("if ") {
        return Some(("if", condition.trim()));
    }
    if let Some(condition) = header.strip_prefix("while ") {
        return Some(("while", condition.trim()));
    }
    if let Some(rest) = header.strip_prefix("for ") {
        return Some(("for", rest.trim()));
    }
    None
}

fn source_lines(source: &str) -> Result<Vec<SourceLine>, String> {
    let mut lines = Vec::new();
    for (index, raw) in source.lines().enumerate() {
        let text = raw.trim();
        if text.is_empty() || text.starts_with('#') {
            continue;
        }
        let leading = raw.len() - raw.trim_start().len();
        if raw[..leading].contains('\t') {
            return Err(format!(
                "tabs are not supported in AST blocks at line {}",
                index + 1
            ));
        }
        if leading % 4 != 0 {
            return Err(format!("invalid indentation at line {}", index + 1));
        }
        lines.push(SourceLine {
            number: index + 1,
            indent: leading / 4,
            text: text.to_string(),
        });
    }
    Ok(lines)
}

fn parse_block(lines: &[SourceLine], cursor: &mut usize, indent: usize) -> Result<Program, String> {
    let mut program = Program::default();
    while *cursor < lines.len() {
        let line = &lines[*cursor];
        if line.indent < indent {
            break;
        }
        if line.indent > indent {
            return Err(format!("unexpected indentation at line {}", line.number));
        }
        let line_number = line.number;
        let text = line.text.clone();
        if text == "else:" || text == "catch:" || text.starts_with("catch ") {
            break;
        }
        if text == "try:" {
            *cursor += 1;
            if *cursor >= lines.len() || lines[*cursor].indent <= indent {
                return Err(format!(
                    "try requires an indented block at line {line_number}"
                ));
            }
            let body_indent = lines[*cursor].indent;
            let body = parse_block(lines, cursor, body_indent)?;
            if *cursor >= lines.len()
                || lines[*cursor].indent != indent
                || !lines[*cursor].text.starts_with("catch")
            {
                return Err(format!(
                    "try requires a same-level catch clause at line {line_number}"
                ));
            }
            let catch_line = &lines[*cursor];
            let binding = catch_line
                .text
                .strip_prefix("catch")
                .and_then(|rest| rest.strip_suffix(':'))
                .map(str::trim)
                .filter(|name| {
                    !name.is_empty()
                        && name.chars().enumerate().all(|(index, character)| {
                            character == '_'
                                || character.is_ascii_alphanumeric()
                                    && (index > 0 || character.is_ascii_alphabetic())
                        })
                })
                .ok_or_else(|| {
                    format!(
                        "catch expects an error binding at line {}",
                        catch_line.number
                    )
                })?
                .to_string();
            *cursor += 1;
            if *cursor >= lines.len() || lines[*cursor].indent <= indent {
                return Err(format!(
                    "catch requires an indented block at line {}",
                    catch_line.number
                ));
            }
            let catch_indent = lines[*cursor].indent;
            let catch_body = parse_block(lines, cursor, catch_indent)?;
            program.statements.push(Spanned::new(
                Stmt::TryCatch {
                    body,
                    binding,
                    catch_body,
                },
                SourceSpan {
                    line: line_number,
                    column: indent * 4 + 1,
                    length: text.len(),
                },
            ));
        } else if let Some(function) = parse_function_header(&text) {
            *cursor += 1;
            let function = function?;
            if *cursor >= lines.len() || lines[*cursor].indent <= indent {
                return Err(format!(
                    "function requires an indented block at line {line_number}"
                ));
            }
            let body_indent = lines[*cursor].indent;
            let body = parse_block(lines, cursor, body_indent)?;
            let (name, type_params, params, return_type, visibility, is_async, exported) = function;
            program.statements.push(Spanned::new(
                Stmt::Function {
                    name,
                    type_params,
                    params,
                    return_type,
                    body,
                    visibility,
                    is_async,
                    exported,
                },
                SourceSpan {
                    line: line_number,
                    column: indent * 4 + 1,
                    length: text.len(),
                },
            ));
        } else if let Some(class) = parse_class_header(&text) {
            *cursor += 1;
            let class = class?;
            let body = if *cursor >= lines.len() || lines[*cursor].indent <= indent {
                Program::default()
            } else {
                let body_indent = lines[*cursor].indent;
                parse_block(lines, cursor, body_indent)?
            };
            let (name, base) = class;
            program.statements.push(Spanned::new(
                Stmt::Class { name, base, body },
                SourceSpan {
                    line: line_number,
                    column: indent * 4 + 1,
                    length: text.len(),
                },
            ));
        } else if let Some((kind, header)) = parse_control_header(&text) {
            if header.is_empty() {
                return Err(format!("missing {kind} condition at line {line_number}"));
            }
            *cursor += 1;
            if *cursor >= lines.len() || lines[*cursor].indent <= indent {
                return Err(format!(
                    "{kind} requires an indented block at line {line_number}"
                ));
            }
            let body_indent = lines[*cursor].indent;
            let body = parse_block(lines, cursor, body_indent)?;
            let condition_or_iterable = if kind == "for" {
                let (binding, iterable) = header.split_once(" in ").ok_or_else(|| {
                    format!("for expects '<name> in <expression>' at line {line_number}")
                })?;
                if binding.trim().is_empty() {
                    return Err(format!("for binding is missing at line {line_number}"));
                }
                let statement = Stmt::For {
                    binding: binding.trim().to_string(),
                    iterable: parse_expression(iterable.trim())?,
                    body,
                };
                program.statements.push(Spanned::new(
                    statement,
                    SourceSpan {
                        line: line_number,
                        column: indent * 4 + 1,
                        length: text.len(),
                    },
                ));
                continue;
            } else {
                parse_expression(header)?
            };
            let statement = if kind == "if" {
                let else_branch = if *cursor < lines.len()
                    && lines[*cursor].indent == indent
                    && lines[*cursor].text == "else:"
                {
                    *cursor += 1;
                    if *cursor >= lines.len() || lines[*cursor].indent <= indent {
                        return Err(format!(
                            "else requires an indented block at line {line_number}"
                        ));
                    }
                    let else_indent = lines[*cursor].indent;
                    Some(parse_block(lines, cursor, else_indent)?)
                } else {
                    None
                };
                Stmt::If {
                    condition: condition_or_iterable,
                    then_branch: body,
                    else_branch,
                }
            } else {
                Stmt::While {
                    condition: condition_or_iterable,
                    body,
                }
            };
            program.statements.push(Spanned::new(
                statement,
                SourceSpan {
                    line: line_number,
                    column: indent * 4 + 1,
                    length: text.len(),
                },
            ));
        } else {
            *cursor += 1;
            let mut statement = parse_statement(&text)?;
            statement.span.line = line_number;
            statement.span.column = indent * 4 + 1;
            program.statements.push(statement);
        }
    }
    Ok(program)
}

pub(crate) fn parse_program(source: &str) -> Result<Program, String> {
    let lines = source_lines(source)?;
    if lines.is_empty() {
        return Ok(Program::default());
    }
    let root_indent = lines[0].indent;
    if root_indent != 0 {
        return Err(format!(
            "program must start at indentation level zero at line {}",
            lines[0].number
        ));
    }
    let mut cursor = 0;
    let program = parse_block(&lines, &mut cursor, 0)?;
    if cursor < lines.len() {
        return Err(format!(
            "unexpected '{}' at line {}",
            lines[cursor].text, lines[cursor].number
        ));
    }
    Ok(program)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_precedence_and_preserves_expression_span() {
        let expression = parse_expression("1 + 2 * 3").expect("valid expression");
        assert_eq!(expression.span.line, 1);
        assert!(matches!(
            expression.node,
            Expr::Binary {
                op: BinaryOp::Add,
                right,
                ..
            } if matches!(right.node, Expr::Binary { op: BinaryOp::Multiply, .. })
        ));
    }

    #[test]
    fn parses_call_and_index_postfix_nodes() {
        let expression = parse_expression("items(1)[0]").expect("valid postfix expression");
        assert!(
            matches!(expression.node, Expr::Index { target, .. } if matches!(target.node, Expr::Call { .. }))
        );
    }

    #[test]
    fn rejects_trailing_tokens_with_location() {
        let error = parse_expression("1 2").expect_err("trailing tokens must fail");
        assert!(error.contains("unexpected token after expression at 1:3"));
    }

    #[test]
    fn parses_statement_forms_with_spans() {
        assert!(
            matches!(parse_statement("value = 1 + 2").unwrap().node, Stmt::Assignment { name, .. } if name == "value")
        );
        assert!(matches!(
            parse_statement("return value").unwrap().node,
            Stmt::Return(Some(_))
        ));
        assert!(matches!(
            parse_statement("break").unwrap().node,
            Stmt::Break
        ));
        assert!(matches!(
            parse_statement("continue").unwrap().node,
            Stmt::Continue
        ));
        assert!(matches!(
            parse_statement("value + 1").unwrap().node,
            Stmt::Expression(_)
        ));
    }

    #[test]
    fn rejects_invalid_statement_targets_and_missing_values() {
        assert!(parse_statement("1value = 2").is_err());
        assert!(parse_statement("value =").is_err());
        assert!(matches!(
            parse_statement("returning").unwrap().node,
            Stmt::Expression(_)
        ));
    }

    #[test]
    fn parses_indented_control_flow_programs() {
        let program = parse_program(
            "if ready:\n    value = 1\nelse:\n    value = 2\nwhile value:\n    break\nfor item in items:\n    continue\n",
        )
        .expect("valid control-flow program");
        assert_eq!(program.statements.len(), 3);
        assert!(matches!(
            program.statements[0].node,
            Stmt::If {
                else_branch: Some(_),
                ..
            }
        ));
        assert!(matches!(program.statements[1].node, Stmt::While { .. }));
        assert!(
            matches!(program.statements[2].node, Stmt::For { ref binding, .. } if binding == "item")
        );
    }

    #[test]
    fn parses_function_and_class_declarations() {
        let program = parse_program(
            "fn add(a: number, b: number) -> number:\n    return a + b\nclass Child(Parent):\n    value = 1\n",
        )
        .expect("valid declarations");
        assert!(matches!(
            program.statements[0].node,
            Stmt::Function { ref name, ref params, ref return_type, .. }
                if name == "add"
                    && params.len() == 2
                    && params[0].0 == "a"
                    && params[0].1.as_deref() == Some("number")
                    && return_type.as_deref() == Some("number")
        ));
        assert!(matches!(
            program.statements[1].node,
            Stmt::Class { ref name, ref base, .. }
                if name == "Child" && base.as_deref() == Some("Parent")
        ));
    }

    #[test]
    fn parses_native_propagation_and_boolean_prefix_expressions() {
        let propagation = parse_expression("some(1)?").expect("valid propagation expression");
        assert!(
            matches!(propagation.node, Expr::Propagate(value) if matches!(value.node, Expr::Call { .. }))
        );
        let negation = parse_expression("not ready").expect("valid boolean prefix expression");
        assert!(matches!(
            negation.node,
            Expr::Unary {
                op: UnaryOp::Not,
                ..
            }
        ));
    }

    #[test]
    fn parses_async_function_and_await_expression() {
        let expression = parse_expression("await load() ").expect("valid await expression");
        assert!(
            matches!(expression.node, Expr::Await(value) if matches!(value.node, Expr::Call { .. }))
        );
        let program = parse_program("async fn load() -> number:\n    return 7\n")
            .expect("valid async declaration");
        assert!(matches!(
            program.statements[0].node,
            Stmt::Function { is_async: true, .. }
        ));
    }

    #[test]
    fn parses_runtime_statement_forms() {
        let program = parse_program(
            "let total: number = 1 + 2\nsay total\nobj.value = total\nimport \"math.zp\"\nuse \"helpers.zp\"\n",
        )
        .expect("valid runtime statements");
        assert!(
            matches!(program.statements[0].node, Stmt::Declaration { ref name, ref annotation, .. } if name == "total" && annotation.as_deref() == Some("number"))
        );
        assert!(matches!(program.statements[1].node, Stmt::Say(_)));
        assert!(
            matches!(program.statements[2].node, Stmt::Assignment { ref name, .. } if name == "obj.value")
        );
        assert!(
            matches!(program.statements[3].node, Stmt::Import { ref path, explicit: true, alias: None } if path == "math.zp")
        );
        assert!(
            matches!(program.statements[4].node, Stmt::Import { ref path, explicit: false, alias: None } if path == "helpers.zp")
        );
    }

    #[test]
    fn parses_exported_bindings_and_functions() {
        let binding = parse_statement("export let answer: number = 42")
            .expect("exported binding should parse");
        assert!(matches!(
            binding.node,
            Stmt::Declaration {
                ref name,
                exported: true,
                ..
            } if name == "answer"
        ));

        let program = parse_program("export fn greet(name):\n    return name\n")
            .expect("exported function should parse");
        assert!(matches!(
            program.statements[0].node,
            Stmt::Function {
                ref name,
                exported: true,
                ..
            } if name == "greet"
        ));
    }

    #[test]
    fn parses_explicit_module_declarations() {
        let program = parse_program("module app.core\nimport app.core as core\n")
            .expect("valid module syntax");
        assert!(
            matches!(program.statements[0].node, Stmt::Module { ref name } if name == "app.core")
        );
        assert!(
            matches!(program.statements[1].node, Stmt::Import { ref path, alias: Some(ref alias), explicit: true } if path == "app.core" && alias == "core")
        );
    }

    #[test]
    fn parses_raise_statements_and_rejects_missing_values() {
        let program = parse_program("raise \"boom\"\n").expect("valid raise statement");
        assert!(matches!(program.statements[0].node, Stmt::Raise(_)));
        assert!(parse_program("raise\n").is_err());
        assert!(parse_program("raise \n").is_err());
    }

    #[test]
    fn rejects_unknown_annotations_at_parse_time() {
        assert!(parse_program("let value: unknown_type = 1\n")
            .unwrap_err()
            .contains("unknown type annotation 'unknown_type'"));
        assert!(
            parse_program("fn load(value: unknown_type):\n    return value\n")
                .unwrap_err()
                .contains("unknown type annotation 'unknown_type'")
        );
        assert!(parse_program("fn load() -> unknown_type:\n    return 1\n")
            .unwrap_err()
            .contains("unknown type annotation 'unknown_type'"));
    }

    #[test]
    fn accepts_supported_generic_annotations_at_parse_time() {
        parse_program("let values: list<number> = [1, 2]\n")
            .expect("supported generic annotations should parse");
        parse_program("let record: map<text, number> = {\"count\": 1}\n")
            .expect("supported map annotations should parse");
    }

    #[test]
    fn rejects_malformed_declaration_headers() {
        assert!(parse_program("fn add(a: number:\n    return a\n").is_err());
        assert!(parse_program("class Child(Parent:\n    value = 1\n").is_err());
        assert!(parse_program("fn add():\nvalue = 1\n").is_err());
    }

    #[test]
    fn rejects_invalid_control_flow_blocks() {
        assert!(parse_program("if ready:\nvalue = 1\n").is_err());
        assert!(parse_program("for item:\n    continue\n").is_err());
        assert!(parse_program("while ready:\n    break\nelse:\n    break\n").is_err());
        assert!(parse_program("    value = 1\n").is_err());
    }

    #[test]
    fn preserves_source_span_on_ast_nodes() {
        let span = SourceSpan {
            line: 3,
            column: 5,
            length: 2,
        };
        let node = Spanned::new(Expr::Literal(Literal::Number(42)), span.clone());
        assert_eq!(node.span, span);
        assert_eq!(node.node, Expr::Literal(Literal::Number(42)));
    }

    #[test]
    fn malformed_program_corpus_is_deterministic_and_panic_free() {
        let corpus = [
            "    say 1",
            "if true:\n  say 1\n    say 2",
            "fn broken(:\n    return",
            "let value = [1, 2",
            "class Broken:\n    fn init(",
            "if true:\n    if false:\n        say 1\n  say 2",
            "return )",
            "",
        ];
        for source in corpus {
            let first = std::panic::catch_unwind(|| parse_program(source));
            let second = std::panic::catch_unwind(|| parse_program(source));
            assert!(first.is_ok(), "parser panicked for {source:?}");
            assert_eq!(
                first
                    .as_ref()
                    .ok()
                    .and_then(|result| result.as_ref().ok())
                    .map(|program| program.statements.len()),
                second
                    .as_ref()
                    .ok()
                    .and_then(|result| result.as_ref().ok())
                    .map(|program| program.statements.len())
            );
        }
    }

    #[test]
    fn represents_nested_binary_expression_shape() {
        let span = SourceSpan {
            line: 1,
            column: 1,
            length: 1,
        };
        let left = Spanned::new(Expr::Literal(Literal::Number(1)), span.clone());
        let right = Spanned::new(Expr::Literal(Literal::Number(2)), span.clone());
        let expression = Expr::Binary {
            left: Box::new(left),
            op: BinaryOp::Add,
            right: Box::new(right),
        };
        assert!(matches!(
            expression,
            Expr::Binary {
                op: BinaryOp::Add,
                ..
            }
        ));
    }
}
