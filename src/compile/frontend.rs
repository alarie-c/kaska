use std::{collections::HashMap, fs::{self, File}, io::{self, Write}};

use crate::{ast::{expr::{Expr, ExprKind, Operator}, stmt::{Stmt, StmtKind}}};
use super::{backend::{get, store, RegisterAlloc, SymbolTable}, instructions::Instruction};

pub struct Compiler<'a> {
    stream: &'a Vec<Stmt>,
    instrs: Vec<Instruction>,
    allocator: RegisterAlloc,
    symbols: SymbolTable,
}

impl<'a> Compiler<'a> {
    pub fn new(stream: &'a Vec<Stmt>) -> Compiler<'a> {
        Compiler {
            stream,
            instrs: vec![],
            allocator: RegisterAlloc::new(),
            symbols: HashMap::new(),
        }
    }

    pub fn compile(&mut self, name: String) -> io::Result<File> {
        let mut file = File::create(name + ".ksb")?;
        
        // emit instructions
        for stmt in self.stream {
            self.compile_stmt(&stmt);
        }

        // write all the instructions to .ksb
        for inst in &self.instrs {
            file.write_all(inst.to_string().as_bytes())?;
        }

        file.write_all("DUMP\nEXIT".as_bytes())?;
        return Ok(file);
    }
}

impl<'a> Compiler<'a> {
    fn emit(&mut self, instr: Instruction) {
        self.instrs.push(instr);
    }

    fn compile_expr(&mut self, expr: &Expr) -> usize {
        match &expr.kind {
            ExprKind::Integer { value } => {
                let register = self.allocator.alloc();
                self.emit(Instruction::LoadInt(register, *value));
                return register;
            }

            ExprKind::Ident { name } => {
                let register = get(&mut self.symbols, name).expect("Ident doesn't exist");
                return register;
            }

            ExprKind::Binary { lhs, rhs, op } => {
                let lhs_r = self.compile_expr(lhs);
                let rhs_r = self.compile_expr(rhs);
                let dest = self.allocator.alloc();

                match &op {
                    Operator::Plus => self.emit(Instruction::Add(dest, lhs_r, rhs_r)),
                    Operator::Minus => self.emit(Instruction::Sub(dest, lhs_r, rhs_r)),
                    _ => unreachable!()
                }

                return dest;
            }

            _ => unimplemented!("compile expr"),
        }
    }
   
    fn compile_stmt(&mut self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::VariableDecl { mutable: _, name, value, typ: _ } => {
                let register = self.compile_expr(value);
                store(&mut self.symbols, name, register);
            }

            _ => unimplemented!("compile stmt"),
        }
    }
}