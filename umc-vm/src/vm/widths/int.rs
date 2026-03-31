use std::cmp::Ordering;

use crate::vm::helper::read_int;
use crate::vm::memory::{MemoryAccessError, MemoryManager};
use crate::vm::state::{StoreFor, StorePrim};
use crate::vm::types::int::ArbitraryInt;
use crate::vm::types::vector::VecValue;
use crate::vm::types::{BinaryOp, CastInto, UnaryOp};
use crate::vm::widths::{
    WidthBinaryOp, WidthOptions, WidthUnaryOp, compute_broadcast, compute_vector,
};
use umc_model::RegWidth;
use umc_model::instructions::{VectorBroadcastParams, VectorVectorParams};
use umc_model::reg_model::{Reg, RegOrConstant, SignedRegT};

/// Abstraction over the specialised widths provided by the given state implementation
#[derive(Debug)]
pub enum IntWidth {
    I32,
    I64,
    Arbitrary(RegWidth),
}

impl IntWidth {
    /// Convert the given width exactly to a supported integer domain
    pub fn from_width(width: RegWidth) -> Self {
        match width {
            i32::BITS => Self::I32,
            i64::BITS => Self::I64,
            w => Self::Arbitrary(w),
        }
    }

    /// Convert the given width into a integer domain that fits the given width
    /// Useful for comparing two values or if you don't need wrapping behaviour
    pub fn from_width_fitting(width: RegWidth) -> Self {
        match width {
            w if w <= i32::BITS => Self::I32,
            w if w <= i64::BITS => Self::I64,
            w => Self::Arbitrary(w),
        }
    }

    /// Store a u64 into the given register, casting to fit as necessary
    #[allow(unused)]
    pub fn store_i64<S>(reg: Reg<SignedRegT>, state: &mut S, val: i64)
    where
        S: StorePrim<i32, SignedRegT>
            + StorePrim<i64, SignedRegT>
            + StoreFor<ArbitraryInt, SignedRegT>,
    {
        match Self::from_width(reg.width) {
            Self::I32 => state.store_prim(reg, val as i32),
            Self::I64 => state.store_prim(reg, val as i64),
            Self::Arbitrary(w) => {
                let mut arb_val: ArbitraryInt = val.cast_into();
                arb_val.resize_to(w);
                state.store(reg, arb_val);
            }
        };
    }
}

impl<STATE> WidthOptions<STATE> for IntWidth
where
    STATE: StorePrim<i32, SignedRegT>
        + StorePrim<i64, SignedRegT>
        + StoreFor<ArbitraryInt, SignedRegT>,
{
    type RT = SignedRegT;

    fn compare(
        p1: &RegOrConstant<Self::RT>,
        p2: &RegOrConstant<Self::RT>,
        state: &STATE,
    ) -> Option<Ordering> {
        let domain = Self::from_width_fitting(
            p1.width()
                .unwrap_or(u64::BITS)
                .max(p2.width().unwrap_or(u64::BITS)),
        );

        match domain {
            Self::I32 => {
                let v1: i32 = read_int(p1, state);
                let v2: i32 = read_int(p2, state);
                v1.partial_cmp(&v2)
            }
            Self::I64 => {
                let v1: i64 = read_int(p1, state);
                let v2: i64 = read_int(p2, state);
                v1.partial_cmp(&v2)
            }
            Self::Arbitrary(_) => {
                todo!("signed integers");
            }
        }
    }

    fn is_zero(reg: &RegOrConstant<Self::RT>, state: &STATE) -> bool {
        let reg = match reg {
            RegOrConstant::Reg(reg) => reg,
            RegOrConstant::Const(c) => return *c == 0,
        };
        match Self::from_width(reg.width) {
            Self::I32 => {
                let v: i32 = state.read_prim(*reg).unwrap_or(0);
                v == 0
            }
            Self::I64 => {
                let v: i64 = state.read_prim(*reg).unwrap_or(0);
                v == 0
            }
            Self::Arbitrary(_) => {
                let v: &ArbitraryInt = state.read(*reg).unwrap_or(ArbitraryInt::ZERO_REF);
                v == ArbitraryInt::ZERO_REF
            }
        }
    }

    fn store_into_memory<M: MemoryManager>(
        reg: Reg<Self::RT>,
        state: &STATE,
        memory: &mut M,
        address: &M::Address,
    ) -> Result<(), MemoryAccessError<M::Address>> {
        match Self::from_width(reg.width) {
            Self::I32 => {
                let v: i32 = state.read_prim(reg).unwrap_or(0);
                memory.store_prim(v, address)
            }
            Self::I64 => {
                let v: i64 = state.read_prim(reg).unwrap_or(0);
                memory.store_prim(v, address)
            }
            Self::Arbitrary(_) => {
                let v: &ArbitraryInt = state.read(reg).unwrap_or(ArbitraryInt::ZERO_REF);
                // memory.store(v, address)
                todo!("Implement arbitrary int serialisation");
            }
        }
    }
}

impl<STATE, OP> WidthUnaryOp<STATE, OP> for IntWidth
where
    STATE: StorePrim<i32, SignedRegT>
        + StorePrim<i64, SignedRegT>
        + StoreFor<ArbitraryInt, SignedRegT>,
    OP: UnaryOp<i32> + UnaryOp<i64> + UnaryOp<ArbitraryInt>,
{
    type RT = SignedRegT;

    fn operate_unary_in_domain(
        &self,
        dst: Reg<Self::RT>,
        p: &RegOrConstant<Self::RT>,
        state: &mut STATE,
        op: &OP,
    ) {
        match self {
            Self::I32 => {
                let mut v: i32 = read_int(&p, state);
                op.operate(&mut v);
                state.store_prim(dst, v);
            }
            Self::I64 => {
                let mut v: i64 = read_int(&p, state);
                op.operate(&mut v);
                state.store_prim(dst, v);
            }
            Self::Arbitrary(w) => {
                // TODO: Can reduce cost of clone here
                let mut v: ArbitraryInt = read_int(&p, state);
                v.resize_to(*w);
                op.operate(&mut v);
                state.store(dst, v);
            }
        }
    }
}

impl<STATE, OP> WidthBinaryOp<STATE, OP> for IntWidth
where
    STATE: StorePrim<i32, SignedRegT>
        + StorePrim<i64, SignedRegT>
        + StoreFor<ArbitraryInt, SignedRegT>,
    OP: BinaryOp<i32> + BinaryOp<i64> + BinaryOp<ArbitraryInt>,
{
    type RT = SignedRegT;

    /// Operate in the current domain
    fn operate_binary_in_domain(
        &self,
        dst: Reg<Self::RT>,
        p1: &RegOrConstant<Self::RT>,
        p2: &RegOrConstant<Self::RT>,
        state: &mut STATE,
        operation: &OP,
    ) {
        match self {
            Self::I32 => {
                let mut v1: i32 = read_int(p1, state);
                let v2: i32 = read_int(p2, state);
                operation.operate(&mut v1, &v2);
                state.store_prim(dst, v1);
            }
            Self::I64 => {
                let mut v1: i64 = read_int(p1, state);
                let v2: i64 = read_int(p2, state);
                operation.operate(&mut v1, &v2);
                state.store_prim(dst, v1);
            }
            Self::Arbitrary(w) => {
                let mut v1: ArbitraryInt = read_int(p1, state);
                let v2: ArbitraryInt = read_int(p2, state);
                v1.resize_to(*w);
                operation.operate(&mut v1, &v2);
                state.store(dst, v1);
            }
        }
    }
    /// Operate a vector-value operation in the given domain
    fn operate_binary_broadcast_in_domain(
        &self,
        params: &VectorBroadcastParams<Self::RT>,
        state: &mut STATE,
        op: &OP,
    ) {
        let length = params.length() as usize;

        match self {
            Self::I32 => {
                let mut vector: VecValue<i32> = state
                    .read_multi_prim(params.vec_param(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let v: i32 = read_int(params.value_param(), state);
                compute_broadcast(&mut vector, &v, op, params.is_reversed());
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            Self::I64 => {
                let mut vector: VecValue<i64> = state
                    .read_multi_prim(params.vec_param(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let v: i64 = read_int(params.value_param(), state);
                compute_broadcast(&mut vector, &v, op, params.is_reversed());
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            Self::Arbitrary(w) => {
                let mut vector: VecValue<ArbitraryInt> = state
                    .read_multi(params.vec_param(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let v: ArbitraryInt = read_int(params.value_param(), state);

                for x in vector.as_slice_mut() {
                    x.resize_to(*w);
                }

                compute_broadcast(&mut vector, &v, op, params.is_reversed());
                state.store_multi_clone(*params.dst(), vector.as_slice());
            }
        }
    }

    fn operate_binary_vector_in_domain(
        &self,
        params: &VectorVectorParams<Self::RT>,
        state: &mut STATE,
        op: &OP,
    ) {
        let length = params.length() as usize;
        match self {
            Self::I32 => {
                let mut vector: VecValue<i32> = state
                    .read_multi_prim(params.p1(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let param: Option<&VecValue<i32>> = state.read_multi_prim(params.p2(), length);
                compute_vector(&mut vector, param, &0, op);
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            Self::I64 => {
                let mut vector: VecValue<i64> = state
                    .read_multi_prim(params.p1(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let param: Option<&VecValue<i64>> = state.read_multi_prim(params.p2(), length);
                compute_vector(&mut vector, param, &0, op);
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            Self::Arbitrary(w) => {
                let mut vector: VecValue<ArbitraryInt> = state
                    .read_multi(params.p1(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let param: Option<&VecValue<ArbitraryInt>> = state.read_multi(params.p2(), length);
                for v in vector.as_slice_mut() {
                    v.resize_to(*w);
                }
                compute_vector(&mut vector, param, ArbitraryInt::ZERO_REF, op);
                state.store_multi_clone(*params.dst(), vector.as_slice());
            }
        }
    }
}
