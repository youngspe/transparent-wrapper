pub mod mappings;

use core::{marker::PhantomData, mem::ManuallyDrop};

mod sealed {
    pub trait Variance {}
    pub trait Mutability {}
}

pub unsafe trait TransparentMapping {
    type Inner: ?Sized;
    type Outer: ?Sized;

    type MutabilityIn: Mutability;
    type MutabilityOut: Mutability;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner;
    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer;

    fn as_inner<P: SafeMapIn<Self>>(outer: P) -> P::Converted
    where
        Self: SafeMappingToInner,
    {
        unsafe { Self::as_inner_unchecked(outer) }
    }

    unsafe fn as_inner_unchecked<P: MapIn<Self>>(outer: P) -> P::Converted {
        unsafe { outer.convert(Self::ptr_into_inner) }
    }

    fn as_outer<P: SafeMapOut<Self>>(inner: P) -> P::Converted {
        unsafe { Self::as_outer_unchecked(inner) }
    }

    unsafe fn as_outer_unchecked<P: MapOut<Self>>(inner: P) -> P::Converted {
        unsafe { inner.convert(Self::ptr_into_outer) }
    }

    fn into_inner(outer: Self::Outer) -> Self::Inner
    where
        Self: SafeMappingToInner,
        Self::Outer: Sized,
        Self::Inner: Sized,
    {
        unsafe { Self::into_inner_unchecked(outer) }
    }

    unsafe fn into_inner_unchecked(outer: Self::Outer) -> Self::Inner
    where
        Self::Outer: Sized,
        Self::Inner: Sized,
    {
        unsafe { dangerous_transmute(outer) }
    }

    fn into_outer(inner: Self::Inner) -> Self::Outer
    where
        Self: SafeMappingToOuter,
        Self::Outer: Sized,
        Self::Inner: Sized,
    {
        unsafe { Self::into_outer_unchecked(inner) }
    }

    unsafe fn into_outer_unchecked(inner: Self::Inner) -> Self::Outer
    where
        Self::Outer: Sized,
        Self::Inner: Sized,
    {
        unsafe { dangerous_transmute(inner) }
    }
}

pub unsafe trait SafeMappingToInner: TransparentMapping {}
pub unsafe trait SafeMappingToOuter: TransparentMapping {}
pub trait SafeMapping: SafeMappingToInner + SafeMappingToOuter {}

pub trait Mutability: sealed::Mutability {
    type Combine<M: Mutability>: Mutability;
}
pub trait MutabilityFor<P: Mappable>: Mutability {}

#[non_exhaustive]
pub struct Shared {}
#[non_exhaustive]
pub struct Unique {}

impl sealed::Mutability for Shared {}
impl sealed::Mutability for Unique {}
impl Mutability for Shared {
    type Combine<M: Mutability> = M;
}
impl Mutability for Unique {
    type Combine<M: Mutability> = Self;
}

impl<P: Mappable> MutabilityFor<P> for Shared {}
impl<P: Mappable<Mutability = Unique>> MutabilityFor<P> for Unique {}

impl<M: ?Sized + SafeMappingToInner + SafeMappingToOuter> SafeMapping for M {}

/// Like transmute but extra risky because it doesn't have any checks for sizeof.
unsafe fn dangerous_transmute<T, U>(src: T) -> U {
    let mut src = ManuallyDrop::new(src);
    let ptr = &mut src as *mut ManuallyDrop<T> as *mut ManuallyDrop<U>;
    unsafe { ManuallyDrop::take(&mut *ptr) }
}

pub trait Mappable {
    type Target: ?Sized;
    type Mutability: Mutability;
}

pub trait Variance: sealed::Variance {}
pub trait VarianceFor<M: ?Sized + TransparentMapping>: Variance {}

#[non_exhaustive]
pub struct Covariant {}
#[non_exhaustive]
pub struct Invariant {}

impl sealed::Variance for Covariant {}
impl sealed::Variance for Invariant {}
impl Variance for Covariant {}
impl Variance for Invariant {}
impl<M: ?Sized + SafeMappingToInner> VarianceFor<M> for Covariant {}
impl<M: ?Sized + SafeMapping> VarianceFor<M> for Invariant {}

pub unsafe trait MapTo<U: ?Sized>: Mappable {
    type Converted: MapTo<Self::Target, Target = U, Converted = Self>;
    type Variance: Variance;
    unsafe fn convert(self, f: impl Fn(*mut Self::Target) -> *mut U) -> Self::Converted;
}

pub trait MapIn<M: ?Sized + TransparentMapping>: MapTo<M::Inner, Target = M::Outer> {}
pub trait MapOut<M: ?Sized + TransparentMapping>: MapTo<M::Outer, Target = M::Inner> {}

pub trait SafeMapIn<M: ?Sized + TransparentMapping>: MapIn<M> {}
pub trait SafeMapOut<M: ?Sized + TransparentMapping>: MapOut<M> {}

impl<P, M> MapIn<M> for P
where
    P: MapTo<M::Inner, Target = M::Outer>,
    M: ?Sized + TransparentMapping,
    M::MutabilityIn: MutabilityFor<P>,
{
}

impl<P, M> MapOut<M> for P
where
    P: MapTo<M::Outer, Target = M::Inner>,
    M: ?Sized + TransparentMapping,
    M::MutabilityOut: MutabilityFor<P>,
{
}

impl<P, M> SafeMapIn<M> for P
where
    P: MapIn<M>,
    P::Variance: VarianceFor<M>,
    M: ?Sized + TransparentMapping,
{
}

impl<P, M> SafeMapOut<M> for P
where
    P: MapOut<M>,
    P::Variance: VarianceFor<Inverse<M>>,
    M: ?Sized + TransparentMapping,
{
}

pub fn as_inner_by<M, P>(outer: P) -> P::Converted
where
    M: ?Sized + SafeMappingToInner,
    P: SafeMapIn<M>,
{
    M::as_inner(outer)
}

pub fn as_inner<M, P>(outer: P) -> P::Converted
where
    M: ?Sized + SafeMappingToInner<Outer = M>,
    P: SafeMapIn<M>,
{
    as_inner_by::<M, P>(outer)
}

pub fn as_outer_by<M, P>(outer: P) -> P::Converted
where
    M: ?Sized + SafeMappingToOuter,
    P: SafeMapOut<M>,
{
    M::as_outer(outer)
}
pub fn as_outer<M, P>(inner: P) -> P::Converted
where
    M: ?Sized + SafeMappingToOuter<Outer = M>,
    P: SafeMapOut<M>,
{
    as_outer_by::<M, P>(inner)
}

pub fn into_inner_by<M>(outer: M::Outer) -> M::Inner
where
    M: ?Sized + SafeMappingToInner,
    M::Inner: Sized,
    M::Outer: Sized,
{
    M::into_inner(outer)
}

pub fn into_inner<M>(outer: M) -> M::Inner
where
    M: SafeMappingToInner<Outer = M>,
    M::Inner: Sized,
{
    into_inner_by::<M>(outer)
}

pub fn into_outer_by<M>(inner: M::Inner) -> M::Outer
where
    M: ?Sized + SafeMappingToOuter,
    M::Inner: Sized,
    M::Outer: Sized,
{
    M::into_outer(inner)
}

pub fn into_outer<M>(inner: M::Inner) -> M
where
    M: SafeMappingToOuter<Outer = M>,
    M::Inner: Sized,
{
    into_outer_by::<M>(inner)
}

#[non_exhaustive]
pub struct Identity<T: ?Sized>(PhantomData<T>);

unsafe impl<T: ?Sized> TransparentMapping for Identity<T> {
    type Inner = T;
    type Outer = T;
    type MutabilityIn = Shared;
    type MutabilityOut = Shared;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        outer
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        inner
    }
}

unsafe impl<T: ?Sized> SafeMappingToInner for Identity<T> {}
unsafe impl<T: ?Sized> SafeMappingToOuter for Identity<T> {}

pub struct Inverse<M: ?Sized>(PhantomData<fn() -> M>);

unsafe impl<M: ?Sized + TransparentMapping> TransparentMapping for Inverse<M> {
    type Inner = M::Outer;
    type Outer = M::Inner;
    type MutabilityIn = M::MutabilityOut;
    type MutabilityOut = M::MutabilityIn;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        M::ptr_into_outer(outer)
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        M::ptr_into_inner(inner)
    }
}

unsafe impl<M: ?Sized + SafeMappingToInner> SafeMappingToOuter for Inverse<M> {}
unsafe impl<M: ?Sized + SafeMappingToOuter> SafeMappingToInner for Inverse<M> {}

impl<M: ?Sized + Wrapper> Wrapper for Inverse<M> {
    type Wrapping = Inverse<M::Wrapping>;
}

pub struct Compose<Outer: ?Sized, Inner: ?Sized>(
    PhantomData<fn() -> Outer>,
    PhantomData<fn() -> Inner>,
);

unsafe impl<Outer, Inner> TransparentMapping for Compose<Outer, Inner>
where
    Outer: ?Sized + TransparentMapping,
    Inner: ?Sized + TransparentMapping<Outer = Outer::Inner>,
{
    type Inner = Inner::Inner;
    type Outer = Outer::Outer;
    type MutabilityIn = <Outer::MutabilityIn as Mutability>::Combine<Inner::MutabilityIn>;
    type MutabilityOut = <Inner::MutabilityOut as Mutability>::Combine<Outer::MutabilityOut>;

    fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
        Inner::ptr_into_inner(Outer::ptr_into_inner(outer))
    }

    fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
        Outer::ptr_into_outer(Inner::ptr_into_outer(inner))
    }
}

unsafe impl<Outer, Inner> SafeMappingToInner for Compose<Outer, Inner>
where
    Outer: ?Sized + SafeMappingToInner,
    Inner: ?Sized + SafeMappingToInner<Outer = Outer::Inner>,
{
}

unsafe impl<Outer, Inner> SafeMappingToOuter for Compose<Outer, Inner>
where
    Outer: ?Sized + SafeMappingToOuter,
    Inner: ?Sized + SafeMappingToOuter<Outer = Outer::Inner>,
{
}

impl<Outer, Inner> Wrapper for Compose<Outer, Inner>
where
    Outer: ?Sized + Wrapper,
    Inner: ?Sized + Wrapper,
    Compose<Outer::Wrapping, Inner::Wrapping>: TransparentMapping,
{
    type Wrapping = Compose<Outer::Wrapping, Inner::Wrapping>;
}

pub trait Wrapper {
    type Wrapping: ?Sized + TransparentMapping;

    fn unwrapped<P: SafeMapIn<Self::Wrapping>>(outer: P) -> P::Converted
    where
        Self::Wrapping: SafeMappingToInner,
    {
        <Self::Wrapping as TransparentMapping>::as_inner(outer)
    }

    unsafe fn unwrapped_unchecked<P: MapIn<Self::Wrapping>>(outer: P) -> P::Converted {
        unsafe { <Self::Wrapping as TransparentMapping>::as_inner_unchecked(outer) }
    }

    fn wrapped<P: SafeMapOut<Self::Wrapping>>(inner: P) -> P::Converted
    where
        Self::Wrapping: SafeMappingToOuter,
    {
        <Self::Wrapping as TransparentMapping>::as_outer(inner)
    }

    unsafe fn wrapped_unchecked<P: MapOut<Self::Wrapping>>(inner: P) -> P::Converted {
        unsafe { <Self::Wrapping as TransparentMapping>::as_outer_unchecked(inner) }
    }
}

pub type Wrapping<M> = <M as Wrapper>::Wrapping;

pub fn unwrapped_by<M, P>(outer: P) -> P::Converted
where
    M: ?Sized + Wrapper,
    M::Wrapping: SafeMappingToInner,
    P: SafeMapIn<M::Wrapping>,
{
    M::unwrapped(outer)
}

pub fn unwrapped<M, P>(outer: P) -> P::Converted
where
    M: ?Sized + Wrapper,
    M::Wrapping: SafeMappingToInner<Outer = M>,
    P: SafeMapIn<M::Wrapping>,
{
    unwrapped_by::<M, P>(outer)
}

pub fn wrapped_by<M, P>(inner: P) -> P::Converted
where
    M: ?Sized + Wrapper,
    M::Wrapping: SafeMappingToOuter,
    P: SafeMapOut<M::Wrapping>,
{
    M::wrapped(inner)
}

pub fn wrapped<M, P>(inner: P) -> P::Converted
where
    M: ?Sized + Wrapper,
    M::Wrapping: SafeMappingToOuter<Outer = M>,
    P: SafeMapOut<M::Wrapping>,
{
    wrapped_by::<M, P>(inner)
}
