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

impl NumReg {
    pub fn with_index(&self, i: RegIndex) -> Self {
        Self {
            index: i,
            width: self.width,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumVecReg {
    pub index: RegIndex,
    pub width: RegWidth,
    pub length: RegWidth,
}

pub trait RegTypeT: Clone {
    const LETTER: char;
    type R: Debug + Clone + PartialEq;
    type C: Debug + Clone + PartialEq;

    fn reg_type(r: &Self::R) -> RegType;

    fn index(r: &Self::R) -> RegIndex;

    fn with_index(reg_set: &Self::R, r: RegIndex) -> Self::R;
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnsignedRegT;
impl RegTypeT for UnsignedRegT {
    const LETTER: char = 'u';
    type R = NumReg;
    type C = u64;

    fn reg_type(r: &Self::R) -> RegType {
        RegType::Num(NumRegType::UnsignedInt(r.width))
    }

    fn index(r: &Self::R) -> RegIndex {
        r.index
    }

    fn with_index(reg_set: &Self::R, i: RegIndex) -> Self::R {
        reg_set.with_index(i)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SignedRegT;
impl RegTypeT for SignedRegT {
    const LETTER: char = 'i';
    type R = NumReg;
    type C = i64;

    fn reg_type(r: &Self::R) -> RegType {
        RegType::Num(NumRegType::SignedInt(r.width))
    }

    fn index(r: &Self::R) -> RegIndex {
        r.index
    }

    fn with_index(reg_set: &Self::R, i: RegIndex) -> Self::R {
        reg_set.with_index(i)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FloatRegT;
impl RegTypeT for FloatRegT {
    const LETTER: char = 'f';
    type R = NumReg;
    type C = f64;

    fn reg_type(r: &Self::R) -> RegType {
        RegType::Num(NumRegType::Float(r.width))
    }

    fn index(r: &Self::R) -> RegIndex {
        r.index
    }

    fn with_index(reg_set: &Self::R, i: RegIndex) -> Self::R {
        reg_set.with_index(i)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemRegT;
impl RegTypeT for MemRegT {
    const LETTER: char = 'm';
    type R = RegIndex;
    type C = std::convert::Infallible;

    fn reg_type(_: &Self::R) -> RegType {
        RegType::MemoryAddress
    }

    fn index(r: &Self::R) -> RegIndex {
        *r
    }

    fn with_index(_: &Self::R, i: RegIndex) -> Self::R {
        i
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InstrRegT;
impl RegTypeT for InstrRegT {
    const LETTER: char = 'n';
    type R = RegIndex;
    type C = usize;

    fn reg_type(_: &Self::R) -> RegType {
        RegType::InstructionAddress
    }

    fn index(r: &Self::R) -> RegIndex {
        *r
    }

    fn with_index(_: &Self::R, i: RegIndex) -> Self::R {
        i
    }
}

/// Type-safe Register
#[derive(Debug, PartialEq, Clone)]
pub struct Reg<RT: RegTypeT>(pub RT::R);

impl<RT: RegTypeT> Reg<RT> {
    pub fn index(&self) -> RegIndex {
        RT::index(&self.0)
    }

    pub fn with_index(&self, i: RegIndex) -> Self {
        Reg(RT::with_index(&self.0, i))
    }

    /// Check if the two registers are equal ignoring their indices
    pub fn eq_ignoring_index(&self, other: &Reg<RT>) -> bool {
        const DUMMY_INDEX: RegIndex = 0;
        RT::with_index(&self.0, DUMMY_INDEX) == RT::with_index(&other.0, DUMMY_INDEX)
    }
}

impl<RT: RegTypeT> Copy for Reg<RT> where RT::R: Copy {}

/// Type-safe Register or Constant Operand
#[derive(Debug, PartialEq, Clone)]
pub enum RegOrConstant<RT: RegTypeT> {
    Reg(Reg<RT>),
    Const(RT::C),
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
