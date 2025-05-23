use std::{ collections::HashMap, fmt::Display };
use crate::parser::ast::Expr;

/// Represents the any type of an expression in the program.
///
/// Will eventually be extended to include intersection and union types.
pub enum Type {
    Int,
    Float,
    Str,
    Bool,
    None,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Str => write!(f, "str"),
            Self::Bool => write!(f, "boll"),
            Self::None => write!(f, "none"),
        }
    }
}

/// Used to keep track of typing metadata for AST nodes
pub type Metadata<'a> = HashMap<usize, Typing<'a>>;

pub enum Typing<'a> {
    /// Typing metadata for normal expressions
    Expr {
        typ: Type,
        nullible: bool,
        lvalue: bool,
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

/// Represents any level of scope (including global) and all the names in that scope
pub struct Context {
    names: HashMap<String, Type>,
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
    pub fn store(&mut self, id: &String, typ: Type) {
        let _ = self.names.insert(id.to_string(), typ);
    }

    /// Looks for a name in this context and returns it's type.
    pub fn lookup(&mut self, id: &String) -> Option<&Type> {
        return self.names.get(id);
    }
}
