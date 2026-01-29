use crate::vm::types::UMCOffset;

/// The address type
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
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

impl UMCOffset for InstructionAddress {
    fn offset(&mut self, offset: isize) {
        self.0 = self.0.wrapping_add_signed(offset);
    }
}
