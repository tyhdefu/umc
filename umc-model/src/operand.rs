use std::fmt::Display;

use crate::RegisterSet;

#[derive(Debug, PartialEq, Clone)]
pub struct RegOperand {
    pub set: RegisterSet,
    pub index: u32,
}

impl Display for RegOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.set, self.index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Reg(RegOperand),
    UnsignedConstant(u64),
    SignedConstant(i64),
    FloatConstant(f64),
    LabelConstant(usize),
    MemLabelConstant(usize),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(reg) => write!(f, "{}", reg),
            Operand::UnsignedConstant(c) => write!(f, "{:#X}", c),
            Operand::SignedConstant(c) => write!(f, "{:#X}", c),
            Operand::FloatConstant(c) => write!(f, "{:}", c),
            Operand::LabelConstant(c) => write!(f, "{:#X}", c),
            Operand::MemLabelConstant(c) => write!(f, "{:#X}", c),
        }
    }
}
