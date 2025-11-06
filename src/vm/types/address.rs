use std::ops::{BitAndAssign, BitXorAssign};

use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{CastFrom, UMCArithmetic};

/// The address type
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Address(usize);

impl Address {
    pub fn new(x: usize) -> Self {
        Self(x)
    }

    /// Interpret this address as a program counter location
    pub fn pc(&self) -> usize {
        self.0
    }
}

impl Default for Address {
    fn default() -> Self {
        Self(0)
    }
}

impl UMCArithmetic for Address {
    fn add(&mut self, rhs: &Self) {
        self.0 = self.0.wrapping_add(rhs.0);
    }

    fn not(&mut self) {}

    fn and(&mut self, rhs: &Self) {
        self.0.bitand_assign(rhs.0);
    }

    fn xor(&mut self, rhs: &Self) {
        self.0.bitxor_assign(rhs.0);
    }
}

impl CastFrom<u32> for Address {
    fn cast_from(value: &u32) -> Self {
        Self(*value as usize)
    }
}

impl CastFrom<u64> for Address {
    fn cast_from(value: &u64) -> Self {
        Self(*value as usize)
    }
}

impl CastFrom<ArbitraryUnsignedInt> for Address {
    fn cast_from(value: &ArbitraryUnsignedInt) -> Self {
        Self(value.data().get(0).copied().unwrap_or(0))
    }
}
