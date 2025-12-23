use crate::vm::types::uint::ArbitraryUnsignedInt;

pub mod address;
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

    /// Bitwise AND
    fn and(&mut self, rhs: &Self);
    /// Bitwise XOR
    fn xor(&mut self, rhs: &Self);

    /// Logical bitwise NOT
    fn not(&mut self);
}

pub enum BinaryArithmeticOp {
    Add,
    Sub,
    And,
    Xor,
}

impl BinaryArithmeticOp {
    pub fn operate<T>(&self, a: &mut T, b: &T)
    where
        T: UMCArithmetic,
    {
        match &self {
            Self::Add => a.add(b),
            Self::Sub => a.sub(b),
            Self::And => a.and(b),
            Self::Xor => a.xor(b),
        }
    }
}

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
