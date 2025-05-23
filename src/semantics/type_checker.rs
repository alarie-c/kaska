use std::collections::HashMap;
use crate::{ common::{ errors::ErrorBuffer, types::Metadata }, parser::ast::Stmt };
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
}
