use crate::vm::types::{CastFrom, UMCArithmetic};

#[derive(Clone, Debug, PartialEq)]
pub struct ArbitraryUnsignedInt {
    bits: u32,
    // Least significant values first
    data: Vec<usize>,
}

// TODO: Display implementation

impl ArbitraryUnsignedInt {
    pub fn new(bits: u32) -> Self {
        Self { bits, data: vec![] }
    }

    pub fn data(&self) -> &[usize] {
        &self.data[..]
    }
}

// Casts between integer types
impl CastFrom<u64> for u32 {
    fn cast_from(value: u64) -> Self {
        value as u32
    }
}

impl CastFrom<ArbitraryUnsignedInt> for u32 {
    fn cast_from(value: ArbitraryUnsignedInt) -> Self {
        #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
        compile_error!("Only 32-bit and 64-bit archectures supported");

        value.data.first().copied().map(|v| v as u32).unwrap_or(0)
    }
}

impl CastFrom<ArbitraryUnsignedInt> for u64 {
    fn cast_from(value: ArbitraryUnsignedInt) -> Self {
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
    fn cast_from(value: u32) -> Self {
        value as u64
    }
}

impl CastFrom<u32> for ArbitraryUnsignedInt {
    fn cast_from(value: u32) -> Self {
        #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
        compile_error!("Only 32-bit and 64-bit archectures supported");

        Self {
            bits: 32,
            data: vec![value as usize],
        }
    }
}

impl CastFrom<u64> for ArbitraryUnsignedInt {
    fn cast_from(value: u64) -> Self {
        #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
        compile_error!("Only 32-bit and 64-bit archectures supported");

        Self {
            bits: 64,
            #[cfg(target_pointer_width = "64")]
            data: vec![value as usize],
            #[cfg(target_pointer_width = "32")]
            data: vec![value >> 32 as usize, value as usize],
        }
    }
}

impl UMCArithmetic for u32 {
    fn add(&mut self, rhs: &Self) {
        *self = u32::wrapping_add(*self, *rhs)
    }
}

impl UMCArithmetic for u64 {
    fn add(&mut self, rhs: &Self) {
        *self = u64::wrapping_add(*self, *rhs)
    }
}

impl UMCArithmetic for ArbitraryUnsignedInt {
    fn add(&mut self, rhs: &Self) {
        self.data.reserve(rhs.data.len() - self.data.len());
        let mut rem_bits = self.bits;
        let mut carry = false;
        for (i, v) in rhs.data.iter().enumerate() {
            if i < self.data.len() {
                let (res, c) = self.data[i].carrying_add(*v, carry);
                self.data[i] = res;
                carry = c;
            } else {
                self.data.push(*v);
            }
            rem_bits -= usize::BITS;
        }
        if carry && rem_bits >= 1 {
            self.data.push(1);
        }
        if rem_bits < usize::BITS {
            let mask = usize::MAX >> (usize::BITS - rem_bits);
            self.data.last_mut().map(|v| *v &= mask);
        }
    }
}
