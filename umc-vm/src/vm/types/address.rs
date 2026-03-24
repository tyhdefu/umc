use crate::vm::{memory::Serializable, types::UMCOffset};

/// The address type
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct InstructionAddress(usize);

impl InstructionAddress {
    pub const SIZE_BYTES: u32 = size_of::<usize>() as u32;

    pub const PROGRAM_START: InstructionAddress = InstructionAddress(0);

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

impl Serializable for InstructionAddress {
    fn read_from(bytes: &[u8]) -> Result<Self, ()> {
        usize::read_from(bytes).map(|x| InstructionAddress::new(x))
    }

    fn write_to(&self, bytes: &mut [u8]) -> Result<(), ()> {
        self.0.write_to(bytes)
    }
}
