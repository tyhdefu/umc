use std::fmt::Display;

use crate::bytecode::{Operand, RegOperand};
use crate::model::{NumRegType, RegIndex, RegType, RegWidth, RegisterSet};

pub type MemReg = RegIndex;
pub type InstrReg = RegIndex;

#[derive(Debug, Clone)]
pub struct NumReg {
    pub index: RegIndex,
    pub width: RegWidth,
}

#[derive(Debug, Clone)]
pub struct NumVecReg {
    pub index: RegIndex,
    pub width: RegWidth,
    pub length: RegWidth,
}

pub enum InstructionValidateError {
    InvalidOpCount {
        expected: usize,
        got: usize,
    },
    ExpectedDstReg,
    CannotInferReg {
        op_index: usize,
    },
    InvalidRegType {
        op_index: usize,
    },
    InconsistentOperand {
        op_index: usize,
    },
    /// Operand inconsistent because width narrowing is not allowed implicitly
    CannotNarrowWidth {
        op_index: usize,
    },
}

#[derive(Clone, Debug)]
pub enum RegOrConstant<R, C> {
    Reg(R),
    Const(C),
}

impl<C> RegOrConstant<NumReg, C> {
    pub fn width(&self) -> Option<RegWidth> {
        match self {
            Self::Reg(r) => Some(r.width),
            Self::Const(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum IntReg {
    Signed(NumReg),
    Unsigned(NumReg),
}

#[derive(Debug)]
pub enum ConsistentNumOp<C> {
    Single(NumReg, RegOrConstant<NumReg, C>, RegOrConstant<NumReg, C>),
    VectorBroadcast(NumVecReg, RegIndex, RegOrConstant<NumReg, C>),
    VectorVector(NumVecReg, RegIndex, RegIndex),
}

#[derive(Debug)]
pub enum AnyCoherentNumOp {
    UnsignedInt(ConsistentNumOp<u64>),
    SignedInt(ConsistentNumOp<i64>),
    Float(ConsistentNumOp<f64>),
}

#[derive(Debug)]
pub enum AddParams {
    UnsignedInt(ConsistentNumOp<u64>),
    SignedInt(ConsistentNumOp<i64>),
    Float(ConsistentNumOp<f64>),

    MemAddress(MemReg, MemReg, RegOrConstant<IntReg, i64>),
    InstrAddress(
        InstrReg,
        RegOrConstant<InstrReg, usize>,
        RegOrConstant<IntReg, i64>,
    ),
}

#[derive(Debug)]
pub enum MovParams {
    UnsignedInt(NumReg, RegOrConstant<NumReg, u64>),
    SignedInt(NumReg, RegOrConstant<NumReg, i64>),
    Float(NumReg, RegOrConstant<NumReg, f64>),

    MemAddress(MemReg, MemReg),
    InstrAddress(InstrReg, RegOrConstant<InstrReg, usize>),
}

#[derive(Debug)]
pub enum NotParams {
    UnsignedInt(NumReg, RegOrConstant<NumReg, u64>),
    SignedInt(NumReg, RegOrConstant<NumReg, i64>),
}

#[derive(Debug)]
pub struct ComparisonParams {
    /// Unsigned integer register to store 1 or 0 in dependending on the comparison result
    dst: NumReg,
    args: ConsistentComparison,
}

#[derive(Debug)]
pub enum ConsistentComparison {
    UnsignedCompare(RegOrConstant<NumReg, u64>, RegOrConstant<NumReg, u64>),
    SignedCompare(RegOrConstant<NumReg, i64>, RegOrConstant<NumReg, i64>),
    FloatCompare(RegOrConstant<NumReg, f64>, RegOrConstant<NumReg, f64>),
    MemAddressCompare(MemReg, MemReg),
    InstrAddressCompare(
        RegOrConstant<InstrReg, usize>,
        RegOrConstant<InstrReg, usize>,
    ),
}

#[derive(Debug)]
pub enum CompareToZero {
    Unsigned(RegOrConstant<NumReg, u64>),
    Signed(RegOrConstant<NumReg, i64>),
}

impl RegOrConstant<NumReg, u64> {
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

impl RegOrConstant<NumReg, i64> {
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

impl RegOrConstant<NumReg, f64> {
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

impl RegOrConstant<MemReg, usize> {
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

impl RegOrConstant<InstrReg, usize> {
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

impl RegOrConstant<IntReg, i64> {
    pub fn from_int(op: &Operand) -> Result<Self, ()> {
        match op {
            Operand::Reg(reg) => Ok(match reg.set {
                RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(width))) => {
                    Self::Reg(IntReg::Unsigned(NumReg {
                        index: reg.index,
                        width,
                    }))
                }
                RegisterSet::Single(RegType::Num(NumRegType::SignedInt(width))) => {
                    Self::Reg(IntReg::Signed(NumReg {
                        index: reg.index,
                        width,
                    }))
                }
                _ => return Err(()),
            }),
            Operand::UnsignedConstant(c) => Ok(Self::Const(*c as i64)),
            Operand::SignedConstant(c) => Ok(Self::Const(*c)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
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
    /// Jump to the given location unconditionally
    Jmp(RegOrConstant<InstrReg, usize>),
    /// Conditionally branch to the given location (op1) if the second operand is zero
    Bz(RegOrConstant<InstrReg, usize>, CompareToZero),
    /// Conditionally branch to the given location (op1) if the second operand is not zero
    Bnz(RegOrConstant<InstrReg, usize>, CompareToZero),
    /// Print the given register (debugging)
    Dbg(RegOperand),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov(params) => write!(f, "mov {}", params),
            Instruction::Add(params) => write!(f, "add {}", params),
            Instruction::Sub(params) => write!(f, "sub {}", params),
            Instruction::And(params) => write!(f, "and {}", params),
            Instruction::Xor(params) => write!(f, "xor {}", params),
            Instruction::Not(params) => write!(f, "not {}", params),
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

impl Display for RegOrConstant<NumReg, u64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "u{}:{}", r.width, r.index),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}

impl Display for RegOrConstant<NumReg, i64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "i{}:{}", r.width, r.index),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}

impl Display for RegOrConstant<NumReg, f64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegOrConstant::Reg(r) => write!(f, "f{}:{}", r.width, r.index),
            RegOrConstant::Const(c) => write!(f, "#{}", c),
        }
    }
}

impl Display for RegOrConstant<RegIndex, usize> {
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

impl Display for AnyCoherentNumOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn write_single<R, C>(
            f: &mut std::fmt::Formatter<'_>,
            c: char,
            dst: &NumReg,
            p1: &RegOrConstant<R, C>,
            p2: &RegOrConstant<R, C>,
        ) -> std::fmt::Result
        where
            RegOrConstant<R, C>: Display,
        {
            write!(f, "{c}{dst}, {p1}, {p2}")
        }

        fn write_broadcast<R, C>(
            f: &mut std::fmt::Formatter<'_>,
            c: char,
            dst: &NumVecReg,
            p1: RegIndex,
            p2: &RegOrConstant<R, C>,
        ) -> std::fmt::Result
        where
            RegOrConstant<R, C>: Display,
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
