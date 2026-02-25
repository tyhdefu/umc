//! Abstractions over specialised widths in UMC

use std::cmp::Ordering;

use umc_model::RegWidth;
use umc_model::instructions::{VectorBroadcastParams, VectorVectorParams};
use umc_model::reg_model::{Reg, RegOrConstant, RegTypeT, UnsignedRegT};

use crate::vm::helper::read_uint;
use crate::vm::memory::{MemoryAccessError, MemoryManager};
use crate::vm::state::{StoreFor, StorePrim};
use crate::vm::types::UnaryOp;
use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::vector::VecValue;
use crate::vm::types::{BinaryOp, CastInto};

/// Provides useful operations that handle dealing with different widths
pub trait WidthOptions<STATE> {
    type RT: RegTypeT;

    /// Compare two values
    fn compare(
        p1: &RegOrConstant<Self::RT>,
        p2: &RegOrConstant<Self::RT>,
        state: &STATE,
    ) -> Option<Ordering>;

    /// Check if the given value is zero
    fn is_zero(reg: &RegOrConstant<Self::RT>, state: &STATE) -> bool;

    /// Store the given value into memory
    fn store_into_memory<M: MemoryManager>(
        reg: Reg<Self::RT>,
        state: &STATE,
        memory: &mut M,
        address: &M::Address,
    ) -> Result<(), MemoryAccessError<M::Address>>;
}

pub trait WidthUnaryOp<STATE, OP> {
    type RT: RegTypeT;

    /// Perform a unary operation on a value, and store it into the destination
    fn operate_unary_in_domain(
        &self,
        dst: Reg<Self::RT>,
        p: &RegOrConstant<Self::RT>,
        state: &mut STATE,
        op: &OP,
    );
}

pub trait WidthBinaryOp<STATE, OP> {
    type RT: RegTypeT;

    /// Peform a binary operation on a value, and store it into the destination
    fn operate_binary_in_domain(
        &self,
        dst: Reg<Self::RT>,
        p1: &RegOrConstant<Self::RT>,
        p2: &RegOrConstant<Self::RT>,
        state: &mut STATE,
        operation: &OP,
    );

    /// Peform a binary broadcasting operation, and store the result into the destination
    fn operate_binary_broadcast_in_domain(
        &self,
        params: &VectorBroadcastParams<Self::RT>,
        state: &mut STATE,
        op: &OP,
    );

    /// Peform a binary vector-vector operation, and tstore the result into the destination
    fn operate_binary_vector_in_domain(
        &self,
        params: &VectorVectorParams<Self::RT>,
        op: &OP,
        state: &mut STATE,
    );
}

////////// Specific implementations ///////////

/// Abstraction over the specialised widths provided by the given state implementation
pub enum UIntWidth {
    U32,
    U64,
    Arbitrary(RegWidth),
}

impl<STATE> WidthOptions<STATE> for UIntWidth
where
    STATE: StorePrim<u32, UnsignedRegT>
        + StorePrim<u64, UnsignedRegT>
        + StoreFor<ArbitraryUnsignedInt, UnsignedRegT>,
{
    type RT = UnsignedRegT;

    fn compare(
        p1: &RegOrConstant<UnsignedRegT>,
        p2: &RegOrConstant<UnsignedRegT>,
        state: &STATE,
    ) -> Option<Ordering> {
        let domain = Self::from_width_fitting(largest_width(p1.width(), p2.width(), u64::BITS));

        match domain {
            UIntWidth::U32 => {
                let v1: u32 = read_uint(p1, state);
                let v2: u32 = read_uint(p2, state);
                v1.partial_cmp(&v2)
            }
            UIntWidth::U64 => {
                let v1: u32 = read_uint(p1, state);
                let v2: u32 = read_uint(p2, state);
                v1.partial_cmp(&v2)
            }
            UIntWidth::Arbitrary(_) => {
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
            UIntWidth::U32 => {
                let v: u32 = state.read_prim(*reg).unwrap_or(0);
                v == 0
            }
            UIntWidth::U64 => {
                let v: u64 = state.read_prim(*reg).unwrap_or(0);
                v == 0
            }
            UIntWidth::Arbitrary(_) => {
                let v: &ArbitraryUnsignedInt =
                    state.read(*reg).unwrap_or(ArbitraryUnsignedInt::ZERO_REF);
                v == ArbitraryUnsignedInt::ZERO_REF
            }
        }
    }

    fn store_into_memory<M: MemoryManager>(
        reg: Reg<UnsignedRegT>,
        state: &STATE,
        memory: &mut M,
        address: &M::Address,
    ) -> Result<(), MemoryAccessError<M::Address>> {
        match Self::from_width(reg.width) {
            UIntWidth::U32 => {
                let v: u32 = state.read_prim(reg).unwrap_or(0);
                memory.store_prim(v, address)
            }
            UIntWidth::U64 => {
                let v: u64 = state.read_prim(reg).unwrap_or(0);
                memory.store_prim(v, address)
            }
            UIntWidth::Arbitrary(_) => {
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
        dst: Reg<UnsignedRegT>,
        p: &RegOrConstant<UnsignedRegT>,
        state: &mut STATE,
        op: &OP,
    ) {
        match self {
            UIntWidth::U32 => {
                let mut v: u32 = read_uint(&p, state);
                op.operate(&mut v);
                state.store_prim(dst, v);
            }
            UIntWidth::U64 => {
                let mut v: u64 = read_uint(&p, state);
                op.operate(&mut v);
                state.store_prim(dst, v);
            }
            UIntWidth::Arbitrary(w) => {
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
        dst: Reg<UnsignedRegT>,
        p1: &RegOrConstant<UnsignedRegT>,
        p2: &RegOrConstant<UnsignedRegT>,
        state: &mut STATE,
        operation: &OP,
    ) {
        match self {
            UIntWidth::U32 => {
                let mut v1: u32 = read_uint(p1, state);
                let v2: u32 = read_uint(p2, state);
                operation.operate(&mut v1, &v2);
                state.store_prim(dst, v1);
            }
            UIntWidth::U64 => {
                let mut v1: u64 = read_uint(p1, state);
                let v2: u64 = read_uint(p2, state);
                operation.operate(&mut v1, &v2);
                state.store_prim(dst, v1);
            }
            UIntWidth::Arbitrary(w) => {
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
        params: &VectorBroadcastParams<UnsignedRegT>,
        state: &mut STATE,
        op: &OP,
    ) {
        let length = params.length() as usize;

        match self {
            UIntWidth::U32 => {
                let mut vector: VecValue<u32> = state
                    .read_multi_prim(params.vec_param(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let v: u32 = read_uint(params.value_param(), state);
                compute_broadcast(&mut vector, &v, op, params.is_reversed());
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            UIntWidth::U64 => {
                let mut vector: VecValue<u64> = state
                    .read_multi_prim(params.vec_param(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let v: u64 = read_uint(params.value_param(), state);
                compute_broadcast(&mut vector, &v, op, params.is_reversed());
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            UIntWidth::Arbitrary(w) => {
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
        params: &VectorVectorParams<UnsignedRegT>,
        op: &OP,
        state: &mut STATE,
    ) {
        let length = params.length() as usize;
        match self {
            UIntWidth::U32 => {
                let mut vector: VecValue<u32> = state
                    .read_multi_prim(params.p1(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let param: Option<&VecValue<u32>> = state.read_multi_prim(params.p2(), length);
                compute_vector(&mut vector, param, &0, op);
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            UIntWidth::U64 => {
                let mut vector: VecValue<u64> = state
                    .read_multi_prim(params.p1(), length)
                    .cloned()
                    .unwrap_or(VecValue::from_repeated_default(length));
                let param: Option<&VecValue<u64>> = state.read_multi_prim(params.p2(), length);
                compute_vector(&mut vector, param, &0, op);
                state.store_multi_copy_prim(*params.dst(), vector.as_slice());
            }
            UIntWidth::Arbitrary(w) => {
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
            w if w < u32::BITS => Self::U32,
            w if w < u64::BITS => Self::U64,
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
            UIntWidth::U32 => state.store_prim(reg, val as u32),
            UIntWidth::U64 => state.store_prim(reg, val as u64),
            UIntWidth::Arbitrary(w) => {
                let mut arb_val: ArbitraryUnsignedInt = val.cast_into();
                arb_val.resize_to(w);
                state.store(reg, arb_val);
            }
        };
    }
}

fn compute_vector<T, OP>(v1: &mut VecValue<T>, v2: Option<&VecValue<T>>, default_v: &T, op: &OP)
where
    OP: BinaryOp<T>,
{
    match v2 {
        Some(v2) => {
            v1.vector_op(v2, |a, b| op.operate(a, b));
        }
        None => {
            v1.broadcast_op(default_v, |a, b| op.operate(a, b));
        }
    }
}

fn compute_broadcast<T, OP>(vector: &mut VecValue<T>, broadcast_value: &T, op: &OP, reversed: bool)
where
    T: Clone + Default,
    OP: BinaryOp<T>,
{
    if reversed {
        vector.broadcast_op_reversed(&*broadcast_value, |a, b| op.operate(a, b));
    } else {
        vector.broadcast_op(&*broadcast_value, |a, b| op.operate(a, b));
    }
}

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
