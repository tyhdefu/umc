//! Shared between AST and Bytecode

pub mod binary;
pub mod instructions;
pub mod operand;
pub mod parse;
pub mod reg_model;
pub mod unparse;

use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone)]
pub struct Program {
    pub instructions: Vec<instructions::Instruction>,
    /// List of pre-initialised memory
    pub pre_init_mem: Vec<Vec<u8>>,

    /// Labels to identify each pre-initialised memory block
    pub mem_labels: HashMap<String, usize>,
    /// Labels for instructions
    pub instr_labels: HashMap<String, usize>,
}

impl Program {
    /// Create a basic program from just instructions
    pub fn from_instrs(instructions: Vec<instructions::Instruction>) -> Self {
        Self {
            instructions,
            pre_init_mem: vec![],
            mem_labels: HashMap::new(),
            instr_labels: HashMap::new(),
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.instructions {
            write!(f, "{}\n", i)?;
        }
        Ok(())
    }
}

/// The type used for how large a register can be
pub type RegWidth = u32;
/// The type used for the index of a register set
pub type RegIndex = u32;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum RegType {
    Num(NumRegType),
    InstructionAddress,
    MemoryAddress,
}

impl FromStr for RegType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        match s {
            "m" => return Ok(Self::MemoryAddress),
            "n" => return Ok(Self::InstructionAddress),
            _ => {}
        }
        NumRegType::from_str(s).map(|n| Self::Num(n))
    }
}

impl Display for RegType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegType::Num(num_reg) => write!(f, "{}", num_reg),
            RegType::InstructionAddress => write!(f, "n"),
            RegType::MemoryAddress => write!(f, "m"),
        }
    }
}

/// Number Register
/// The values from these
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum NumRegType {
    UnsignedInt(RegWidth),
    SignedInt(RegWidth),
    Float(RegWidth),
}

impl FromStr for NumRegType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let w: RegWidth = s[1..].parse().map_err(|_| ())?;
        Ok(match &s[0..1] {
            "i" => Self::SignedInt(w),
            "u" => Self::UnsignedInt(w),
            "f" => Self::Float(w),
            _ => return Err(()),
        })
    }
}

impl TryFrom<RegType> for NumRegType {
    type Error = ();

    fn try_from(value: RegType) -> Result<Self, Self::Error> {
        match value {
            RegType::Num(num) => Ok(num),
            _ => Err(()),
        }
    }
}

impl Display for NumRegType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SignedInt(w) => write!(f, "i{}", w),
            Self::UnsignedInt(w) => write!(f, "u{}", w),
            Self::Float(w) => write!(f, "f{}", w),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum RegisterSet {
    Single(RegType),
    Vector(RegType, RegWidth),
}

impl RegisterSet {
    pub fn single_num(t: NumRegType) -> Self {
        Self::Single(RegType::Num(t))
    }
}

impl Display for RegisterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(reg_type) => write!(f, "{}", reg_type),
            Self::Vector(reg_type, l) => write!(f, "{}x{}", reg_type, l),
        }
    }
}
