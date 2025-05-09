use crate::ast::{ expr::{ Expr, ExprKind }, stmt::Stmt };

use super::value::{ Scope, Type, Value };

pub struct Resolver {
    ast: Vec<Stmt>,
    scopes: Vec<Scope>,
    curr_scope: usize,
}

impl Resolver {
    pub fn new(ast: Vec<Stmt>) -> Self {
        Resolver { ast, scopes: vec![Scope::empty()], curr_scope: 0 }
    }

    pub fn resolve(&mut self) {}
}

impl Resolver {
    fn current(&mut self) -> &mut Scope {
        return self.scopes.get_mut(self.curr_scope).unwrap();
    }

    fn enter(&mut self) {
        self.scopes.push(Scope::empty());
        self.curr_scope = self.scopes.len() - 1;
    }

    fn leave(&mut self) {
        if self.curr_scope > 0 {
            self.scopes.pop();
            self.curr_scope = self.scopes.len() - 1;
        }
    }

    fn push(&mut self, value: Value) {
        self.current().add(value);
    }

    fn lookup(&self, name: &String) -> Option<&Value> {
        for i in (0..=self.curr_scope).rev() {
            let scope = &self.scopes[i];
            if let Some(value) = scope.lookup(name) {
                return Some(value);
            }
        }
        None
    }
}

impl Resolver {
    fn resolve_expr(&mut self, expr: &Expr) -> Type {
        match &expr.kind {
            // literals
            crate::ast::expr::ExprKind::Integer { value: _ } => Type::Integer,
            crate::ast::expr::ExprKind::Float { value: _ } => Type::Float,
            crate::ast::expr::ExprKind::String { value: _ } => Type::String,
            crate::ast::expr::ExprKind::Boolean { value: _ } => Type::Boolean,

            // symbols/values
            crate::ast::expr::ExprKind::Ident { name } => {
                if let Some(value) = self.lookup(&name) {
                    return value.typ;
                }
                return Type::Nil;
            }

            crate::ast::expr::ExprKind::Assignment { assignee, value, op: _ } => {
                match assignee.kind {
                    ExprKind::Ident { name: _ } => {}
                    _ => panic!("assignee not a ident"),
                }

                let original_type = self.resolve_expr(&assignee);
                let new_type = self.resolve_expr(&value);

                // assert similarity
                if original_type != new_type {
                    panic!("wrong type brochacho!!!");
                }

                return new_type;
            }

            crate::ast::expr::ExprKind::Binary { lhs, rhs, op: _ } => {
                let lhs_type = self.resolve_expr(&lhs);
                let rhs_type = self.resolve_expr(&rhs);

                if lhs_type == rhs_type {
                    return lhs_type;
                }

                if
                    (lhs_type == Type::Integer && rhs_type == Type::Float) ||
                    (lhs_type == Type::Float && rhs_type == Type::Integer)
                {
                    return Type::Float;
                }

                panic!("cant add these two things brother");
                // return Type::Nil;
            }

            _ => unimplemented!("expr"),
        }
    }
}
