use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

use umc_model::RegType;
use umc_model::RegisterSet;

#[derive(Debug)]
pub enum ParseError {
    RegErr(ParseRegError, RangeInclusive<usize>),
    InvalidConstant(RangeInclusive<usize>),
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

/// Parse a register set:
/// - u32
/// - i16
/// - f64
/// - f32x4
fn parse_reg_set(s: &str) -> Result<RegisterSet, ParseRegError> {
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

        let reg_set: RegisterSet = parse_reg_set(reg_set)?;
        Ok(Self {
            set: Some(reg_set),
            index,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Reg(ASTRegisterOperand),
    UnsignedConstant(u64),
    NegativeConstant(i64),
    FloatConstant(f64),
    Label(String),
}

impl Operand {
    pub fn parse_constant(s: &str, range: RangeInclusive<usize>) -> Result<Self, ParseError> {
        if let Some(s) = s.strip_prefix('#') {
            if s.contains('.') {
                return f64::from_str(s)
                    .map(|c| Self::FloatConstant(c))
                    .map_err(|_| ParseError::InvalidConstant(range));
            } else if s.starts_with('-') {
                return i64::from_str(s)
                    .map(|c| Operand::NegativeConstant(c))
                    .map_err(|_| ParseError::InvalidConstant(range));
            } else {
                return u64::from_str(s)
                    .map(|c| Self::UnsignedConstant(c))
                    .map_err(|_| ParseError::InvalidConstant(range));
            }
        }
        if let Some(s) = s.strip_prefix("0b") {
            return u64::from_str_radix(s, 2)
                .map(|c| Self::UnsignedConstant(c))
                .map_err(|_| ParseError::InvalidConstant(range));
        }
        if let Some(s) = s.strip_prefix("0x") {
            return u64::from_str_radix(s, 16)
                .map(|c| Self::UnsignedConstant(c))
                .map_err(|_| ParseError::InvalidConstant(range));
        }
        Err(ParseError::InvalidConstant(range))
    }
}

pub type OperandWithLoc = (Operand, usize, RangeInclusive<usize>);

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<OperandWithLoc>,
    pub loc: RangeInclusive<usize>,
}

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub label: Option<(String, RangeInclusive<usize>)>,
    pub instr: Instruction,
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTRegisterOperand, RegType, RegisterSet};
    use umc_model::NumRegType;

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
                set: Some(RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(
                    32
                )))),
                index: 0
            },
            "u32:0".parse().unwrap()
        );
    }

    #[test]
    fn parse_reg_operand_vector() {
        assert_eq!(
            ASTRegisterOperand {
                set: Some(RegisterSet::Vector(RegType::Num(NumRegType::Float(64)), 4)),
                index: 0
            },
            "f64x4:0".parse().unwrap()
        )
    }
}
