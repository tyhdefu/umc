use std::fmt::Debug;
use std::fmt::Display;

use crate::RegIndex;
use crate::RegWidth;
use crate::RegisterSet;
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
        let mut vec_param = self.dst_full_index.clone();
        vec_param.index = self.p_index;
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
            Instruction::Jal(d, r) => write!(f, "jal {}, {}", d, r),
            Instruction::Bz(reg_or_constant, compare_to_zero) => {
                write!(f, "bz {}, {}", reg_or_constant, compare_to_zero)
            }
            Instruction::Bnz(reg_or_constant, compare_to_zero) => {
                write!(f, "bnz {}, {}", reg_or_constant, compare_to_zero)
            }
            Instruction::Alloc(mem_reg, size) => {
                write!(f, "alloc {}, {}", mem_reg, size)
            }
            Instruction::Free(mem_reg) => {
                write!(f, "free {}", mem_reg)
            }
            Instruction::Load(reg, mem_reg) => {
                write!(f, "load {}, {}", reg, mem_reg)
            }
            Instruction::Store(mem_reg, reg) => {
                write!(f, "store {}, {}", mem_reg, reg)
            }
            Instruction::SizeOf(reg, register_set) => {
                write!(f, "sizeof <{}>, {}", register_set, reg)
            }
            Instruction::Cast(cast) => write!(f, "cast {}", cast),
            Instruction::ECall(ecall) => write!(f, "ecall {}", ecall),
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

impl Display for AddParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddParams::UnsignedInt(consistent_op) => write!(f, "{}", consistent_op),
            AddParams::SignedInt(consistent_op) => write!(f, "{}", consistent_op),
            AddParams::Float(consistent_op) => write!(f, "{}", consistent_op),
            AddParams::MemAddress(dst, mem_reg, offset_reg) => {
                write!(f, "{}, {}, {}", dst, mem_reg, offset_reg)
            }
            AddParams::InstrAddress(dst, instr_reg, offset_reg) => {
                write!(f, "{}, {}, {}", dst, instr_reg, offset_reg)
            }
        }
    }
}

impl Display for OffsetOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OffsetOp::Unsigned(x) => write!(f, "{}", x),
            OffsetOp::Signed(x) => write!(f, "{}", x),
        }
    }
}

impl<RT: RegTypeT> Display for ConsistentOp<RT>
where
    RegOrConstant<RT>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsistentOp::Single(dst, p1, p2) => write!(f, "{dst}, {p1}, {p2}"),
            ConsistentOp::VectorBroadcast(params) => write!(f, "{}", params),
            ConsistentOp::VectorVector(params) => write!(f, "{}", params),
        }
    }
}

impl Display for AnyConsistentNumOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyConsistentNumOp::UnsignedInt(num_op) => write!(f, "{}", num_op),
            AnyConsistentNumOp::SignedInt(num_op) => write!(f, "{}", num_op),
            AnyConsistentNumOp::Float(num_op) => write!(f, "{}", num_op),
        }
    }
}

impl<RT: RegTypeT> Display for VectorBroadcastParams<RT>
where
    RegOrConstant<RT>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dst = self.dst_full_index.index;
        let p1 = self.p_index;
        let p2 = self.value_param();
        write!(
            f,
            "{0}{1}x{2}:{dst}, {0}{1}x{2}:{p1}, {p2}",
            RT::LETTER,
            self.dst_full_index.width,
            self.length
        )
    }
}

impl<RT: RegTypeT> Display for VectorVectorParams<RT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dst = self.dst_full_index.index;
        let p1 = self.p1_index;
        let p2 = self.p2_index;

        write!(
            f,
            "{0}{1}x{2}:{dst}, {0}{1}x{2}:{p1}, {0}{1}x{2}:{p2}",
            RT::LETTER,
            self.dst_full_index.width,
            self.length
        )
    }
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

impl Display for SimpleCast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleCast::Resize(ResizeCast::Unsigned(d, p)) => write!(f, "{}, {}", d, p),
            SimpleCast::Resize(ResizeCast::Signed(d, p)) => write!(f, "{}, {}", d, p),
            SimpleCast::Resize(ResizeCast::Float(d, p)) => write!(f, "{}, {}", d, p),
            SimpleCast::IgnoreSigned(p) => write!(f, "{}, {}", p.dst(), p.from()),
            SimpleCast::AddSign(p) => write!(f, "{}, {}", p.dst(), p.from()),
        }
    }
}

impl Display for AnyReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyReg::Single(x) => write!(f, "{}", x),
            AnyReg::Vector(reg, length) => {
                fn write_vec_reg<RT: RegTypeT>(
                    f: &mut std::fmt::Formatter<'_>,
                    reg: &Reg<RT>,
                    length: RegWidth,
                ) -> std::fmt::Result
                where
                    RT::WIDTH: Display,
                {
                    write!(f, "{}{}x{}:{}", RT::LETTER, reg.width, length, reg.index)
                }
                match reg {
                    AnySingleReg::Unsigned(reg) => write_vec_reg(f, reg, *length),
                    AnySingleReg::Signed(reg) => write_vec_reg(f, reg, *length),
                    AnySingleReg::Float(reg) => write_vec_reg(f, reg, *length),
                    AnySingleReg::Instr(reg) => write_vec_reg(f, reg, *length),
                    AnySingleReg::Mem(reg) => write_vec_reg(f, reg, *length),
                }
            }
        }
    }
}

impl Display for AnySingleReg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnySingleReg::Unsigned(reg) => write!(f, "{}", reg),
            AnySingleReg::Signed(reg) => write!(f, "{}", reg),
            AnySingleReg::Float(reg) => write!(f, "{}", reg),
            AnySingleReg::Instr(reg) => write!(f, "{}", reg),
            AnySingleReg::Mem(reg) => write!(f, "{}", reg),
        }
    }
}

impl Display for AnySingleRegOrConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnySingleRegOrConstant::Unsigned(x) => write!(f, "{}", x),
            AnySingleRegOrConstant::Signed(x) => write!(f, "{}", x),
            AnySingleRegOrConstant::Float(x) => write!(f, "{}", x),
            AnySingleRegOrConstant::Instr(x) => write!(f, "{}", x),
            AnySingleRegOrConstant::Mem(x) => write!(f, "{}", x),
        }
    }
}

impl Display for ECallParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.dst, self.code)?;
        for arg in &self.args {
            write!(f, ", {}", arg)?;
        }
        Ok(())
    }
}
