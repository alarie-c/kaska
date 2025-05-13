use std::{ collections::HashMap, fmt::Display };

pub(in crate::analysis) struct Value {
    pub typ: Type,
    pub mutable: bool,
}

impl Value {
    pub(in crate::analysis) fn new(typ: Type, mutable: bool) -> Value {
        Value {
            typ,
            mutable,
        }
    }
}

/// Represents any level of scope in a program
pub(in crate::analysis) struct Scope {
    /// All of the values indexed by their identifiers' names
    items: HashMap<String, Value>,
}

impl Scope {
    /// Create a brand new, empty scope
    pub(in crate::analysis) fn empty() -> Scope {
        Scope {
            items: HashMap::<String, Value>::new(),
        }
    }

    /// Add a new value to this scope and index it with `name`
    pub(in crate::analysis) fn add(&mut self, name: &String, value: Value) {
        self.items.insert(name.to_owned(), value);
    }

    /// Look for a particular value in this scope and this scope only
    pub(in crate::analysis) fn lookup(&self, name: &String) -> Option<&Value> {
        return self.items.get(name);
    }
}

/// Very basic enum type for holding the primitive data types of a program
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::analysis) enum Type {
    /// 32-bit signed integer
    Integer,

    /// 32-bit floating point
    Float,

    /// Regular string type
    String,

    /// True or false
    Boolean,

    /// Nil/null type
    Nil,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::String => write!(f, "string"),
            Self::Boolean => write!(f, "bool"),
            Self::Nil => write!(f, "nil"),
        }
    }
}
