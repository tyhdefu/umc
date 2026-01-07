use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use crate::vm::types::{CastFrom, UMCArithmetic};

impl UMCArithmetic for f32 {
    fn add(&mut self, rhs: &Self) {
        self.add_assign(rhs);
    }

    fn sub(&mut self, rhs: &Self) {
        self.sub_assign(rhs);
    }

    fn modulo(&mut self, rhs: &Self) {
        *self = self.rem_euclid(*rhs);
    }

    fn mul(&mut self, rhs: &Self) {
        self.mul_assign(rhs);
    }

    fn div(&mut self, rhs: &Self) {
        self.div_assign(rhs);
    }
}

impl UMCArithmetic for f64 {
    fn add(&mut self, rhs: &Self) {
        self.add_assign(rhs);
    }

    fn sub(&mut self, rhs: &Self) {
        self.sub_assign(rhs);
    }

    fn modulo(&mut self, rhs: &Self) {
        *self = self.rem_euclid(*rhs);
    }

    fn mul(&mut self, rhs: &Self) {
        self.mul_assign(rhs);
    }

    fn div(&mut self, rhs: &Self) {
        self.div_assign(rhs);
    }
}

impl CastFrom<f64> for f32 {
    fn cast_from(value: &f64) -> Self {
        *value as f32
    }
}

impl CastFrom<f32> for f64 {
    fn cast_from(value: &f32) -> Self {
        *value as f64
    }
}
