use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};

use crate::vm::types::uint::ArbitraryUnsignedInt;
use crate::vm::types::{CastFrom, CastInto, UMCArithmetic, UMCBitwise};

#[derive(PartialEq)]
pub struct ArbitraryInt {}

#[allow(unused_variables)]
impl UMCArithmetic for ArbitraryInt {
    fn add(&mut self, rhs: &Self) {
        todo!()
    }

    fn sub(&mut self, rhs: &Self) {
        todo!()
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

#[allow(unused_variables)]
impl UMCBitwise for ArbitraryInt {
    fn and(&mut self, rhs: &Self) {
        todo!()
    }

    fn or(&mut self, rhs: &Self) {
        todo!()
    }

    fn xor(&mut self, rhs: &Self) {
        todo!()
    }

    fn not(&mut self) {
        todo!()
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
        todo!()
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

// TODO: casts to ArbitraryUnsignedInt
