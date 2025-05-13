use crate::{
    ast::{ expr::{ Expr, ExprKind }, stmt::{ Stmt, StmtKind } },
    errors::{ Error, ErrorBuffer, ErrorKind },
};
use super::value::{ Scope, Type, Value };

pub(crate) struct Resolver<'a> {
    ast: &'a Vec<Stmt>,
    scopes: Vec<Scope>,
    curr_scope: usize,
}

impl<'a> Resolver<'a> {
    pub(crate) fn new(ast: &'a Vec<Stmt>) -> Self {
        Resolver { ast, scopes: vec![Scope::empty()], curr_scope: 0 }
    }

    pub(crate) fn resolve(&mut self) -> ErrorBuffer {
        let mut errors: ErrorBuffer = vec![];
        for stmt in self.ast {
            match self.resolve_stmt(stmt) {
                Ok(_) => {}
                Err(e) => errors.push(e),
            }
        }
        return errors;
    }
}

impl<'a> Resolver<'a> {
    fn current(&mut self) -> &mut Scope {
        return self.scopes.get_mut(self.curr_scope).unwrap();
    }

    // fn enter(&mut self) {
    //     self.scopes.push(Scope::empty());
    //     self.curr_scope = self.scopes.len() - 1;
    // }

    // fn leave(&mut self) {
    //     if self.curr_scope > 0 {
    //         self.scopes.pop();
    //         self.curr_scope = self.scopes.len() - 1;
    //     }
    // }

    /// Push a new value to the current level scope
    fn push(&mut self, name: &String, value: Value) {
        self.current().add(name, value);
    }

    /// Look for the first possible instance of a value in every scope
    /// and return that value as soon as it's found
    fn lookup(&self, name: &String) -> Option<&Value> {
        // iterate through backwards, starting at the current scope and working
        // our way to the front of the scope vec (global scope)
        for i in (0..=self.curr_scope).rev() {
            match self.scopes[i].lookup(name) {
                Some(v) => return Some(v),
                None => continue,
            }
        }
        return None;
    }
}

impl<'a> Resolver<'a> {
    fn resolve_typename(&mut self, name: &String) -> Type {
        match name.as_str() {
            "int" => Type::Integer,
            "float" => Type::Float,
            "string" => Type::String,
            "bool" => Type::Boolean,
            _ => Type::Nil,
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match &stmt.kind {
            StmtKind::VariableDecl { mutable, name, value, typ } => {
                // decide if the current type is matching the strong typ
                let value_type = self.resolve_expr(&value)?;

                // if a type annotation is present, check it
                if let Some(typ) = typ {
                    let strong_type = match &typ.kind {
                        ExprKind::Ident { name } => self.resolve_typename(&name),
                        _ => Type::Nil,
                    };

                    // todo: type coercion
                    if strong_type != value_type {
                        return Err(
                            Error::new(
                                ErrorKind::TypeMismatch,
                                value.span.clone(),
                                format!(
                                    "declaration has the type annotation '{}' but the the type of the value is '{}' and it cannot be coerced to '{}'",
                                    strong_type,
                                    value_type,
                                    strong_type
                                ).as_str(),
                                true
                            )
                        );
                    }
                }

                // create the value and push
                self.push(name, Value::new(value_type, *mutable));
            }
            StmtKind::Expression { expr } => {
                let _ = self.resolve_expr(&expr)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<Type, Error> {
        match &expr.kind {
            // literals
            ExprKind::Integer { value: _ } => Ok(Type::Integer),
            ExprKind::Float { value: _ } => Ok(Type::Float),
            ExprKind::String { value: _ } => Ok(Type::String),
            ExprKind::Boolean { value: _ } => Ok(Type::Boolean),

            // symbols/values
            ExprKind::Ident { name } => {
                if let Some(value) = self.lookup(&name) {
                    return Ok(value.typ);
                }
                return Ok(Type::Nil);
            }

            ExprKind::Assignment { assignee, value, op: _ } => {
                let name: &String;
                match &assignee.kind {
                    ExprKind::Ident { name: n } => {
                        name = n;
                    }
                    _ => {
                        return Err(
                            Error::new(
                                ErrorKind::SyntaxError,
                                assignee.span.clone(),
                                "cannot assign to a non-identifier",
                                true
                            )
                        );
                    }
                }

                // get the assignee value details
                match self.lookup(name) {
                    Some(value) => if !value.mutable {
                        return Err(
                            Error::new(
                                ErrorKind::AssignToConstant,
                                expr.span.clone(),
                                "cannot assign to a constant variable",
                                true
                            )
                        );
                    }
                    None => {
                        return Err(
                            Error::new(
                                ErrorKind::UnknownIdentifier,
                                assignee.span.clone(),
                                "this identifier has not been declared",
                                true
                            )
                        );
                    }
                }

                let original_type = self.resolve_expr(&assignee)?;
                let new_type = self.resolve_expr(&value)?;

                // assert similarity
                if original_type != new_type {
                    return Err(
                        Error::new(
                            ErrorKind::TypeMismatch,
                            value.span.clone(),
                            format!(
                                "cannot assign '{}' to a variable of type '{}'",
                                new_type,
                                original_type
                            ).as_str(),
                            true
                        )
                    );
                }

                return Ok(new_type);
            }

            ExprKind::Binary { lhs, rhs, op: _ } => {
                let lhs_type = self.resolve_expr(&lhs)?;
                let rhs_type = self.resolve_expr(&rhs)?;

                if lhs_type == rhs_type {
                    return Ok(lhs_type);
                }

                if
                    (lhs_type == Type::Integer && rhs_type == Type::Float) ||
                    (lhs_type == Type::Float && rhs_type == Type::Integer)
                {
                    return Ok(Type::Float);
                }

                return Err(
                    Error::new(
                        ErrorKind::TypeMismatch,
                        expr.span.clone(),
                        "these types are not compatible in arithmetic binary operations",
                        true
                    )
                );
            }

            _ => unimplemented!("expr"),
        }
    }
}
