use crate::instructions::{
    AnyCoherentNumOp, CompareToZero, ConsistentComparison, ConsistentNumOp, FloatRegT, InstrReg,
    InstrRegT, Instruction, MemReg, MovParams, NumReg, RegOrConstant, RegTypeT, SignedRegT,
    UnsignedRegT,
};
use crate::operand::{Operand, RegOperand};
use crate::{NumRegType, RegType, RegisterSet};

pub fn instr_to_raw(instr: &Instruction) -> Vec<Operand> {
    match instr {
        Instruction::Nop => vec![],
        Instruction::Mov(mov_params) => mov_to_raw(mov_params),
        Instruction::Add(num_op) => num_op_to_raw(num_op),
        Instruction::Sub(num_op) => num_op_to_raw(num_op),
        Instruction::And(num_op) => num_op_to_raw(num_op),
        Instruction::Xor(num_op) => num_op_to_raw(num_op),
        Instruction::Not(not_params) => todo!(),
        Instruction::Compare { cond, dst, args } => {
            let mut vec = cmp_to_raw(args);
            vec.insert(0, Operand::Reg(u_reg(dst)));
            vec
        }
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
        MovParams::UnsignedInt(num_reg, reg_or_constant) => {
            vec![Operand::Reg(u_reg(num_reg)), reg_or_constant.into()]
        }
        MovParams::SignedInt(num_reg, reg_or_constant) => {
            vec![Operand::Reg(i_reg(num_reg)), reg_or_constant.into()]
        }
        MovParams::Float(num_reg, reg_or_constant) => {
            vec![Operand::Reg(f_reg(num_reg)), reg_or_constant.into()]
        }
        MovParams::MemAddress(m1, m2) => vec![Operand::Reg(m_reg(m1)), Operand::Reg(m_reg(m2))],
        MovParams::InstrAddress(reg, reg_or_constant) => {
            vec![Operand::Reg(n_reg(reg)), reg_or_constant.into()]
        }
    }
}

fn num_op_to_raw(num_op: &AnyCoherentNumOp) -> Vec<Operand> {
    fn to_raw<'a, RT>(c: &'a ConsistentNumOp<RT>) -> Vec<Operand>
    where
        RT: RegTypeT<R = NumReg>,
        Operand: From<&'a RegOrConstant<RT>>,
    {
        match c {
            ConsistentNumOp::Single(num_reg, reg_or_constant1, reg_or_constant2) => {
                let reg_type = RT::reg_type(num_reg);
                vec![
                    Operand::Reg(RegOperand {
                        set: RegisterSet::Single(reg_type),
                        index: num_reg.index,
                    }),
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

fn cmp_zero_to_raw(cmp: &CompareToZero) -> Operand {
    match cmp {
        CompareToZero::Unsigned(reg_or_constant) => reg_or_constant.into(),
        CompareToZero::Signed(reg_or_constant) => reg_or_constant.into(),
    }
}

fn cmp_to_raw(args: &ConsistentComparison) -> Vec<Operand> {
    match args {
        ConsistentComparison::UnsignedCompare(a, b) => vec![a.into(), b.into()],
        ConsistentComparison::SignedCompare(a, b) => vec![a.into(), b.into()],
        ConsistentComparison::FloatCompare(a, b) => vec![a.into(), b.into()],
        ConsistentComparison::MemAddressCompare(a, b) => {
            vec![Operand::Reg(m_reg(a)), Operand::Reg(m_reg(b))]
        }
        ConsistentComparison::InstrAddressCompare(a, b) => vec![a.into(), b.into()],
    }
}

fn u_reg(reg: &NumReg) -> RegOperand {
    RegOperand {
        set: RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(reg.width))),
        index: reg.index,
    }
}

fn i_reg(reg: &NumReg) -> RegOperand {
    RegOperand {
        set: RegisterSet::Single(RegType::Num(NumRegType::SignedInt(reg.width))),
        index: reg.index,
    }
}

fn f_reg(reg: &NumReg) -> RegOperand {
    RegOperand {
        set: RegisterSet::Single(RegType::Num(NumRegType::Float(reg.width))),
        index: reg.index,
    }
}

fn m_reg(reg: &MemReg) -> RegOperand {
    RegOperand {
        set: RegisterSet::Single(RegType::MemoryAddress),
        index: *reg,
    }
}

fn n_reg(reg: &InstrReg) -> RegOperand {
    RegOperand {
        set: RegisterSet::Single(RegType::InstructionAddress),
        index: *reg,
    }
}

impl From<&RegOrConstant<UnsignedRegT>> for Operand {
    fn from(value: &RegOrConstant<UnsignedRegT>) -> Self {
        match value {
            RegOrConstant::Reg(num_reg) => Operand::Reg(u_reg(num_reg)),
            RegOrConstant::Const(c) => Operand::UnsignedConstant(*c),
        }
    }
}

impl From<&RegOrConstant<SignedRegT>> for Operand {
    fn from(value: &RegOrConstant<SignedRegT>) -> Self {
        match value {
            RegOrConstant::Reg(num_reg) => Operand::Reg(i_reg(num_reg)),
            RegOrConstant::Const(c) => Operand::SignedConstant(*c),
        }
    }
}

impl From<&RegOrConstant<FloatRegT>> for Operand {
    fn from(value: &RegOrConstant<FloatRegT>) -> Self {
        match value {
            RegOrConstant::Reg(num_reg) => Operand::Reg(f_reg(num_reg)),
            RegOrConstant::Const(c) => Operand::FloatConstant(*c),
        }
    }
}

impl From<&RegOrConstant<InstrRegT>> for Operand {
    fn from(value: &RegOrConstant<InstrRegT>) -> Self {
        match value {
            RegOrConstant::Reg(reg) => Operand::Reg(n_reg(reg)),
            RegOrConstant::Const(c) => Operand::LabelConstant(*c),
        }
    }
}
