use std::fmt::Display;
use std::num::NonZeroUsize;
use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use awint::{Awi, Bits};
use umc_model::RegWidth;

use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{CastFrom, CastInto, UMCArithmetic, UMCBitwise};

#[derive(PartialEq, Clone, Debug)]
pub struct ArbitraryInt {
    // None signifies a zero-width integer
    inner: Option<Awi>,
}

impl ArbitraryInt {
    pub const ZERO: ArbitraryInt = ArbitraryInt { inner: None };
    pub const ZERO_REF: &'static ArbitraryInt = &Self::ZERO;

    pub fn zero(width: RegWidth) -> Self {
        match NonZeroUsize::try_from(width as usize) {
            Ok(nonzero) => Self {
                inner: Some(Awi::zero(nonzero)),
            },
            Err(_) => Self { inner: None },
        }
    }

    #[cfg(test)]
    pub fn add_u32(&mut self, v: u32) {
        if let Some(s) = &mut self.inner {
            let mut value = Awi::from_u32(v);
            value.resize(s.nzbw(), true);

            s.add_(&value).unwrap();
        }
    }

    pub fn resize_to(&mut self, bits: RegWidth) {
        match NonZeroUsize::try_from(bits as usize) {
            // Requested width is non-zero
            Ok(non_zero_width) => match &mut self.inner {
                Some(v) => {
                    v.sign_resize(non_zero_width);
                }
                None => {
                    self.inner = Some(Awi::zero(non_zero_width));
                }
            },
            // Requested width is zero
            Err(_) => {
                self.inner = None;
            }
        }
    }

    fn op_nonzero<F>(&mut self, other: &Self, op: F)
    where
        F: FnOnce(&mut Awi, &Awi),
    {
        if let Some(s) = &mut self.inner {
            match &other.inner {
                Some(o) => {
                    if o.bw() == s.bw() {
                        // Same bit width, simple
                        op(s, o);
                    } else {
                        // Different bit width, make a new integer with a matching width
                        let mut same_other = Awi::zero(s.nzbw());
                        same_other.sign_resize_(o);
                        op(s, &same_other);
                    }
                }
                None => {
                    let same_zero = Awi::zero(s.nzbw());
                    op(s, &same_zero)
                }
            };
        }
    }
}

impl Default for ArbitraryInt {
    fn default() -> Self {
        ArbitraryInt { inner: None }
    }
}

impl Display for ArbitraryInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            Some(i) => write!(f, "{:b}", i),
            None => write!(f, "0"),
        }
    }
}

#[allow(unused_variables)]
impl UMCArithmetic for ArbitraryInt {
    fn add(&mut self, rhs: &Self) {
        match (&mut self.inner, &rhs.inner) {
            (Some(s), Some(o)) => {
                let mut awi = Awi::zero(s.nzbw());
                awi.sign_resize_(o);
                s.add_(&awi).unwrap();
            }
            _ => {}
        }
    }

    fn sub(&mut self, rhs: &Self) {
        self.op_nonzero(rhs, |a, b| a.sub_(b).unwrap());
    }

    fn div(&mut self, rhs: &Self) {
        self.op_nonzero(rhs, |a, b| {
            let mut quo = Awi::zero(a.nzbw());
            let mut rem = Awi::zero(a.nzbw());
            let mut div_copy = b.clone();

            Bits::idivide(&mut quo, &mut rem, a, &mut div_copy).unwrap();
            *a = quo;
        });
    }

    fn modulo(&mut self, rhs: &Self) {
        self.op_nonzero(rhs, |a, b| {
            let mut quo = Awi::zero(a.nzbw());
            let mut rem = Awi::zero(a.nzbw());
            let mut div_copy = b.clone();

            Bits::idivide(&mut quo, &mut rem, a, &mut div_copy).unwrap();
            *a = rem;
        });
    }

    fn mul(&mut self, rhs: &Self) {
        self.op_nonzero(rhs, |a, b| {
            let mut temp = Awi::zero(a.nzbw());
            a.mul_(b, &mut temp).unwrap()
        });
    }
}

impl UMCBitwise for ArbitraryInt {
    fn and(&mut self, rhs: &Self) {
        self.op_nonzero(rhs, |a, b| a.and_(b).unwrap());
    }

    fn or(&mut self, rhs: &Self) {
        self.op_nonzero(rhs, |a, b| a.or_(b).unwrap());
    }

    fn xor(&mut self, rhs: &Self) {
        self.op_nonzero(rhs, |a, b| a.xor_(b).unwrap());
    }

    fn not(&mut self) {
        if let Some(s) = &mut self.inner {
            s.not_();
        }
    }
}

impl UMCArithmetic for i32 {
    fn add(&mut self, rhs: &Self) {
        *self = self.wrapping_add(*rhs);
    }

    fn sub(&mut self, rhs: &Self) {
        *self = self.wrapping_sub(*rhs);
    }

    fn modulo(&mut self, rhs: &Self) {
        *self = *self % *rhs;
    }

    fn mul(&mut self, rhs: &Self) {
        *self = self.wrapping_mul(*rhs);
    }

    fn div(&mut self, rhs: &Self) {
        *self = *self / *rhs;
    }
}

impl UMCBitwise for i32 {
    fn and(&mut self, rhs: &Self) {
        self.bitand_assign(*rhs);
    }

    fn or(&mut self, rhs: &Self) {
        self.bitor_assign(rhs);
    }

    fn xor(&mut self, rhs: &Self) {
        self.bitxor_assign(*rhs);
    }

    fn not(&mut self) {
        *self = !*self;
    }
}

impl UMCArithmetic for i64 {
    fn add(&mut self, rhs: &Self) {
        *self = self.wrapping_add(*rhs);
    }

    fn sub(&mut self, rhs: &Self) {
        *self = self.wrapping_sub(*rhs);
    }

    fn modulo(&mut self, rhs: &Self) {
        *self = *self % *rhs;
    }

    fn mul(&mut self, rhs: &Self) {
        *self = self.wrapping_mul(*rhs);
    }

    fn div(&mut self, rhs: &Self) {
        *self = *self / *rhs;
    }
}

impl UMCBitwise for i64 {
    fn and(&mut self, rhs: &Self) {
        self.bitand_assign(*rhs);
    }

    fn or(&mut self, rhs: &Self) {
        self.bitor_assign(rhs);
    }

    fn xor(&mut self, rhs: &Self) {
        self.bitxor_assign(*rhs);
    }

    fn not(&mut self) {
        *self = !*self;
    }
}

// casts to i32
impl CastFrom<i64> for i32 {
    fn cast_from(value: &i64) -> Self {
        *value as Self
    }
}

impl CastFrom<ArbitraryInt> for i32 {
    fn cast_from(value: &ArbitraryInt) -> Self {
        value.inner.as_ref().map(|v| v.to_i32()).unwrap_or(0)
    }
}

impl CastFrom<u32> for i32 {
    fn cast_from(value: &u32) -> Self {
        *value as Self
    }
}

impl CastFrom<u64> for i32 {
    fn cast_from(value: &u64) -> Self {
        *value as Self
    }
}

impl CastFrom<ArbitraryUnsignedInt> for i32 {
    fn cast_from(value: &ArbitraryUnsignedInt) -> Self {
        let v: u32 = value.cast_into();
        v as Self
    }
}

// casts to i64
impl CastFrom<i32> for i64 {
    fn cast_from(value: &i32) -> Self {
        *value as Self
    }
}

impl CastFrom<u32> for i64 {
    fn cast_from(value: &u32) -> Self {
        *value as Self
    }
}

impl CastFrom<u64> for i64 {
    fn cast_from(value: &u64) -> Self {
        *value as Self
    }
}

impl CastFrom<ArbitraryUnsignedInt> for i64 {
    fn cast_from(value: &ArbitraryUnsignedInt) -> Self {
        let v: u64 = value.cast_into();
        v as Self
    }
}

// Casts to ArbitraryInt
impl CastFrom<i32> for ArbitraryInt {
    fn cast_from(value: &i32) -> Self {
        Self {
            inner: Some(Awi::from_i32(*value)),
        }
    }
}

impl CastFrom<i64> for ArbitraryInt {
    fn cast_from(value: &i64) -> Self {
        Self {
            inner: Some(Awi::from_i64(*value)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vm::types::{UMCArithmetic, int::ArbitraryInt};

    #[test]
    fn add_wrapping_arbitrary_int() {
        let mut a = ArbitraryInt::zero(8);
        a.add_u32(120);

        let mut b = ArbitraryInt::zero(32);
        b.add_u32(10);

        a.add(&b);

        panic!("{}", a);
    }
}
