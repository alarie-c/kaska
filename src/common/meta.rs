use std::{ collections::HashMap, fmt::Display };
use crate::parser::ast::{ Expr, Operator };
use super::errors::Error;

// ----------------------------------------------------------------- \\
// TYPING STRUCTURES
// ----------------------------------------------------------------- \\

/// Represents the any type of an expression in the program.
/// Will eventually be extended to include intersection and union types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    Function(Vec<Type>, Box<Type>),
    None,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Str => write!(f, "str"),
            Self::Bool => write!(f, "bool"),
            Self::Function(pars, ret) => write!(f, "Func ({pars:?}) -> {ret}"),
            Self::None => write!(f, "none"),
        }
    }
}

impl Type {
    pub fn get_primitive_from_ident(id: &String) -> Option<Type> {
        match id.as_str() {
            "int" => Some(Type::Int),
            "float" => Some(Type::Float),
            "str" => Some(Type::Str),
            "bool" => Some(Type::Bool),
            "None" => Some(Type::None),
            _ => None,
        }
    }
}

// ----------------------------------------------------------------- \\
// METADATA STRUCTURES
// ----------------------------------------------------------------- \\

/// High level metadata structure that incorporates all aspects of semantic
/// analysis in the program into one structure.
pub struct Metadata {
    types: HashMap<usize, Type>,
    symbols: HashMap<usize, SymbolInfo>,
    scopes: HashMap<usize, ScopeInfo>,
    // overloads: HashMap<usize, OverloadInfo>,
}

// ----------------------------------------------------------------- \\
// SYMBOLS
// ----------------------------------------------------------------- \\

pub struct SymbolInfo {
    name: String,
    kind: SymbolKind,
    node: usize,
    ty: Type,
}

pub enum SymbolKind {
    Variable,
    Function,
    Type,
}

// ----------------------------------------------------------------- \\
// SCOPES
// ----------------------------------------------------------------- \\

pub struct ScopeInfo {
    id: usize,
    parent: Option<usize>,
    symbols: HashMap<String, SymbolInfo>
}