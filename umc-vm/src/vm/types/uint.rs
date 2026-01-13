use std::fmt::Display;
use std::iter::repeat_n;
use std::ops::BitXorAssign;
use std::ops::{BitAndAssign, BitOrAssign};
use std::usize;

use crate::vm::types::{CastFrom, CastInto, UMCArithmetic, UMCBitwise};

#[derive(Clone, Debug)]
pub struct ArbitraryUnsignedInt {
    bits: u32,
    // Least significant values first
    data: Vec<usize>,
}

impl ArbitraryUnsignedInt {
    pub const ZERO: ArbitraryUnsignedInt = ArbitraryUnsignedInt::new(0);
    pub const ZERO_REF: &'static ArbitraryUnsignedInt = &Self::ZERO;

    /// Creates a zero-value
    pub const fn new(bits: u32) -> Self {
        Self { bits, data: vec![] }
    }

    pub fn resized_clone(&self, new_bits: u32) -> Self {
        let new_len = new_bits.div_ceil(usize::BITS) as usize;
        let mut copy = Self {
            bits: new_bits,
            data: self.data.iter().take(new_len).copied().collect(),
        };
        copy.mask_top();
        return copy;
    }

    pub fn set_bits(&mut self, new_bits: u32) {
        self.bits = new_bits;
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
            self.data.last_mut().map(|v| *v &= mask);
        }
    }

    /// The maximum size of the data vector when fully populated
    fn max_size(&self) -> usize {
        self.bits.div_ceil(usize::BITS) as usize
    }

    /// Grows the data vector to the maximum size, filling with zeros
    fn grow_to_max(&mut self) {
        self.grow_to_max_with(0);
    }

    fn grow_to_max_with(&mut self, v: usize) {
        let extra_len = self.max_size() - self.data.len();
        self.data.extend(repeat_n(v, extra_len));
    }
}

impl Default for ArbitraryUnsignedInt {
    fn default() -> Self {
        Self::new(0)
    }
}

impl PartialEq for ArbitraryUnsignedInt {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for ArbitraryUnsignedInt {}

impl PartialOrd for ArbitraryUnsignedInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ArbitraryUnsignedInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for i in 0..(self.data.len().max(other.data.len())) {
            let x1 = self.data.get(i).copied().unwrap_or(0);
            let x2 = other.data.get(i).copied().unwrap_or(0);
            let cmp = x1.cmp(&x2);
            match cmp {
                std::cmp::Ordering::Equal => continue,
                v => return v,
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl Display for ArbitraryUnsignedInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Start with the most significant chunks, skipping leading zeros
        let mut iter = self.data.iter().rev().skip_while(|x| **x == 0);
        write!(f, "{:#X}", iter.next().copied().unwrap_or(0))?;

        for v in iter {
            /// Two Hex digits per byte
            const ALIGNMENT: usize = size_of::<usize>() * 2;
            write!(f, "{v:0ALIGNMENT$X}")?;
        }
        Ok(())
    }
}

impl CastFrom<ArbitraryUnsignedInt> for ArbitraryUnsignedInt {
    fn cast_from(value: &ArbitraryUnsignedInt) -> Self {
        value.clone()
    }
}

impl UMCArithmetic for u32 {
    fn add(&mut self, rhs: &Self) {
        *self = self.wrapping_add(*rhs)
    }

    fn sub(&mut self, rhs: &Self) {
        *self = self.wrapping_sub(*rhs)
    }

    fn modulo(&mut self, rhs: &Self) {
        *self = *self % *rhs;
    }

    fn mul(&mut self, rhs: &Self) {
        *self = self.wrapping_mul(*rhs);
    }

    fn div(&mut self, rhs: &Self) {
        *self = *self / rhs;
    }
}

impl UMCBitwise for u32 {
    fn not(&mut self) {
        *self = !*self
    }

    fn or(&mut self, rhs: &Self) {
        self.bitor_assign(rhs);
    }

    fn and(&mut self, rhs: &Self) {
        self.bitand_assign(rhs);
    }

    fn xor(&mut self, rhs: &Self) {
        self.bitxor_assign(rhs);
    }
}

impl UMCArithmetic for u64 {
    fn add(&mut self, rhs: &Self) {
        *self = self.wrapping_add(*rhs)
    }

    fn sub(&mut self, rhs: &Self) {
        *self = self.wrapping_sub(*rhs)
    }

    fn modulo(&mut self, rhs: &Self) {
        *self = *self % *rhs;
    }

    fn mul(&mut self, rhs: &Self) {
        *self = self.wrapping_mul(*rhs);
    }

    fn div(&mut self, rhs: &Self) {
        *self = *self / rhs;
    }
}

impl UMCBitwise for u64 {
    fn not(&mut self) {
        *self = !*self
    }

    fn or(&mut self, rhs: &Self) {
        self.bitor_assign(rhs);
    }

    fn and(&mut self, rhs: &Self) {
        self.bitand_assign(rhs);
    }

    fn xor(&mut self, rhs: &Self) {
        self.bitxor_assign(rhs);
    }
}

impl UMCArithmetic for ArbitraryUnsignedInt {
    fn add(&mut self, rhs: &Self) {
        self.data.reserve(rhs.data.len() - self.data.len());
        let mut carry = false;
        for (i, v) in rhs.data.iter().enumerate().take(self.max_size()) {
            if i < self.data.len() {
                let (res, c) = self.data[i].carrying_add(*v, carry);
                self.data[i] = res;
                carry = c;
            } else {
                self.data.push(*v);
            }
        }
        // Carry through until we find non-max or simply overflow (never put the carry anywhere)
        for i in (rhs.data.len())..(self.max_size()) {
            if !carry {
                break;
            }
            if i >= self.data.len() {
                // Full overflow
                self.data.push(1);
                break;
            }

            let (res, c) = self.data[i].carrying_add(0, true);
            carry = c;
            self.data[i] = res;
        }

        self.mask_top();
    }

    fn sub(&mut self, rhs: &Self) {
        let mut borrow = false;
        for (i, v) in rhs.data.iter().enumerate().take(self.max_size()) {
            if i < self.data.len() {
                let (res, b) = self.data[i].borrowing_sub(*v, borrow);
                self.data[i] = res;
                borrow = b;
            } else {
                let (res, b) = 0usize.borrowing_sub(*v, borrow);
                self.data.push(res);
                borrow = b;
            }
        }
        // Carry borrow through until we find non-zero or simply underflow (never borrow)
        for i in (rhs.data.len())..(self.max_size()) {
            if !borrow {
                break;
            }
            if i >= self.data.len() {
                // Full underflow
                self.grow_to_max_with(usize::MAX);
                break;
            }

            let (res, b) = self.data[i].borrowing_sub(0, true);
            borrow = b;
            self.data[i] = res;
        }

        self.mask_top();
    }

    fn modulo(&mut self, rhs: &Self) {
        todo!()
    }

    fn mul(&mut self, rhs: &Self) {
        todo!()
    }

    fn div(&mut self, rhs: &Self) {
        todo!()
    }
}

impl UMCBitwise for ArbitraryUnsignedInt {
    fn not(&mut self) {
        // Any sparse 0s will become non-zero, so fill vec first:
        self.grow_to_max();
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

    fn and(&mut self, rhs: &Self) {
        // AND results in 0 if LHS is zero, so no need to pad / extend the vec
        for (i, x) in self.data.iter_mut().enumerate() {
            let y = rhs.data.get(i).copied().unwrap_or(0);
            x.bitand_assign(y);
        }
        self.mask_top();
    }

    fn or(&mut self, rhs: &Self) {
        self.grow_to_max(); // TODO: Could copy from other as needed beyond end
        for (i, x) in self.data.iter_mut().enumerate() {
            let y = rhs.data.get(i).copied().unwrap_or(0);
            x.bitor_assign(y);
        }
        self.mask_top();
    }

    fn xor(&mut self, rhs: &Self) {
        self.grow_to_max();
        for (i, x) in self.data.iter_mut().enumerate() {
            let y = rhs.data.get(i).copied().unwrap_or(0);
            x.bitxor_assign(y);
        }
        self.mask_top();
    }
}

// u32 casts
impl CastFrom<u64> for u32 {
    fn cast_from(value: &u64) -> Self {
        *value as Self
    }
}

impl CastFrom<ArbitraryUnsignedInt> for u32 {
    fn cast_from(value: &ArbitraryUnsignedInt) -> Self {
        #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
        compile_error!("Only 32-bit and 64-bit archectures supported");

        value.data.first().copied().map(|v| v as u32).unwrap_or(0)
    }
}

impl CastFrom<i32> for u32 {
    fn cast_from(value: &i32) -> Self {
        *value as Self
    }
}

impl CastFrom<i64> for u32 {
    fn cast_from(value: &i64) -> Self {
        *value as Self
    }
}

// u64 casts
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

impl CastFrom<i32> for u64 {
    fn cast_from(value: &i32) -> Self {
        *value as Self
    }
}

impl CastFrom<i64> for u64 {
    fn cast_from(value: &i64) -> Self {
        *value as Self
    }
}

// Arbitrary unsigned casts

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

impl CastFrom<i32> for ArbitraryUnsignedInt {
    fn cast_from(value: &i32) -> Self {
        let v = *value as u32;
        v.cast_into()
    }
}

impl CastFrom<i64> for ArbitraryUnsignedInt {
    fn cast_from(value: &i64) -> Self {
        let v = *value as u64;
        v.cast_into()
    }
}
