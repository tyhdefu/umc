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
    LabelConstant(usize),
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(reg) => write!(f, "{}", reg),
            Operand::UnsignedConstant(c) => write!(f, "{:#X}", c),
            Operand::LabelConstant(c) => write!(f, "{:#X}", c),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    /// Move the operand into the destination register
    Mov(RegOperand, Operand),
    /// Add the two operands and store in destination register
    Add(RegOperand, Operand, Operand),
    /// Subtract the second operand from the first register
    Sub(RegOperand, Operand, Operand),
    /// Bitwise AND
    And(RegOperand, Operand, Operand),
    /// Bitwise XOR
    Xor(RegOperand, Operand, Operand),
    /// Bitwise Logical NOT
    Not(RegOperand, Operand),
    /// Jump to the given location unconditionally
    Jmp(Operand),
    /// Conditionally branch to the given location (op1) if the second operand is zero
    Bz(Operand, Operand),
    /// Conditionally branch to the given location (op1) if the second operand is not zero
    Bnz(Operand, Operand),
    /// Print the given register (debugging)
    Dbg(RegOperand),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov(dst, op1) => write!(f, "mov {}, {}", dst, op1),
            Instruction::Add(dst, op1, op2) => write!(f, "add {}, {}, {}", dst, op1, op2),
            Instruction::Sub(dst, op1, op2) => write!(f, "sub {}, {}, {}", dst, op1, op2),
            Instruction::And(dst, op1, op2) => write!(f, "and {}, {}, {}", dst, op1, op2),
            Instruction::Xor(dst, op1, op2) => write!(f, "xor {}, {}, {}", dst, op1, op2),
            Instruction::Jmp(op1) => write!(f, "jmp {}", op1),
            Instruction::Bz(op1, op2) => write!(f, "bz {}, {}", op1, op2),
            Instruction::Bnz(op1, op2) => write!(f, "bnz {}, {}", op1, op2),
            Instruction::Not(dst, op1) => write!(f, "not {},{}", dst, op1),
            Instruction::Dbg(dst) => write!(f, "dbg {}", dst),
        }
    }
}
