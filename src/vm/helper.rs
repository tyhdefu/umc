use std::cmp::Ordering;

use crate::model::instructions::{
    AnyCoherentNumOp, CompareToZero, ConsistentComparison, ConsistentNumOp, InstrReg, MovParams,
    NotParams, NumReg, RegOrConstant,
};
use crate::vm::state::{ArbStoreFor, RegState, StoreFor};
use crate::vm::types::address::InstructionAddress;
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{
    BinaryArithmeticOp, CastInto, CastSingleSigned, CastSingleUnsigned, UMCArithmetic,
};

pub fn execute_mov(params: &MovParams, state: &mut RegState) {
    match params {
        MovParams::UnsignedInt(r, reg_or_constant) => {
            let num_op = AnyCoherentNumOp::UnsignedInt(ConsistentNumOp::Single(
                r.clone(),
                reg_or_constant.clone(),
                RegOrConstant::Const(0),
            ));
            execute_arithmetic(&num_op, BinaryArithmeticOp::Add, state);
        }
        MovParams::SignedInt(r, reg_or_constant) => {
            let num_op = AnyCoherentNumOp::SignedInt(ConsistentNumOp::Single(
                r.clone(),
                reg_or_constant.clone(),
                RegOrConstant::Const(0),
            ));
            execute_arithmetic(&num_op, BinaryArithmeticOp::Add, state);
        }
        MovParams::Float(_, _) => todo!(),
        MovParams::MemAddress(_, _) => todo!(),
        MovParams::InstrAddress(dst, p) => {
            let addr = read_iaddr(p, state);
            state.store(*dst, addr);
        }
    }
}

pub fn execute_arithmetic(params: &AnyCoherentNumOp, op: BinaryArithmeticOp, state: &mut RegState) {
    match params {
        AnyCoherentNumOp::UnsignedInt(param_kind) => match param_kind {
            ConsistentNumOp::Single(dst, p1, p2) => match dst.width {
                u32::BITS => {
                    let mut p1: u32 = read_uint(&p1, state);
                    let p2: u32 = read_uint(&p2, state);
                    op.operate(&mut p1, &p2);
                    state.store(dst.index, p1);
                }
                u64::BITS => {
                    let mut p1: u64 = read_uint(&p1, state);
                    let p2: u64 = read_uint(&p2, state);
                    op.operate(&mut p1, &p2);
                    state.store(dst.index, p1);
                }
                _ => {
                    let mut p1: ArbitraryUnsignedInt = read_uint(&p1, state);
                    let p2: ArbitraryUnsignedInt = read_uint(&p2, state);
                    p1.resize(dst.width);
                    op.operate(&mut p1, &p2);
                    state.store_arb(dst.index, dst.width, p1);
                }
            },
            ConsistentNumOp::VectorBroadcast(_, _, _) => todo!(),
            ConsistentNumOp::VectorVector(_, _, _) => todo!(),
        },
        AnyCoherentNumOp::SignedInt(param_kind) => match param_kind {
            ConsistentNumOp::Single(dst, p1, p2) => match dst.width {
                i32::BITS => {
                    let mut p1: i32 = read_int(&p1, state);
                    let p2: i32 = read_int(&p2, state);
                    op.operate(&mut p1, &p2);
                    state.store(dst.index, p1);
                }
                i64::BITS => {
                    let mut p1: i32 = read_int(&p1, state);
                    let p2: i32 = read_int(&p2, state);
                    op.operate(&mut p1, &p2);
                    state.store(dst.index, p1);
                }
                _ => todo!(),
            },
            ConsistentNumOp::VectorBroadcast(_, _, _) => todo!(),
            ConsistentNumOp::VectorVector(_, _, _) => todo!(),
        },
        AnyCoherentNumOp::Float(_) => todo!(),
    }
}

fn execute_comparison(comparison: &ConsistentComparison, state: &RegState) -> Option<Ordering> {
    match comparison {
        ConsistentComparison::UnsignedCompare(op1, op2) => {
            let width = op1.width().or(op2.width()).unwrap_or(u64::BITS);
            match width {
                w if w <= u32::BITS => {
                    let v1: u32 = read_uint(op1, state);
                    let v2: u32 = read_uint(op2, state);
                    v1.partial_cmp(&v2)
                }
                w if w < u64::BITS => {
                    let v1: u64 = read_uint(op1, state);
                    let v2: u64 = read_uint(op2, state);
                    v1.partial_cmp(&v2)
                }
                _ => {
                    let v1: ArbitraryUnsignedInt = read_uint(op1, state);
                    let v2: ArbitraryUnsignedInt = read_uint(op2, state);
                    v1.partial_cmp(&v2)
                }
            }
        }
        ConsistentComparison::SignedCompare(op1, op2) => todo!(),
        ConsistentComparison::FloatCompare(op1, op2) => todo!(),
        ConsistentComparison::MemAddressCompare(op1, op2) => todo!(),
        ConsistentComparison::InstrAddressCompare(op1, op2) => todo!(),
    }
}

pub fn execute_not(params: &NotParams, state: &mut RegState) {
    match params {
        NotParams::UnsignedInt(d, p1) => match d.width {
            u32::BITS => {
                let mut v: u32 = read_uint(p1, state);
                v.not();
                state.store(d.index, v);
            }
            u64::BITS => {
                let mut v: u64 = read_uint(p1, state);
                v.not();
                state.store(d.index, v);
            }
            _ => {
                let mut v: ArbitraryUnsignedInt = read_uint(p1, state);
                v.not();
                state.store_arb(d.index, d.width, v);
            }
        },
        NotParams::SignedInt(..) => todo!(),
    }
}

pub fn read_uint<T>(op: &RegOrConstant<NumReg, u64>, state: &RegState) -> T
where
    T: CastSingleUnsigned,
{
    match op {
        RegOrConstant::Reg(num_reg) => match num_reg.width {
            u32::BITS => {
                let v: u32 = state.read(num_reg.index).unwrap_or_default();
                v.cast_into()
            }
            u64::BITS => {
                let v: u64 = state.read(num_reg.index).unwrap_or_default();
                v.cast_into()
            }
            _ => {
                let v: &ArbitraryUnsignedInt = state
                    .read_arb(num_reg.index, num_reg.width)
                    .unwrap_or(ArbitraryUnsignedInt::ZERO_REF);
                v.cast_into()
            }
        },
        RegOrConstant::Const(c) => c.cast_into(),
    }
}

pub fn read_int<T>(op: &RegOrConstant<NumReg, i64>, state: &RegState) -> T
where
    T: CastSingleSigned,
{
    match op {
        RegOrConstant::Reg(num_reg) => match num_reg.width {
            i32::BITS => {
                let v: i32 = state.read(num_reg.index).unwrap_or_default();
                v.cast_into()
            }
            i64::BITS => {
                let v: i64 = state.read(num_reg.index).unwrap_or_default();
                v.cast_into()
            }
            _ => {
                todo!();
            }
        },
        RegOrConstant::Const(c) => c.cast_into(),
    }
}

pub fn read_iaddr(p: &RegOrConstant<InstrReg, usize>, state: &RegState) -> InstructionAddress {
    match p {
        RegOrConstant::Reg(r) => state.read(*r).unwrap_or_default(),
        RegOrConstant::Const(c) => InstructionAddress::new(*c),
    }
}

pub fn is_zero(p: &CompareToZero, state: &RegState) -> bool {
    match p {
        CompareToZero::Unsigned(r) => read_uint::<u32>(r, state) == 0,
        CompareToZero::Signed(r) => read_int::<i32>(r, state) == 0,
    }
}
