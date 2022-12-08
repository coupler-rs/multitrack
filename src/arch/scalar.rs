#![allow(non_camel_case_types)]

use core::fmt::{self, Debug};
use core::num::Wrapping;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use core::ops::{Index, IndexMut};
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
use core::slice;

use crate::simd::*;
use crate::{Arch, Convert16, Convert32, Convert64, Convert8, Task};

pub struct ScalarImpl;

impl Arch for ScalarImpl {
    type f32 = f32x1;
    type f64 = f64x1;

    type u8 = u8x1;
    type u16 = u16x1;
    type u32 = u32x1;
    type u64 = u64x1;

    type i8 = i8x1;
    type i16 = i16x1;
    type i32 = i32x1;
    type i64 = i64x1;

    type m8 = m8x1;
    type m16 = m16x1;
    type m32 = m32x1;
    type m64 = m64x1;

    const NAME: &'static str = "scalar";

    #[inline]
    fn invoke<T: Task>(task: T) -> T::Result {
        task.run::<ScalarImpl>()
    }
}

macro_rules! scalar_type {
    ($scalar:ident, $inner:ident, $mask:ident) => {
        #[derive(Copy, Clone, Default)]
        #[repr(transparent)]
        pub struct $scalar($inner);

        impl Simd for $scalar {
            type Elem = $inner;
            type Mask = $mask;

            const LANES: usize = 1;

            #[inline]
            fn new(elem: Self::Elem) -> Self {
                $scalar(elem)
            }
        }

        impl LanesEq for $scalar {
            type Output = $mask;

            #[inline]
            fn eq(&self, other: &$scalar) -> $mask {
                $mask((self.0 == other.0).into())
            }
        }

        impl LanesOrd for $scalar {
            #[inline]
            fn lt(&self, other: &$scalar) -> $mask {
                $mask((self.0 < other.0).into())
            }
        }

        impl Index<usize> for $scalar {
            type Output = $inner;

            #[inline]
            fn index(&self, index: usize) -> &$inner {
                assert!(index == 0);
                &self.0
            }
        }

        impl IndexMut<usize> for $scalar {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut $inner {
                assert!(index == 0);
                &mut self.0
            }
        }

        impl Debug for $scalar {
            #[inline]
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().entry(&self.0).finish()
            }
        }

        impl Select<$scalar> for $mask {
            #[inline]
            fn select(self, if_true: $scalar, if_false: $scalar) -> $scalar {
                if self.0.into() {
                    if_true
                } else {
                    if_false
                }
            }
        }
    };
}

macro_rules! wrapping_scalar_type {
    ($scalar:ident, $inner:ident, $mask:ident) => {
        #[derive(Copy, Clone, Default)]
        #[repr(transparent)]
        pub struct $scalar(Wrapping<$inner>);

        impl Simd for $scalar {
            type Elem = $inner;
            type Mask = $mask;

            const LANES: usize = 1;

            #[inline]
            fn new(elem: Self::Elem) -> Self {
                $scalar(Wrapping(elem))
            }
        }

        impl LanesEq for $scalar {
            type Output = $mask;

            #[inline]
            fn eq(&self, other: &$scalar) -> $mask {
                $mask((self.0 == other.0).into())
            }
        }

        impl LanesOrd for $scalar {
            #[inline]
            fn lt(&self, other: &$scalar) -> $mask {
                $mask((self.0 < other.0).into())
            }
        }

        impl Index<usize> for $scalar {
            type Output = $inner;

            #[inline]
            fn index(&self, index: usize) -> &$inner {
                assert!(index == 0);
                &self.0 .0
            }
        }

        impl IndexMut<usize> for $scalar {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut $inner {
                assert!(index == 0);
                &mut self.0 .0
            }
        }

        impl Debug for $scalar {
            #[inline]
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.debug_list().entry(&self.0 .0).finish()
            }
        }

        impl Select<$scalar> for $mask {
            #[inline]
            fn select(self, if_true: $scalar, if_false: $scalar) -> $scalar {
                if self.0.into() {
                    if_true
                } else {
                    if_false
                }
            }
        }
    };
}

macro_rules! impl_as_slice {
    ($scalar:ident) => {
        impl AsSlice for $scalar {
            #[inline]
            fn as_slice(&self) -> &[Self::Elem] {
                slice::from_ref(&self.0)
            }

            #[inline]
            fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
                slice::from_mut(&mut self.0)
            }

            #[inline]
            fn from_slice(slice: &[Self::Elem]) -> Self {
                Self::new(slice[0])
            }

            #[inline]
            fn write_to_slice(&self, slice: &mut [Self::Elem]) {
                slice[0] = self.0;
            }

            #[inline]
            fn align_slice(slice: &[Self::Elem]) -> (&[Self::Elem], &[Self], &[Self::Elem]) {
                unsafe { slice.align_to::<Self>() }
            }

            #[inline]
            fn align_mut_slice(
                slice: &mut [Self::Elem],
            ) -> (&mut [Self::Elem], &mut [Self], &mut [Self::Elem]) {
                unsafe { slice.align_to_mut::<Self>() }
            }
        }
    };
}

macro_rules! impl_as_slice_wrapping {
    ($scalar:ident) => {
        impl AsSlice for $scalar {
            #[inline]
            fn as_slice(&self) -> &[Self::Elem] {
                slice::from_ref(&self.0 .0)
            }

            #[inline]
            fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
                slice::from_mut(&mut self.0 .0)
            }

            #[inline]
            fn from_slice(slice: &[Self::Elem]) -> Self {
                Self::new(slice[0])
            }

            #[inline]
            fn write_to_slice(&self, slice: &mut [Self::Elem]) {
                slice[0] = self.0 .0;
            }

            #[inline]
            fn align_slice(slice: &[Self::Elem]) -> (&[Self::Elem], &[Self], &[Self::Elem]) {
                unsafe { slice.align_to::<Self>() }
            }

            #[inline]
            fn align_mut_slice(
                slice: &mut [Self::Elem],
            ) -> (&mut [Self::Elem], &mut [Self], &mut [Self::Elem]) {
                unsafe { slice.align_to_mut::<Self>() }
            }
        }
    };
}

macro_rules! impl_convert8 {
    ($vector:ident) => {
        impl Convert8<ScalarImpl> for $vector {}

        impl Widen<f32x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(f32x1),
            {
                unimplemented!()
            }
        }

        impl Widen<f64x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(f64x1),
            {
                unimplemented!()
            }
        }

        impl Convert<u8x1> for $vector {
            fn convert(self) -> u8x1 {
                unimplemented!()
            }
        }

        impl Widen<u16x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(u16x1),
            {
                unimplemented!()
            }
        }

        impl Widen<u32x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(u32x1),
            {
                unimplemented!()
            }
        }

        impl Widen<u64x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(u64x1),
            {
                unimplemented!()
            }
        }

        impl Convert<i8x1> for $vector {
            fn convert(self) -> i8x1 {
                unimplemented!()
            }
        }

        impl Widen<i16x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(i16x1),
            {
                unimplemented!()
            }
        }

        impl Widen<i32x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(i32x1),
            {
                unimplemented!()
            }
        }

        impl Widen<i64x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(i64x1),
            {
                unimplemented!()
            }
        }
    };
}

macro_rules! impl_convert16 {
    ($vector:ident) => {
        impl Convert16<ScalarImpl> for $vector {}

        impl Widen<f32x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(f32x1),
            {
                unimplemented!()
            }
        }

        impl Widen<f64x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(f64x1),
            {
                unimplemented!()
            }
        }

        impl Narrow<u8x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Convert<u16x1> for $vector {
            fn convert(self) -> u16x1 {
                unimplemented!()
            }
        }

        impl Widen<u32x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(u32x1),
            {
                unimplemented!()
            }
        }

        impl Widen<u64x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(u64x1),
            {
                unimplemented!()
            }
        }

        impl Narrow<i8x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Convert<i16x1> for $vector {
            fn convert(self) -> i16x1 {
                unimplemented!()
            }
        }

        impl Widen<i32x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(i32x1),
            {
                unimplemented!()
            }
        }

        impl Widen<i64x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(i64x1),
            {
                unimplemented!()
            }
        }
    };
}

macro_rules! impl_convert32 {
    ($vector:ident) => {
        impl Convert32<ScalarImpl> for $vector {}

        impl Convert<f32x1> for $vector {
            fn convert(self) -> f32x1 {
                unimplemented!()
            }
        }

        impl Widen<f64x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(f64x1),
            {
                unimplemented!()
            }
        }

        impl Narrow<u8x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Narrow<u16x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Convert<u32x1> for $vector {
            fn convert(self) -> u32x1 {
                unimplemented!()
            }
        }

        impl Widen<u64x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(u64x1),
            {
                unimplemented!()
            }
        }

        impl Narrow<i8x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Narrow<i16x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Convert<i32x1> for $vector {
            fn convert(self) -> i32x1 {
                unimplemented!()
            }
        }

        impl Widen<i64x1> for $vector {
            fn widen<F>(self, _consume: F)
            where
                F: FnMut(i64x1),
            {
                unimplemented!()
            }
        }
    };
}

macro_rules! impl_convert64 {
    ($vector:ident) => {
        impl Convert64<ScalarImpl> for $vector {}

        impl Narrow<f32x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Convert<f64x1> for $vector {
            fn convert(self) -> f64x1 {
                unimplemented!()
            }
        }

        impl Narrow<u8x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Narrow<u16x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Narrow<u32x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Convert<u64x1> for $vector {
            fn convert(self) -> u64x1 {
                unimplemented!()
            }
        }

        impl Narrow<i8x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Narrow<i16x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Narrow<i32x1> for $vector {
            fn narrow<F>(_produce: F)
            where
                F: FnMut() -> Self,
            {
                unimplemented!()
            }
        }

        impl Convert<i64x1> for $vector {
            fn convert(self) -> i64x1 {
                unimplemented!()
            }
        }
    };
}

macro_rules! impl_float {
    ($float:ident) => {
        impl Float for $float {}

        impl Add for $float {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                $float(self.0 + rhs.0)
            }
        }

        impl AddAssign for $float {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl Sub for $float {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                $float(self.0 - rhs.0)
            }
        }

        impl SubAssign for $float {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl Mul for $float {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self {
                $float(self.0 * rhs.0)
            }
        }

        impl MulAssign for $float {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
            }
        }

        impl Div for $float {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self {
                $float(self.0 / rhs.0)
            }
        }

        impl DivAssign for $float {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0;
            }
        }

        impl Neg for $float {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                $float(-self.0)
            }
        }
    };
}

macro_rules! impl_int {
    ($int:ident) => {
        impl Int for $int {}

        impl Add for $int {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                $int(self.0 + rhs.0)
            }
        }

        impl AddAssign for $int {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl Sub for $int {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                $int(self.0 - rhs.0)
            }
        }

        impl SubAssign for $int {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl Mul for $int {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self {
                $int(self.0 * rhs.0)
            }
        }

        impl MulAssign for $int {
            #[inline]
            fn mul_assign(&mut self, rhs: Self) {
                self.0 *= rhs.0;
            }
        }

        impl Div for $int {
            type Output = Self;

            #[inline]
            fn div(self, rhs: Self) -> Self {
                $int(self.0 / rhs.0)
            }
        }

        impl DivAssign for $int {
            #[inline]
            fn div_assign(&mut self, rhs: Self) {
                self.0 /= rhs.0;
            }
        }

        impl Neg for $int {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                $int(-self.0)
            }
        }

        impl Shl<usize> for $int {
            type Output = Self;

            #[inline]
            fn shl(self, rhs: usize) -> Self {
                $int(self.0 << rhs)
            }
        }

        impl ShlAssign<usize> for $int {
            #[inline]
            fn shl_assign(&mut self, rhs: usize) {
                self.0 <<= rhs;
            }
        }

        impl Shr<usize> for $int {
            type Output = Self;

            #[inline]
            fn shr(self, rhs: usize) -> Self {
                $int(self.0 >> rhs)
            }
        }

        impl ShrAssign<usize> for $int {
            #[inline]
            fn shr_assign(&mut self, rhs: usize) {
                self.0 >>= rhs;
            }
        }

        impl BitAnd for $int {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output {
                $int(self.0 & rhs.0)
            }
        }

        impl BitAndAssign for $int {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl BitOr for $int {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                $int(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $int {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl BitXor for $int {
            type Output = Self;

            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                $int(self.0 ^ rhs.0)
            }
        }

        impl BitXorAssign for $int {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }

        impl Not for $int {
            type Output = Self;

            #[inline]
            fn not(self) -> Self::Output {
                $int(!self.0)
            }
        }
    };
}

macro_rules! impl_mask {
    ($mask:ident, $uint:ident) => {
        impl Mask for $mask {}

        impl BitAnd for $mask {
            type Output = Self;

            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output {
                $mask(self.0 & rhs.0)
            }
        }

        impl BitAndAssign for $mask {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }

        impl BitOr for $mask {
            type Output = Self;

            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                $mask(self.0 | rhs.0)
            }
        }

        impl BitOrAssign for $mask {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }

        impl BitXor for $mask {
            type Output = Self;

            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                $mask(self.0 ^ rhs.0)
            }
        }

        impl BitXorAssign for $mask {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }

        impl Not for $mask {
            type Output = Self;

            #[inline]
            fn not(self) -> Self::Output {
                $mask(!self.0)
            }
        }

        impl Convert<$uint> for $mask {
            fn convert(self) -> $uint {
                $uint(Wrapping(if self.0 { !0 } else { 0 }))
            }
        }
    };
}

scalar_type! { f32x1, f32, m32x1 }
scalar_type! { f64x1, f64, m64x1 }
impl_as_slice! { f32x1 }
impl_as_slice! { f64x1 }
impl_convert32! { f32x1 }
impl_convert64! { f64x1 }
impl_float! { f64x1 }
impl_float! { f32x1 }

wrapping_scalar_type! { u8x1, u8, m8x1 }
wrapping_scalar_type! { u16x1, u16, m16x1 }
wrapping_scalar_type! { u32x1, u32, m32x1 }
wrapping_scalar_type! { u64x1, u64, m64x1 }
impl_as_slice_wrapping! { u8x1 }
impl_as_slice_wrapping! { u16x1 }
impl_as_slice_wrapping! { u32x1 }
impl_as_slice_wrapping! { u64x1 }
impl_convert8! { u8x1 }
impl_convert16! { u16x1 }
impl_convert32! { u32x1 }
impl_convert64! { u64x1 }
impl_int! { u8x1 }
impl_int! { u16x1 }
impl_int! { u32x1 }
impl_int! { u64x1 }

wrapping_scalar_type! { i8x1, i8, m8x1 }
wrapping_scalar_type! { i16x1, i16, m16x1 }
wrapping_scalar_type! { i32x1, i32, m32x1 }
wrapping_scalar_type! { i64x1, i64, m64x1 }
impl_as_slice_wrapping! { i8x1 }
impl_as_slice_wrapping! { i16x1 }
impl_as_slice_wrapping! { i32x1 }
impl_as_slice_wrapping! { i64x1 }
impl_convert8! { i8x1 }
impl_convert16! { i16x1 }
impl_convert32! { i32x1 }
impl_convert64! { i64x1 }
impl_int! { i8x1 }
impl_int! { i16x1 }
impl_int! { i32x1 }
impl_int! { i64x1 }

scalar_type! { m8x1, bool, m8x1 }
scalar_type! { m16x1, bool, m16x1 }
scalar_type! { m32x1, bool, m32x1 }
scalar_type! { m64x1, bool, m64x1 }
impl_mask! { m8x1, u8x1 }
impl_mask! { m16x1, u16x1 }
impl_mask! { m32x1, u32x1 }
impl_mask! { m64x1, u64x1 }
