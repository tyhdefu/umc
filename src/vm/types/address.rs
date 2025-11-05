use crate::vm::types::UMCArithmetic;

/// The address type
#[derive(PartialEq)]
pub struct Address(usize);

impl UMCArithmetic for Address {
    fn add(&mut self, rhs: &Self) {
        self.0 += rhs.0
    }

    fn not(&mut self) {}
}
