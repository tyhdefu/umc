use std::fmt::Debug;
use std::fmt::Display;

use crate::RegIndex;
use crate::operand::RegOperand;
use crate::reg_model::*;

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
    /// Multiply two registers
    Mul(AnyCoherentNumOp),
    /// Divide the first register by the second register
    Div(AnyCoherentNumOp),
    /// Remainder of the first operand when divided by the second
    Mod(AnyCoherentNumOp),

    /// Bitwise AND
    And(AnyCoherentNumOp),
    /// Bitwise OR
    Or(AnyCoherentNumOp),
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

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Nop => write!(f, "nop"),
            Instruction::Mov(params) => write!(f, "mov {}", params),
            Instruction::Add(params) => write!(f, "add {}", params),
            Instruction::Sub(params) => write!(f, "sub {}", params),
            Instruction::Mul(params) => write!(f, "mul {}", params),
            Instruction::Div(params) => write!(f, "div {}", params),
            Instruction::Mod(params) => write!(f, "mod {}", params),
            Instruction::And(params) => write!(f, "and {}", params),
            Instruction::Or(params) => write!(f, "or {}", params),
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
