use std::ops::{BitAndAssign, BitXorAssign};

use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{CastFrom, UMCAddSub, UMCArithmetic};

/// The address type
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct InstructionAddress(usize);

impl InstructionAddress {
    pub fn new(x: usize) -> Self {
        Self(x)
    }

    /// Get the program counter equivalent of this address
    pub fn pc(&self) -> usize {
        self.0
    }
}

impl Default for InstructionAddress {
    fn default() -> Self {
        Self(0)
    }
}

impl UMCAddSub for InstructionAddress {
    fn add(&mut self, rhs: &Self) {
        self.0 = self.0.wrapping_add(rhs.0);
    }

    fn sub(&mut self, rhs: &Self) {
        self.0 = self.0.wrapping_sub(rhs.0);
    }
}

impl CastFrom<u32> for InstructionAddress {
    fn cast_from(value: &u32) -> Self {
        Self(*value as usize)
    }
}

impl CastFrom<u64> for InstructionAddress {
    fn cast_from(value: &u64) -> Self {
        Self(*value as usize)
    }
}

impl CastFrom<ArbitraryUnsignedInt> for InstructionAddress {
    fn cast_from(value: &ArbitraryUnsignedInt) -> Self {
        Self(value.data().get(0).copied().unwrap_or(0))
    }
}
