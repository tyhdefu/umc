//! Shared between AST and Bytecode
use std::{fmt::Display, str::FromStr};

/// The type used for how large a register can be
pub type RegWidth = u32;
/// The type used for the index of a register set
pub type RegIndex = u32;

#[derive(Debug, PartialEq, Clone)]
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

impl Display for RegType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            RegType::SignedInt => 'i',
            RegType::UnsignedInt => 'u',
            RegType::Float => 'f',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RegisterSet {
    Single(RegType, RegWidth),
    Vector(RegType, RegWidth, RegWidth),
}

impl Display for RegisterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(reg_type, w) => write!(f, "{}{}", reg_type, w),
            Self::Vector(reg_type, w, l) => write!(f, "{}{}x{}", reg_type, w, l),
        }
    }
}
