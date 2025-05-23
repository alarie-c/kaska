use std::collections::HashMap;
use crate::{ common::{ errors::{Error, ErrorBuffer, ErrorKind}, types::{Metadata, Type, Typing} }, parser::ast::{Expr, ExprKind, Stmt, StmtKind} };
use super::symbol_table::SymbolTable;

/// Container for all type checking logic and structures, including the
/// symbol map for name resolution, metadata for semantic analysis, and
/// the error buffer for warnings and errors that are emitted during this process.
pub struct TypeChecker<'a> {
    ast: &'a Vec<Stmt>,
    pub symbols: SymbolTable,
    pub metadata: Metadata<'a>,
    pub ebuffer: ErrorBuffer,
}

impl<'a> TypeChecker<'a> {
    /// Returns an empty type checker with everything initialized to nothing
    pub fn new(ast: &'a Vec<Stmt>) -> TypeChecker<'a> {
        return TypeChecker {
            ast,
            symbols: SymbolTable::new(),
            metadata: HashMap::new(),
            ebuffer: vec![],
        };
    }

    pub fn resolve(&mut self) {
        for stmt in self.ast {
            match self.stmt(stmt) {
                Ok(_) => {},
                Err(e) => self.ebuffer.push(e),
            }
        }
    }

    pub fn metadata(&mut self, id: usize, typing: Typing<'a>) {
        let _ = self.metadata.insert(id, typing);
    }
}

impl<'a> TypeChecker<'a> {
    fn expr(&mut self, expr: &Expr) -> Result<Type, Error> {
        match &expr.kind {
            ExprKind::Integer { value: _ } => {
                let md = Typing::Expr { typ: Type::Int, nullible: false, lvalue: false };
                self.metadata(expr.uid, md);
                return Ok(Type::Int)
            }
            ExprKind::Float { value: _ } => {
                let md = Typing::Expr { typ: Type::Float, nullible: false, lvalue: false };
                self.metadata(expr.uid, md);
                return Ok(Type::Float)
            }

            ExprKind::Ident { name } => {
                match Type::get_primitive_from_ident(name) {
                    Some(t) => return Ok(t),
                    None => {},
                }
                
                match self.symbols
                    .lookup(name)
                    .map(|s| *s )
                {
                    Some(sym) => {
                        let md = Typing::Expr { typ: sym.typ, nullible: sym.nullible, lvalue: false };
                        self.metadata(expr.uid, md);
                        return Ok(sym.typ)
                    }
                    None => {
                        let err = Error::new(
                            ErrorKind::UnknownIdentifier,
                            expr.span.clone(),
                            format!("identifier '{}' not defined", name),
                            true
                        );
                        return Err(err);
                    }
                }
            }
            _ => unimplemented!("expr")
        }
    }
}

impl<'a> TypeChecker<'a> {
    fn stmt(&mut self, stmt: &Stmt) -> Result<(), Error> {
        match &stmt.kind {
            StmtKind::Variable { name, typ: _, value } => {
                // TODO: Assert strong type is equal to value type
                let t_val = self.expr(&value)?;
                let sym = SymbolTable::sym(t_val, false);
                self.symbols.store_local(&name, sym);
                return Ok(())
            }
            StmtKind::Function { name, ret, params, body } => {
                let t_ret: Type;
                if ret.is_some() {
                    let span = ret.as_ref().unwrap().span.clone();
                    t_ret = self.expr(ret.as_ref().unwrap())?;
                    
                    // warn for redundant "-> None" code
                    if t_ret == Type::None {
                        let w = Error::new(
                            ErrorKind::RedundantCode,
                            span,
                            "'-> None' unecessary as functions return None by default"
                                .to_string(),
                            false
                        );
                        self.ebuffer.push(w);
                    }
                } else {
                    t_ret = Type::None;
                }

                // process these params and create their metadata
                let arity = params.len();
                for p in params { self.expr(&p)?; }

                // create metadata for this function node
                let md = Typing::Function { ret: t_ret, nullible: false, arity };
                self.metadata(stmt.uid, md);

                // store this function's ID
                let sym = SymbolTable::sym(Type::Function, false);
                self.symbols.store_local(&name, sym);

                // process these statements
                for s in body { self.stmt(&s)?; }

                return Ok(())
            }
        }
    }
}
