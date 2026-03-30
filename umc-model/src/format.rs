use std::collections::HashMap;
use std::fmt::Display;

use crate::instructions::*;
use crate::reg_model::{InstrRegT, MemRegT, NumReg, NumVecReg, Reg, RegOrConstant, RegTypeT};
use crate::{RegType, RegWidth, RegisterSet};

pub enum DisplayAssemblyParams<'a> {
    /// Display raw values - does not produce valid UMC assembly
    Raw,
    /// Display labels given by the map
    WithSymbols {
        instr_labels: &'a HashMap<usize, String>,
        mem_labels: &'a HashMap<usize, String>,
    },
}

impl<'a> DisplayAssemblyParams<'a> {
    pub fn fmt_instr_label(
        &self,
        f: &mut std::fmt::Formatter,
        constant: &<InstrRegT as RegTypeT>::C,
    ) -> std::fmt::Result {
        match self {
            DisplayAssemblyParams::Raw => {
                write!(f, "{:#X}", constant)
            }
            DisplayAssemblyParams::WithSymbols {
                instr_labels,
                mem_labels: _,
            } => match instr_labels.get(constant) {
                Some(l) => write!(f, ".{}", l),
                None => write!(f, ".({:#X})", constant),
            },
        }
    }

    pub fn fmt_mem_label(
        &self,
        f: &mut std::fmt::Formatter,
        constant: &<MemRegT as RegTypeT>::C,
    ) -> std::fmt::Result {
        match self {
            DisplayAssemblyParams::Raw => {
                write!(f, "{:#X}", constant)
            }
            DisplayAssemblyParams::WithSymbols {
                instr_labels: _,
                mem_labels,
            } => match mem_labels.get(&(*constant as usize)) {
                Some(l) => write!(f, "&{}", l),
                None => write!(f, "&({:#X})", constant),
            },
        }
    }

    pub fn get_instr_label(&self, i: usize) -> Option<&str> {
        match self {
            DisplayAssemblyParams::Raw => None,
            DisplayAssemblyParams::WithSymbols {
                instr_labels,
                mem_labels: _,
            } => instr_labels.get(&i).map(|x| x.as_str()),
        }
    }
}

/// Format the instruction or operand as it would appear in UMC assembly
pub trait DisplayAssembly {
    /// Format as assembly
    /// [DisplayAssemblyParams] dictate how labels are resolved
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result;
}

#[macro_export]
macro_rules! impl_display_delgate_raw {
    ($t:ty) => {
        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.fmt_assembly(f, &DisplayAssemblyParams::Raw)
            }
        }
    };
}

struct ParamPair<'a, A: DisplayAssembly, B: DisplayAssembly>(&'a A, &'a B);

impl<'a, A, B> DisplayAssembly for ParamPair<'a, A, B>
where
    A: DisplayAssembly,
    B: DisplayAssembly,
{
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        self.0.fmt_assembly(f, opts)?;
        write!(f, ", ")?;
        self.1.fmt_assembly(f, opts)
    }
}

impl DisplayAssembly for Instruction {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        fn write_instr<P: DisplayAssembly>(
            f: &mut std::fmt::Formatter<'_>,
            opcode: &str,
            args: &P,
            opts: &DisplayAssemblyParams,
        ) -> std::fmt::Result {
            f.write_str(opcode)?;
            f.write_str(" ")?;
            args.fmt_assembly(f, opts)
        }

        match self {
            Instruction::Nop => write!(f, "nop"),
            Instruction::Mov(params) => write_instr(f, "mov", params, opts),
            Instruction::Add(params) => write_instr(f, "add", params, opts),
            Instruction::Sub(params) => write_instr(f, "sub", params, opts),
            Instruction::Mul(params) => write_instr(f, "mul", params, opts),
            Instruction::Div(params) => write_instr(f, "div", params, opts),
            Instruction::Mod(params) => write_instr(f, "mod", params, opts),
            Instruction::And(params) => write_instr(f, "and", params, opts),
            Instruction::Or(params) => write_instr(f, "or", params, opts),
            Instruction::Xor(params) => write_instr(f, "xor", params, opts),
            Instruction::Not(params) => write_instr(f, "not", params, opts),
            Instruction::Compare { cond, params } => {
                let opcode = match cond {
                    BinaryCondition::Equal => "eq",
                    BinaryCondition::GreaterThan => "gt",
                    BinaryCondition::GreaterThanOrEqualTo => "ge",
                    BinaryCondition::LessThan => "lt",
                    BinaryCondition::LessThanOrEqualTo => "le",
                };
                write_instr(f, opcode, params, opts)
            }
            Instruction::Jmp(reg_or_constant) => write_instr(f, "jmp", reg_or_constant, opts),
            Instruction::Jal(d, r) => write_instr(f, "jal", &ParamPair(d, r), opts),
            Instruction::Bz(reg_or_constant, compare_to_zero) => {
                write_instr(f, "bz", &ParamPair(reg_or_constant, compare_to_zero), opts)
            }
            Instruction::Bnz(reg_or_constant, compare_to_zero) => {
                write_instr(f, "bnz", &ParamPair(reg_or_constant, compare_to_zero), opts)
            }
            Instruction::Alloc(mem_reg, size) => {
                write_instr(f, "alloc", &ParamPair(mem_reg, size), opts)
            }
            Instruction::Free(mem_reg) => write_instr(f, "free", mem_reg, opts),
            Instruction::Load(reg, mem_reg) => {
                write_instr(f, "load", &ParamPair(reg, mem_reg), opts)
            }
            Instruction::Store(mem_reg, reg) => {
                write_instr(f, "store", &ParamPair(mem_reg, reg), opts)
            }
            Instruction::SizeOf(reg, register_set) => match register_set {
                RegisterSet::Single(RegType::InstructionAddress) => {
                    write_instr(f, "nsize", reg, opts)
                }
                RegisterSet::Single(RegType::MemoryAddress) => write_instr(f, "msize", reg, opts),
                _ => {
                    write!(f, "sizeof <{}> ", register_set)?;
                    reg.fmt_assembly(f, opts)
                }
            },
            Instruction::Cast(cast) => write_instr(f, "cast", cast, opts),
            Instruction::ECall(ecall) => write_instr(f, "ecall", ecall, opts),
            Instruction::Dbg(reg_operand) => write_instr(f, "dbg", reg_operand, opts),
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

impl DisplayAssembly for CompareToZero {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _params: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            CompareToZero::Unsigned(x) => write!(f, "{}", x),
            CompareToZero::Signed(x) => write!(f, "{}", x),
        }
    }
}

impl DisplayAssembly for MovParams {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            MovParams::UnsignedInt(reg, p) => write!(f, "{}, {}", reg, p),
            MovParams::SignedInt(reg, p) => write!(f, "{}, {}", reg, p),
            MovParams::Float(reg, p) => write!(f, "{}, {}", reg, p),
            MovParams::MemAddress(to, from) => ParamPair(to, from).fmt_assembly(f, opts),
            MovParams::InstrAddress(to, p) => ParamPair(to, p).fmt_assembly(f, opts),
        }
    }
}

impl DisplayAssembly for NotParams {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
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

impl DisplayAssembly for CompareParams {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        write!(f, "{}, ", self.dst)?;
        self.args.fmt_assembly(f, opts)
    }
}

impl DisplayAssembly for ConsistentComparison {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match &self {
            Self::UnsignedCompare(p1, p2) => ParamPair(p1, p2).fmt_assembly(f, opts),
            Self::SignedCompare(p1, p2) => ParamPair(p1, p2).fmt_assembly(f, opts),
            Self::FloatCompare(p1, p2) => ParamPair(p1, p2).fmt_assembly(f, opts),
            Self::MemAddressCompare(i1, i2) => ParamPair(i1, i2).fmt_assembly(f, opts),
            Self::InstrAddressCompare(p1, p2) => ParamPair(p1, p2).fmt_assembly(f, opts),
        }
    }
}

impl DisplayAssembly for AddParams {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            AddParams::UnsignedInt(consistent_op) => consistent_op.fmt_assembly(f, opts),
            AddParams::SignedInt(consistent_op) => consistent_op.fmt_assembly(f, opts),
            AddParams::Float(consistent_op) => consistent_op.fmt_assembly(f, opts),
            AddParams::MemAddress(dst, mem_reg, offset_reg) => {
                write!(f, "{}, ", dst)?;
                ParamPair(mem_reg, offset_reg).fmt_assembly(f, opts)
            }
            AddParams::InstrAddress(dst, instr_reg, offset_reg) => {
                write!(f, "{}, ", dst)?;
                ParamPair(instr_reg, offset_reg).fmt_assembly(f, opts)
            }
        }
    }
}

impl DisplayAssembly for OffsetOp {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            OffsetOp::Unsigned(x) => write!(f, "{}", x),
            OffsetOp::Signed(x) => write!(f, "{}", x),
        }
    }
}
impl_display_delgate_raw!(OffsetOp);

impl<RT: RegTypeT> DisplayAssembly for ConsistentOp<RT>
where
    RegOrConstant<RT>: DisplayAssembly,
{
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            ConsistentOp::Single(dst, p1, p2) => {
                write!(f, "{dst}, ")?;
                ParamPair(p1, p2).fmt_assembly(f, opts)
            }
            ConsistentOp::VectorBroadcast(params) => params.fmt_assembly(f, opts),
            ConsistentOp::VectorVector(params) => params.fmt_assembly(f, opts),
        }
    }
}

impl DisplayAssembly for AnyConsistentNumOp {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            AnyConsistentNumOp::UnsignedInt(num_op) => num_op.fmt_assembly(f, opts),
            AnyConsistentNumOp::SignedInt(num_op) => num_op.fmt_assembly(f, opts),
            AnyConsistentNumOp::Float(num_op) => num_op.fmt_assembly(f, opts),
        }
    }
}

impl<RT: RegTypeT> DisplayAssembly for VectorBroadcastParams<RT>
where
    RegOrConstant<RT>: DisplayAssembly,
{
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        let dst = self.dst().index;
        let p1 = self.vec_param().index;
        let p2 = self.value_param();
        write!(
            f,
            "{0}{1}x{2}:{dst}, {0}{1}x{2}:{p1}, ",
            RT::LETTER,
            self.dst().width,
            self.length()
        )?;
        p2.fmt_assembly(f, opts)
    }
}

impl<RT: RegTypeT> DisplayAssembly for VectorVectorParams<RT> {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        let dst = self.dst().index;
        let p1 = self.p1().index;
        let p2 = self.p2().index;

        write!(
            f,
            "{0}{1}x{2}:{dst}, {0}{1}x{2}:{p1}, {0}{1}x{2}:{p2}",
            RT::LETTER,
            self.dst().width,
            self.length()
        )
    }
}

impl DisplayAssembly for SimpleCast {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            SimpleCast::Resize(ResizeCast::Unsigned(d, p)) => write!(f, "{}, {}", d, p),
            SimpleCast::Resize(ResizeCast::Signed(d, p)) => write!(f, "{}, {}", d, p),
            SimpleCast::Resize(ResizeCast::Float(d, p)) => write!(f, "{}, {}", d, p),
            SimpleCast::IgnoreSigned(p) => write!(f, "{}, {}", p.dst(), p.from()),
            SimpleCast::AddSign(p) => write!(f, "{}, {}", p.dst(), p.from()),
        }
    }
}

impl DisplayAssembly for AnyReg {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            AnyReg::Single(x) => x.fmt_assembly(f, opts),
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
impl_display_delgate_raw!(AnyReg);

impl DisplayAssembly for AnySingleReg {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            AnySingleReg::Unsigned(reg) => write!(f, "{}", reg),
            AnySingleReg::Signed(reg) => write!(f, "{}", reg),
            AnySingleReg::Float(reg) => write!(f, "{}", reg),
            AnySingleReg::Instr(reg) => write!(f, "{}", reg),
            AnySingleReg::Mem(reg) => write!(f, "{}", reg),
        }
    }
}
impl_display_delgate_raw!(AnySingleReg);

impl DisplayAssembly for AnySingleRegOrConstant {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        match self {
            AnySingleRegOrConstant::Unsigned(x) => write!(f, "{}", x),
            AnySingleRegOrConstant::Signed(x) => write!(f, "{}", x),
            AnySingleRegOrConstant::Float(x) => write!(f, "{}", x),
            AnySingleRegOrConstant::Instr(x) => x.fmt_assembly(f, opts),
            AnySingleRegOrConstant::Mem(x) => x.fmt_assembly(f, opts),
        }
    }
}

impl DisplayAssembly for ECallParams {
    fn fmt_assembly(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        opts: &DisplayAssemblyParams,
    ) -> std::fmt::Result {
        ParamPair(&self.dst, &self.code).fmt_assembly(f, opts)?;
        for arg in &self.args {
            write!(f, ", ")?;
            arg.fmt_assembly(f, opts)?;
        }
        Ok(())
    }
}
