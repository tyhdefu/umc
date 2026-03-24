use crate::instructions::{
    AddParams, AnyConsistentNumOp, AnyReg, AnySingleReg, AnySingleRegOrConstant, CompareParams,
    CompareToZero, ConsistentComparison, ConsistentOp, ECallParams, Instruction, MovParams,
    NotParams, OffsetOp, ResizeCast, SimpleCast,
};
use crate::operand::{Operand, RegOperand};
use crate::reg_model::{
    FloatRegT, InstrRegT, MemRegT, Reg, RegOrConstant, RegTypeT, SignedRegT, UnsignedRegT,
};
use crate::{NumRegType, RegType, RegWidth, RegisterSet};

pub fn instr_to_raw(instr: &Instruction) -> Vec<Operand> {
    match instr {
        Instruction::Nop => vec![],
        Instruction::Mov(mov_params) => mov_to_raw(mov_params),
        Instruction::Add(num_op) => add_op_to_raw(num_op),
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
        Instruction::Jal(d, r) => vec![d.into(), Operand::Reg(r.into())],
        Instruction::Bz(reg_or_constant, compare_to_zero) => {
            vec![reg_or_constant.into(), cmp_zero_to_raw(compare_to_zero)]
        }
        Instruction::Bnz(reg_or_constant, compare_to_zero) => {
            vec![reg_or_constant.into(), cmp_zero_to_raw(compare_to_zero)]
        }
        Instruction::Alloc(mem_reg, size) => vec![Operand::Reg(mem_reg.into()), size.into()],
        Instruction::Free(mem_reg) => vec![Operand::Reg(mem_reg.into())],
        Instruction::Load(reg, mem_reg) => {
            vec![Operand::Reg(reg.into()), mem_reg.into()]
        }
        Instruction::Store(mem_reg, reg) => {
            vec![mem_reg.into(), Operand::Reg(reg.into())]
        }
        Instruction::SizeOf(reg, rs) => vec![
            Operand::Reg(RegOperand {
                set: rs.clone(),
                index: 0,
            }),
            Operand::Reg(reg.into()),
        ],
        Instruction::Cast(simple_cast) => simple_cast_to_raw(simple_cast),
        Instruction::ECall(ecall) => ecall_to_raw(ecall),
        Instruction::Dbg(reg_operand) => vec![Operand::Reg(reg_operand.into())],
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
        MovParams::MemAddress(m1, m2) => vec![Operand::Reg(m1.into()), m2.into()],
        MovParams::InstrAddress(reg, reg_or_constant) => {
            vec![Operand::Reg(reg.into()), reg_or_constant.into()]
        }
    }
}

fn add_op_to_raw(add_op: &AddParams) -> Vec<Operand> {
    match add_op {
        AddParams::UnsignedInt(consistent_op) => consistent_op_to_raw(consistent_op),
        AddParams::SignedInt(consistent_op) => consistent_op_to_raw(consistent_op),
        AddParams::Float(consistent_op) => consistent_op_to_raw(consistent_op),
        AddParams::MemAddress(reg, reg1, reg_or_constant) => {
            vec![
                Operand::Reg(reg.into()),
                reg1.into(),
                reg_or_constant.into(),
            ]
        }
        AddParams::InstrAddress(reg, reg_or_constant, reg_or_constant1) => {
            vec![
                Operand::Reg(reg.into()),
                reg_or_constant.into(),
                reg_or_constant1.into(),
            ]
        }
    }
}

fn num_op_to_raw(num_op: &AnyConsistentNumOp) -> Vec<Operand> {
    match num_op {
        AnyConsistentNumOp::UnsignedInt(c) => consistent_op_to_raw(c),
        AnyConsistentNumOp::SignedInt(c) => consistent_op_to_raw(c),
        AnyConsistentNumOp::Float(c) => consistent_op_to_raw(c),
    }
}
fn consistent_op_to_raw<'a, RT>(c: &'a ConsistentOp<RT>) -> Vec<Operand>
where
    RT: RegTypeT<WIDTH = RegWidth> + 'static,
    for<'x> &'x RegOrConstant<RT>: Into<Operand>,
    for<'x> &'x Reg<RT>: Into<RegOperand>,
{
    match c {
        ConsistentOp::Single(reg, reg_or_constant1, reg_or_constant2) => {
            vec![
                Operand::Reg(reg.into()),
                reg_or_constant1.into(),
                reg_or_constant2.into(),
            ]
        }
        ConsistentOp::VectorBroadcast(params) => {
            let p1 = params.vec_param();
            vec![
                Operand::Reg(params.dst().into()),
                Operand::Reg((&p1).into()),
                params.value_param().into(),
            ]
        }
        ConsistentOp::VectorVector(params) => {
            vec![
                Operand::Reg(params.dst().into()),
                Operand::Reg((&params.p1()).into()),
                Operand::Reg((&params.p2()).into()),
            ]
        }
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
            vec![dst, a.into(), b.into()]
        }
        ConsistentComparison::InstrAddressCompare(a, b) => vec![dst, a.into(), b.into()],
    }
}

fn simple_cast_to_raw(params: &SimpleCast) -> Vec<Operand> {
    match params {
        SimpleCast::Resize(ResizeCast::Unsigned(dst, p)) => {
            vec![Operand::Reg(dst.into()), p.into()]
        }
        SimpleCast::Resize(ResizeCast::Signed(dst, p)) => vec![Operand::Reg(dst.into()), p.into()],
        SimpleCast::Resize(ResizeCast::Float(dst, p)) => vec![Operand::Reg(dst.into()), p.into()],
        SimpleCast::IgnoreSigned(c) => vec![Operand::Reg(c.dst().into()), c.from().into()],
        SimpleCast::AddSign(c) => vec![Operand::Reg(c.dst().into()), c.from().into()],
    }
}

fn ecall_to_raw(params: &ECallParams) -> Vec<Operand> {
    let mut operands: Vec<Operand> = Vec::with_capacity(params.args.len() + 2);
    operands.push(Operand::Reg((&params.dst).into()));
    operands.push((&params.code).into());

    for arg in &params.args {
        operands.push(arg.into());
    }
    operands
}

impl From<&Reg<UnsignedRegT>> for RegOperand {
    fn from(reg: &Reg<UnsignedRegT>) -> Self {
        RegOperand {
            set: RegisterSet::Single(RegType::Num(NumRegType::UnsignedInt(reg.width))),
            index: reg.index,
        }
    }
}

impl From<&Reg<SignedRegT>> for RegOperand {
    fn from(reg: &Reg<SignedRegT>) -> Self {
        RegOperand {
            set: RegisterSet::Single(RegType::Num(NumRegType::SignedInt(reg.width))),
            index: reg.index,
        }
    }
}

impl From<&Reg<FloatRegT>> for RegOperand {
    fn from(reg: &Reg<FloatRegT>) -> Self {
        RegOperand {
            set: RegisterSet::Single(RegType::Num(NumRegType::Float(reg.width))),
            index: reg.index,
        }
    }
}

impl From<&Reg<MemRegT>> for RegOperand {
    fn from(reg: &Reg<MemRegT>) -> Self {
        RegOperand {
            set: RegisterSet::Single(RegType::MemoryAddress),
            index: reg.index,
        }
    }
}

impl From<&Reg<InstrRegT>> for RegOperand {
    fn from(reg: &Reg<InstrRegT>) -> Self {
        RegOperand {
            set: RegisterSet::Single(RegType::InstructionAddress),
            index: reg.index,
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

impl From<&RegOrConstant<MemRegT>> for Operand {
    fn from(value: &RegOrConstant<MemRegT>) -> Self {
        match value {
            RegOrConstant::Reg(reg) => Operand::Reg(reg.into()),
            RegOrConstant::Const(c) => Operand::MemLabelConstant(*c as usize),
        }
    }
}

impl From<&AnySingleRegOrConstant> for Operand {
    fn from(value: &AnySingleRegOrConstant) -> Self {
        match value {
            AnySingleRegOrConstant::Unsigned(x) => x.into(),
            AnySingleRegOrConstant::Signed(x) => x.into(),
            AnySingleRegOrConstant::Float(x) => x.into(),
            AnySingleRegOrConstant::Instr(x) => x.into(),
            AnySingleRegOrConstant::Mem(x) => x.into(),
        }
    }
}

impl From<&AnyReg> for RegOperand {
    fn from(value: &AnyReg) -> Self {
        let (single_reg, length) = match value {
            AnyReg::Single(single_reg) => (single_reg, None),
            AnyReg::Vector(single_reg, length) => (single_reg, Some(length)),
        };
        let mut reg_op: RegOperand = single_reg.into();
        if let Some(length) = length {
            if let RegisterSet::Single(reg_type) = reg_op.set {
                reg_op.set = RegisterSet::Vector(reg_type, *length);
            }
        }
        reg_op
    }
}

impl From<&AnySingleReg> for RegOperand {
    fn from(value: &AnySingleReg) -> Self {
        match value {
            AnySingleReg::Unsigned(reg) => reg.into(),
            AnySingleReg::Signed(reg) => reg.into(),
            AnySingleReg::Float(reg) => reg.into(),
            AnySingleReg::Instr(reg) => reg.into(),
            AnySingleReg::Mem(reg) => reg.into(),
        }
    }
}

impl From<&OffsetOp> for Operand {
    fn from(value: &OffsetOp) -> Self {
        match value {
            OffsetOp::Unsigned(x) => x.into(),
            OffsetOp::Signed(x) => x.into(),
        }
    }
}
