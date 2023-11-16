use core::marker::PhantomData;

mod sealed {
    pub trait UnpinGuard {}
}

#[doc(hidden)]
pub struct UnpinGuardWrapper<'lt, T: ?Sized>(PhantomData<&'lt ()>, T);
#[doc(hidden)]
pub unsafe trait UnpinGuard: sealed::UnpinGuard {}
impl<T: ?Sized + Unpin> sealed::UnpinGuard for UnpinGuardWrapper<'_, T> {}
unsafe impl<T: ?Sized + Unpin> UnpinGuard for UnpinGuardWrapper<'_, T> {}
