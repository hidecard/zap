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

#[cfg(test)]
mod tests {
    use super::*;

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
