use std::cmp::Ordering;

use crate::model::RegWidth;
use crate::model::instructions::{
    AnyCoherentNumOp, BinaryCondition, CompareToZero, ConsistentComparison, ConsistentNumOp,
    InstrReg, MovParams, NotParams, NumReg, RegOrConstant,
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
    fn compute_binary<T, F, R>(op: BinaryArithmeticOp, read: F, p1: &R, p2: &R) -> T
    where
        T: UMCArithmetic,
        F: Fn(&R) -> T,
    {
        let mut p1: T = read(p1);
        let p2: T = read(p2);
        op.operate(&mut p1, &p2);
        p1
    }

    fn compute_unsigned<T>(
        op: BinaryArithmeticOp,
        dst: &NumReg,
        p1: &RegOrConstant<NumReg, u64>,
        p2: &RegOrConstant<NumReg, u64>,
        state: &mut RegState,
    ) where
        T: UMCArithmetic + CastSingleUnsigned + Copy,
        RegState: StoreFor<T>,
    {
        let result: T = compute_binary(op, |r| read_uint(r, state), p1, p2);
        state.store(dst.index, result)
    }

    fn compute_signed<T>(
        op: BinaryArithmeticOp,
        dst: &NumReg,
        p1: &RegOrConstant<NumReg, i64>,
        p2: &RegOrConstant<NumReg, i64>,
        state: &mut RegState,
    ) where
        T: UMCArithmetic + CastSingleSigned + Copy,
        RegState: StoreFor<T>,
    {
        let result: T = compute_binary(op, |r| read_int(r, state), p1, p2);
        state.store(dst.index, result);
    }

    match params {
        AnyCoherentNumOp::UnsignedInt(param_kind) => match param_kind {
            ConsistentNumOp::Single(dst, p1, p2) => match dst.width {
                u32::BITS => compute_unsigned::<u32>(op, dst, p1, p2, state),
                u64::BITS => compute_unsigned::<u64>(op, dst, p1, p2, state),
                _ => {
                    let mut p1: ArbitraryUnsignedInt = read_uint(&p1, state);
                    let p2: ArbitraryUnsignedInt = read_uint(&p2, state);
                    p1.set_bits(dst.width);
                    op.operate(&mut p1, &p2);
                    state.store_arb(dst.index, dst.width, p1);
                }
            },
            ConsistentNumOp::VectorBroadcast(_, _, _) => todo!(),
            ConsistentNumOp::VectorVector(_, _, _) => todo!(),
        },
        AnyCoherentNumOp::SignedInt(param_kind) => match param_kind {
            ConsistentNumOp::Single(dst, p1, p2) => match dst.width {
                i32::BITS => compute_signed::<i32>(op, dst, p1, p2, state),
                i64::BITS => compute_signed::<i64>(op, dst, p1, p2, state),
                _ => todo!(),
            },
            ConsistentNumOp::VectorBroadcast(_, _, _) => todo!(),
            ConsistentNumOp::VectorVector(_, _, _) => todo!(),
        },
        AnyCoherentNumOp::Float(_) => todo!(),
    }
}

pub fn execute_comparison(
    cond: &BinaryCondition,
    dst: &NumReg,
    args: &ConsistentComparison,
    state: &mut RegState,
) {
    let result = compare(&args, state)
        .map(|r| match cond {
            BinaryCondition::Equal => r.is_eq(),
            BinaryCondition::GreaterThan => r.is_gt(),
            BinaryCondition::GreaterThanOrEqualTo => r.is_ge(),
            BinaryCondition::LessThan => r.is_lt(),
            BinaryCondition::LessThanOrEqualTo => r.is_le(),
        })
        .unwrap_or(false);
    match dst.width {
        u32::BITS => {
            state.store(dst.index, result as u32);
        }
        u64::BITS => {
            state.store(dst.index, result as u64);
        }
        w => {
            let v: ArbitraryUnsignedInt = (result as u32).cast_into();
            println!("Storing: {}", v);
            state.store_arb(dst.index, w, v);
        }
    }
}

pub fn compare(comparison: &ConsistentComparison, state: &RegState) -> Option<Ordering> {
    /// Get the largest register widths of the two operands
    /// It is assumed that constants have been validated by the assembler
    /// to use less bits than the other operand
    fn largest_width(a: Option<RegWidth>, b: Option<RegWidth>, default: RegWidth) -> RegWidth {
        match (a, b) {
            (Some(x), Some(y)) => x.max(y),
            (Some(x), None) => x,
            (None, Some(y)) => y,
            (None, None) => default,
        }
    }

    match comparison {
        ConsistentComparison::UnsignedCompare(op1, op2) => {
            let width = largest_width(op1.width(), op2.width(), u64::BITS);
            match width {
                w if w <= u32::BITS => {
                    let v1: u32 = read_uint(op1, state);
                    let v2: u32 = read_uint(op2, state);
                    println!("{v1} vs {v2}");
                    v1.partial_cmp(&v2)
                }
                w if w <= u64::BITS => {
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
        ConsistentComparison::SignedCompare(op1, op2) => {
            let width = op1.width().or(op2.width()).unwrap_or(i64::BITS);
            match width {
                w if w <= i32::BITS => {
                    let v1: i32 = read_int(op1, state);
                    let v2: i32 = read_int(op2, state);
                    v1.partial_cmp(&v2)
                }
                w if w <= i64::BITS => {
                    let v1: i64 = read_int(op1, state);
                    let v2: i64 = read_int(op2, state);
                    v1.partial_cmp(&v2)
                }
                _ => todo!(),
            }
        }
        ConsistentComparison::FloatCompare(_, _) => todo!(),
        ConsistentComparison::MemAddressCompare(_, _) => todo!(),
        ConsistentComparison::InstrAddressCompare(_, _) => todo!(),
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
