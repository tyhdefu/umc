use std::cmp::Ordering;
use std::iter::repeat_n;

use crate::vm::memory::safe::SafeAddress;
use crate::vm::state::{RegState as RegStateRaw, StoreFor, StorePrim};
use crate::vm::types::address::InstructionAddress;
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::vector::VecValue;
use crate::vm::types::{
    BinaryArithmeticOp, BinaryBitwiseOp, BinaryOp, CastInto, CastSingleFloat, CastSingleSigned,
    CastSingleUnsigned, UMCBitwise,
};
use umc_model::RegWidth;
use umc_model::instructions::{
    AnyConsistentNumOp, AnyReg, AnySingleReg, BinaryCondition, CompareParams, CompareToZero,
    ConsistentComparison, ConsistentOp, MovParams, NotParams, VectorBroadcastParams,
    VectorVectorParams,
};
use umc_model::reg_model::{
    FloatRegT, InstrRegT, Reg, RegOrConstant, RegTypeT, SignedRegT, UnsignedRegT,
};

// TODO: Make these helpers and the VM have a switchable Memory implementation
type RegState = RegStateRaw<SafeAddress>;

pub fn execute_mov(params: &MovParams, state: &mut RegState) {
    match params {
        MovParams::UnsignedInt(r, reg_or_constant) => {
            let num_op = AnyConsistentNumOp::UnsignedInt(ConsistentOp::Single(
                r.clone(),
                reg_or_constant.clone(),
                RegOrConstant::Const(0),
            ));
            execute_arithmetic(&num_op, BinaryArithmeticOp::Add, state);
        }
        MovParams::SignedInt(r, reg_or_constant) => {
            let num_op = AnyConsistentNumOp::SignedInt(ConsistentOp::Single(
                r.clone(),
                reg_or_constant.clone(),
                RegOrConstant::Const(0),
            ));
            execute_arithmetic(&num_op, BinaryArithmeticOp::Add, state);
        }
        MovParams::Float(r, reg_or_constant) => {
            let num_op = AnyConsistentNumOp::Float(ConsistentOp::Single(
                r.clone(),
                reg_or_constant.clone(),
                RegOrConstant::Const(0.0),
            ));
            execute_arithmetic(&num_op, BinaryArithmeticOp::Add, state);
        }
        MovParams::MemAddress(_, _) => todo!(),
        MovParams::InstrAddress(dst, p) => {
            let addr = read_iaddr(p, state);
            state.store(*dst, addr);
        }
    }
}

fn compute_binary<T, F, R, O>(op: O, read: F, p1: &R, p2: &R) -> T
where
    O: BinaryOp<T>,
    F: Fn(&R) -> T,
{
    let mut p1: T = read(p1);
    let p2: T = read(p2);
    op.operate(&mut p1, &p2);
    p1
}

fn compute_unsigned<O, T>(
    op: O,
    dst: &Reg<UnsignedRegT>,
    p1: &RegOrConstant<UnsignedRegT>,
    p2: &RegOrConstant<UnsignedRegT>,
    state: &mut RegState,
) where
    O: BinaryOp<T>,
    T: CastSingleUnsigned + Copy,
    RegState: StorePrim<T, UnsignedRegT>,
{
    let result: T = compute_binary(op, |r| read_uint(r, state), p1, p2);
    state.store_prim(*dst, result);
}

fn compute_unsigned_broadcast<O, T>(
    op: O,
    params: &VectorBroadcastParams<UnsignedRegT>,
    state: &mut RegState,
) where
    O: BinaryOp<T>,
    T: CastSingleUnsigned + Copy + Default,
    RegState: StorePrim<T, UnsignedRegT>,
{
    let mut x: VecValue<T> = state
        .read_multi_prim(params.vec_param(), params.length() as usize)
        .cloned()
        .unwrap_or_else(|| VecValue::from_repeated_default(params.length() as usize));

    let v: T = read_uint(params.value_param(), state);

    if params.is_reversed() {
        x.broadcast_op_reversed(&v, |a, b| op.operate(a, b));
    } else {
        x.broadcast_op(&v, |a, b| op.operate(a, b));
    }

    state.store_multi_copy_prim(*params.dst(), x.as_slice());
}

fn compute_vec<O, RT, T>(op: O, params: &VectorVectorParams<RT>, state: &mut RegState)
where
    O: BinaryOp<T>,
    RT: RegTypeT,
    T: Copy + Default,
    RegState: StorePrim<T, RT>,
{
    let mut x: VecValue<T> = state
        .read_multi_prim(params.p1(), params.length() as usize)
        .cloned()
        .unwrap_or_else(|| VecValue::from_repeated_default(params.length() as usize));
    let y: Option<&VecValue<T>> = state.read_multi_prim(params.p2(), params.length() as usize);

    match y {
        Some(y) => {
            x.vector_op(y, |a, b| op.operate(a, b));
        }
        None => {
            let zero = Default::default();
            for v in x.as_slice_mut() {
                op.operate(v, &zero);
            }
        }
    }

    state.store_multi_copy_prim(params.dst().clone(), x.as_slice());
}

fn compute_signed<O, T>(
    op: O,
    dst: &Reg<SignedRegT>,
    p1: &RegOrConstant<SignedRegT>,
    p2: &RegOrConstant<SignedRegT>,
    state: &mut RegState,
) where
    O: BinaryOp<T>,
    T: CastSingleSigned + Copy,
    RegState: StorePrim<T, SignedRegT>,
{
    let result: T = compute_binary(op, |r| read_int(r, state), p1, p2);
    state.store_prim(*dst, result);
}

fn compute_float<O, T>(
    op: O,
    dst: &Reg<FloatRegT>,
    p1: &RegOrConstant<FloatRegT>,
    p2: &RegOrConstant<FloatRegT>,
    state: &mut RegState,
) where
    O: BinaryOp<T>,
    T: CastSingleFloat + Copy,
    RegState: StorePrim<T, FloatRegT>,
{
    let result: T = compute_binary(op, |r| read_float(r, state), p1, p2);
    state.store_prim(*dst, result);
}

pub fn execute_arithmetic(
    params: &AnyConsistentNumOp,
    op: BinaryArithmeticOp,
    state: &mut RegState,
) {
    match params {
        AnyConsistentNumOp::UnsignedInt(param_kind) => match param_kind {
            ConsistentOp::Single(dst, p1, p2) => match dst.width {
                u32::BITS => compute_unsigned::<_, u32>(op, dst, p1, p2, state),
                u64::BITS => compute_unsigned::<_, u64>(op, dst, p1, p2, state),
                _ => {
                    let mut p1: ArbitraryUnsignedInt = read_uint(&p1, state);
                    let p2: ArbitraryUnsignedInt = read_uint(&p2, state);
                    p1.set_bits(dst.width);
                    op.operate(&mut p1, &p2);
                    state.store(*dst, p1);
                }
            },
            ConsistentOp::VectorBroadcast(params) => match params.width() {
                u32::BITS => compute_unsigned_broadcast::<_, u32>(op, params, state),
                u64::BITS => compute_unsigned_broadcast::<_, u32>(op, params, state),
                _ => todo!("Unsigned Arbitrary Vectors todo"),
            },
            ConsistentOp::VectorVector(params) => match params.width() {
                u32::BITS => compute_vec::<_, _, u32>(op, params, state),
                u64::BITS => compute_vec::<_, _, u64>(op, params, state),
                _ => todo!("Unsigned Arbitrary Vectors todo"),
            },
        },
        AnyConsistentNumOp::SignedInt(param_kind) => match param_kind {
            ConsistentOp::Single(dst, p1, p2) => match dst.width {
                i32::BITS => compute_signed::<_, i32>(op, dst, p1, p2, state),
                i64::BITS => compute_signed::<_, i64>(op, dst, p1, p2, state),
                _ => todo!(),
            },
            ConsistentOp::VectorBroadcast(_) => todo!(),
            ConsistentOp::VectorVector(_) => todo!(),
        },
        AnyConsistentNumOp::Float(param_kind) => match param_kind {
            ConsistentOp::Single(dst, p1, p2) => match dst.width {
                32 => compute_float::<_, f32>(op, dst, p1, p2, state),
                64 => compute_float::<_, f64>(op, dst, p1, p2, state),
                _ => panic!("Floats must be 32 or 64-bit"),
            },
            ConsistentOp::VectorBroadcast(_) => todo!(),
            ConsistentOp::VectorVector(_) => todo!(),
        },
    }
}

pub fn execute_bitwise(params: &AnyConsistentNumOp, op: BinaryBitwiseOp, state: &mut RegState) {
    match params {
        AnyConsistentNumOp::UnsignedInt(num_op) => match num_op {
            ConsistentOp::Single(dst, p1, p2) => match dst.width {
                u32::BITS => compute_unsigned::<_, u32>(op, dst, p1, p2, state),
                u64::BITS => compute_unsigned::<_, u64>(op, dst, p1, p2, state),
                _ => {
                    let mut p1: ArbitraryUnsignedInt = read_uint(&p1, state);
                    let p2: ArbitraryUnsignedInt = read_uint(&p2, state);
                    p1.set_bits(dst.width);
                    op.operate(&mut p1, &p2);
                    state.store(*dst, p1);
                }
            },
            ConsistentOp::VectorBroadcast(_) => todo!(),
            ConsistentOp::VectorVector(_) => todo!(),
        },
        AnyConsistentNumOp::SignedInt(num_op) => match num_op {
            ConsistentOp::Single(dst, p1, p2) => match dst.width {
                i32::BITS => compute_signed::<_, i32>(op, dst, p1, p2, state),
                i64::BITS => compute_signed::<_, i64>(op, dst, p1, p2, state),
                _ => todo!(),
            },
            ConsistentOp::VectorBroadcast(_) => todo!(),
            ConsistentOp::VectorVector(_) => todo!(),
        },
        AnyConsistentNumOp::Float(_) => panic!("TODO: Make new num op for bitwise"),
    }
}

pub fn execute_comparison(cond: &BinaryCondition, params: &CompareParams, state: &mut RegState) {
    let result = compare(&params.args, state)
        .map(|r| match cond {
            BinaryCondition::Equal => r.is_eq(),
            BinaryCondition::GreaterThan => r.is_gt(),
            BinaryCondition::GreaterThanOrEqualTo => r.is_ge(),
            BinaryCondition::LessThan => r.is_lt(),
            BinaryCondition::LessThanOrEqualTo => r.is_le(),
        })
        .unwrap_or(false);
    let dst = &params.dst;
    match dst.width {
        u32::BITS => {
            state.store_prim(*dst, result as u32);
        }
        u64::BITS => {
            state.store_prim(*dst, result as u64);
        }
        _ => {
            let v: ArbitraryUnsignedInt = (result as u32).cast_into();
            state.store(*dst, v);
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
        ConsistentComparison::FloatCompare(op1, op2) => {
            let width = op1.width().or(op2.width()).unwrap_or(64);
            match width {
                w if w <= 32 => {
                    let v1: f32 = read_float(op1, state);
                    let v2: f32 = read_float(op2, state);
                    v1.partial_cmp(&v2)
                }
                w if w <= 64 => {
                    let v1: f64 = read_float(op1, state);
                    let v2: f64 = read_float(op2, state);
                    v1.partial_cmp(&v2)
                }
                _ => panic!("Only 32-bit or 64-bit floats supported"),
            }
        }
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
                state.store_prim(*d, v);
            }
            u64::BITS => {
                let mut v: u64 = read_uint(p1, state);
                v.not();
                state.store_prim(*d, v);
            }
            _ => {
                let mut v: ArbitraryUnsignedInt = read_uint(p1, state);
                v.not();
                state.store(*d, v);
            }
        },
        NotParams::SignedInt(..) => todo!(),
    }
}

pub fn execute_debug(reg: &AnyReg, state: &RegState) {
    match reg {
        AnyReg::Single(AnySingleReg::Unsigned(reg)) => {
            let x: ArbitraryUnsignedInt = read_uint(&RegOrConstant::Reg(reg.clone()), state);
            println!("{} = {}", reg, x);
        }
        AnyReg::Single(AnySingleReg::Signed(reg)) => {
            let reg_ref = RegOrConstant::Reg(reg.clone());
            let x: i64 = read_int(&reg_ref, state);
            println!("{} = {:X}", reg_ref, x);
        }
        AnyReg::Single(AnySingleReg::Float(reg)) => {
            let reg_ref = RegOrConstant::Reg(reg.clone());
            let x: f64 = read_float(&reg_ref, state);
            println!("{} = {}", reg_ref, x);
        }
        AnyReg::Single(AnySingleReg::Instr(reg)) => {
            let reg_ref = RegOrConstant::Reg(reg.clone());
            let x: InstructionAddress = read_iaddr(&reg_ref, state);
            println!("{} = {:?}", reg_ref, x);
        }
        AnyReg::Single(AnySingleReg::Mem(_)) => todo!(),
        AnyReg::Vector(AnySingleReg::Unsigned(reg), l) => {
            let x: Vec<ArbitraryUnsignedInt> =
                read_uint_vec(&reg, *l, state).unwrap_or_else(|| {
                    repeat_n(ArbitraryUnsignedInt::ZERO.clone(), *l as usize).collect()
                });
            println!("{}", VecValue::from_vec(x));
        }
        _ => todo!("debug on this register not yet supported"),
    }
}

pub fn read_uint<T>(op: &RegOrConstant<UnsignedRegT>, state: &RegState) -> T
where
    T: CastSingleUnsigned,
{
    match op {
        RegOrConstant::Reg(num_reg) => match num_reg.width {
            u32::BITS => {
                let v: u32 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            u64::BITS => {
                let v: u64 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            _ => {
                let v: &ArbitraryUnsignedInt = state
                    .read(*num_reg)
                    .unwrap_or(ArbitraryUnsignedInt::ZERO_REF);
                v.cast_into()
            }
        },
        RegOrConstant::Const(c) => c.cast_into(),
    }
}

pub fn read_uint_vec<T>(
    reg: &Reg<UnsignedRegT>,
    length: RegWidth,
    state: &RegState,
) -> Option<Vec<T>>
where
    T: CastSingleUnsigned,
{
    Some(match reg.width {
        u32::BITS => {
            let v: &VecValue<u32> = state.read_multi_prim(*reg, length as usize)?;
            v.as_slice().iter().map(|x| x.cast_into()).collect()
        }
        u64::BITS => {
            let v: &VecValue<u64> = state.read_multi_prim(*reg, length as usize)?;
            v.as_slice().iter().map(|x| x.cast_into()).collect()
        }
        _ => {
            let v: &VecValue<ArbitraryUnsignedInt> = state.read_multi(*reg, length as usize)?;
            v.as_slice().iter().map(|x| x.cast_into()).collect()
        }
    })
}

pub fn read_int<T>(op: &RegOrConstant<SignedRegT>, state: &RegState) -> T
where
    T: CastSingleSigned,
{
    match op {
        RegOrConstant::Reg(num_reg) => match num_reg.width {
            i32::BITS => {
                let v: i32 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            i64::BITS => {
                let v: i64 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            _ => {
                todo!();
            }
        },
        RegOrConstant::Const(c) => c.cast_into(),
    }
}

pub fn read_float<T>(op: &RegOrConstant<FloatRegT>, state: &RegState) -> T
where
    T: CastSingleFloat,
{
    match op {
        RegOrConstant::Reg(num_reg) => match num_reg.width {
            32 => {
                let v: f32 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            64 => {
                let v: f64 = state.read_prim(*num_reg).unwrap_or_default();
                v.cast_into()
            }
            _ => panic!("Floats can only be 32-bit or 64-bit"),
        },
        RegOrConstant::Const(c) => c.cast_into(),
    }
}

pub fn read_iaddr(p: &RegOrConstant<InstrRegT>, state: &RegState) -> InstructionAddress {
    match p {
        RegOrConstant::Reg(r) => state.read(*r).copied().unwrap_or_default(),
        RegOrConstant::Const(c) => InstructionAddress::new(*c),
    }
}

pub fn is_zero(p: &CompareToZero, state: &RegState) -> bool {
    // TODO: This isn't right
    match p {
        CompareToZero::Unsigned(r) => read_uint::<u32>(r, state) == 0,
        CompareToZero::Signed(r) => read_int::<i32>(r, state) == 0,
    }
}
