use crate::{Arch, Possible, Supported, Task};

mod avx2;
mod sse2;

pub struct Sse2;

impl Possible for Sse2 {
    #[inline]
    fn supported() -> bool {
        is_x86_feature_detected!("sse2")
    }

    #[inline]
    unsafe fn invoke_unchecked<T: Task>(task: T) -> T::Result {
        sse2::Sse2Impl::invoke(task)
    }
}

#[cfg(target_feature = "sse2")]
unsafe impl Supported for Sse2 {}

pub struct Avx2;

impl Possible for Avx2 {
    #[inline]
    fn supported() -> bool {
        is_x86_feature_detected!("avx2")
    }

    #[inline]
    unsafe fn invoke_unchecked<T: Task>(task: T) -> T::Result {
        avx2::Avx2Impl::invoke(task)
    }
}

#[cfg(target_feature = "avx2")]
unsafe impl Supported for Avx2 {}
