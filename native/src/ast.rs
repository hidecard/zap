#![allow(dead_code)]

use crate::lexer::SourceSpan;

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
    Call {
        callee: Box<Spanned<Expr>>,
        args: Vec<Spanned<Expr>>,
    },
    Index {
        target: Box<Spanned<Expr>>,
        index: Box<Spanned<Expr>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Stmt {
    Expression(Spanned<Expr>),
    Assignment { name: String, value: Spanned<Expr> },
    Return(Option<Spanned<Expr>>),
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
                            args.push(self.parse_expression(0)?);
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
