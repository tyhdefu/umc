use std::fmt::Display;

use crate::model::RegisterSet;

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

#[derive(Debug)]
pub enum Operand {
    Reg(RegOperand),
    UnsignedConstant(u64),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(reg) => write!(f, "{}", reg),
            Operand::UnsignedConstant(c) => write!(f, "#{}", c), // TODO: Always format as hex?
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    /// Move the operand into the destination register
    Mov(RegOperand, Operand),
    /// Add the two operands and store in destination register
    Add(RegOperand, Operand, Operand),
    /// Print the given register (debugging)
    Dbg(RegOperand),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov(dst, op1) => write!(f, "mov {}, {}", dst, op1),
            Instruction::Add(dst, op1, op2) => write!(f, "add {}, {}, {}", dst, op1, op2),
            Instruction::Dbg(dst) => write!(f, "dbg {}", dst),
        }
    }
}
