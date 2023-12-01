use core::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
};

use crate::TransparentMapping;

use super::{SafeMappingToInner, SafeMappingToOuter, Shared, Unique};

pub struct WrapManuallyDrop<T: ?Sized>(PhantomData<ManuallyDrop<T>>);

unsafe impl<T: ?Sized> TransparentMapping for WrapManuallyDrop<T> {
    type Inner = T;
    type Outer = ManuallyDrop<T>;
    type MutabilityIn = Shared;
    type MutabilityOut = Shared;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut _
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut _
    }
}

unsafe impl<T: ?Sized> SafeMappingToInner for WrapManuallyDrop<T> {}
unsafe impl<T: ?Sized> SafeMappingToOuter for WrapManuallyDrop<T> {}

pub struct WrapCell<T: ?Sized>(PhantomData<Cell<T>>);

unsafe impl<T: ?Sized> TransparentMapping for WrapCell<T> {
    type Inner = T;
    type Outer = Cell<T>;
    type MutabilityIn = Unique;
    type MutabilityOut = Unique;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut _
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut _
    }
}

unsafe impl<T: ?Sized> SafeMappingToInner for WrapCell<T> {}
unsafe impl<T: ?Sized> SafeMappingToOuter for WrapCell<T> {}

pub struct WrapUnsafeCell<T: ?Sized>(PhantomData<UnsafeCell<T>>);

unsafe impl<T: ?Sized> TransparentMapping for WrapUnsafeCell<T> {
    type Inner = T;
    type Outer = UnsafeCell<T>;
    type MutabilityIn = Unique;
    type MutabilityOut = Unique;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut _
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut _
    }
}

unsafe impl<T: ?Sized> SafeMappingToInner for WrapUnsafeCell<T> {}
unsafe impl<T: ?Sized> SafeMappingToOuter for WrapUnsafeCell<T> {}

pub struct WrapMaybeUninit<T>(PhantomData<MaybeUninit<T>>);

unsafe impl<T> TransparentMapping for WrapMaybeUninit<T> {
    type Inner = T;
    type Outer = MaybeUninit<T>;
    type MutabilityIn = Shared;
    type MutabilityOut = Shared;

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
    type MutabilityIn = Shared;
    type MutabilityOut = Shared;

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
    type MutabilityIn = Shared;
    type MutabilityOut = Shared;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer as *mut _
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner as *mut [_; 1]
    }
}

unsafe impl<T> SafeMappingToOuter for WrapSlice<T> {}
