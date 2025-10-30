use crate::model::RegType;
use crate::model::RegWidth;

#[derive(Debug, PartialEq, Clone)]
pub enum RegisterSet {
    Single(RegType, RegWidth),
    Vector(RegType, RegWidth, RegWidth),
}

#[derive(Debug, PartialEq, Clone)]
pub struct RegOperand {
    pub set: RegisterSet,
    pub index: u32,
}

#[derive(Debug)]
pub enum Operand {
    Reg(RegOperand),
    UnsignedConstant(u64),
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
