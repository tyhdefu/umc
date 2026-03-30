use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;

use crate::NumRegType;
use crate::RegType;
use crate::RegisterSet;
use crate::format::DisplayAssembly;
use crate::format::DisplayAssemblyParams;
use crate::impl_display_delgate_raw;
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

/// Dummy zero-sized struct for when there is no width for a type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoWidth {}

pub trait RegTypeT: Clone {
    const LETTER: char;

    /// The type of width (may be non-applicable)
    type WIDTH: Debug + Clone + Eq + Hash + Display;
    type C: Debug + Clone + PartialEq;

    fn reg_type(r: &Self::WIDTH) -> RegType;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct UnsignedRegT;
impl RegTypeT for UnsignedRegT {
    const LETTER: char = 'u';
    type WIDTH = RegWidth;
    type C = u64;

    fn reg_type(width: &Self::WIDTH) -> RegType {
        RegType::Num(NumRegType::UnsignedInt(*width))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SignedRegT;
impl RegTypeT for SignedRegT {
    const LETTER: char = 'i';
    type WIDTH = RegWidth;
    type C = i64;

    fn reg_type(width: &Self::WIDTH) -> RegType {
        RegType::Num(NumRegType::SignedInt(*width))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FloatRegT;
impl RegTypeT for FloatRegT {
    const LETTER: char = 'f';
    type WIDTH = RegWidth;
    type C = f64;

    fn reg_type(width: &Self::WIDTH) -> RegType {
        RegType::Num(NumRegType::Float(*width))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MemRegT;
impl RegTypeT for MemRegT {
    const LETTER: char = 'm';
    type WIDTH = NoWidth;
    type C = u32;

    fn reg_type(_: &Self::WIDTH) -> RegType {
        RegType::MemoryAddress
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct InstrRegT;
impl RegTypeT for InstrRegT {
    const LETTER: char = 'n';
    type WIDTH = NoWidth;
    type C = usize;

    fn reg_type(_: &Self::WIDTH) -> RegType {
        RegType::InstructionAddress
    }
}

/// Type-safe Register
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Reg<RT: RegTypeT> {
    pub index: RegIndex,
    pub width: RT::WIDTH,
}

impl<RT: RegTypeT> Reg<RT> {
    /// Check if the two registers are equal ignoring their indices
    pub fn eq_ignoring_index(&self, other: &Reg<RT>) -> bool {
        self.width == other.width
    }
}

impl<RT: RegTypeT<WIDTH = NoWidth>> Reg<RT> {
    pub fn from_index(index: RegIndex) -> Self {
        Self {
            index,
            width: NoWidth {},
        }
    }
}

impl<RT: RegTypeT> Copy for Reg<RT> where RT::WIDTH: Copy {}

/// Type-safe Register or Constant Operand
#[derive(Debug, PartialEq, Clone)]
pub enum RegOrConstant<RT: RegTypeT> {
    Reg(Reg<RT>),
    Const(RT::C),
}

impl<RT: RegTypeT> RegOrConstant<RT> {
    /// Wrap type-safe register into a reg or constant
    pub fn from_reg(reg: Reg<RT>) -> Self {
        Self::Reg(reg)
    }
}

impl<RT: RegTypeT<WIDTH = NoWidth>> RegOrConstant<RT> {
    /// Wrap register value in type-safe wrapper
    pub fn reg(index: RegIndex) -> Self {
        Self::Reg(Reg {
            index,
            width: NoWidth {},
        })
    }
}

impl<RT: RegTypeT<WIDTH = RegWidth>> RegOrConstant<RT> {
    pub fn num_reg(num_reg: NumReg) -> Self {
        Self::Reg(Reg {
            index: num_reg.index,
            width: num_reg.width,
        })
    }
}

impl<RT: RegTypeT<WIDTH = RegWidth>> RegOrConstant<RT> {
    pub fn width(&self) -> Option<RegWidth> {
        match self {
            Self::Reg(r) => Some(r.width),
            Self::Const(_) => None,
        }
    }
}

impl<RT: RegTypeT> DisplayAssembly for Reg<RT>
where
    RT::WIDTH: Display,
{
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        write!(f, "{}{}:{}", RT::LETTER, self.width, self.index)
    }
}

impl<RT: RegTypeT> Display for Reg<RT>
where
    RT::WIDTH: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_assembly(f, &DisplayAssemblyParams::Raw)
    }
}

impl DisplayAssembly for RegOrConstant<UnsignedRegT> {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "{}", r),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}
impl_display_delgate_raw!(RegOrConstant<UnsignedRegT>);

impl DisplayAssembly for RegOrConstant<SignedRegT> {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "{}", r),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}
impl_display_delgate_raw!(RegOrConstant<SignedRegT>);

impl DisplayAssembly for RegOrConstant<FloatRegT> {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "{}", r),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}
impl_display_delgate_raw!(RegOrConstant<FloatRegT>);

impl DisplayAssembly for RegOrConstant<InstrRegT> {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(reg) => write!(f, "{}", reg),
            RegOrConstant::Const(c) => opts.fmt_instr_label(f, c),
        }
    }
}
impl_display_delgate_raw!(RegOrConstant<InstrRegT>);

impl DisplayAssembly for RegOrConstant<MemRegT> {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(reg) => write!(f, "{}", reg),
            RegOrConstant::Const(c) => opts.fmt_mem_label(f, c),
        }
    }
}
impl_display_delgate_raw!(RegOrConstant<MemRegT>);

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
                    return Ok(Self::num_reg(NumReg {
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
                    return Ok(Self::num_reg(NumReg {
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
                    return Ok(Self::num_reg(NumReg {
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
            Operand::MemLabelConstant(c) => Ok(Self::Const(*c as u32)),
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

impl Display for NoWidth {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
