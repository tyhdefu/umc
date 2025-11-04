use std::fmt::Display;

use crate::vm::types::{CastFrom, UMCArithmetic};

#[derive(Clone, Debug, PartialEq)]
pub struct ArbitraryUnsignedInt {
    bits: u32,
    // Least significant values first
    data: Vec<usize>,
}

// TODO: Display implementation

impl ArbitraryUnsignedInt {
    pub const fn new(bits: u32) -> Self {
        Self { bits, data: vec![] }
    }

    pub fn data(&self) -> &[usize] {
        &self.data[..]
    }

    fn used_bits(&self) -> u32 {
        self.data.len() as u32 * usize::BITS
    }

    /// Mask out any overflown values
    fn mask_top(&mut self) {
        if self.data.len() as u32 > self.bits / usize::BITS {
            let rem_bits = self.bits % usize::BITS;
            let mask = usize::MAX >> (usize::BITS - rem_bits);
            println!("Masking top: {:x}", mask);
            self.data.last_mut().map(|v| *v &= mask);
            println!("{:?}", self.data);
        }
    }
}

impl Default for ArbitraryUnsignedInt {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Display for ArbitraryUnsignedInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data.is_empty() {
            return write!(f, "0x0");
        }
        write!(f, "0x")?;

        for v in self.data.iter().rev() {
            write!(f, "{:X}", v)?;
        }
        Ok(())
    }
}

impl CastFrom<ArbitraryUnsignedInt> for ArbitraryUnsignedInt {
    fn cast_from(value: &ArbitraryUnsignedInt) -> Self {
        value.clone()
    }
}

// Casts between integer types
impl CastFrom<u64> for u32 {
    fn cast_from(value: &u64) -> Self {
        *value as u32
    }
}

impl CastFrom<ArbitraryUnsignedInt> for u32 {
    fn cast_from(value: &ArbitraryUnsignedInt) -> Self {
        #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
        compile_error!("Only 32-bit and 64-bit archectures supported");

        value.data.first().copied().map(|v| v as u32).unwrap_or(0)
    }
}

impl CastFrom<ArbitraryUnsignedInt> for u64 {
    fn cast_from(value: &ArbitraryUnsignedInt) -> Self {
        #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
        compile_error!("Only 32-bit and 64-bit archectures supported");

        #[cfg(target_pointer_width = "64")]
        return value.data.first().copied().map(|v| v as u64).unwrap_or(0);

        #[cfg(target_pointer_width = "32")]
        {
            let lower = self.data.get(0).copied().unwrap_or(0) as u64;
            let upper = self.data.get(1).copied().unwrap_or(0) as u64;
            return lower + (upper << usize::BITS);
        }
    }
}

impl CastFrom<u32> for u64 {
    fn cast_from(value: &u32) -> Self {
        *value as u64
    }
}

impl CastFrom<u32> for ArbitraryUnsignedInt {
    fn cast_from(value: &u32) -> Self {
        #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
        compile_error!("Only 32-bit and 64-bit archectures supported");

        Self {
            bits: 32,
            data: vec![*value as usize],
        }
    }
}

impl CastFrom<u64> for ArbitraryUnsignedInt {
    fn cast_from(value: &u64) -> Self {
        #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
        compile_error!("Only 32-bit and 64-bit archectures supported");

        Self {
            bits: 64,
            #[cfg(target_pointer_width = "64")]
            data: vec![*value as usize],
            #[cfg(target_pointer_width = "32")]
            data: vec![*value >> 32 as usize, *value as usize],
        }
    }
}

impl UMCArithmetic for u32 {
    fn add(&mut self, rhs: &Self) {
        *self = u32::wrapping_add(*self, *rhs)
    }

    fn not(&mut self) {
        *self = !*self
    }
}

impl UMCArithmetic for u64 {
    fn add(&mut self, rhs: &Self) {
        *self = u64::wrapping_add(*self, *rhs)
    }

    fn not(&mut self) {
        *self = !*self
    }
}

impl UMCArithmetic for ArbitraryUnsignedInt {
    fn add(&mut self, rhs: &Self) {
        self.data.reserve(rhs.data.len() - self.data.len());
        let mut carry = false;
        for (i, v) in rhs.data.iter().enumerate() {
            if i < self.data.len() {
                let (res, c) = self.data[i].carrying_add(*v, carry);
                self.data[i] = res;
                carry = c;
            } else {
                self.data.push(*v);
            }
        }
        let used_bits = self.used_bits();
        if carry && self.bits > used_bits {
            self.data.push(1); // Free to expand adding the carry
            return;
        }

        self.mask_top();
    }

    fn not(&mut self) {
        for v in self.data.iter_mut() {
            *v = !*v;
        }
        // Also need to fully pad with 1s if we are sparsely represented
        let used = self.used_bits();
        if used < self.bits {
            for _ in 1..((used / usize::BITS) + 1) {
                self.data.push(usize::MAX);
            }
        }
        self.mask_top();
    }
}
