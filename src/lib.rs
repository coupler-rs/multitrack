use core::fmt::Debug;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use core::ops::{Index, IndexMut};

pub mod mask;
use mask::*;

mod scalar;

#[allow(non_camel_case_types)]
pub trait Arch
where
    Self::m32: Select<Self::f32> + Select<Self::m32>,
{
    type f32: Simd<Arch = Self, Elem = f32> + Num;
    type m32: Simd<Arch = Self, Elem = m32> + Mask;
}

pub trait Simd: Copy + Clone + Debug + Default + Send + Sync + Sized
where
    Self: LanesEq,
    Self: Index<usize, Output = Self::Elem> + IndexMut<usize, Output = Self::Elem>,
{
    type Arch: Arch;
    type Elem;

    const LANES: usize;

    fn new(elem: Self::Elem) -> Self;

    fn as_slice(&self) -> &[Self::Elem];
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
    fn from_slice(slice: &[Self::Elem]) -> Self;
    fn write_to_slice(&self, slice: &mut [Self::Elem]);
    fn align_slice(slice: &[Self::Elem]) -> (&[Self::Elem], &[Self], &[Self::Elem]);
    fn align_mut_slice(
        slice: &mut [Self::Elem],
    ) -> (&mut [Self::Elem], &mut [Self], &mut [Self::Elem]);
}

pub trait Num: Sized
where
    Self: LanesOrd,
    Self: Add<Output = Self> + AddAssign,
    Self: Sub<Output = Self> + SubAssign,
    Self: Mul<Output = Self> + MulAssign,
    Self: Div<Output = Self> + DivAssign,
    Self: Rem<Output = Self> + RemAssign,
    Self: Neg,
{
}

pub trait Mask: Sized
where
    Self: BitAnd<Output = Self> + BitAndAssign,
    Self: BitOr<Output = Self> + BitOrAssign,
    Self: BitXor<Output = Self> + BitXorAssign,
    Self: Not<Output = Self>,
{
}

pub trait LanesEq<Rhs = Self>: Sized {
    type Output: Mask + Select<Self>;

    fn eq(&self, other: &Self) -> Self::Output;

    fn ne(&self, other: &Self) -> Self::Output {
        !self.eq(other)
    }
}

pub trait LanesOrd<Rhs = Self>: LanesEq<Rhs> {
    fn lt(&self, other: &Self) -> Self::Output;

    fn le(&self, other: &Self) -> Self::Output {
        self.lt(other) | self.eq(other)
    }

    fn gt(&self, other: &Self) -> Self::Output {
        !self.le(other)
    }

    fn ge(&self, other: &Self) -> Self::Output {
        !self.lt(other)
    }

    fn max(self, other: Self) -> Self {
        other.lt(&self).select(self, other)
    }

    fn min(self, other: Self) -> Self {
        self.lt(&other).select(self, other)
    }

    fn clamp(self, min: Self, max: Self) -> Self {
        self.max(min).min(max)
    }
}

pub trait Select<V> {
    fn select(self, if_true: V, if_false: V) -> V;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        fn f<A: Arch>() {
            let mut x = A::f32::new(0.0);
            x[0] = 1.0;

            let y = A::f32::new(2.0);

            let z = x.lt(&y).select(x + y, x * y);

            assert_eq!(z[0], 3.0);
        }

        f::<scalar::Scalar>();
    }

    #[test]
    fn align_slice() {
        fn f<A: Arch>() {
            let mut a = [0.0; 100];

            let (prefix, middle, suffix) = A::f32::align_mut_slice(&mut a);
            for x in prefix {
                *x += 1.0;
            }
            for x in middle {
                *x += A::f32::new(1.0);
            }
            for x in suffix {
                *x += 1.0;
            }

            assert_eq!(&a, &[1.0; 100]);
        }

        f::<scalar::Scalar>();
    }
}
