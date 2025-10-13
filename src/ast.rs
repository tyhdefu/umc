use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum RegType {
    SignedInt,
    UnsignedInt,
    Float,
}

impl FromStr for RegType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "i" => Ok(Self::SignedInt),
            "u" => Ok(Self::UnsignedInt),
            "f" => Ok(Self::Float),
            _ => Err(()),
        }
    }
}

type RegWidth = u32;

#[derive(Debug, PartialEq)]
pub enum RegisterSet {
    Inferred,
    Single(RegType, RegWidth),
    Vector(RegType, RegWidth, u32),
}

#[derive(Debug, PartialEq)]
pub struct RegisterOperand {
    pub set: RegisterSet,
    pub index: u32,
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Reg(RegisterOperand),
    Constant(u64),
    Label(String),
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<Operand>,
}
