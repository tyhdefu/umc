use std::cmp::Ordering;

use crate::vm::helper::read_uint;
use crate::vm::memory::{MemoryAccessError, MemoryManager};
use crate::vm::state::{StoreFor, StorePrim};
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::vector::VecValue;
use crate::vm::types::{BinaryOp, CastInto, UnaryOp};
use crate::vm::widths::{
    WidthBinaryOp, WidthOptions, WidthUnaryOp, compute_broadcast, compute_vector,
};
use umc_model::RegWidth;
use umc_model::instructions::{VectorBroadcastParams, VectorVectorParams};
use umc_model::reg_model::{Reg, RegOrConstant, UnsignedRegT};

/// Abstraction over the specialised widths provided by the given state implementation
#[derive(Debug)]
pub enum UIntWidth {
    U32,
    U64,
    Arbitrary(RegWidth),
}

impl UIntWidth {
    /// Convert the given width exactly to a supported integer domain
    pub fn from_width(width: RegWidth) -> Self {
        match width {
            u32::BITS => Self::U32,
            u64::BITS => Self::U64,
            w => Self::Arbitrary(w),
        }
    }

    /// Convert the given width into a integer domain that fits the given width
    /// Useful for comparing two values or if you don't need wrapping behaviour
    pub fn from_width_fitting(width: RegWidth) -> Self {
        match width {
            w if w <= u32::BITS => Self::U32,
            w if w <= u64::BITS => Self::U64,
            w => Self::Arbitrary(w),
        }
    }

    /// Store a u64 into the given register, casting to fit as necessary
    pub fn store_u64<S>(reg: Reg<UnsignedRegT>, state: &mut S, val: u64)
    where
        S: StorePrim<u32, UnsignedRegT>
            + StorePrim<u64, UnsignedRegT>
            + StoreFor<ArbitraryUnsignedInt, UnsignedRegT>,
    {
        match Self::from_width(reg.width) {
            Self::U32 => state.store_prim(reg, val as u32),
            Self::U64 => state.store_prim(reg, val as u64),
            Self::Arbitrary(w) => {
                let mut arb_val: ArbitraryUnsignedInt = val.cast_into();
                arb_val.resize_to(w);
                state.store(reg, arb_val);
            }
        };
    }
}

impl<STATE> WidthOptions<STATE> for UIntWidth
where
    STATE: StorePrim<u32, UnsignedRegT>
        + StorePrim<u64, UnsignedRegT>
        + StoreFor<ArbitraryUnsignedInt, UnsignedRegT>,
{
    type RT = UnsignedRegT;

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
            Self::U32 => {
                let v1: u32 = read_uint(p1, state);
                let v2: u32 = read_uint(p2, state);
                v1.partial_cmp(&v2)
            }
            Self::U64 => {
                let v1: u64 = read_uint(p1, state);
                let v2: u64 = read_uint(p2, state);
                v1.partial_cmp(&v2)
            }
            Self::Arbitrary(_) => {
                // TODO: Only need references
                let v1: ArbitraryUnsignedInt = read_uint(p1, state);
                let v2: ArbitraryUnsignedInt = read_uint(p1, state);

                v1.partial_cmp(&v2)
            }
        }
    }

    fn is_zero(reg: &RegOrConstant<Self::RT>, state: &STATE) -> bool {
        let reg = match reg {
            RegOrConstant::Reg(reg) => reg,
            RegOrConstant::Const(c) => return *c == 0,
        };
        match Self::from_width(reg.width) {
            Self::U32 => {
                let v: u32 = state.read_prim(*reg).unwrap_or(0);
                v == 0
            }
            Self::U64 => {
                let v: u64 = state.read_prim(*reg).unwrap_or(0);
                v == 0
            }
            Self::Arbitrary(_) => {
                let v: &ArbitraryUnsignedInt =
                    state.read(*reg).unwrap_or(ArbitraryUnsignedInt::ZERO_REF);
                v == ArbitraryUnsignedInt::ZERO_REF
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
            Self::U32 => {
                let v: u32 = state.read_prim(reg).unwrap_or(0);
                memory.store_prim(v, address)
            }
            Self::U64 => {
                let v: u64 = state.read_prim(reg).unwrap_or(0);
                memory.store_prim(v, address)
            }
            Self::Arbitrary(_) => {
                let v: &ArbitraryUnsignedInt =
                    state.read(reg).unwrap_or(ArbitraryUnsignedInt::ZERO_REF);
                memory.store(v, address)
            }
        }
    }
}

impl<STATE, OP> WidthUnaryOp<STATE, OP> for UIntWidth
where
    STATE: StorePrim<u32, UnsignedRegT>
        + StorePrim<u64, UnsignedRegT>
        + StoreFor<ArbitraryUnsignedInt, UnsignedRegT>,
    OP: UnaryOp<u32> + UnaryOp<u64> + UnaryOp<ArbitraryUnsignedInt>,
{
    type RT = UnsignedRegT;

    fn operate_unary_in_domain(
        &self,
        dst: Reg<Self::RT>,
        p: &RegOrConstant<Self::RT>,
        state: &mut STATE,
        op: &OP,
    ) {
        match self {
            Self::U32 => {
                let mut v: u32 = read_uint(&p, state);
                op.operate(&mut v);
                state.store_prim(dst, v);
            }
            Self::U64 => {
                let mut v: u64 = read_uint(&p, state);
                op.operate(&mut v);
                state.store_prim(dst, v);
            }
            Self::Arbitrary(w) => {
                // TODO: Can reduce cost of clone here
                let mut v: ArbitraryUnsignedInt = read_uint(&p, state);
                v.resize_to(*w);
                op.operate(&mut v);
                state.store(dst, v);
            }
        }
    }
}

impl<STATE, OP> WidthBinaryOp<STATE, OP> for UIntWidth
where
    STATE: StorePrim<u32, UnsignedRegT>
        + StorePrim<u64, UnsignedRegT>
        + StoreFor<ArbitraryUnsignedInt, UnsignedRegT>,
    OP: BinaryOp<u32> + BinaryOp<u64> + BinaryOp<ArbitraryUnsignedInt>,
{
    type RT = UnsignedRegT;

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
            Self::U32 => {
                let mut v1: u32 = read_uint(p1, state);
                let v2: u32 = read_uint(p2, state);
                operation.operate(&mut v1, &v2);
                state.store_prim(dst, v1);
            }
            Self::U64 => {
                let mut v1: u64 = read_uint(p1, state);
                let v2: u64 = read_uint(p2, state);
                operation.operate(&mut v1, &v2);
                state.store_prim(dst, v1);
            }
            Self::Arbitrary(w) => {
                let mut v1: ArbitraryUnsignedInt = read_uint(p1, state);
                let v2: ArbitraryUnsignedInt = read_uint(p2, state);
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
            Self::U32 => {
                let mut vector: VecValue<u32> = state
                    .read_multi_prim(params.vec_param(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let v: u32 = read_uint(params.value_param(), state);
                compute_broadcast(&mut vector, &v, op, params.is_reversed());
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            Self::U64 => {
                let mut vector: VecValue<u64> = state
                    .read_multi_prim(params.vec_param(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let v: u64 = read_uint(params.value_param(), state);
                compute_broadcast(&mut vector, &v, op, params.is_reversed());
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            Self::Arbitrary(w) => {
                let mut vector: VecValue<ArbitraryUnsignedInt> = state
                    .read_multi(params.vec_param(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let v: ArbitraryUnsignedInt = read_uint(params.value_param(), state);

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
            Self::U32 => {
                let mut vector: VecValue<u32> = state
                    .read_multi_prim(params.p1(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let param: Option<&VecValue<u32>> = state.read_multi_prim(params.p2(), length);
                compute_vector(&mut vector, param, &0, op);
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            Self::U64 => {
                let mut vector: VecValue<u64> = state
                    .read_multi_prim(params.p1(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let param: Option<&VecValue<u64>> = state.read_multi_prim(params.p2(), length);
                compute_vector(&mut vector, param, &0, op);
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            Self::Arbitrary(w) => {
                let mut vector: VecValue<ArbitraryUnsignedInt> = state
                    .read_multi(params.p1(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let param: Option<&VecValue<ArbitraryUnsignedInt>> =
                    state.read_multi(params.p2(), length);
                for v in vector.as_slice_mut() {
                    v.resize_to(*w);
                }
                compute_vector(&mut vector, param, ArbitraryUnsignedInt::ZERO_REF, op);
                state.store_multi_clone(*params.dst(), vector.as_slice());
            }
        }
    }
}
