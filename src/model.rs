//! Shared between AST and Bytecode
use std::{
    fmt::{Display, Write},
    str::FromStr,
};

/// The type used for how large a register can be
pub type RegWidth = u32;
/// The type used for the index of a register set
pub type RegIndex = u32;

#[derive(Debug, PartialEq, Clone)]
pub enum RegType {
    SignedInt(RegWidth),
    UnsignedInt(RegWidth),
    Float(RegWidth),
    Address,
}

impl FromStr for RegType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(());
        }
        if s.starts_with("a") {
            if s == "a" {
                return Ok(Self::Address);
            }
            return Err(());
        }
        let w: RegWidth = s[1..].parse().map_err(|_| ())?;
        Ok(match &s[0..1] {
            "i" => Self::SignedInt(w),
            "u" => Self::UnsignedInt(w),
            "f" => Self::Float(w),
            _ => return Err(()),
        })
    }
}

impl Display for RegType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Address => f.write_char('a'),
            Self::SignedInt(w) => write!(f, "i{}", w),
            Self::UnsignedInt(w) => write!(f, "u{}", w),
            Self::Float(w) => write!(f, "f{}", w),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RegisterSet {
    Single(RegType),
    Vector(RegType, RegWidth),
}

impl Display for RegisterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(reg_type) => write!(f, "{}", reg_type),
            Self::Vector(reg_type, l) => write!(f, "{}x{}", reg_type, l),
        }
    }
}
