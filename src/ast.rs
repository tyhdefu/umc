use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub enum ParseError {
    RegErr(ParseRegError),
}

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

#[derive(Debug, PartialEq)]
pub enum RegisterSet {
    Inferred,
    Single(RegType, RegWidth),
    Vector(RegType, RegWidth, u32),
}

impl FromStr for RegisterSet {
    type Err = ParseRegError;

    /// Parse a register set:
    /// - u32
    /// - i16
    /// - f64
    /// - vf32
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseRegError::InvalidRegisterType);
        }
        if s.starts_with('v') {
            let reg_type: RegType = s[1..2]
                .parse()
                .map_err(|_| ParseRegError::InvalidRegisterType)?;
            let (width, count) = s[2..].split_once('x').ok_or(ParseRegError::InvalidFormat)?;
            let width: RegWidth = width.parse()?;
            let count: u32 = count.parse()?;
            Ok(RegisterSet::Vector(reg_type, width, count))
        } else {
            let reg_type: RegType = s[0..1]
                .parse()
                .map_err(|_| ParseRegError::InvalidRegisterType)?;
            let width: RegWidth = s[1..].parse()?;
            Ok(RegisterSet::Single(reg_type, width))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RegisterOperand {
    pub set: RegisterSet,
    pub index: u32,
}

impl FromStr for RegisterOperand {
    type Err = ParseRegError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (reg_set, index) = s.split_once(':').ok_or(ParseRegError::InvalidFormat)?;
        let index: u32 = index.parse().map_err(ParseRegError::InvalidInt)?;

        if reg_set.is_empty() {
            return Ok(Self {
                set: RegisterSet::Inferred,
                index,
            });
        }

        let reg_set: RegisterSet = reg_set.parse()?;
        Ok(Self {
            set: reg_set,
            index,
        })
    }
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

#[cfg(test)]
mod tests {
    use crate::ast::{RegType, RegisterOperand, RegisterSet};

    #[test]
    fn parse_reg_operand_inferred() {
        assert_eq!(
            RegisterOperand {
                set: RegisterSet::Inferred,
                index: 2
            },
            ":2".parse().unwrap()
        )
    }

    #[test]
    fn parse_reg_operand_single() {
        assert_eq!(
            RegisterOperand {
                set: RegisterSet::Single(RegType::UnsignedInt, 32),
                index: 0
            },
            "u32:0".parse().unwrap()
        );
    }

    #[test]
    fn parse_reg_operand_vector() {
        assert_eq!(
            RegisterOperand {
                set: RegisterSet::Vector(RegType::Float, 64, 4),
                index: 0
            },
            "vf64x4:0".parse().unwrap()
        )
    }
}
