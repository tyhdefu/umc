use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::model::RegType;
use crate::model::RegisterSet;

#[derive(Debug)]
pub enum ParseError {
    RegErr(ParseRegError),
}

#[derive(Debug)]
pub enum ParseRegError {
    /// The width, count or register index is not an valid integer
    InvalidInt(ParseIntError),
    /// The structure of the string is wrong
    InvalidFormat,
    /// The type of register is not a known type
    InvalidRegisterType,
}

impl From<ParseIntError> for ParseRegError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidInt(value)
    }
}

impl FromStr for RegisterSet {
    type Err = ParseRegError;

    /// Parse a register set:
    /// - u32
    /// - i16
    /// - f64
    /// - f32x4
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseRegError::InvalidRegisterType);
        }
        if s.contains('x') {
            let (reg_type_str, count) = s.split_once('x').ok_or(ParseRegError::InvalidFormat)?;
            let reg_type: RegType = reg_type_str
                .parse()
                .map_err(|_| ParseRegError::InvalidRegisterType)?;
            let count: u32 = count.parse()?;
            Ok(RegisterSet::Vector(reg_type, count))
        } else {
            let reg_type: RegType = s.parse().map_err(|_| ParseRegError::InvalidRegisterType)?;
            Ok(RegisterSet::Single(reg_type))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ASTRegisterOperand {
    pub set: Option<RegisterSet>,
    pub index: u32,
}

impl FromStr for ASTRegisterOperand {
    type Err = ParseRegError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (reg_set, index) = s.split_once(':').ok_or(ParseRegError::InvalidFormat)?;
        let index: u32 = index.parse().map_err(ParseRegError::InvalidInt)?;

        if reg_set.is_empty() {
            return Ok(Self { set: None, index });
        }

        let reg_set: RegisterSet = reg_set.parse()?;
        Ok(Self {
            set: Some(reg_set),
            index,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Operand {
    Reg(ASTRegisterOperand),
    Constant(u64),
    Label(String),
}

pub type OperandWithLoc = (Operand, usize, RangeInclusive<usize>);

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<OperandWithLoc>,
    pub loc: RangeInclusive<usize>,
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub label: Option<String>,
    pub instr: Instruction,
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTRegisterOperand, RegType, RegisterSet};

    #[test]
    fn parse_reg_operand_inferred() {
        assert_eq!(
            ASTRegisterOperand {
                set: None,
                index: 2
            },
            ":2".parse().unwrap()
        )
    }

    #[test]
    fn parse_reg_operand_single() {
        assert_eq!(
            ASTRegisterOperand {
                set: Some(RegisterSet::Single(RegType::UnsignedInt(32))),
                index: 0
            },
            "u32:0".parse().unwrap()
        );
    }

    #[test]
    fn parse_reg_operand_vector() {
        assert_eq!(
            ASTRegisterOperand {
                set: Some(RegisterSet::Vector(RegType::Float(64), 4)),
                index: 0
            },
            "f64x4:0".parse().unwrap()
        )
    }
}
