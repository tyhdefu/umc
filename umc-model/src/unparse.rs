use crate::instructions::{
    AnyCoherentNumOp, CompareParams, CompareToZero, ConsistentComparison, ConsistentNumOp,
    Instruction, MovParams, NotParams,
};
use crate::operand::{Operand, RegOperand};
use crate::reg_model::{
    FloatRegT, InstrRegT, MemRegT, NumReg, Reg, RegOrConstant, RegTypeT, SignedRegT, UnsignedRegT,
};
use crate::{NumRegType, RegType, RegisterSet};

pub fn instr_to_raw(instr: &Instruction) -> Vec<Operand> {
    match instr {
        Instruction::Nop => vec![],
        Instruction::Mov(mov_params) => mov_to_raw(mov_params),
        Instruction::Add(num_op) => num_op_to_raw(num_op),
        Instruction::Sub(num_op) => num_op_to_raw(num_op),
        Instruction::Mul(num_op) => num_op_to_raw(num_op),
        Instruction::Div(num_op) => num_op_to_raw(num_op),
        Instruction::Mod(num_op) => num_op_to_raw(num_op),
        Instruction::And(num_op) => num_op_to_raw(num_op),
        Instruction::Or(num_op) => num_op_to_raw(num_op),
        Instruction::Xor(num_op) => num_op_to_raw(num_op),
        Instruction::Not(not_params) => not_to_raw(not_params),
        Instruction::Compare { cond: _, params } => cmp_to_raw(params),
        Instruction::Jmp(reg_or_constant) => vec![reg_or_constant.into()],
        Instruction::Bz(reg_or_constant, compare_to_zero) => {
            vec![reg_or_constant.into(), cmp_zero_to_raw(compare_to_zero)]
        }
        Instruction::Bnz(reg_or_constant, compare_to_zero) => {
            vec![reg_or_constant.into(), cmp_zero_to_raw(compare_to_zero)]
        }
        Instruction::Dbg(reg_operand) => vec![Operand::Reg(reg_operand.clone())],
    }
}

fn mov_to_raw(mov_params: &MovParams) -> Vec<Operand> {
    match mov_params {
        MovParams::UnsignedInt(reg, reg_or_constant) => {
            vec![Operand::Reg(reg.into()), reg_or_constant.into()]
        }
        MovParams::SignedInt(reg, reg_or_constant) => {
            vec![Operand::Reg(reg.into()), reg_or_constant.into()]
        }
        MovParams::Float(reg, reg_or_constant) => {
            vec![Operand::Reg(reg.into()), reg_or_constant.into()]
        }
        MovParams::MemAddress(m1, m2) => vec![Operand::Reg(m1.into()), Operand::Reg(m2.into())],
        MovParams::InstrAddress(reg, reg_or_constant) => {
            vec![Operand::Reg(reg.into()), reg_or_constant.into()]
        }
    }
}

fn num_op_to_raw(num_op: &AnyCoherentNumOp) -> Vec<Operand> {
    fn to_raw<'a, RT>(c: &'a ConsistentNumOp<RT>) -> Vec<Operand>
    where
        RT: RegTypeT<R = NumReg>,
        Operand: From<&'a RegOrConstant<RT>>,
        RegOperand: From<&'a Reg<RT>>,
    {
        match c {
            ConsistentNumOp::Single(reg, reg_or_constant1, reg_or_constant2) => {
                vec![
                    Operand::Reg(reg.into()),
                    reg_or_constant1.into(),
                    reg_or_constant2.into(),
                ]
            }
            ConsistentNumOp::VectorBroadcast(num_vec_reg, i, reg_or_constant) => {
                let reg_type = RT::reg_type(&NumReg {
                    index: num_vec_reg.index,
                    width: num_vec_reg.width,
                });
                let reg_set = RegisterSet::Vector(reg_type, num_vec_reg.length);
                vec![
                    Operand::Reg(RegOperand {
                        set: reg_set.clone(),
                        index: num_vec_reg.index,
                    }),
                    Operand::Reg(RegOperand {
                        set: reg_set,
                        index: *i,
                    }),
                    reg_or_constant.into(),
                ]
            }
            ConsistentNumOp::VectorVector(num_vec_reg, i1, i2) => {
                let reg_type = RT::reg_type(&NumReg {
                    index: num_vec_reg.index,
                    width: num_vec_reg.width,
                });
                let reg_set = RegisterSet::Vector(reg_type, num_vec_reg.length);
                vec![
                    Operand::Reg(RegOperand {
                        set: reg_set.clone(),
                        index: num_vec_reg.index,
                    }),
                    Operand::Reg(RegOperand {
                        set: reg_set.clone(),
                        index: *i1,
                    }),
                    Operand::Reg(RegOperand {
                        set: reg_set,
                        index: *i2,
                    }),
                ]
            }
        }
    }
    match num_op {
        AnyCoherentNumOp::UnsignedInt(c) => to_raw(c),
        AnyCoherentNumOp::SignedInt(c) => to_raw(c),
        AnyCoherentNumOp::Float(c) => to_raw(c),
    }
}

fn not_to_raw(params: &NotParams) -> Vec<Operand> {
    match params {
        NotParams::UnsignedInt(reg, reg_or_constant) => {
            vec![Operand::Reg(reg.into()), reg_or_constant.into()]
        }
        NotParams::SignedInt(reg, reg_or_constant) => {
            vec![Operand::Reg(reg.into()), reg_or_constant.into()]
        }
    }
}

fn cmp_zero_to_raw(cmp: &CompareToZero) -> Operand {
    match cmp {
        CompareToZero::Unsigned(reg_or_constant) => reg_or_constant.into(),
        CompareToZero::Signed(reg_or_constant) => reg_or_constant.into(),
    }
}

fn cmp_to_raw(params: &CompareParams) -> Vec<Operand> {
    let dst: Operand = Operand::Reg((&params.dst).into());
    match &params.args {
        ConsistentComparison::UnsignedCompare(a, b) => vec![dst, a.into(), b.into()],
        ConsistentComparison::SignedCompare(a, b) => vec![dst, a.into(), b.into()],
        ConsistentComparison::FloatCompare(a, b) => vec![dst, a.into(), b.into()],
        ConsistentComparison::MemAddressCompare(a, b) => {
            vec![dst, Operand::Reg(a.into()), Operand::Reg(b.into())]
        }
        ConsistentComparison::InstrAddressCompare(a, b) => vec![dst, a.into(), b.into()],
    }
}

impl From<&Reg<UnsignedRegT>> for RegOperand {
    fn from(reg: &Reg<UnsignedRegT>) -> Self {
        RegOperand {
            set: RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(reg.0.width))),
            index: reg.0.index,
        }
    }
}

impl From<&Reg<SignedRegT>> for RegOperand {
    fn from(reg: &Reg<SignedRegT>) -> Self {
        RegOperand {
            set: RegisterSet::Single(RegType::Num(NumRegType::SignedInt(reg.0.width))),
            index: reg.0.index,
        }
    }
}

impl From<&Reg<FloatRegT>> for RegOperand {
    fn from(reg: &Reg<FloatRegT>) -> Self {
        RegOperand {
            set: RegisterSet::Single(RegType::Num(NumRegType::Float(reg.0.width))),
            index: reg.0.index,
        }
    }
}

impl From<&Reg<MemRegT>> for RegOperand {
    fn from(reg: &Reg<MemRegT>) -> Self {
        RegOperand {
            set: RegisterSet::Single(RegType::MemoryAddress),
            index: reg.0,
        }
    }
}

impl From<&Reg<InstrRegT>> for RegOperand {
    fn from(reg: &Reg<InstrRegT>) -> Self {
        RegOperand {
            set: RegisterSet::Single(RegType::InstructionAddress),
            index: reg.0,
        }
    }
}

impl From<&RegOrConstant<UnsignedRegT>> for Operand {
    fn from(value: &RegOrConstant<UnsignedRegT>) -> Self {
        match value {
            RegOrConstant::Reg(reg) => Operand::Reg(reg.into()),
            RegOrConstant::Const(c) => Operand::UnsignedConstant(*c),
        }
    }
}

impl From<&RegOrConstant<SignedRegT>> for Operand {
    fn from(value: &RegOrConstant<SignedRegT>) -> Self {
        match value {
            RegOrConstant::Reg(reg) => Operand::Reg(reg.into()),
            RegOrConstant::Const(c) => Operand::SignedConstant(*c),
        }
    }
}

impl From<&RegOrConstant<FloatRegT>> for Operand {
    fn from(value: &RegOrConstant<FloatRegT>) -> Self {
        match value {
            RegOrConstant::Reg(reg) => Operand::Reg(reg.into()),
            RegOrConstant::Const(c) => Operand::FloatConstant(*c),
        }
    }
}

impl From<&RegOrConstant<InstrRegT>> for Operand {
    fn from(value: &RegOrConstant<InstrRegT>) -> Self {
        match value {
            RegOrConstant::Reg(reg) => Operand::Reg(reg.into()),
            RegOrConstant::Const(c) => Operand::LabelConstant(*c),
        }
    }
}
