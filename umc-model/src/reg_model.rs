use std::fmt::Debug;
use std::fmt::Display;

use crate::NumRegType;
use crate::RegType;
use crate::RegisterSet;
use crate::operand::Operand;
use crate::{RegIndex, RegWidth};

pub type MemReg = RegIndex;
pub type InstrReg = RegIndex;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct NumReg {
    pub index: RegIndex,
    pub width: RegWidth,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumVecReg {
    pub index: RegIndex,
    pub width: RegWidth,
    pub length: RegWidth,
}

pub trait RegTypeT {
    const LETTER: char;
    type R: Debug + Clone + PartialEq;
    type C: Debug + Clone + PartialEq;

    fn reg_type(r: &Self::R) -> RegType;
}

#[derive(Debug, PartialEq)]
pub struct UnsignedRegT;
impl RegTypeT for UnsignedRegT {
    const LETTER: char = 'u';
    type R = NumReg;
    type C = u64;

    fn reg_type(r: &Self::R) -> RegType {
        RegType::Num(NumRegType::UnsignedInt(r.width))
    }
}

#[derive(Debug, PartialEq)]
pub struct SignedRegT;
impl RegTypeT for SignedRegT {
    const LETTER: char = 'i';
    type R = NumReg;
    type C = i64;

    fn reg_type(r: &Self::R) -> RegType {
        RegType::Num(NumRegType::SignedInt(r.width))
    }
}

#[derive(Debug, PartialEq)]
pub struct FloatRegT;
impl RegTypeT for FloatRegT {
    const LETTER: char = 'f';
    type R = NumReg;
    type C = f64;

    fn reg_type(r: &Self::R) -> RegType {
        RegType::Num(NumRegType::Float(r.width))
    }
}

#[derive(Debug, PartialEq)]
pub struct MemRegT;
impl RegTypeT for MemRegT {
    const LETTER: char = 'm';
    type R = RegIndex;
    type C = usize;

    fn reg_type(_: &Self::R) -> RegType {
        RegType::MemoryAddress
    }
}

#[derive(Debug, PartialEq)]
pub struct InstrRegT;
impl RegTypeT for InstrRegT {
    const LETTER: char = 'n';
    type R = RegIndex;
    type C = usize;

    fn reg_type(_: &Self::R) -> RegType {
        RegType::InstructionAddress
    }
}

/// Type-safe Register
#[derive(Debug, PartialEq)]
pub struct Reg<RT: RegTypeT>(pub RT::R);

impl<RT: RegTypeT> Copy for Reg<RT> where RT::R: Copy {}

impl<RT: RegTypeT> Clone for Reg<RT> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// Type-safe Register or Constant Operand
#[derive(Debug, PartialEq)]
pub enum RegOrConstant<RT: RegTypeT> {
    Reg(Reg<RT>),
    Const(RT::C),
}

impl<RT: RegTypeT> Clone for RegOrConstant<RT> {
    fn clone(&self) -> Self {
        match self {
            Self::Reg(r) => Self::Reg(r.clone()),
            Self::Const(c) => Self::Const(c.clone()),
        }
    }
}

impl<RT: RegTypeT> RegOrConstant<RT> {
    /// Wrap register value in type-safe wrapper
    pub fn reg(reg: RT::R) -> Self {
        Self::Reg(Reg(reg))
    }
}

impl<RT: RegTypeT<R = NumReg>> RegOrConstant<RT> {
    pub fn width(&self) -> Option<RegWidth> {
        match self {
            Self::Reg(r) => Some(r.0.width),
            Self::Const(_) => None,
        }
    }
}

impl<RT: RegTypeT> Display for Reg<RT>
where
    RT::R: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", RT::LETTER, self.0)
    }
}

impl Display for RegOrConstant<UnsignedRegT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "{}", r),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}

impl Display for RegOrConstant<SignedRegT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "{}", r),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}

impl Display for RegOrConstant<FloatRegT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "{}", r),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}

impl Display for RegOrConstant<InstrRegT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(reg) => write!(f, "{}", reg),
            RegOrConstant::Const(c) => write!(f, "0x{}", c),
        }
    }
}

impl Reg<UnsignedRegT> {
    pub fn from_unsigned(op: &Operand) -> Result<Self, ()> {
        match RegOrConstant::from_unsigned(op)? {
            RegOrConstant::Reg(reg) => Ok(reg),
            RegOrConstant::Const(_) => Err(()),
        }
    }
}

impl RegOrConstant<UnsignedRegT> {
    pub fn from_unsigned(op: &Operand) -> Result<Self, ()> {
        match op {
            Operand::Reg(reg) => {
                if let RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(width))) = reg.set {
                    return Ok(RegOrConstant::reg(NumReg {
                        index: reg.index,
                        width,
                    }));
                }
                return Err(());
            }
            Operand::UnsignedConstant(c) => Ok(RegOrConstant::Const(*c)),
            _ => Err(()),
        }
    }
}

impl Reg<SignedRegT> {
    pub fn from_signed(op: &Operand) -> Result<Self, ()> {
        match RegOrConstant::from_signed(op)? {
            RegOrConstant::Reg(reg) => Ok(reg),
            RegOrConstant::Const(_) => Err(()),
        }
    }
}

impl RegOrConstant<SignedRegT> {
    pub fn from_signed(op: &Operand) -> Result<Self, ()> {
        match op {
            Operand::Reg(reg) => {
                if let RegisterSet::Single(RegType::Num(NumRegType::SignedInt(width))) = reg.set {
                    return Ok(Self::reg(NumReg {
                        index: reg.index,
                        width,
                    }));
                }
                return Err(());
            }
            Operand::SignedConstant(x) => Ok(Self::Const(*x)),
            Operand::UnsignedConstant(x) => Ok(Self::Const(x.clone().try_into().map_err(|_| ())?)),
            _ => return Err(()),
        }
    }
}

impl Reg<FloatRegT> {
    pub fn from_float(op: &Operand) -> Result<Self, ()> {
        match RegOrConstant::from_float(op)? {
            RegOrConstant::Reg(reg) => Ok(reg),
            RegOrConstant::Const(_) => Err(()),
        }
    }
}

impl RegOrConstant<FloatRegT> {
    pub fn from_float(op: &Operand) -> Result<Self, ()> {
        match op {
            Operand::Reg(reg) => {
                if let RegisterSet::Single(RegType::Num(NumRegType::Float(width))) = reg.set {
                    return Ok(Self::reg(NumReg {
                        index: reg.index,
                        width,
                    }));
                }
                return Err(());
            }
            Operand::FloatConstant(x) => Ok(Self::Const(*x)),
            _ => return Err(()),
        }
    }
}

impl Reg<MemRegT> {
    pub fn from_mem_reg(op: &Operand) -> Result<Self, ()> {
        match RegOrConstant::from_mem_addr(op)? {
            RegOrConstant::Reg(reg) => Ok(reg),
            RegOrConstant::Const(_) => Err(()),
        }
    }
}

impl RegOrConstant<MemRegT> {
    pub fn from_mem_addr(op: &Operand) -> Result<Self, ()> {
        match op {
            Operand::Reg(reg) => {
                if let RegisterSet::Single(RegType::MemoryAddress) = reg.set {
                    return Ok(Self::reg(reg.index));
                }
                return Err(());
            }
            Operand::LabelConstant(l) => Ok(Self::Const(*l)),
            _ => Err(()),
        }
    }
}

impl Reg<InstrRegT> {
    pub fn from_instr_addr(op: &Operand) -> Result<Self, ()> {
        match RegOrConstant::from_instr_addr(op)? {
            RegOrConstant::Reg(reg) => Ok(reg),
            RegOrConstant::Const(_) => Err(()),
        }
    }
}

impl RegOrConstant<InstrRegT> {
    pub fn from_instr_addr(op: &Operand) -> Result<Self, ()> {
        match op {
            Operand::Reg(reg) => {
                if let RegisterSet::Single(RegType::InstructionAddress) = reg.set {
                    return Ok(Self::reg(reg.index));
                }
                Err(())
            }
            Operand::LabelConstant(l) => Ok(Self::Const(*l)),
            _ => Err(()),
        }
    }
}
