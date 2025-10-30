//! Shared between AST and Bytecode
use std::str::FromStr;

pub type RegWidth = u32;
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
