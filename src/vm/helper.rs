use crate::bytecode::{Operand, RegOperand};
use crate::model::{NumRegType, RegIndex, RegType, RegisterSet};
use crate::vm::state::{ArbStoreFor, RegState, StoreFor};
use crate::vm::types::address::InstructionAddress;
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{
    AddSubOp, BinaryArithmeticOp, CastInto, CastSingleAny, UMCAddSub, UMCArithmetic,
};

pub fn compute_mov(state: &mut RegState, dst: &RegOperand, src: &Operand) {
    compute_addsub(
        state,
        dst,
        src,
        &Operand::UnsignedConstant(0),
        AddSubOp::Add,
    );
}

pub fn compute_addsub(
    state: &mut RegState,
    dst_op: &RegOperand,
    op1: &Operand,
    op2: &Operand,
    add_sub_op: AddSubOp,
) {
    match &dst_op.set {
        RegisterSet::Single(RegType::Num(num)) => {
            compute_arith(
                state,
                &num,
                dst_op.index,
                op1,
                op2,
                BinaryArithmeticOp::AddOrSub(add_sub_op),
            );
        }
        RegisterSet::Single(RegType::MemoryAddress) => todo!(),
        RegisterSet::Single(RegType::InstructionAddress) => {
            let mut v1: InstructionAddress = read_single_as_iaddress(state, op1).unwrap();
            let v2: InstructionAddress = read_single_as_iaddress(state, op2).unwrap();
            v1.add(&v2);
            state.store(dst_op.index, v1);
        }
        RegisterSet::Vector(_, _) => todo!(),
    };
}

pub fn compute_arith(
    state: &mut RegState,
    dst_type: &NumRegType,
    dst_idx: RegIndex,
    op1: &Operand,
    op2: &Operand,
    operation: BinaryArithmeticOp,
) {
    fn comp<T>(
        state: &mut RegState,
        idx: RegIndex,
        op1: &Operand,
        op2: &Operand,
        operation: BinaryArithmeticOp,
    ) where
        RegState: StoreFor<T>,
        T: Copy + Default + UMCArithmetic + CastSingleAny,
    {
        let mut v1: T = read_single_as(state, op1).unwrap();
        let v2: T = read_single_as(state, op2).unwrap();
        operation.operate(&mut v1, &v2);
        state.store(idx, v1);
    }

    match dst_type {
        NumRegType::UnsignedInt(u32::BITS) => comp::<u32>(state, dst_idx, op1, op2, operation),
        NumRegType::UnsignedInt(u64::BITS) => comp::<u64>(state, dst_idx, op1, op2, operation),
        NumRegType::UnsignedInt(w) => {
            let mut v1: ArbitraryUnsignedInt = read_single_as(state, op1).unwrap();
            let v2: ArbitraryUnsignedInt = read_single_as(state, op2).unwrap();
            operation.operate(&mut v1, &v2);
            state.store_arb(dst_idx, *w, v1);
        }
        NumRegType::SignedInt(i32::BITS) => todo!(), //comp::<i32>(state, dst_idx, op1, op2, operation),
        NumRegType::SignedInt(i64::BITS) => todo!(), //comp::<i64>(state, dst_idx, op1, op2, operation),
        NumRegType::SignedInt(_) => todo!(),
        NumRegType::Float(_) => todo!(),
    }
}

/// Read a single value and cast it to the specified type if required.
pub fn read_single_as<'a, T>(state: &'a RegState, operand: &Operand) -> Result<T, ()>
where
    T: CastSingleAny + Default,
{
    match operand {
        Operand::Reg(reg) => match &reg.set {
            RegisterSet::Single(RegType::Num(num)) => {
                Ok(read_single_num_as::<T>(state, &num, reg.index))
            }
            RegisterSet::Single(RegType::InstructionAddress) => Err(()),
            RegisterSet::Single(RegType::MemoryAddress) => Err(()),
            RegisterSet::Vector(_, _) => Err(()),
        },
        Operand::UnsignedConstant(c) => Ok((*c).cast_into()),
        Operand::LabelConstant(_) => Err(()),
    }
}

pub fn read_single_num_as<T>(state: &RegState, op_type: &NumRegType, idx: RegIndex) -> T
where
    T: CastSingleAny + Default,
{
    match op_type {
        NumRegType::UnsignedInt(u32::BITS) => {
            let v: u32 = state.read(idx).unwrap_or_default();
            v.cast_into()
        }
        NumRegType::UnsignedInt(u64::BITS) => {
            let v: u64 = state.read(idx).unwrap_or_default();
            v.cast_into()
        }
        NumRegType::UnsignedInt(w) => {
            let v: ArbitraryUnsignedInt = state.read_arb(idx, *w).cloned().unwrap_or_default();
            v.cast_into()
        }
        NumRegType::SignedInt(_) => todo!(),
        NumRegType::Float(_) => todo!(),
    }
}

pub fn read_single_as_iaddress(
    state: &RegState,
    operand: &Operand,
) -> Result<InstructionAddress, ()> {
    match operand {
        Operand::Reg(reg) => match reg.set {
            RegisterSet::Single(RegType::InstructionAddress) => {
                let v: InstructionAddress = state.read(reg.index).unwrap_or_default();
                Ok(v.cast_into())
            }
            _ => read_single_as(state, operand),
        },
        Operand::UnsignedConstant(c) => Ok(InstructionAddress::new(*c as usize)),
        Operand::LabelConstant(c) => Ok(InstructionAddress::new(*c)),
    }
}
