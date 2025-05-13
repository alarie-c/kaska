use std::fmt::Display;
use std::fs::File;
use std::io::{self, Write};

use crate::ast::expr::{Expr, ExprKind, Operator};
use crate::ast::stmt::{Stmt, StmtKind};

pub enum Instruction {
    /// `LDI dest val`
    LoadInteger(u8, i32),
    
    /// `ADD lhs rhs dest`
    Add(u8, u8, u8),
    
    /// `SUB lhs rhs dest`
    Sub(u8, u8, u8),
    
    /// `MUL lhs rhs dest`
    Mul(u8, u8, u8),
    
    /// `DIV lhs rhs dest`
    Div(u8, u8, u8),

    /// PRINT src
    Print(u8),
}

impl Instruction {
    pub(in crate::compiler) fn encode_expr(expr: &Expr) -> Vec<Instruction> {
        match &expr.kind {
            ExprKind::Binary { lhs, rhs, op } => {
                let load_lhs = match lhs.kind {
                    ExprKind::Integer { value } => Self::LoadInteger(0, value),
                    _ => unimplemented!("load nonint binary"),
                };

                let load_rhs = match rhs.kind {
                    ExprKind::Integer { value } => Self::LoadInteger(1, value),
                    _ => unimplemented!("load nonint binary"),
                };

                match op {
                    Operator::Plus => return vec![load_lhs, load_rhs, Self::Add(0, 1, 0)],
                    Operator::Minus => return vec![load_lhs, load_rhs, Self::Sub(0, 1, 0)],
                    _ => unimplemented!("operator"),
                }
            }

            _ => unimplemented!("encode expr"),
        }
    }

    pub(in crate::compiler) fn encode_stmt(stmt: &Stmt) -> Vec<Instruction> {
        match &stmt.kind {
            StmtKind::VariableDecl { mutable: _, name: _, value, typ: _ } => {
                let load_value = Self::encode_expr(&value);
                return load_value;
            }
            StmtKind::Expression { expr } => return Self::encode_expr(expr),
            _ => unimplemented!("encode stmt")
        }
    }
}

impl Instruction {
    pub(in crate::compiler) fn to_string(&self) -> String {
        match self {
            Self::LoadInteger(a, v) => format!("LDI R{} {}\n", a, v),
            Self::Add(lhs, rhs, dest) => format!("ADD R{} R{} R{}\n", lhs, rhs, dest),
            Self::Sub(lhs, rhs, dest) => format!("SUB R{} R{} R{}\n", lhs, rhs, dest),
            Self::Mul(lhs, rhs, dest) => format!("MUL R{} R{} R{}\n", lhs, rhs, dest),
            Self::Div(lhs, rhs, dest) => format!("DIV R{} R{} R{}\n", lhs, rhs, dest),
            Self::Print(src) => format!("PRINT R{}\n", src),
        }
    }
}

pub struct Compiler<'a> {
    stream: &'a Vec<Stmt>,
}

impl<'a> Compiler<'a> {
    pub fn compile_file(src_name: &String, stream: &'a Vec<Stmt>) -> io::Result<File> {
        let compiler = Self { stream };
        
        let mut file = File::create(
            format!("{}.ksb", src_name).as_str()
        )?;

        for stmt in compiler.stream {
            let insts = Instruction::encode_stmt(stmt);
            for i in insts {
                let string = i.to_string();
                file.write_all(string.as_bytes())?;
            }
        }

        file.write_all("EXIT".as_bytes())?;

        return Ok(file);
    }
}