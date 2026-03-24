use std::fmt::Display;
use std::iter::repeat_n;
use std::ops::BitXorAssign;
use std::ops::{BitAndAssign, BitOrAssign};
use std::usize;

use crate::vm::types::{CastFrom, CastInto, UMCArithmetic, UMCBitwise};

#[derive(Clone, Debug)]
pub struct ArbitraryUnsignedInt {
    bits: u32,
    // Least significant values first (little-endian overall, target dependent within)
    data: Vec<usize>,
}

impl ArbitraryUnsignedInt {
    pub const ZERO: ArbitraryUnsignedInt = ArbitraryUnsignedInt::new(0);
    pub const ZERO_REF: &'static ArbitraryUnsignedInt = &Self::ZERO;

    /// Creates a zero-value
    pub const fn new(bits: u32) -> Self {
        Self { bits, data: vec![] }
    }

    #[cfg(test)]
    pub fn new_from_u32(bits: u32, value: u32) -> Self {
        assert!(usize::BITS >= u32::BITS);

        Self {
            bits,
            data: vec![value as usize],
        }
    }

    #[cfg(test)]
    pub fn add_u64(&mut self, other: u64) {
        let other: Self = (&other).cast_into();
        self.add(&other);
    }

    pub fn increment(&mut self) {
        for i in 0..(self.max_size()) {
            if i < self.data.len() {
                let (v, carry) = self.data[i].carrying_add(0, true);
                self.data[i] = v;
                if !carry {
                    break;
                }
            } else {
                self.data.push(1);
            }
        }
    }

    pub fn as_usize(&self) -> usize {
        self.data.first().copied().unwrap_or(0)
    }

    pub fn from_bytes(bits: u32, buf: &[u8]) -> Result<Self, ()> {
        let full_chunks = (bits / usize::BITS) as usize;
        println!("Full chunks: {}", full_chunks);
        let mut data = vec![];
        for i in 0..full_chunks {
            let c = i * size_of::<usize>();
            let slice = &buf[c..(c + size_of::<usize>())];
            let chunk = usize::from_le_bytes(slice.try_into().unwrap());
            data.push(chunk);
        }

        let partial_chunks = ((bits % usize::BITS) / u8::BITS) as usize;
        println!("Partial chunks: {}", partial_chunks);
        let mut last_chunk: usize = 0;
        for i in 0..partial_chunks {
            let index = (full_chunks * size_of::<usize>()) + i;
            last_chunk += (buf[index] as usize) << (u8::BITS * i as u32);
        }

        data.push(last_chunk);

        // TODO: Reverse?
        Ok(Self { bits, data })
    }

    pub fn write_bytes(&self, buf: &mut [u8]) -> Result<(), ()> {
        let req_bytes = self.bits.div_ceil(u8::BITS) as usize;
        if buf.len() < req_bytes {
            return Err(());
        }
        for (i, x) in self
            .data
            .iter()
            .flat_map(|v| v.to_le_bytes().into_iter())
            .take(req_bytes)
            .enumerate()
        {
            buf[i] = x;
        }
        Ok(())
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut vec = vec![0; self.bits.div_ceil(u8::BITS) as usize];
        self.write_bytes(&mut vec).unwrap();
        vec
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

    /// Resize to the given number of bits, narrowing the value if necessary
    pub fn resize_to(&mut self, bits: u32) {
        self.set_bits(bits);
        self.data.truncate(self.max_size());
        self.mask_top();
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

impl UMCArithmetic for bool {
    fn add(&mut self, rhs: &Self) {
        self.bitxor_assign(rhs);
    }

    fn sub(&mut self, rhs: &Self) {
        self.bitxor_assign(rhs);
    }

    fn mul(&mut self, rhs: &Self) {
        self.bitand_assign(rhs);
    }

    fn div(&mut self, rhs: &Self) {
        // TOOD: Divide by zero?
        assert_eq!(*rhs, true, "Divide by boolean false");
    }

    fn modulo(&mut self, rhs: &Self) {
        assert_eq!(*rhs, true, "Modulo by boolean false");
        *self = false
    }
}

impl UMCBitwise for bool {
    fn and(&mut self, rhs: &Self) {
        self.bitand_assign(rhs);
    }

    fn or(&mut self, rhs: &Self) {
        self.bitor_assign(rhs);
    }

    fn xor(&mut self, rhs: &Self) {
        self.bitxor_assign(rhs);
    }

    fn not(&mut self) {
        *self = !*self
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

    fn mul(&mut self, rhs: &Self) {
        let mut dest = ArbitraryUnsignedInt::new(self.bits);

        // Very basic implementation: Add X number of times
        let value = (&*self).max(rhs);
        let iterations = (&*self).min(rhs);

        let mut counter = ArbitraryUnsignedInt::new(iterations.bits);

        while counter < *iterations {
            dest.add(value);
            counter.increment();
        }

        *self = dest;
        self.mask_top();
    }

    fn div(&mut self, rhs: &Self) {
        // Very basic implementation: just counter how many times we fit in
        if *rhs == ArbitraryUnsignedInt::ZERO {
            panic!("Divide by zero!");
        }
        let mut count: u32 = 0;
        while *self >= *rhs {
            self.sub(&rhs);
            count += 1;
        }
        // Hack but at least we have everything implemented
        self.data = vec![count as usize];
    }

    fn modulo(&mut self, rhs: &Self) {
        if *self < *rhs {
            return;
        }
        if *rhs == ArbitraryUnsignedInt::ZERO {
            panic!("Modulo by zero!");
        }
        // Very basic implementation: just keep subtracting rhs until we are in bound
        while *self >= *rhs {
            self.sub(rhs);
        }
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

// u1 (boolean) casts

impl CastFrom<u32> for bool {
    fn cast_from(value: &u32) -> Self {
        (value & 0b1) == 1
    }
}

impl CastFrom<u64> for bool {
    fn cast_from(value: &u64) -> Self {
        (value & 0b1) == 1
    }
}

impl CastFrom<ArbitraryUnsignedInt> for bool {
    fn cast_from(value: &ArbitraryUnsignedInt) -> Self {
        (value.as_usize() & 0b1) == 1
    }
}

// u32 casts
impl CastFrom<bool> for u32 {
    fn cast_from(value: &bool) -> Self {
        *value as u32
    }
}

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
impl CastFrom<bool> for u64 {
    fn cast_from(value: &bool) -> Self {
        *value as u64
    }
}

impl CastFrom<u32> for u64 {
    fn cast_from(value: &u32) -> Self {
        *value as u64
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
            let lower = value.data.get(0).copied().unwrap_or(0) as u64;
            let upper = value.data.get(1).copied().unwrap_or(0) as u64;
            return lower + (upper << usize::BITS);
        }
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

impl CastFrom<bool> for ArbitraryUnsignedInt {
    fn cast_from(value: &bool) -> Self {
        ArbitraryUnsignedInt {
            bits: 32,
            data: vec![*value as usize],
        }
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
            data: vec![(*value as u32) as usize, ((*value >> 32) as u32) as usize],
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

#[cfg(test)]
mod tests {
    use crate::vm::memory::SerializableArb;
    use crate::vm::types::uint::ArbitraryUnsignedInt;
    use crate::vm::types::{CastInto, UMCArithmetic};

    #[test]
    fn cast_from_u64_one() {
        let one: u64 = 1;
        let mut v: ArbitraryUnsignedInt = one.cast_into();
        v.resize_to(1);
        assert_eq!(1u8.to_le_bytes(), v.to_le_bytes().as_slice());
    }

    fn check_serialize(expected: ArbitraryUnsignedInt) {
        let mut buf = vec![0; 256];
        expected.write_to(&mut buf).unwrap();

        let got = ArbitraryUnsignedInt::from_bytes(expected.bits, &buf).unwrap();
        assert_eq!(expected, got);
    }

    #[test]
    fn serialize_arbitrary_8() {
        let mut v = ArbitraryUnsignedInt::new(8);
        v.add_u64(255);
        check_serialize(v);
    }

    #[test]
    fn serialize_arbitrary_32() {
        let mut v = ArbitraryUnsignedInt::new(32);
        v.add_u64(42);
        check_serialize(v);
    }

    #[test]
    fn serialize_arbitrary_48() {
        let mut v = ArbitraryUnsignedInt::new(48);
        v.add_u64(u32::MAX as u64 + 1915);
        check_serialize(v);
    }

    #[test]
    fn add_arbitrary() {
        let mut a = ArbitraryUnsignedInt::new(8);
        a.add_u64(200);
        let mut b = ArbitraryUnsignedInt::new(8);
        b.add_u64(100);

        a.add(&b);
        assert_eq!("0x2C", format!("{}", a));
    }

    #[test]
    fn mul_wrapping_arb_int() {
        let mut a = ArbitraryUnsignedInt::new(8);
        a.add_u64(150);
        let mut b = ArbitraryUnsignedInt::new(8);
        b.add_u64(10);
        a.mul(&b);

        let expected = 150u8.wrapping_mul(10);
        assert_eq!(format!("{:#X}", expected), format!("{}", a));
    }

    #[test]
    fn mul_0() {
        let zero = ArbitraryUnsignedInt::new_from_u32(16, 0);

        let mut zero_result = zero.clone();
        let mut one_result = ArbitraryUnsignedInt::new_from_u32(16, 1);
        let mut large_result = ArbitraryUnsignedInt::new_from_u32(16, 0xF123);

        zero_result.mul(&zero);
        one_result.mul(&zero);
        large_result.mul(&zero);

        assert_eq!(zero_result, zero);
        assert_eq!(one_result, zero);
        assert_eq!(large_result, zero);
    }

    #[test]
    fn div_by_one() {
        let a = ArbitraryUnsignedInt::new_from_u32(8, 42);
        let b = ArbitraryUnsignedInt::new_from_u32(1, 1);

        let mut result = a.clone();
        result.div(&b);
        assert_eq!(a, result);
    }

    #[test]
    fn div_by_two() {
        let mut a = ArbitraryUnsignedInt::new_from_u32(8, 42);
        let b = ArbitraryUnsignedInt::new_from_u32(2, 2);

        a.div(&b);

        let expected = ArbitraryUnsignedInt::new_from_u32(8, 21);
        assert_eq!(expected, a);
    }

    #[test]
    fn mod_2() {
        let mut a = ArbitraryUnsignedInt::new_from_u32(8, 255);
        let b = ArbitraryUnsignedInt::new_from_u32(2, 2);

        a.modulo(&b);

        let expected = ArbitraryUnsignedInt::new_from_u32(8, 1);
        assert_eq!(expected, a);
    }
}
