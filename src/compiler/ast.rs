use crate::common::span::Span;

use super::lexer::TokenKind;

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
            TokenKind::Plus => Some(Operator::Plus),
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

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn new(kind: StmtKind, span: Span) -> Stmt {
        Stmt { kind, span }
    }
}

#[derive(Debug)]
pub enum StmtKind {
    VariableDecl {
        mutable: bool,
        name: String,
        value: Box<Expr>,
        typ: Option<Box<Expr>>,
    },

    ProcedureDecl {
        name: String,
        params: Box<Expr>,
        returns: Option<Box<Expr>>,
    },

    Expression {
        expr: Box<Expr>,
    },
}
