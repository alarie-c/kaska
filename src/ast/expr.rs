use crate::{ span::Span, token::TokenKind };

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

impl Expr {
    pub fn new(kind: ExprKind, span: Span) -> Expr {
        Expr { kind, span }
    }
}

#[derive(Debug)]
pub enum ExprKind {
    // literals expressions
    Integer {
        value: i32,
    },
    Float {
        value: f32,
    },
    String {
        value: String,
    },
    Boolean {
        value: bool,
    },
    Ident {
        name: String,
    },

    // compound expressions
    Call {
        callee: Box<Expr>,
        args: Vec<Box<Expr>>,
    },
    Assignment {
        assignee: Box<Expr>,
        value: Box<Expr>,
        op: Operator,
    },

    // operator/operand expressions
    Binary {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: Operator,
    },
    Postfix {
        op: Operator,
        lhs: Box<Expr>,
    },
    Infix {
        op: Operator,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    AssignEqual,
}

impl Operator {
    pub fn expect_binary(tk: &TokenKind) -> Option<Operator> {
        match tk {
            TokenKind::Minus => Some(Operator::Minus),
            _ => None,
        }
    }

    pub fn expect_assign(tk: &TokenKind) -> Option<Operator> {
        match tk {
            TokenKind::Equal => Some(Operator::AssignEqual),
            _ => None,
        }
    }
}
