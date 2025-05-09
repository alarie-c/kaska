use crate::span::Span;

use super::expr::Expr;

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
