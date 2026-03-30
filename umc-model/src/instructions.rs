use std::fmt::Debug;
use std::fmt::Display;

use crate::RegIndex;
use crate::RegWidth;
use crate::RegisterSet;
use crate::format::DisplayAssembly;
use crate::format::DisplayAssemblyParams;
use crate::reg_model::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /// No-op
    Nop,
    /// Move the operand into the destination register
    Mov(MovParams),

    /// Add the two operands and store in destination register
    Add(AddParams),
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
    /// Jump and save the address of the next instruction into the given register
    Jal(RegOrConstant<InstrRegT>, Reg<InstrRegT>),
    /// Conditionally branch to the given location (op1) if the second operand is zero
    Bz(RegOrConstant<InstrRegT>, CompareToZero),
    /// Conditionally branch to the given location (op1) if the second operand is not zero
    Bnz(RegOrConstant<InstrRegT>, CompareToZero),

    /// Allocate a block of memory of at least x bytes
    Alloc(Reg<MemRegT>, RegOrConstant<UnsignedRegT>),
    /// Release a block of memory
    Free(Reg<MemRegT>),

    /// Load a register from a memory address
    Load(AnySingleReg, RegOrConstant<MemRegT>),
    /// Store a register into a memory address
    // TODO: Can we store constants? Not unless they are sized
    Store(RegOrConstant<MemRegT>, AnySingleReg),

    // Get the number of bytes required for a load or store for the given register type
    SizeOf(Reg<UnsignedRegT>, RegisterSet),

    // Simple cast based on the registers
    Cast(SimpleCast),

    /// Environment call
    ECall(ECallParams),
    // TODO: Float to integer cast
    /// Print the given register (debugging)
    Dbg(AnyReg),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_assembly(f, &DisplayAssemblyParams::Raw)
    }
}

/// Any type of register including vector registers
#[derive(Debug, PartialEq, Clone)]
pub enum AnyReg {
    Single(AnySingleReg),
    Vector(AnySingleReg, RegWidth),
}

/// Any type of register
#[derive(Debug, PartialEq, Clone)]
pub enum AnySingleReg {
    Unsigned(Reg<UnsignedRegT>),
    Signed(Reg<SignedRegT>),
    Float(Reg<FloatRegT>),
    Instr(Reg<InstrRegT>),
    Mem(Reg<MemRegT>),
}

/// Any type - either register or a constant
#[derive(Debug, PartialEq, Clone)]
pub enum AnySingleRegOrConstant {
    Unsigned(RegOrConstant<UnsignedRegT>),
    Signed(RegOrConstant<SignedRegT>),
    Float(RegOrConstant<FloatRegT>),
    Instr(RegOrConstant<InstrRegT>),
    Mem(RegOrConstant<MemRegT>),
}

impl AnySingleRegOrConstant {
    pub fn from_any_reg(any_reg: AnySingleReg) -> Self {
        match any_reg {
            AnySingleReg::Unsigned(reg) => Self::Unsigned(RegOrConstant::Reg(reg)),
            AnySingleReg::Signed(reg) => Self::Signed(RegOrConstant::Reg(reg)),
            AnySingleReg::Float(reg) => Self::Float(RegOrConstant::Reg(reg)),
            AnySingleReg::Instr(reg) => Self::Instr(RegOrConstant::Reg(reg)),
            AnySingleReg::Mem(reg) => Self::Mem(RegOrConstant::Reg(reg)),
        }
    }
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
    vec_index: RegIndex,
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
            vec_index: vec_param,
            value: single,
            reversed,
        }
    }

    pub fn dst(&self) -> &Reg<RT> {
        &self.dst_full_index
    }

    pub fn vec_param(&self) -> Reg<RT> {
        let mut vec_param = self.dst_full_index.clone();
        vec_param.index = self.vec_index;
        vec_param
    }

    pub fn width(&self) -> RT::WIDTH {
        self.dst_full_index.width.clone()
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
        let mut p1 = self.dst_full_index.clone();
        p1.index = self.p1_index;
        p1
    }

    pub fn p2(&self) -> Reg<RT> {
        let mut p2 = self.dst_full_index.clone();
        p2.index = self.p2_index;
        p2
    }

    pub fn width(&self) -> RT::WIDTH {
        self.dst_full_index.width.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnyConsistentNumOp {
    UnsignedInt(ConsistentOp<UnsignedRegT>),
    SignedInt(ConsistentOp<SignedRegT>),
    Float(ConsistentOp<FloatRegT>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AddParams {
    UnsignedInt(ConsistentOp<UnsignedRegT>),
    SignedInt(ConsistentOp<SignedRegT>),
    Float(ConsistentOp<FloatRegT>),

    MemAddress(Reg<MemRegT>, RegOrConstant<MemRegT>, OffsetOp),
    InstrAddress(Reg<InstrRegT>, RegOrConstant<InstrRegT>, OffsetOp),
}

/// A positive or negative integer offset
#[derive(Debug, Clone, PartialEq)]
pub enum OffsetOp {
    Unsigned(RegOrConstant<UnsignedRegT>),
    Signed(RegOrConstant<SignedRegT>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MovParams {
    UnsignedInt(Reg<UnsignedRegT>, RegOrConstant<UnsignedRegT>),
    SignedInt(Reg<SignedRegT>, RegOrConstant<SignedRegT>),
    Float(Reg<FloatRegT>, RegOrConstant<FloatRegT>),

    MemAddress(Reg<MemRegT>, RegOrConstant<MemRegT>),
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
    MemAddressCompare(RegOrConstant<MemRegT>, RegOrConstant<MemRegT>),
    InstrAddressCompare(RegOrConstant<InstrRegT>, RegOrConstant<InstrRegT>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompareToZero {
    Unsigned(RegOrConstant<UnsignedRegT>),
    Signed(RegOrConstant<SignedRegT>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ECallParams {
    pub dst: AnyReg,
    pub code: RegOrConstant<UnsignedRegT>,
    // TODO: Allow vector arguments too
    pub args: Vec<AnySingleRegOrConstant>,
}

/// A simple cast that can be inferred solely based on the operands
#[derive(Debug, PartialEq, Clone)]
pub enum SimpleCast {
    /// Narrow / widen within the same type
    Resize(ResizeCast),
    /// Bitwise convert a signed integer to a unsigned one
    /// > This is dangerous if the signed register is negative
    IgnoreSigned(IntegerCast<UnsignedRegT, SignedRegT>),
    /// Bitwise convert an unsigned integer to signed one
    /// > This is dangerous if the top bits of the unsigned register are used
    AddSign(IntegerCast<SignedRegT, UnsignedRegT>),
    // TODO: Vector casts?
}

#[derive(Debug, Clone, PartialEq)]
pub struct IntegerCast<TO: RegTypeT, FROM: RegTypeT>
where
    TO: RegTypeT<WIDTH = FROM::WIDTH>,
{
    to: Reg<TO>,
    from: RegOrConstant<FROM>,
}

impl<TO, FROM> IntegerCast<TO, FROM>
where
    TO: RegTypeT<WIDTH = RegWidth>,
    FROM: RegTypeT<WIDTH = RegWidth>,
{
    pub fn try_create(to: Reg<TO>, from: RegOrConstant<FROM>) -> Result<Self, ()> {
        if let Some(width) = from.width()
            && width != to.width
        {
            return Err(());
        }
        Ok(Self { to, from })
    }

    pub fn dst(&self) -> &Reg<TO> {
        &self.to
    }

    pub fn from(&self) -> &RegOrConstant<FROM> {
        &self.from
    }

    pub fn width(&self) -> RegWidth {
        self.to.width
    }
}

/// A narrowing or widening cast
/// > Note that widening is allowed implicitly in all instructions
#[derive(Debug, PartialEq, Clone)]
pub enum ResizeCast {
    /// Zero-extension or truncation
    Unsigned(Reg<UnsignedRegT>, RegOrConstant<UnsignedRegT>),
    /// Sign-extension or truncation
    Signed(Reg<SignedRegT>, RegOrConstant<SignedRegT>),
    /// Precision increase / decrease
    Float(Reg<FloatRegT>, RegOrConstant<FloatRegT>),
}
