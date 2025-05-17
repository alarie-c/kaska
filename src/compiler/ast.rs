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
    Parameter {
        name: String,
        ty: Box<Expr>,
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
    VarDecl {
        mutable: bool,
        name: String,
        value: Expr,
        typ: Option<Expr>,
    },

    ProcDef {
        name: String,
        params: Vec<Expr>,
        returns: Option<Expr>,
        body: Vec<Stmt>,
    },

    Expr {
        expr: Expr,
    },

    Return {
        expr: Expr,
    },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    // arithmetic operators
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Floor,

    // assignment operators
    Eq,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    ExpEq,
    FloorEq,

    // logical operators
    BitAnd,
    LogAnd,
    BitOr,
    LogOr,

    // comparison operators
    Lt,
    LtEq,
    Mt,
    MtEq,
    Bang,
    BangEq,
    EqEq,
}

impl Operator {
    pub(in crate::compiler) fn expect_binary(tk: &TokenKind) -> Option<Operator> {
        match tk {
            TokenKind::Plus => Some(Operator::Add),
            TokenKind::Minus => Some(Operator::Sub),
            TokenKind::Star => Some(Operator::Mul),
            TokenKind::Slash => Some(Operator::Div),
            TokenKind::StarStar => Some(Operator::Exp),
            TokenKind::SlashSlash => Some(Operator::Floor),

            // logical operators
            TokenKind::PipePipe => Some(Operator::LogOr),
            TokenKind::AmprsndAmprsnd => Some(Operator::LogAnd),

            // comparison operators
            TokenKind::Less => Some(Operator::Lt),
            TokenKind::LessEqual => Some(Operator::LtEq),
            TokenKind::More => Some(Operator::Mt),
            TokenKind::MoreEqual => Some(Operator::MtEq),
            TokenKind::Bang => Some(Operator::Bang),
            TokenKind::BangEqual => Some(Operator::BangEq),
            TokenKind::EqualEqual => Some(Operator::EqEq),
            _ => None,
        }
    }

    pub(in crate::compiler) fn expect_assign(tk: &TokenKind) -> Option<Operator> {
        match tk {
            TokenKind::Equal => Some(Operator::Eq),
            TokenKind::PlusEqual => Some(Operator::AddEq),
            TokenKind::MinusEqual => Some(Operator::SubEq),
            TokenKind::StarEqual => Some(Operator::MulEq),
            TokenKind::SlashEqual => Some(Operator::DivEq),
            TokenKind::StarStarEqual => Some(Operator::ExpEq),
            TokenKind::SlashSlashEqual => Some(Operator::FloorEq),
            _ => None,
        }
    }
}
