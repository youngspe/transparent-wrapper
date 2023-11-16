#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod custom_cast;
#[doc(hidden)]
pub mod helpers;
mod impls;
mod macros;

pub use custom_cast::{
    as_inner, as_inner_by, as_outer, as_outer_by, into_inner, into_inner_by, into_outer,
    into_outer_by, TransparentMapping,
};
