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

#[derive(Debug, Clone, PartialEq)]
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
        params: CompareParams,
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

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryCondition {
    Equal,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
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

#[derive(Debug, PartialEq)]
pub enum ConsistentNumOp<RT: RegTypeT<R = NumReg>> {
    Single(Reg<RT>, RegOrConstant<RT>, RegOrConstant<RT>),
    VectorBroadcast(NumVecReg, RegIndex, RegOrConstant<RT>),
    VectorVector(NumVecReg, RegIndex, RegIndex),
}

impl<RT: RegTypeT<R = NumReg>> Clone for ConsistentNumOp<RT> {
    fn clone(&self) -> Self {
        match self {
            Self::Single(arg0, arg1, arg2) => {
                Self::Single(arg0.clone(), arg1.clone(), arg2.clone())
            }
            Self::VectorBroadcast(arg0, arg1, arg2) => {
                Self::VectorBroadcast(arg0.clone(), arg1.clone(), arg2.clone())
            }
            Self::VectorVector(arg0, arg1, arg2) => {
                Self::VectorVector(arg0.clone(), arg1.clone(), arg2.clone())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

    MemAddress(Reg<MemRegT>, Reg<MemRegT>, RegOrConstant<SignedRegT>),
    InstrAddress(
        Reg<InstrRegT>,
        RegOrConstant<InstrRegT>,
        RegOrConstant<SignedRegT>,
    ),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MovParams {
    UnsignedInt(Reg<UnsignedRegT>, RegOrConstant<UnsignedRegT>),
    SignedInt(Reg<SignedRegT>, RegOrConstant<SignedRegT>),
    Float(Reg<FloatRegT>, RegOrConstant<FloatRegT>),

    MemAddress(Reg<MemRegT>, Reg<MemRegT>),
    InstrAddress(Reg<InstrRegT>, RegOrConstant<InstrRegT>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotParams {
    UnsignedInt(Reg<UnsignedRegT>, RegOrConstant<UnsignedRegT>),
    SignedInt(Reg<SignedRegT>, RegOrConstant<SignedRegT>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompareParams {
    /// Unsigned integer register
    pub dst: Reg<UnsignedRegT>,
    /// The operands being compared
    pub args: ConsistentComparison,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConsistentComparison {
    UnsignedCompare(RegOrConstant<UnsignedRegT>, RegOrConstant<UnsignedRegT>),
    SignedCompare(RegOrConstant<SignedRegT>, RegOrConstant<SignedRegT>),
    FloatCompare(RegOrConstant<FloatRegT>, RegOrConstant<FloatRegT>),
    MemAddressCompare(Reg<MemRegT>, Reg<MemRegT>),
    InstrAddressCompare(RegOrConstant<InstrRegT>, RegOrConstant<InstrRegT>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompareToZero {
    Unsigned(RegOrConstant<UnsignedRegT>),
    Signed(RegOrConstant<SignedRegT>),
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
            Instruction::Compare { cond, params } => {
                let opcode = match cond {
                    BinaryCondition::Equal => "eq",
                    BinaryCondition::GreaterThan => "gt",
                    BinaryCondition::GreaterThanOrEqualTo => "ge",
                    BinaryCondition::LessThan => "lt",
                    BinaryCondition::LessThanOrEqualTo => "le",
                };
                write!(f, "{opcode} {params}")
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
            MovParams::UnsignedInt(reg, p) => write!(f, "{}, {}", reg, p),
            MovParams::SignedInt(reg, p) => write!(f, "{}, {}", reg, p),
            MovParams::Float(reg, p) => write!(f, "{}, {}", reg, p),
            MovParams::MemAddress(to, from) => write!(f, "{}, {}", to, from),
            MovParams::InstrAddress(to, p) => write!(f, "{}, {}", to, p),
        }
    }
}

impl Display for NotParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotParams::UnsignedInt(reg, reg_or_constant) => {
                write!(f, "{}, {}", reg, reg_or_constant)
            }
            NotParams::SignedInt(reg, reg_or_constant) => {
                write!(f, "{}, {}", reg, reg_or_constant)
            }
        }
    }
}

impl Display for CompareParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.dst, self.args)
    }
}

impl Display for ConsistentComparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::UnsignedCompare(p1, p2) => write!(f, "{p1}, {p2}"),
            Self::SignedCompare(p1, p2) => write!(f, "{p1}, {p2}"),
            Self::FloatCompare(p1, p2) => write!(f, "{p1}, {p2}"),
            Self::MemAddressCompare(i1, i2) => write!(f, "{i1}, {i2}"),
            Self::InstrAddressCompare(p1, p2) => write!(f, "{p1}, {p2}"),
        }
    }
}

impl Display for AnyCoherentNumOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn write_single<RT: RegTypeT>(
            f: &mut std::fmt::Formatter<'_>,
            dst: &Reg<RT>,
            p1: &RegOrConstant<RT>,
            p2: &RegOrConstant<RT>,
        ) -> std::fmt::Result
        where
            RegOrConstant<RT>: Display,
            Reg<RT>: Display,
        {
            write!(f, "{dst}, {p1}, {p2}")
        }

        fn write_broadcast<RT: RegTypeT>(
            f: &mut std::fmt::Formatter<'_>,
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
                RT::LETTER,
                dst.width,
                dst.length
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
                ConsistentNumOp::Single(dst, p1, p2) => write_single(f, dst, p1, p2),
                ConsistentNumOp::VectorBroadcast(dst, p1, p2) => write_broadcast(f, dst, *p1, p2),
                ConsistentNumOp::VectorVector(dst, p1, p2) => {
                    write_vec_vec(f, UnsignedRegT::LETTER, dst, *p1, *p2)
                }
            },
            AnyCoherentNumOp::SignedInt(num_op) => match num_op {
                ConsistentNumOp::Single(dst, p1, p2) => write_single(f, dst, p1, p2),
                ConsistentNumOp::VectorBroadcast(dst, p1, p2) => write_broadcast(f, dst, *p1, p2),
                ConsistentNumOp::VectorVector(dst, p1, p2) => {
                    write_vec_vec(f, SignedRegT::LETTER, dst, *p1, *p2)
                }
            },
            AnyCoherentNumOp::Float(num_op) => match num_op {
                ConsistentNumOp::Single(dst, p1, p2) => write_single(f, dst, p1, p2),
                ConsistentNumOp::VectorBroadcast(dst, p1, p2) => write_broadcast(f, dst, *p1, p2),
                ConsistentNumOp::VectorVector(dst, p1, p2) => {
                    write_vec_vec(f, FloatRegT::LETTER, dst, *p1, *p2)
                }
            },
        }
    }
}
