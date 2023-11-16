use core::{
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
};

use crate::TransparentMapping;

use super::{SafeMappingToInner, SafeMappingToOuter};

pub struct WrapManuallyDrop<T: ?Sized>(PhantomData<ManuallyDrop<T>>);

unsafe impl<T: ?Sized> TransparentMapping for WrapManuallyDrop<T> {
    type Inner = T;
    type Outer = ManuallyDrop<T>;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut _
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut _
    }
}

unsafe impl<T: ?Sized> SafeMappingToInner for WrapManuallyDrop<T> {}
unsafe impl<T: ?Sized> SafeMappingToOuter for WrapManuallyDrop<T> {}

pub struct WrapMaybeUninit<T>(PhantomData<MaybeUninit<T>>);

unsafe impl<T> TransparentMapping for WrapMaybeUninit<T> {
    type Inner = T;
    type Outer = MaybeUninit<T>;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut _
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut _
    }
}

unsafe impl<T> SafeMappingToOuter for WrapMaybeUninit<T> {}

pub struct WrapArray<T>(PhantomData<[T; 1]>);

unsafe impl<T> TransparentMapping for WrapArray<T> {
    type Inner = T;
    type Outer = [T; 1];

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut _
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut _
    }
}

unsafe impl<T> SafeMappingToInner for WrapArray<T> {}
unsafe impl<T> SafeMappingToOuter for WrapArray<T> {}

pub struct WrapSlice<T>(PhantomData<[T]>);

unsafe impl<T> TransparentMapping for WrapSlice<T> {
    type Inner = T;
    type Outer = [T];

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut _
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut [_; 1]
    }
}

unsafe impl<T> SafeMappingToOuter for WrapSlice<T> {}
