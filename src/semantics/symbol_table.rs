use crate::common::types::{ Context, Symbol, Type, Typing };

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

    pub fn sym(typ: Type, nullible: bool) -> Symbol {
        return Symbol {
            typ,
            nullible,
        };
    }

    pub fn local(&mut self) -> &mut Context {
        assert_ne!(self.context_map.len(), 0, "global scope is missing?");
        return self.context_map.last_mut().unwrap();
    }

    /// Stores a symbol in this scope specifically
    pub fn store_local(&mut self, id: &String, sym: Symbol) {
        let local = self.local();
        local.store(id, sym);
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

    pub fn lookup(&self, id: &String) -> Option<&Symbol> {
        for ctx in self.context_map.iter().rev() {
            if let Some(sym) = ctx.lookup(id) {
                return Some(sym)
            }   
        }
        return None;
    }
}
