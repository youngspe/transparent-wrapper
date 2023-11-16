mod alloc_impls;
use core::{
    cell::{Cell, UnsafeCell},
    cmp::Reverse,
    mem::{ManuallyDrop, MaybeUninit},
    ops::Deref,
    pin::Pin,
    ptr::NonNull,
    sync::atomic::AtomicPtr,
};

use crate::{
    custom_cast::{
        mappings::{WrapArray, WrapManuallyDrop, WrapMaybeUninit, WrapSlice},
        Covariant, Invariant, MapTo, Mappable, SafeMapping, SafeMappingToInner, SafeMappingToOuter,
        Wrapper,
    },
    TransparentMapping,
};

unsafe impl<M: ?Sized + TransparentMapping> TransparentMapping for ManuallyDrop<M> {
    type Inner = ManuallyDrop<M::Inner>;
    type Outer = ManuallyDrop<M::Outer>;

    #[inline(always)]
    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        M::ptr_into_inner(outer as *mut _) as *mut _
    }

    #[inline(always)]
    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        M::ptr_into_outer(inner as *mut _) as *mut _
    }
}

unsafe impl<M: ?Sized + SafeMappingToInner> SafeMappingToInner for ManuallyDrop<M> {}
unsafe impl<M: ?Sized + SafeMappingToOuter> SafeMappingToOuter for ManuallyDrop<M> {}

impl<T: ?Sized> Wrapper for ManuallyDrop<T> {
    type Wrapping = WrapManuallyDrop<T>;
}

unsafe impl<M: TransparentMapping> TransparentMapping for MaybeUninit<M> {
    type Inner = ManuallyDrop<M::Inner>;
    type Outer = ManuallyDrop<M::Outer>;

    #[inline(always)]
    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        M::ptr_into_inner(outer as *mut _) as *mut _
    }

    #[inline(always)]
    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        M::ptr_into_outer(inner as *mut _) as *mut _
    }
}

impl<T> Wrapper for MaybeUninit<T> {
    type Wrapping = WrapMaybeUninit<T>;
}

unsafe impl<M: SafeMappingToInner> SafeMappingToInner for MaybeUninit<M> {}
unsafe impl<M: SafeMappingToOuter> SafeMappingToOuter for MaybeUninit<M> {}

impl<T: ?Sized> Mappable for *const T {
    type Target = T;
}

impl<T: ?Sized> Mappable for *mut T {
    type Target = T;
}

impl<T: ?Sized> Mappable for NonNull<T> {
    type Target = T;
}

impl<T> Mappable for AtomicPtr<T> {
    type Target = T;
}

impl<T: ?Sized> Mappable for &T {
    type Target = T;
}

impl<T: ?Sized> Mappable for &mut T {
    type Target = T;
}

impl<P> Mappable for Pin<P>
where
    P: Mappable,
{
    type Target = P::Target;
}

impl<P> Mappable for ManuallyDrop<P>
where
    P: Mappable,
{
    type Target = P::Target;
}

impl<P> Mappable for Option<P>
where
    P: Mappable,
{
    type Target = P::Target;
}

impl<P, E> Mappable for Result<P, E>
where
    P: Mappable,
{
    type Target = P::Target;
}

unsafe impl<T: ?Sized, U: ?Sized> MapTo<U> for *const T {
    type Converted = *const U;
    type Variance = Covariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        f(self as *mut _)
    }
}

unsafe impl<T: ?Sized, U: ?Sized> MapTo<U> for *mut T {
    type Converted = *mut U;
    type Variance = Invariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        f(self)
    }
}

unsafe impl<T: ?Sized, U: ?Sized> MapTo<U> for NonNull<T> {
    type Converted = NonNull<U>;
    type Variance = Covariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { NonNull::new_unchecked(f(self.as_ptr())) }
    }
}

unsafe impl<T, U> MapTo<U> for AtomicPtr<T> {
    type Converted = AtomicPtr<U>;
    type Variance = Invariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        AtomicPtr::new(f(self.into_inner()))
    }
}

unsafe impl<'a, T: ?Sized, U: ?Sized> MapTo<U> for &'a T
where
    U: 'a,
{
    type Converted = &'a U;
    type Variance = Covariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { &*(f(self as *const _ as *mut _) as *const _) }
    }
}

unsafe impl<'a, T: ?Sized, U: ?Sized> MapTo<U> for &'a mut T
where
    U: 'a,
{
    type Converted = &'a mut U;
    type Variance = Invariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { &mut *(f(self)) }
    }
}

unsafe impl<P, U> MapTo<U> for Pin<P>
where
    P: MapTo<U> + Deref<Target = <P as Mappable>::Target>,
    P::Converted: Deref<Target = U>,
    U: ?Sized,
{
    type Converted = Pin<P::Converted>;
    type Variance = P::Variance;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { Pin::new_unchecked(Pin::into_inner_unchecked(self).convert(f)) }
    }
}

unsafe impl<P: MapTo<U>, U: ?Sized> MapTo<U> for ManuallyDrop<P> {
    type Converted = ManuallyDrop<P::Converted>;
    type Variance = P::Variance;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { ManuallyDrop::new(ManuallyDrop::into_inner(self).convert(f)) }
    }
}

unsafe impl<P, U> MapTo<U> for Option<P>
where
    P: MapTo<U>,
    U: ?Sized,
{
    type Converted = Option<P::Converted>;
    type Variance = Covariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { self.map(|x| x.convert(f)) }
    }
}

unsafe impl<P, E, U> MapTo<U> for Result<P, E>
where
    P: MapTo<U>,
    U: ?Sized,
{
    type Converted = Result<P::Converted, E>;
    type Variance = Covariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { self.map(|x| x.convert(f)) }
    }
}

unsafe impl<M: TransparentMapping, const LEN: usize> TransparentMapping for [M; LEN]
where
    M::Inner: Sized,
    M::Outer: Sized,
{
    type Inner = [M::Inner; LEN];
    type Outer = [M::Outer; LEN];

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut [M::Inner; LEN]
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut [M::Outer; LEN]
    }
}

unsafe impl<M: SafeMappingToInner, const LEN: usize> SafeMappingToInner for [M; LEN]
where
    M::Inner: Sized,
    M::Outer: Sized,
{
}

unsafe impl<M: SafeMappingToOuter, const LEN: usize> SafeMappingToOuter for [M; LEN]
where
    M::Inner: Sized,
    M::Outer: Sized,
{
}

impl<T> Wrapper for [T; 1] {
    type Wrapping = WrapArray<T>;
}

unsafe impl<M: TransparentMapping> TransparentMapping for [M]
where
    M::Inner: Sized,
    M::Outer: Sized,
{
    type Inner = [M::Inner];
    type Outer = [M::Outer];

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut [M::Inner]
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut [M::Outer]
    }
}

unsafe impl<M: SafeMappingToInner> SafeMappingToInner for [M]
where
    M::Inner: Sized,
    M::Outer: Sized,
{
}

unsafe impl<M: SafeMappingToOuter> SafeMappingToOuter for [M]
where
    M::Inner: Sized,
    M::Outer: Sized,
{
}

impl<T> Wrapper for [T] {
    type Wrapping = WrapSlice<T>;
}

unsafe impl<M: ?Sized + TransparentMapping> TransparentMapping for Cell<M> {
    type Inner = Cell<M::Inner>;
    type Outer = Cell<M::Outer>;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        M::ptr_into_inner(outer as *mut M::Outer) as *mut Self::Inner
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        M::ptr_into_outer(inner as *mut M::Inner) as *mut Self::Outer
    }
}

unsafe impl<M: ?Sized + SafeMapping> SafeMappingToInner for Cell<M> {}
unsafe impl<M: ?Sized + SafeMapping> SafeMappingToOuter for Cell<M> {}

unsafe impl<M: ?Sized + TransparentMapping> TransparentMapping for UnsafeCell<M> {
    type Inner = Cell<M::Inner>;
    type Outer = Cell<M::Outer>;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        M::ptr_into_inner(outer as *mut M::Outer) as *mut Self::Inner
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        M::ptr_into_outer(inner as *mut M::Inner) as *mut Self::Outer
    }
}

unsafe impl<M: ?Sized + SafeMapping> SafeMappingToInner for UnsafeCell<M> {}
unsafe impl<M: ?Sized + SafeMapping> SafeMappingToOuter for UnsafeCell<M> {}

unsafe impl<T> TransparentMapping for Reverse<T> {
    type Inner = T;
    type Outer = Self;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut Self::Inner
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut Self::Outer
    }
}

impl<T> Wrapper for Reverse<T> {
    type Wrapping = Self;
}

unsafe impl TransparentMapping for str {
    type Inner = [u8];
    type Outer = Self;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut _
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut _
    }
}

impl Wrapper for str {
    type Wrapping = Self;
}
