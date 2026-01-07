use crate::vm::types::uint::ArbitraryUnsignedInt;

pub mod address;
pub mod float;
pub mod int;
pub mod uint;

pub trait UMCOffset {
    fn offset(&mut self, offset: isize);
}

pub trait UMCArithmetic: PartialEq {
    /// Perform addition according to the UMC rules
    /// Silently and safely overflow if needed
    fn add(&mut self, rhs: &Self);

    /// Perform subtraction according to the UMC rules
    /// Silently and safely underflow if needed
    fn sub(&mut self, rhs: &Self);

    /// Perform multiplication according to the UMC rules
    /// Silently and safely overflow if needed
    fn mul(&mut self, rhs: &Self);

    /// Peform multiplication according to the UMC rules
    fn div(&mut self, rhs: &Self);

    /// Modulo operation
    fn modulo(&mut self, rhs: &Self);
}

pub trait UMCBitwise: PartialEq {
    /// Bitwise AND
    fn and(&mut self, rhs: &Self);
    /// Bitwise XOR
    fn xor(&mut self, rhs: &Self);
    /// Logical bitwise NOT
    fn not(&mut self);
}

pub trait BinaryOp<V> {
    fn operate(&self, a: &mut V, b: &V);
}

pub enum BinaryArithmeticOp {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
}

pub enum BinaryBitwiseOp {
    And,
    Xor,
}

impl<V> BinaryOp<V> for BinaryArithmeticOp
where
    V: UMCArithmetic,
{
    fn operate(&self, a: &mut V, b: &V) {
        match &self {
            Self::Add => a.add(b),
            Self::Sub => a.sub(b),
            Self::Mul => a.mul(b),
            Self::Div => a.div(b),
            Self::Modulo => a.modulo(b),
        }
    }
}

impl<V> BinaryOp<V> for BinaryBitwiseOp
where
    V: UMCBitwise,
{
    fn operate(&self, a: &mut V, b: &V) {
        match &self {
            BinaryBitwiseOp::And => a.and(b),
            BinaryBitwiseOp::Xor => a.xor(b),
        }
    }
}

impl BinaryBitwiseOp {}

/// Any non-vector type that can be cast into from an unsigned integer
pub trait CastSingleUnsigned:
    CastFrom<u32> + CastFrom<u64> + CastFrom<ArbitraryUnsignedInt>
{
}
impl<T> CastSingleUnsigned for T where
    T: CastFrom<u32> + CastFrom<u64> + CastFrom<ArbitraryUnsignedInt>
{
}

/// Any non-vector type that can be cast into from a signed integer
pub trait CastSingleSigned: CastFrom<i32> + CastFrom<i64> /*+ CastFrom<ArbitraryInt>*/ {}
impl<T> CastSingleSigned for T where T: CastFrom<i32> + CastFrom<i64> /*+ CastFrom<ArbitraryInt>*/ {}

pub trait CastSingleFloat: CastFrom<f32> + CastFrom<f64> {}
impl<T> CastSingleFloat for T where T: CastFrom<f32> + CastFrom<f64> {}

/// Any non-vector type that can be cast between all other types
pub trait CastSingleAny: CastSingleUnsigned + CastSingleSigned {}
impl<T> CastSingleAny for T where T: CastSingleUnsigned + CastSingleSigned {}

pub trait CastFrom<T> {
    fn cast_from(value: &T) -> Self;
}

pub trait CastInto<T> {
    fn cast_into(&self) -> T;
}

impl<F, T> CastInto<T> for F
where
    T: CastFrom<F>,
{
    fn cast_into(&self) -> T {
        T::cast_from(self)
    }
}

impl<T> CastFrom<T> for T
where
    T: Copy,
{
    fn cast_from(value: &T) -> Self {
        *value
    }
}

#[cfg(test)]
mod test {
    use crate::vm::types::uint::ArbitraryUnsignedInt;

    use super::*;

    #[test]
    fn test_add_arbitrary_uint() {
        let mut x = ArbitraryUnsignedInt::new(128);
        x.add(&u64::MAX.cast_into());
        x.add(&(3u32.cast_into()));

        assert_eq!(x.data(), &[2, 1])
    }

    #[test]
    fn test_add_overflow_u3() {
        let mut x = ArbitraryUnsignedInt::new(3);
        x.add(&5u32.cast_into());
        x.add(&5u32.cast_into());

        assert_eq!(x.data(), &[2])
    }
}
