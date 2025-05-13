pub(in crate::compile) enum Instruction {
    /// `LOAD dest val`
    LoadInt(usize, i32),

    /// `ADD dest lhs rhs`
    Add(usize, usize, usize),
    
    /// `ADD dest lhs rhs`
    Sub(usize, usize, usize),
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Self::LoadInt(r, v) => format!("LDI R{} {}\n", r, v),
            Self::Add(dest, l, r) => format!("ADD R{} R{} R{}\n", dest, l, r),
            Self::Sub(dest, l, r) => format!("SUB R{} R{} R{}\n", dest, l, r),
        }
    }
}