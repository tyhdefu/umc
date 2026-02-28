//! Abstractions over specialised widths in UMC

pub mod int;
pub mod uint;

use std::cmp::Ordering;

use umc_model::instructions::{VectorBroadcastParams, VectorVectorParams};
use umc_model::reg_model::{Reg, RegOrConstant, RegTypeT};

use crate::vm::memory::{MemoryAccessError, MemoryManager};
use crate::vm::types::BinaryOp;
use crate::vm::types::vector::VecValue;

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
    /// Note that this implicitly peforms widening and narrowing operations
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
    T: Clone,
    OP: BinaryOp<T>,
{
    if reversed {
        vector.broadcast_op_reversed(&*broadcast_value, |a, b| op.operate(a, b));
    } else {
        vector.broadcast_op(&*broadcast_value, |a, b| op.operate(a, b));
    }
}
