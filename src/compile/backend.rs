use std::collections::HashMap;

const MAX_REGISTERS: usize = 16;

pub(in crate::compile) struct RegisterAlloc {
    registers: [bool; MAX_REGISTERS],
}

impl RegisterAlloc {
    pub(in crate::compile) fn new() -> RegisterAlloc {
        RegisterAlloc {
            registers: [false; MAX_REGISTERS],
        }
    }

    pub(in crate::compile) fn alloc(&mut self) -> usize {
        for i in 0..MAX_REGISTERS {
            if self.registers[i] == false {
                self.registers[i] = true;
                return i;
            }
        }
        panic!("No more free registers!");
    }

    pub(in crate::compile) fn free(&mut self, reg: usize) {
        assert!(reg < MAX_REGISTERS, "register out of bounds!");
        assert!(self.registers[reg] == true, "double free!");
        self.registers[reg] = false;
    }

    pub(in crate::compile) fn dump(&self) {
        println!("REGISTERS:");
        for i in 0..MAX_REGISTERS {
            println!("  R{:02}: {}", i, self.registers[i])
        }
    }
}

/// `identifier` -> `register`
pub(in crate::compile) type SymbolTable = HashMap<String, usize>;

pub(in crate::compile) fn store(table: &mut SymbolTable, name: &String, register: usize) {
    let _ = table.insert(name.to_string(), register);
}

pub(in crate::compile) fn get(table: &mut SymbolTable, name: &String) -> Option<usize> {
    return table.get(name).map(|s| *s);
}