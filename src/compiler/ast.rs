use std::fmt::Display;

use crate::common::span::Span;
use super::lexer::TokenKind;

// ----------------------------------------------------------------- \\
// EXPRESSIONS
// ----------------------------------------------------------------- \\

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
        args: Vec<Expr>,
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

}

#[macro_export]
macro_rules! expr {
    (Call, $callee:expr, $args:expr, $span:expr) => {
        Expr::new(ExprKind::Call { callee: Box::new($callee), args: $args }, $span)
    };
    (Binary, $lhs:expr, $rhs:expr, $op:expr, $span:expr) => {
        Expr::new(ExprKind::Binary { lhs: Box::new($lhs), rhs: Box::new($rhs), op: $op }, $span)
    };
    (Assignment, $assignee:expr, $value:expr, $op:expr, $span:expr) => {
        Expr::new(ExprKind::Assignment { assignee: Box::new($assignee), value: Box::new($value), op: $op }, $span)
    };
    (Parameter, $name:expr, $ty:expr, $span:expr) => {
        Expr::new(ExprKind::Parameter { name: $name, ty: Box::new($ty) }, $span)
    };
}

// ----------------------------------------------------------------- \\
// STATEMENTS
// ----------------------------------------------------------------- \\

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
    Variable {
        name: String,
        typ: Option<Expr>,
        value: Expr,
    },

    Function {
        name: String,
        ret: Option<Expr>,
        params: Vec<Expr>,
        body: Vec<Stmt>,
    },
}

#[macro_export]
macro_rules! stmt {
    (Variable, $name:expr, $typ:expr, $value:expr, $span:expr) => {
        Stmt::new(StmtKind::Variable { name: $name, typ: $typ, value: $value }, $span)
    };
    (Function, $name:expr, $ret:expr, $params:expr, $body:expr, $span:expr) => {
        Stmt::new(StmtKind::Function { name: $name, ret: $ret, params: $params, body: $body }, $span)
    };
}

// ----------------------------------------------------------------- \\
// OPERATORS
// ----------------------------------------------------------------- \\

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
    pub(in crate::compiler) fn binary(tk: &TokenKind) -> Option<Operator> {
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

    pub(in crate::compiler) fn assignment(tk: &TokenKind) -> Option<Operator> {
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

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Exp => write!(f, "**"),
            Operator::Floor => write!(f, "//"),
            Operator::Eq => write!(f, "="),
            Operator::AddEq => write!(f, "+="),
            Operator::SubEq => write!(f, "-="),
            Operator::MulEq => write!(f, "*="),
            Operator::DivEq => write!(f, "/="),
            Operator::ExpEq => write!(f, "**="),
            Operator::FloorEq => write!(f, "//="),
            Operator::BitAnd => write!(f, "&"),
            Operator::LogAnd => write!(f, "and"),
            Operator::BitOr => write!(f, "|"),
            Operator::LogOr => write!(f, "or"),
            Operator::Lt => write!(f, "<"),
            Operator::LtEq => write!(f, "<="),
            Operator::Mt => write!(f, ">"),
            Operator::MtEq => write!(f, ">="),
            Operator::Bang => write!(f, "!"),
            Operator::BangEq => write!(f, "!="),
            Operator::EqEq => write!(f, "=="),
        }
    }
}
