use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    Integer,
    Float,
    String,
    Boolean,
    Nil,
}

pub struct Value {
    pub typ: Type,
    pub name: String,
}

impl Value {
    pub fn new(typ: Type, name: String) -> Value {
        Value {
            typ,
            name,
        }
    }
}

pub struct Scope {
    items: HashMap<String, Value>,
}

impl Scope {
    pub fn empty() -> Scope {
        Scope {
            items: HashMap::<String, Value>::new(),
        }
    }

    pub fn add(&mut self, value: Value) {
        self.items.insert(value.name.clone(), value);
    }

    pub fn lookup(&self, name: &String) -> Option<&Value> {
        return self.items.get(name);
    }
}
