use crate::common::types::{ Context, Type };

/// Returns the global scope with all relevant symbols loaded
fn global() -> Context {
    return Context::empty();
}

pub struct SymbolTable {
    context_map: Vec<Context>,
}

impl SymbolTable {
    /// Returns an empty symbol table with nothing in it except global scope
    pub fn new() -> SymbolTable {
        return SymbolTable { context_map: vec![global()] };
    }

    pub fn here(&mut self) -> &mut Context {
        assert_ne!(self.context_map.len(), 0, "global scope is missing?");
        return self.context_map.last_mut().unwrap();
    }

    /// Stores a symbol in this scope specifically
    pub fn store_here(&mut self, id: &String, typ: Type) {
        let here = self.here();
        here.store(id, typ);
    }

    /// Steps out level into lexical scope (removes the last context)
    pub fn step_out(&mut self) {
        assert_ne!(self.context_map.len(), 1, "tried to delete global scope?");
        let _ = self.context_map.pop();
    }

    /// Steps one level into lexical scope
    pub fn step_into(&mut self) {
        self.context_map.push(Context::empty());
    }
}
