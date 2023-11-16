#![cfg(feature = "alloc")]

use core::mem::ManuallyDrop;

use alloc::{
    boxed::Box,
    rc::{self, Rc},
    sync::{self, Arc},
    vec::Vec,
};

use crate::custom_cast::{MapTo, Mappable, Covariant};

impl<T: ?Sized> Mappable for Box<T> {
    type Target = T;
}

impl<T: ?Sized> Mappable for Rc<T> {
    type Target = T;
}

impl<T: ?Sized> Mappable for rc::Weak<T> {
    type Target = T;
}

impl<T: ?Sized> Mappable for Arc<T> {
    type Target = T;
}

impl<T: ?Sized> Mappable for sync::Weak<T> {
    type Target = T;
}

impl<T> Mappable for Vec<T> {
    type Target = T;
}

unsafe impl<T: ?Sized, U: ?Sized> MapTo<U> for Box<T> {
    type Converted = Box<U>;
    type Variance = Covariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { Box::from_raw(f(Box::into_raw(self))) }
    }
}

unsafe impl<T: ?Sized, U: ?Sized> MapTo<U> for Rc<T> {
    type Converted = Rc<U>;
    type Variance = Covariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { Rc::from_raw(f(Rc::into_raw(self) as *mut _)) }
    }
}

unsafe impl<T: ?Sized, U: ?Sized> MapTo<U> for rc::Weak<T> {
    type Converted = rc::Weak<U>;
    type Variance = Covariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { rc::Weak::from_raw(f(rc::Weak::into_raw(self) as *mut _)) }
    }
}

unsafe impl<T: ?Sized, U: ?Sized> MapTo<U> for Arc<T> {
    type Converted = Arc<U>;
    type Variance = Covariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { Arc::from_raw(f(Arc::into_raw(self) as *mut _)) }
    }
}

unsafe impl<T: ?Sized, U: ?Sized> MapTo<U> for sync::Weak<T> {
    type Converted = sync::Weak<U>;
    type Variance = Covariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        unsafe { sync::Weak::from_raw(f(sync::Weak::into_raw(self) as *mut _)) }
    }
}

unsafe impl<T, U> MapTo<U> for Vec<T> {
    type Converted = Vec<U>;
    type Variance = Covariant;

    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted {
        let mut src = ManuallyDrop::new(self);
        let ptr = src.as_mut_ptr();
        let len = src.len();
        let cap = src.capacity();
        unsafe { Vec::from_raw_parts(f(ptr), len, cap) }
    }
}
