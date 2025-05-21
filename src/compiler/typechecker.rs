use std::fmt::{write, Display};

use crate::common::errors::{Error, ErrorBuffer, ErrorKind};
use super::ast::{Expr, ExprKind, Operator, Stmt};

macro_rules! mismatch {
    ($span:expr, $lhs:expr, $rhs:expr, $op:expr) => {
        Error::new(
            ErrorKind::TypeMismatch,
            $span,
            format!("types '{}' and '{}' are not compatible in '{}' operation", $lhs, $rhs, $op),
            true
        )
    };
}

#[derive(PartialEq)]
enum Type {
    Integer,
    Float,
    String,
    Boolean,
    None,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "str"),
            Self::Boolean => write!(f, "bool"),
            Self::None => write!(f, "None"),
        }
    }
}

pub struct TypeChecker<'a> {
    ast: &'a mut Vec<Stmt>,
    pub(self) errs: ErrorBuffer,
}

impl<'a> TypeChecker<'a> {
    pub fn check(ast: &'a mut Vec<Stmt>) -> ErrorBuffer {
        let mut checker = TypeChecker {
            ast,
            errs: vec![]
        };

        // return the error buffer
        return checker.errs.drain(0..).collect();
    }
}

impl<'a> TypeChecker<'a> {
    fn check_expr(&mut self, expr: &Expr) -> Result<Type, Error> {
        match &expr.kind {
            ExprKind::Integer { value: _ } => Ok(Type::Integer),
            ExprKind::Float { value: _ } => Ok(Type::Float),
            ExprKind::Binary { lhs, rhs, op } => self.check_binary(&lhs, &rhs, &op),
            _ => unimplemented!("check expr")
        }
    }

    fn check_binary(&mut self, lhs: &Expr, rhs: &Expr, op: &Operator) -> Result<Type, Error> {
        let t_lhs = self.check_expr(lhs)?;
        let t_rhs = self.check_expr(rhs)?;

        if t_lhs == t_rhs {
            return Ok(t_lhs);
        } else if
            (t_lhs == Type::Integer && t_rhs == Type::Float) ||
            (t_lhs == Type::Float && t_rhs == Type::Integer) {
                return Ok(Type::Float)
        } else {
            let span = lhs.span.start..rhs.span.end;
            return Err(mismatch!(span, t_lhs, t_rhs, op))
        }
    }
}