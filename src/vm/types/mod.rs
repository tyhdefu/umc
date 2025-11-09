use crate::vm::types::uint::ArbitraryUnsignedInt;

pub mod address;
pub mod uint;

pub trait UMCAddSub {
    /// Perform addition according to the UMC rules
    /// Silently and safely overflow if needed
    fn add(&mut self, rhs: &Self);

    /// Perform subtraction according to the UMC rules
    /// Silently and safely underflow if needed
    fn sub(&mut self, rhs: &Self);
}

pub trait UMCArithmetic: UMCAddSub + PartialEq {
    /// Bitwise AND
    fn and(&mut self, rhs: &Self);
    /// Bitwise XOR
    fn xor(&mut self, rhs: &Self);

    /// Logical bitwise NOT
    fn not(&mut self);
}

pub enum AddSubOp {
    Add,
    Sub,
}

impl AddSubOp {
    pub fn operate<T>(&self, a: &mut T, b: &T)
    where
        T: UMCAddSub,
    {
        match self {
            AddSubOp::Add => a.add(b),
            AddSubOp::Sub => a.sub(b),
        }
    }
}

pub enum BinaryArithmeticOp {
    AddOrSub(AddSubOp),
    And,
    Xor,
}

impl BinaryArithmeticOp {
    pub fn operate<T>(&self, a: &mut T, b: &T)
    where
        T: UMCArithmetic,
    {
        match &self {
            Self::AddOrSub(sub_op) => sub_op.operate(a, b),
            Self::And => a.and(b),
            Self::Xor => a.xor(b),
        }
    }
}

pub trait UMCNum: UMCArithmetic {}

/// Any non-vector type that can be cast between all other types
pub trait CastSingleAny: CastFrom<u32> + CastFrom<u64> + CastFrom<ArbitraryUnsignedInt> {}
impl<T> CastSingleAny for T where T: CastFrom<u32> + CastFrom<u64> + CastFrom<ArbitraryUnsignedInt> {}

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
}
