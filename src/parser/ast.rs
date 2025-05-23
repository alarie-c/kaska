use crate::{ common::span::Span, lexer::token::Tk };
use std::fmt::Display;

// ----------------------------------------------------------------- \\
// EXPRESSIONS
// ----------------------------------------------------------------- \\

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub uid: usize,
}

impl Expr {
    pub fn new(uid: usize, kind: ExprKind, span: Span) -> Expr {
        Expr { uid, kind, span }
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
    (Call, $uid:expr, $callee:expr, $args:expr, $span:expr) => {
        Expr::new($uid, ExprKind::Call { callee: Box::new($callee), args: $args }, $span)
    };
    (Binary, $uid:expr, $lhs:expr, $rhs:expr, $op:expr, $span:expr) => {
        Expr::new($uid, ExprKind::Binary { lhs: Box::new($lhs), rhs: Box::new($rhs), op: $op }, $span)
    };
    (Assignment, $uid:expr, $assignee:expr, $value:expr, $op:expr, $span:expr) => {
        Expr::new($uid, ExprKind::Assignment { assignee: Box::new($assignee), value: Box::new($value), op: $op }, $span)
    };
    (Parameter, $uid:expr, $name:expr, $ty:expr, $span:expr) => {
        Expr::new($uid, ExprKind::Parameter { name: $name, ty: Box::new($ty) }, $span)
    };
}

// ----------------------------------------------------------------- \\
// STATEMENTS
// ----------------------------------------------------------------- \\

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
    pub uid: usize,
}

impl Stmt {
    pub fn new(uid: usize, kind: StmtKind, span: Span) -> Stmt {
        Stmt { uid, kind, span }
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
    (Variable, $uid:expr, $name:expr, $typ:expr, $value:expr, $span:expr) => {
        Stmt::new($uid, StmtKind::Variable { name: $name, typ: $typ, value: $value }, $span)
    };
    (Function, $uid:expr, $name:expr, $ret:expr, $params:expr, $body:expr, $span:expr) => {
        Stmt::new($uid, StmtKind::Function { name: $name, ret: $ret, params: $params, body: $body }, $span)
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
    pub fn binary(tk: &Tk) -> Option<Operator> {
        match tk {
            Tk::Plus => Some(Operator::Add),
            Tk::Minus => Some(Operator::Sub),
            Tk::Star => Some(Operator::Mul),
            Tk::Slash => Some(Operator::Div),
            Tk::StarStar => Some(Operator::Exp),
            Tk::SlashSlash => Some(Operator::Floor),

            // logical operators
            Tk::PipePipe => Some(Operator::LogOr),
            Tk::AmprsndAmprsnd => Some(Operator::LogAnd),

            // comparison operators
            Tk::Less => Some(Operator::Lt),
            Tk::LessEqual => Some(Operator::LtEq),
            Tk::More => Some(Operator::Mt),
            Tk::MoreEqual => Some(Operator::MtEq),
            Tk::Bang => Some(Operator::Bang),
            Tk::BangEqual => Some(Operator::BangEq),
            Tk::EqualEqual => Some(Operator::EqEq),
            _ => None,
        }
    }

    pub fn assignment(tk: &Tk) -> Option<Operator> {
        match tk {
            Tk::Equal => Some(Operator::Eq),
            Tk::PlusEqual => Some(Operator::AddEq),
            Tk::MinusEqual => Some(Operator::SubEq),
            Tk::StarEqual => Some(Operator::MulEq),
            Tk::SlashEqual => Some(Operator::DivEq),
            Tk::StarStarEqual => Some(Operator::ExpEq),
            Tk::SlashSlashEqual => Some(Operator::FloorEq),
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
