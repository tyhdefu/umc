pub mod uint;

pub trait UMCArithmetic: PartialEq {
    /// Perform addition according to the UMC rules
    /// Silently and safely overflow if needed
    fn add(&mut self, rhs: &Self);

    //fn sub(&self, rhs: Self) -> Self;
}

pub enum BinaryArithmeticOp {
    Add,
}

impl BinaryArithmeticOp {
    pub fn operate<T>(&self, a: &mut T, b: &T)
    where
        T: UMCArithmetic,
    {
        match &self {
            Self::Add => a.add(b),
        }
    }
}

pub trait UMCNum: UMCArithmetic {}

pub trait CastFrom<T> {
    fn cast_from(value: T) -> Self;
}

pub trait CastInto<T> {
    fn cast_into(self) -> T;
}

impl<F, T> CastInto<T> for F
where
    T: CastFrom<F>,
{
    fn cast_into(self) -> T {
        T::cast_from(self)
    }
}

impl<T> CastFrom<T> for T {
    fn cast_from(value: T) -> Self {
        value
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
