use std::fmt::Debug;
use std::fmt::Display;

use crate::RegIndex;
use crate::RegWidth;
use crate::operand::RegOperand;
use crate::reg_model::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// No-op
    Nop,
    /// Move the operand into the destination register
    Mov(MovParams),

    /// Add the two operands and store in destination register
    Add(AnyConsistentNumOp),
    /// Subtract the second operand from the first register
    Sub(AnyConsistentNumOp),
    /// Multiply two registers
    Mul(AnyConsistentNumOp),
    /// Divide the first register by the second register
    Div(AnyConsistentNumOp),
    /// Remainder of the first operand when divided by the second
    Mod(AnyConsistentNumOp),

    /// Bitwise AND
    And(AnyConsistentNumOp),
    /// Bitwise OR
    Or(AnyConsistentNumOp),
    /// Bitwise XOR
    Xor(AnyConsistentNumOp),
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

/// An binary operation with consistent types
#[derive(Debug, PartialEq, Clone)]
pub enum ConsistentOp<RT: RegTypeT> {
    Single(Reg<RT>, RegOrConstant<RT>, RegOrConstant<RT>),
    VectorBroadcast(VectorBroadcastParams<RT>),
    VectorVector(VectorVectorParams<RT>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct VectorBroadcastParams<RT: RegTypeT> {
    dst_full_index: Reg<RT>,
    length: RegWidth,

    reversed: bool,
    p_index: RegIndex,
    value: RegOrConstant<RT>,
}

impl<RT: RegTypeT> VectorBroadcastParams<RT> {
    pub fn new(
        dst: Reg<RT>,
        length: RegWidth,
        vec_param: RegIndex,
        single: RegOrConstant<RT>,
        reversed: bool,
    ) -> Self {
        Self {
            dst_full_index: dst,
            length: length,
            p_index: vec_param,
            value: single,
            reversed,
        }
    }

    pub fn dst(&self) -> &Reg<RT> {
        &self.dst_full_index
    }

    pub fn vec_param(&self) -> Reg<RT> {
        self.dst_full_index.with_index(self.p_index)
    }

    pub fn length(&self) -> RegWidth {
        self.length
    }

    pub fn value_param(&self) -> &RegOrConstant<RT> {
        &self.value
    }

    /// If this operation takes the form
    // shl u32x4:0, u32x4:0, #2 (normal)
    // shl u32x4:0, #2, u32x4:0 (reverse)
    pub fn is_reversed(&self) -> bool {
        self.reversed
    }
}

impl<RT: RegTypeT<R = NumReg>> VectorBroadcastParams<RT> {
    pub fn width(&self) -> RegWidth {
        self.dst_full_index.0.width
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VectorVectorParams<RT: RegTypeT> {
    dst_full_index: Reg<RT>,
    length: RegWidth,

    p1_index: RegIndex,
    p2_index: RegIndex,
}

impl<RT: RegTypeT> VectorVectorParams<RT> {
    pub fn new(dst: Reg<RT>, length: RegWidth, p1: RegIndex, p2: RegIndex) -> Self {
        Self {
            dst_full_index: dst,
            length: length,
            p1_index: p1,
            p2_index: p2,
        }
    }

    pub fn dst(&self) -> &Reg<RT> {
        &self.dst_full_index
    }

    pub fn length(&self) -> RegWidth {
        self.length
    }

    pub fn p1(&self) -> Reg<RT> {
        self.dst_full_index.with_index(self.p1_index)
    }

    pub fn p2(&self) -> Reg<RT> {
        self.dst_full_index.with_index(self.p2_index)
    }
}

impl<RT: RegTypeT<R = NumReg>> VectorVectorParams<RT> {
    pub fn width(&self) -> RegWidth {
        self.dst_full_index.0.width
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnyConsistentNumOp {
    UnsignedInt(ConsistentOp<UnsignedRegT>),
    SignedInt(ConsistentOp<SignedRegT>),
    Float(ConsistentOp<FloatRegT>),
}

#[derive(Debug, PartialEq)]
pub enum AddParams {
    UnsignedInt(ConsistentOp<UnsignedRegT>),
    SignedInt(ConsistentOp<SignedRegT>),
    Float(ConsistentOp<FloatRegT>),

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

impl Display for AnyConsistentNumOp {
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

        match self {
            AnyConsistentNumOp::UnsignedInt(num_op) => match num_op {
                ConsistentOp::Single(dst, p1, p2) => write_single(f, dst, p1, p2),
                ConsistentOp::VectorBroadcast(params) => write!(f, "{}", params),
                ConsistentOp::VectorVector(params) => {
                    write!(f, "{}", params)
                }
            },
            AnyConsistentNumOp::SignedInt(num_op) => match num_op {
                ConsistentOp::Single(dst, p1, p2) => write_single(f, dst, p1, p2),
                ConsistentOp::VectorBroadcast(params) => write!(f, "{}", params),
                ConsistentOp::VectorVector(params) => {
                    write!(f, "{}", params)
                }
            },
            AnyConsistentNumOp::Float(num_op) => match num_op {
                ConsistentOp::Single(dst, p1, p2) => write_single(f, dst, p1, p2),
                ConsistentOp::VectorBroadcast(params) => write!(f, "{}", params),
                ConsistentOp::VectorVector(params) => {
                    write!(f, "{}", params)
                }
            },
        }
    }
}

impl<RT: RegTypeT<R = NumReg>> Display for VectorBroadcastParams<RT>
where
    RegOrConstant<RT>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dst = self.dst_full_index.0.index;
        let p1 = self.p_index;
        let p2 = self.value_param();
        write!(
            f,
            "{0}{1}x{2}:{dst}, {0}{1}x{2}:{p1}, {p2}",
            RT::LETTER,
            self.dst_full_index.0.width,
            self.length
        )
    }
}

impl<RT: RegTypeT<R = NumReg>> Display for VectorVectorParams<RT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dst = self.dst_full_index.0.index;
        let p1 = self.p1_index;
        let p2 = self.p2_index;

        write!(
            f,
            "{0}{1}x{2}:{dst}, {0}{1}x{2}:{p1}, {0}{1}x{2}:{p2}",
            RT::LETTER,
            self.dst_full_index.0.width,
            self.length
        )
    }
}
