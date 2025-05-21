use std::{collections::HashMap, fmt::{write, Display}};

use super::ast::{Expr, ExprKind, Operator, Stmt, StmtKind};
use crate::common::errors::{Error, ErrorBuffer, ErrorKind};

macro_rules! mismatch {
    ($span:expr, $lhs:expr, $rhs:expr, $op:expr) => {
        Error::new(
            ErrorKind::TypeMismatch,
            $span,
            format!(
                "types '{}' and '{}' are not compatible in '{}' operation",
                $lhs, $rhs, $op
            ),
            true,
        )
    };
}

macro_rules! dne {
    ($span:expr, $name:expr) => {
        Error::new(
            ErrorKind::UnknownIdentifier,
            $span,
            format!(
                "identifier '{}' is not defined",
                $name
            ),
            true,
        )
    };
}

struct Scope {
    symbols: HashMap<String, Type>,
}

impl Scope {
    /// Creates a new empty scope with a blank hash map.
    pub fn empty() -> Scope {
        return Scope {
            symbols: HashMap::new(),
        }
    }

    /// Will attempt to find an identifier in the hash map and return a copied instance
    /// of the type if successful.
    pub fn lookup(&self, id: &String) -> Option<Type> {
        return self.symbols.get(id).map(|t| *t );
    }

    /// Will store the given identifier in the symbols hash map.
    /// There is no protection against duplication.
    pub fn store(&mut self, id: &String, typ: Type) {
        let _ = self.symbols.insert(id.to_string(), typ);
    }
}

#[derive(PartialEq, Copy, Clone)]
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

pub struct TypeChecker {
    scopes: Vec<Scope>,
    pub(self) errs: ErrorBuffer,
}

impl TypeChecker {
    pub fn check(ast: &mut Vec<Stmt>) -> ErrorBuffer {
        let mut checker = TypeChecker {
            scopes: vec![Scope::empty()],
            errs: vec![]
        };
        

        for stmt in ast.iter() {
            match checker.check_stmt(stmt) {
                Ok(_) => {},
                Err(e) => checker.errs.push(e),
            }
        }

        // return the error buffer
        return checker.errs.drain(0..).collect();
    }
}

impl TypeChecker {
    /// Create a new scope and put it at the end of the scopes vec.
    fn enter_scope(&mut self) {
        self.scopes.push(Scope::empty());
    }

    /// Removes the bottom scope (closest scope) from the scopes vec. Will panic if you
    /// try and remove the global scope :D.
    fn exit_scope(&mut self) {
        if self.scopes.len() == 1 {
            panic!("Tried to exit global scope?");
        }
        let _ = self.scopes.pop().unwrap();
    }

    /// Returns the last (closest scope). In the case that the length of scopes is 1,
    /// this will return the global scope.
    fn current_scope(&mut self) -> &mut Scope {
        return self.scopes.last_mut().unwrap();
    }

    /// A helper method to find if an identifier exists in any scope.
    /// Will scan through every scope starting from the bottom (closest scope) and
    /// return the first instance of the symbol.
    fn lookup_all(&self, id: &String) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.lookup(id) {
                return Some(val)
            }
        }
        return None;
    }

    /// Will store a value in the bottom scope (closest scope).
    fn store_here(&mut self, id: &String, typ: Type) {
        let current = self.current_scope();
        current.store(id, typ);
    }
}

impl TypeChecker {
    /// Looks for errors and warnings in a statement and it's expression node
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match &stmt.kind {
            StmtKind::Variable { name, typ, value } => self.check_variable(name, typ, value),
            _ => unimplemented!("check stmt")
        }
    }

    /// Looks for errors and warnings in variable declarations
    fn check_variable(&mut self, name: &String, typ: &Option<Expr>, value: &Expr) -> Result<(), Error> {
        // TODO: make sure the explicit type and value type match
        // TODO: check for shadowing and whether or not to emit a warning
        let t_value = self.check_expr(value)?;
        self.store_here(name, t_value);
        return Ok(());
    }
}

impl TypeChecker {
    /// Evaluates the type of an expression and returns any pertinent errors
    fn check_expr(&mut self, expr: &Expr) -> Result<Type, Error> {
        match &expr.kind {
            ExprKind::Integer { value: _ } => Ok(Type::Integer),
            ExprKind::Float { value: _ } => Ok(Type::Float),
            
            ExprKind::Binary { lhs, rhs, op } => self.check_binary(&lhs, &rhs, &op),
            
            // look for the ident and return error if it doesn't exist
            ExprKind::Ident { name } => if let Some(ty) = self.lookup_all(name) {
                return Ok(ty);
            } else {
                return Err(dne!(expr.span.clone(), name));
            }
            _ => unimplemented!("check expr"),
        }
    }

    /// Evaluates the type of a binary expression based on the types of its operands and the operator.
    fn check_binary(&mut self, lhs: &Expr, rhs: &Expr, op: &Operator) -> Result<Type, Error> {
        let t_lhs = self.check_expr(lhs)?;
        let t_rhs = self.check_expr(rhs)?;

        if t_lhs == t_rhs {
            return Ok(t_lhs);
        } else if (t_lhs == Type::Integer && t_rhs == Type::Float)
            || (t_lhs == Type::Float && t_rhs == Type::Integer)
        {
            return Ok(Type::Float);
        } else {
            let span = lhs.span.start..rhs.span.end;
            return Err(mismatch!(span, t_lhs, t_rhs, op));
        }
    }
}
