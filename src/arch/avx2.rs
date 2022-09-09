#![allow(non_camel_case_types)]

use crate::mask::*;
use crate::{LanesEq, LanesOrd, Mask, Num, Select, Simd};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use core::fmt::{self, Debug};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use core::ops::{Index, IndexMut};
use std::mem;
use std::slice;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct f32x8(__m256);

impl Simd for f32x8 {
    type Elem = f32;

    const LANES: usize = 8;

    fn new(elem: Self::Elem) -> Self {
        unsafe { f32x8(_mm256_set1_ps(elem)) }
    }

    fn as_slice(&self) -> &[Self::Elem] {
        unsafe { slice::from_raw_parts(self as *const Self as *const Self::Elem, Self::LANES) }
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut Self::Elem, Self::LANES) }
    }

    fn from_slice(slice: &[Self::Elem]) -> Self {
        assert!(slice.len() == Self::LANES);
        unsafe { f32x8(_mm256_loadu_ps(slice.as_ptr())) }
    }

    fn write_to_slice(&self, slice: &mut [Self::Elem]) {
        assert!(slice.len() == Self::LANES);
        unsafe {
            _mm256_storeu_ps(slice.as_mut_ptr(), self.0);
        }
    }

    fn align_slice(slice: &[Self::Elem]) -> (&[Self::Elem], &[Self], &[Self::Elem]) {
        unsafe { slice.align_to::<Self>() }
    }

    fn align_mut_slice(
        slice: &mut [Self::Elem],
    ) -> (&mut [Self::Elem], &mut [Self], &mut [Self::Elem]) {
        unsafe { slice.align_to_mut::<Self>() }
    }
}

impl Debug for f32x8 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_slice(), fmt)
    }
}

impl Default for f32x8 {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl LanesEq for f32x8 {
    type Output = m32x8;

    fn eq(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_ps(self.0, other.0, _CMP_EQ_OQ);
            m32x8(_mm256_castps_si256(res))
        }
    }

    fn ne(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_ps(self.0, other.0, _CMP_NEQ_OQ);
            m32x8(_mm256_castps_si256(res))
        }
    }
}

impl LanesOrd for f32x8 {
    fn lt(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_ps(self.0, other.0, _CMP_LT_OQ);
            m32x8(_mm256_castps_si256(res))
        }
    }

    fn le(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_ps(self.0, other.0, _CMP_LE_OQ);
            m32x8(_mm256_castps_si256(res))
        }
    }

    fn gt(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_ps(self.0, other.0, _CMP_GT_OQ);
            m32x8(_mm256_castps_si256(res))
        }
    }

    fn ge(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_ps(self.0, other.0, _CMP_GE_OQ);
            m32x8(_mm256_castps_si256(res))
        }
    }
}

impl Index<usize> for f32x8 {
    type Output = <Self as Simd>::Elem;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for f32x8 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl Num for f32x8 {}

impl Add for f32x8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        unsafe { f32x8(_mm256_add_ps(self.0, rhs.0)) }
    }
}

impl AddAssign for f32x8 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for f32x8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        unsafe { f32x8(_mm256_sub_ps(self.0, rhs.0)) }
    }
}

impl SubAssign for f32x8 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for f32x8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        unsafe { f32x8(_mm256_mul_ps(self.0, rhs.0)) }
    }
}

impl MulAssign for f32x8 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for f32x8 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        unsafe { f32x8(_mm256_div_ps(self.0, rhs.0)) }
    }
}

impl DivAssign for f32x8 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Rem for f32x8 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl RemAssign for f32x8 {
    fn rem_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl Neg for f32x8 {
    type Output = Self;

    fn neg(self) -> Self {
        unsafe { f32x8(_mm256_sub_ps(_mm256_set1_ps(0.0), self.0)) }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct f64x4(__m256d);

impl Simd for f64x4 {
    type Elem = f64;

    const LANES: usize = 4;

    fn new(elem: Self::Elem) -> Self {
        unsafe { f64x4(_mm256_set1_pd(elem)) }
    }

    fn as_slice(&self) -> &[Self::Elem] {
        unsafe { slice::from_raw_parts(self as *const Self as *const Self::Elem, Self::LANES) }
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut Self::Elem, Self::LANES) }
    }

    fn from_slice(slice: &[Self::Elem]) -> Self {
        assert!(slice.len() == Self::LANES);
        unsafe { f64x4(_mm256_loadu_pd(slice.as_ptr())) }
    }

    fn write_to_slice(&self, slice: &mut [Self::Elem]) {
        assert!(slice.len() == Self::LANES);
        unsafe {
            _mm256_storeu_pd(slice.as_mut_ptr(), self.0);
        }
    }

    fn align_slice(slice: &[Self::Elem]) -> (&[Self::Elem], &[Self], &[Self::Elem]) {
        unsafe { slice.align_to::<Self>() }
    }

    fn align_mut_slice(
        slice: &mut [Self::Elem],
    ) -> (&mut [Self::Elem], &mut [Self], &mut [Self::Elem]) {
        unsafe { slice.align_to_mut::<Self>() }
    }
}

impl Debug for f64x4 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_slice(), fmt)
    }
}

impl Default for f64x4 {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl LanesEq for f64x4 {
    type Output = m64x4;

    fn eq(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_pd(self.0, other.0, _CMP_EQ_OQ);
            m64x4(_mm256_castpd_si256(res))
        }
    }

    fn ne(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_pd(self.0, other.0, _CMP_NEQ_OQ);
            m64x4(_mm256_castpd_si256(res))
        }
    }
}

impl Index<usize> for f64x4 {
    type Output = <Self as Simd>::Elem;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for f64x4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl Num for f64x4 {}

impl LanesOrd for f64x4 {
    fn lt(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_pd(self.0, other.0, _CMP_LT_OQ);
            m64x4(_mm256_castpd_si256(res))
        }
    }

    fn le(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_pd(self.0, other.0, _CMP_LE_OQ);
            m64x4(_mm256_castpd_si256(res))
        }
    }

    fn gt(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_pd(self.0, other.0, _CMP_GT_OQ);
            m64x4(_mm256_castpd_si256(res))
        }
    }

    fn ge(&self, other: &Self) -> Self::Output {
        unsafe {
            let res = _mm256_cmp_pd(self.0, other.0, _CMP_GE_OQ);
            m64x4(_mm256_castpd_si256(res))
        }
    }
}

impl Add for f64x4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        unsafe { f64x4(_mm256_add_pd(self.0, rhs.0)) }
    }
}

impl AddAssign for f64x4 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for f64x4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        unsafe { f64x4(_mm256_sub_pd(self.0, rhs.0)) }
    }
}

impl SubAssign for f64x4 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul for f64x4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        unsafe { f64x4(_mm256_mul_pd(self.0, rhs.0)) }
    }
}

impl MulAssign for f64x4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Div for f64x4 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        unsafe { f64x4(_mm256_div_pd(self.0, rhs.0)) }
    }
}

impl DivAssign for f64x4 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Rem for f64x4 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl RemAssign for f64x4 {
    fn rem_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl Neg for f64x4 {
    type Output = Self;

    fn neg(self) -> Self {
        unsafe { f64x4(_mm256_sub_pd(_mm256_set1_pd(0.0), self.0)) }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct m32x8(__m256i);

impl Simd for m32x8 {
    type Elem = m32;

    const LANES: usize = 8;

    fn new(elem: Self::Elem) -> Self {
        unsafe { m32x8(_mm256_set1_epi32(mem::transmute(elem))) }
    }

    fn as_slice(&self) -> &[Self::Elem] {
        unsafe { slice::from_raw_parts(self as *const Self as *const Self::Elem, Self::LANES) }
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut Self::Elem, Self::LANES) }
    }

    fn from_slice(slice: &[Self::Elem]) -> Self {
        assert!(slice.len() == Self::LANES);
        unsafe { m32x8(_mm256_loadu_si256(slice.as_ptr() as *const __m256i)) }
    }

    fn write_to_slice(&self, slice: &mut [Self::Elem]) {
        assert!(slice.len() == Self::LANES);
        unsafe {
            _mm256_storeu_si256(slice.as_mut_ptr() as *mut __m256i, self.0);
        }
    }

    fn align_slice(slice: &[Self::Elem]) -> (&[Self::Elem], &[Self], &[Self::Elem]) {
        unsafe { slice.align_to::<Self>() }
    }

    fn align_mut_slice(
        slice: &mut [Self::Elem],
    ) -> (&mut [Self::Elem], &mut [Self], &mut [Self::Elem]) {
        unsafe { slice.align_to_mut::<Self>() }
    }
}

impl Debug for m32x8 {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_slice(), fmt)
    }
}

impl Default for m32x8 {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl LanesEq for m32x8 {
    type Output = m32x8;

    fn eq(&self, other: &Self) -> Self::Output {
        unsafe { m32x8(_mm256_cmpeq_epi32(self.0, other.0)) }
    }

    fn ne(&self, other: &Self) -> Self::Output {
        unsafe { m32x8(_mm256_xor_si256(self.0, other.0)) }
    }
}

impl LanesOrd for m32x8 {
    fn lt(&self, other: &Self) -> Self::Output {
        unsafe { m32x8(_mm256_andnot_si256(self.0, other.0)) }
    }

    fn gt(&self, other: &Self) -> Self::Output {
        unsafe { m32x8(_mm256_andnot_si256(other.0, self.0)) }
    }
}

impl Index<usize> for m32x8 {
    type Output = <Self as Simd>::Elem;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl IndexMut<usize> for m32x8 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl Mask for m32x8 {}

impl BitAnd for m32x8 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { m32x8(_mm256_and_si256(self.0, rhs.0)) }
    }
}

impl BitAndAssign for m32x8 {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for m32x8 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { m32x8(_mm256_or_si256(self.0, rhs.0)) }
    }
}

impl BitOrAssign for m32x8 {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for m32x8 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { m32x8(_mm256_xor_si256(self.0, rhs.0)) }
    }
}

impl BitXorAssign for m32x8 {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for m32x8 {
    type Output = Self;

    fn not(self) -> Self::Output {
        unsafe { m32x8(_mm256_andnot_si256(self.0, _mm256_set1_epi8(!0))) }
    }
}

impl Select<f32x8> for m32x8 {
    fn select(self, if_true: f32x8, if_false: f32x8) -> f32x8 {
        unsafe {
            let mask = _mm256_castsi256_ps(self.0);
            f32x8(_mm256_blendv_ps(if_false.0, if_true.0, mask))
        }
    }
}

impl Select<m32x8> for m32x8 {
    fn select(self, if_true: m32x8, if_false: m32x8) -> m32x8 {
        unsafe { m32x8(_mm256_blendv_epi8(if_false.0, if_true.0, self.0)) }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct m64x4(__m256i);

impl Mask for m64x4 {}

impl BitAnd for m64x4 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { m64x4(_mm256_and_si256(self.0, rhs.0)) }
    }
}

impl BitAndAssign for m64x4 {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for m64x4 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { m64x4(_mm256_or_si256(self.0, rhs.0)) }
    }
}

impl BitOrAssign for m64x4 {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for m64x4 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { m64x4(_mm256_xor_si256(self.0, rhs.0)) }
    }
}

impl BitXorAssign for m64x4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Not for m64x4 {
    type Output = Self;

    fn not(self) -> Self::Output {
        unsafe { m64x4(_mm256_andnot_si256(self.0, _mm256_set1_epi8(!0))) }
    }
}

impl Select<f64x4> for m64x4 {
    fn select(self, if_true: f64x4, if_false: f64x4) -> f64x4 {
        unsafe {
            let mask = _mm256_castsi256_pd(self.0);
            f64x4(_mm256_blendv_pd(if_false.0, if_true.0, mask))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let x = f32x8::new(3.0);
        let y = x
            .eq(&f32x8::new(3.0))
            .select(f32x8::new(0.0), f32x8::new(1.0));
        assert_eq!(y[0], 0.0);
    }
}
