use std::fmt::Debug;
use std::fmt::Display;

use crate::operand::{Operand, RegOperand};
use crate::{NumRegType, RegIndex, RegType, RegWidth, RegisterSet};

pub type MemReg = RegIndex;
pub type InstrReg = RegIndex;

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// No-op
    Nop,
    /// Move the operand into the destination register
    Mov(MovParams),

    /// Add the two operands and store in destination register
    Add(AnyCoherentNumOp),
    /// Subtract the second operand from the first register
    Sub(AnyCoherentNumOp),
    /// Bitwise AND
    And(AnyCoherentNumOp),

    /// Bitwise XOR
    Xor(AnyCoherentNumOp),
    /// Bitwise Logical NOT
    Not(NotParams),

    /// Comparison operation
    /// Stores 1 into destination if true else 0
    Compare {
        cond: BinaryCondition,
        dst: NumReg,
        args: ConsistentComparison,
    },

    /// Jump to the given location unconditionally
    Jmp(RegOrConstant<InstrRegT>),
    /// Conditionally branch to the given location (op1) if the second operand is zero
    Bz(RegOrConstant<InstrRegT>, CompareToZero),
    /// Conditionally branch to the given location (op1) if the second operand is not zero
    Bnz(RegOrConstant<InstrRegT>, CompareToZero),
    /// Print the given register (debugging)
    Dbg(RegOperand),
}

#[derive(Debug, PartialEq)]
pub enum BinaryCondition {
    Equal,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
}

pub trait RegTypeT {
    type R: Debug + Clone + PartialEq;
    type C: Debug + Clone + PartialEq;

    fn reg_type(r: &Self::R) -> RegType;
}

#[derive(Debug, PartialEq)]
pub struct UnsignedRegT;
impl RegTypeT for UnsignedRegT {
    type R = NumReg;
    type C = u64;

    fn reg_type(r: &Self::R) -> RegType {
        RegType::Num(NumRegType::UnsignedInt(r.width))
    }
}

#[derive(Debug, PartialEq)]
pub struct SignedRegT;
impl RegTypeT for SignedRegT {
    type R = NumReg;
    type C = i64;

    fn reg_type(r: &Self::R) -> RegType {
        RegType::Num(NumRegType::SignedInt(r.width))
    }
}

#[derive(Debug, PartialEq)]
pub struct FloatRegT;
impl RegTypeT for FloatRegT {
    type R = NumReg;
    type C = f64;

    fn reg_type(r: &Self::R) -> RegType {
        RegType::Num(NumRegType::Float(r.width))
    }
}

#[derive(Debug, PartialEq)]
pub struct MemRegT;
impl RegTypeT for MemRegT {
    type R = RegIndex;
    type C = usize;

    fn reg_type(_: &Self::R) -> RegType {
        RegType::MemoryAddress
    }
}

#[derive(Debug, PartialEq)]
pub struct InstrRegT;
impl RegTypeT for InstrRegT {
    type R = RegIndex;
    type C = usize;

    fn reg_type(_: &Self::R) -> RegType {
        RegType::InstructionAddress
    }
}

#[derive(Debug, PartialEq)]
pub enum RegOrConstant<RT: RegTypeT> {
    Reg(RT::R),
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

impl<RT: RegTypeT<R = NumReg>> RegOrConstant<RT> {
    pub fn width(&self) -> Option<RegWidth> {
        match self {
            Self::Reg(r) => Some(r.width),
            Self::Const(_) => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ConsistentNumOp<RT: RegTypeT<R = NumReg>> {
    Single(NumReg, RegOrConstant<RT>, RegOrConstant<RT>),
    VectorBroadcast(NumVecReg, RegIndex, RegOrConstant<RT>),
    VectorVector(NumVecReg, RegIndex, RegIndex),
}

#[derive(Debug, PartialEq)]
pub enum AnyCoherentNumOp {
    UnsignedInt(ConsistentNumOp<UnsignedRegT>),
    SignedInt(ConsistentNumOp<SignedRegT>),
    Float(ConsistentNumOp<FloatRegT>),
}

#[derive(Debug, PartialEq)]
pub enum AddParams {
    UnsignedInt(ConsistentNumOp<UnsignedRegT>),
    SignedInt(ConsistentNumOp<SignedRegT>),
    Float(ConsistentNumOp<FloatRegT>),

    MemAddress(MemReg, MemReg, RegOrConstant<SignedRegT>),
    InstrAddress(
        InstrReg,
        RegOrConstant<InstrRegT>,
        RegOrConstant<SignedRegT>,
    ),
}

#[derive(Debug, PartialEq)]
pub enum MovParams {
    UnsignedInt(NumReg, RegOrConstant<UnsignedRegT>),
    SignedInt(NumReg, RegOrConstant<SignedRegT>),
    Float(NumReg, RegOrConstant<FloatRegT>),

    MemAddress(MemReg, MemReg),
    InstrAddress(InstrReg, RegOrConstant<InstrRegT>),
}

#[derive(Debug, PartialEq)]
pub enum NotParams {
    UnsignedInt(NumReg, RegOrConstant<UnsignedRegT>),
    SignedInt(NumReg, RegOrConstant<SignedRegT>),
}

#[derive(Debug, PartialEq)]
pub enum ConsistentComparison {
    UnsignedCompare(RegOrConstant<UnsignedRegT>, RegOrConstant<UnsignedRegT>),
    SignedCompare(RegOrConstant<SignedRegT>, RegOrConstant<SignedRegT>),
    FloatCompare(RegOrConstant<FloatRegT>, RegOrConstant<FloatRegT>),
    MemAddressCompare(MemReg, MemReg),
    InstrAddressCompare(RegOrConstant<InstrRegT>, RegOrConstant<InstrRegT>),
}

#[derive(Debug, PartialEq)]
pub enum CompareToZero {
    Unsigned(RegOrConstant<UnsignedRegT>),
    Signed(RegOrConstant<SignedRegT>),
}

impl RegOrConstant<UnsignedRegT> {
    pub fn from_unsigned(op: &Operand) -> Result<Self, ()> {
        match op {
            Operand::Reg(reg) => {
                if let RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(width))) = reg.set {
                    return Ok(Self::Reg(NumReg {
                        index: reg.index,
                        width,
                    }));
                }
                return Err(());
            }
            Operand::UnsignedConstant(x) => Ok(Self::Const(*x)),
            _ => return Err(()),
        }
    }
}

impl RegOrConstant<SignedRegT> {
    pub fn from_signed(op: &Operand) -> Result<Self, ()> {
        match op {
            Operand::Reg(reg) => {
                if let RegisterSet::Single(RegType::Num(NumRegType::SignedInt(width))) = reg.set {
                    return Ok(Self::Reg(NumReg {
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

impl RegOrConstant<FloatRegT> {
    pub fn from_float(op: &Operand) -> Result<Self, ()> {
        match op {
            Operand::Reg(reg) => {
                if let RegisterSet::Single(RegType::Num(NumRegType::Float(width))) = reg.set {
                    return Ok(Self::Reg(NumReg {
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

impl RegOrConstant<MemRegT> {
    pub fn from_mem_addr(op: &Operand) -> Result<Self, ()> {
        match op {
            Operand::Reg(reg) => {
                if let RegisterSet::Single(RegType::MemoryAddress) = reg.set {
                    return Ok(Self::Reg(reg.index));
                }
                return Err(());
            }
            Operand::LabelConstant(l) => Ok(Self::Const(*l)),
            _ => Err(()),
        }
    }
}

impl RegOrConstant<InstrRegT> {
    pub fn from_instr_addr(op: &Operand) -> Result<Self, ()> {
        match op {
            Operand::Reg(reg) => {
                if let RegisterSet::Single(RegType::InstructionAddress) = reg.set {
                    return Ok(Self::Reg(reg.index));
                }
                Err(())
            }
            Operand::LabelConstant(l) => Ok(Self::Const(*l)),
            _ => Err(()),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop => write!(f, "nop"),
            Instruction::Mov(params) => write!(f, "mov {}", params),
            Instruction::Add(params) => write!(f, "add {}", params),
            Instruction::Sub(params) => write!(f, "sub {}", params),
            Instruction::And(params) => write!(f, "and {}", params),
            Instruction::Xor(params) => write!(f, "xor {}", params),
            Instruction::Not(params) => write!(f, "not {}", params),
            Instruction::Compare { cond, dst, args } => {
                let opcode = match cond {
                    BinaryCondition::Equal => "eq",
                    BinaryCondition::GreaterThan => "gt",
                    BinaryCondition::GreaterThanOrEqualTo => "ge",
                    BinaryCondition::LessThan => "lt",
                    BinaryCondition::LessThanOrEqualTo => "le",
                };
                write!(f, "{opcode} u{dst}, {args}")
            }
            Instruction::Jmp(reg_or_constant) => write!(f, "jmp {}", reg_or_constant),
            Instruction::Bz(reg_or_constant, compare_to_zero) => {
                write!(f, "bz {}, {}", reg_or_constant, compare_to_zero)
            }
            Instruction::Bnz(reg_or_constant, compare_to_zero) => {
                write!(f, "bnz {}, {}", reg_or_constant, compare_to_zero)
            }
            Instruction::Dbg(reg_operand) => write!(f, "dbg {}", reg_operand),
        }
    }
}

impl Display for NumReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.width, self.index)
    }
}

impl Display for NumVecReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}:{}", self.width, self.length, self.index)
    }
}

impl Display for RegOrConstant<UnsignedRegT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "u{}:{}", r.width, r.index),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}

impl Display for RegOrConstant<SignedRegT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "i{}:{}", r.width, r.index),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}

impl Display for RegOrConstant<FloatRegT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "f{}:{}", r.width, r.index),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}

impl Display for RegOrConstant<InstrRegT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(index) => write!(f, "n:{}", index),
            RegOrConstant::Const(c) => write!(f, "0x{}", c),
        }
    }
}

impl Display for CompareToZero {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompareToZero::Unsigned(x) => write!(f, "{}", x),
            CompareToZero::Signed(x) => write!(f, "{}", x),
        }
    }
}

impl Display for MovParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MovParams::UnsignedInt(num_reg, p) => write!(f, "u{}, {}", num_reg, p),
            MovParams::SignedInt(num_reg, p) => write!(f, "i{}, {}", num_reg, p),
            MovParams::Float(num_reg, p) => write!(f, "f{}, {}", num_reg, p),
            MovParams::MemAddress(to, from) => write!(f, "a:{}, a:{}", to, from),
            MovParams::InstrAddress(to, p) => write!(f, "n:{}, {}", to, p),
        }
    }
}

impl Display for NotParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotParams::UnsignedInt(num_reg, reg_or_constant) => {
                write!(f, "u{}, {}", num_reg, reg_or_constant)
            }
            NotParams::SignedInt(num_reg, reg_or_constant) => {
                write!(f, "i{}, {}", num_reg, reg_or_constant)
            }
        }
    }
}

impl Display for ConsistentComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::UnsignedCompare(p1, p2) => write!(f, "{p1}, {p2}"),
            Self::SignedCompare(p1, p2) => write!(f, "{p1}, {p2}"),
            Self::FloatCompare(p1, p2) => write!(f, "{p1}, {p2}"),
            Self::MemAddressCompare(i1, i2) => write!(f, "a:{i1}, a:{i2}"),
            Self::InstrAddressCompare(p1, p2) => write!(f, "{p1}, {p2}"),
        }
    }
}

impl Display for AnyCoherentNumOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn write_single<RT: RegTypeT>(
            f: &mut std::fmt::Formatter<'_>,
            c: char,
            dst: &NumReg,
            p1: &RegOrConstant<RT>,
            p2: &RegOrConstant<RT>,
        ) -> std::fmt::Result
        where
            RegOrConstant<RT>: Display,
        {
            write!(f, "{c}{dst}, {p1}, {p2}")
        }

        fn write_broadcast<RT: RegTypeT>(
            f: &mut std::fmt::Formatter<'_>,
            c: char,
            dst: &NumVecReg,
            p1: RegIndex,
            p2: &RegOrConstant<RT>,
        ) -> std::fmt::Result
        where
            RegOrConstant<RT>: Display,
        {
            write!(
                f,
                "{0}{dst}, {0}{1}x{2}:{p1}, {p2}",
                c, dst.width, dst.length
            )
        }

        fn write_vec_vec(
            f: &mut std::fmt::Formatter<'_>,
            c: char,
            dst: &NumVecReg,
            p1: RegIndex,
            p2: RegIndex,
        ) -> std::fmt::Result {
            write!(
                f,
                "{0}{dst}, {0}{1}x{2}:{p1}, {0}{1}x{2}:{p2}",
                c, dst.width, dst.length
            )
        }

        match self {
            AnyCoherentNumOp::UnsignedInt(num_op) => match num_op {
                ConsistentNumOp::Single(dst, p1, p2) => write_single(f, 'u', dst, p1, p2),
                ConsistentNumOp::VectorBroadcast(dst, p1, p2) => {
                    write_broadcast(f, 'u', dst, *p1, p2)
                }
                ConsistentNumOp::VectorVector(dst, p1, p2) => write_vec_vec(f, 'u', dst, *p1, *p2),
            },
            AnyCoherentNumOp::SignedInt(num_op) => match num_op {
                ConsistentNumOp::Single(dst, p1, p2) => write_single(f, 'i', dst, p1, p2),
                ConsistentNumOp::VectorBroadcast(dst, p1, p2) => {
                    write_broadcast(f, 'i', dst, *p1, p2)
                }
                ConsistentNumOp::VectorVector(dst, p1, p2) => write_vec_vec(f, 'u', dst, *p1, *p2),
            },
            AnyCoherentNumOp::Float(num_op) => match num_op {
                ConsistentNumOp::Single(dst, p1, p2) => write_single(f, 'f', dst, p1, p2),
                ConsistentNumOp::VectorBroadcast(dst, p1, p2) => {
                    write_broadcast(f, 'f', dst, *p1, p2)
                }
                ConsistentNumOp::VectorVector(dst, p1, p2) => write_vec_vec(f, 'f', dst, *p1, *p2),
            },
        }
    }
}
