use std::{ collections::HashMap, fmt::Display };
use crate::parser::ast::Expr;

use super::errors::Error;

/// Represents the any type of an expression in the program.
///
/// Will eventually be extended to include intersection and union types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    Function,
    None,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Str => write!(f, "str"),
            Self::Bool => write!(f, "bool"),
            Self::Function => write!(f, "Func"),
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

/// Used to keep track of typing metadata for AST nodes
pub type Metadata<'a> = HashMap<usize, Typing<'a>>;

#[derive(Debug)]
pub enum Typing<'a> {
    /// Typing metadata for normal expressions
    Expr {
        typ: Type,
        nullible: bool,
        lvalue: bool,
    },

    Cast {
        from: Type,
        to: Type,
    },

    /// Typing metadata for function parameters
    Param {
        typ: Type,
        nullible: bool,
        default: Option<&'a Expr>,
    },

    /// Typing metadata for functions in the program
    Function {
        ret: Type,
        nullible: bool,
        arity: usize,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct Symbol {
    pub typ: Type,
    pub nullible: bool,
}

/// Represents any level of scope (including global) and all the names in that scope
pub struct Context {
    names: HashMap<String, Symbol>,
}

impl Context {
    /// Returns an empty context with no names in it.
    pub fn empty() -> Context {
        return Context {
            names: HashMap::new(),
        };
    }

    /// Stores a new name in this context. Note that there is no protection against
    /// ID overwrites, the type checker is responsible for knowing when shadowing happens
    /// and when to emit a warning about it.
    pub fn store(&mut self, id: &String, sym: Symbol) {
        let _ = self.names.insert(id.to_string(), sym);
    }

    /// Looks for a name in this context and returns it's type.
    pub fn lookup(&self, id: &String) -> Option<&Symbol> {
        return self.names.get(id);
    }
}
