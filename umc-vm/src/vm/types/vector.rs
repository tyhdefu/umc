use std::fmt::Display;
use std::iter::repeat_n;

#[derive(Debug, PartialEq)]
pub struct VecValue<T>(Box<[T]>);

impl<T: Clone> Clone for VecValue<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn clone_from(&mut self, source: &Self) {
        if source.len() == self.len() {
            self.0.clone_from_slice(source.as_slice());
        } else {
            *self = source.clone();
        }
    }
}

impl<T> Display for VecValue<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

impl<T: Clone> VecValue<T> {
    pub fn from_repeated(v: T, count: usize) -> Self
    where
        T: Clone,
    {
        Self::from_vec(repeat_n(v, count).collect())
    }

    pub fn from_repeated_default(count: usize) -> Self
    where
        T: Default,
    {
        Self::from_vec(vec![T::default(); count])
    }

    /// Replace the contents of this VecValue with the slice
    pub fn clone_from_slice(&mut self, src: &[T]) {
        self.0.clone_from_slice(src);
    }

    pub fn copy_from_slice(&mut self, src: &[T])
    where
        T: Copy,
    {
        self.0.copy_from_slice(src);
    }
}

impl<T> VecValue<T> {
    /// Initialise from an existing vector
    pub fn from_vec(vec: Vec<T>) -> Self {
        Self(vec.into_boxed_slice())
    }

    pub fn as_slice(&self) -> &[T] {
        &self.0[..]
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.0[..]
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Perform a unary operation on each element of this vector value
    pub fn unary_op<F>(&mut self, operation: F)
    where
        F: Fn(&mut T),
    {
        for x in self.0.iter_mut() {
            operation(x);
        }
    }

    /// Perform a vector broadcasting operation
    /// Elements are taken as the LHS and other as the RHS
    pub fn broadcast_op<F>(&mut self, other: &T, operation: F)
    where
        F: Fn(&mut T, &T),
    {
        for x in self.0.iter_mut() {
            operation(x, other);
        }
    }

    /// Perform a vector broadcasting operation in reverse
    /// Elements are taken as the RHS and other as the LHS (cloned for each element)
    pub fn broadcast_op_reversed<F>(&mut self, other: &T, operation: F)
    where
        F: Fn(&mut T, &T),
        T: Clone,
    {
        for x in self.0.iter_mut() {
            let mut v = other.clone();
            operation(&mut v, x);
            *x = v;
        }
    }

    /// Perform a vector-vector element-wise operation
    /// Panics if the vectors are not the same length
    pub fn vector_op<F>(&mut self, other: &Self, operation: F)
    where
        F: Fn(&mut T, &T),
    {
        assert_eq!(self.len(), other.len());

        for (x, y) in self.0.iter_mut().zip(other.0.iter()) {
            operation(x, y)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::vm::types::vector::VecValue;
    use crate::vm::types::{UMCArithmetic, UMCBitwise};

    #[test]
    fn not_elements() {
        let mut v: VecValue<u32> = VecValue::from_vec(vec![0, 1, 2, 3]);
        v.unary_op(UMCBitwise::not);

        let exp: Vec<u32> = [0, 1, 2, 3]
            .iter_mut()
            .map(|x| {
                x.not();
                *x
            })
            .collect();
        assert_eq!(exp.as_slice(), v.as_slice());
    }
    #[test]
    fn broadcast_double() {
        let mut v = VecValue::from_vec(vec![0, 1, 2, 3]);
        v.broadcast_op(&2, UMCArithmetic::mul);
        assert_eq!(&[0, 2, 4, 6], v.as_slice());
    }

    #[test]
    fn broadcast_add() {
        let mut v = VecValue::from_vec(vec![0, 1, 2, 3]);
        v.broadcast_op(&1, UMCArithmetic::add);
        assert_eq!(&[1, 2, 3, 4], v.as_slice());
    }

    #[test]
    fn vector_add() {
        use crate::vm::types::UMCArithmetic;
        let mut v1 = VecValue::from_vec(vec![0, 1, 2, 3]);
        let v2 = VecValue::from_vec(vec![3, 2, 1, 0]);
        v1.vector_op(&v2, UMCArithmetic::add);
        assert_eq!(&[3, 3, 3, 3], v1.as_slice());
    }
}
