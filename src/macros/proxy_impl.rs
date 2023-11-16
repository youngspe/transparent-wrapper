#[doc(hidden)]
#[macro_export]
macro_rules! _debug_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident) $($_rest:tt)*
    ) => {
        impl
        <$($gbounded)*>
        ::core::fmt::Debug
        for $Name <$($gargs)*> where $Inner: ::core::fmt::Debug, $($($where)*)? {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f
                    .debug_tuple(::core::stringify!($Name))
                    .field(&&self.$inner)
                    .finish()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _display_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident) $($_rest:tt)*
    ) => {
        impl
        <$($gbounded)*>
        ::core::fmt::Display
        for $Name <$($gargs)*> where $Inner: ::core::fmt::Display, $($($where)*)? {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                <$Inner as ::core::fmt::Display>::fmt(&self.$inner, f)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _default_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident) $($_rest:tt)*
    ) => {
        impl
        <$($gbounded)*>
        ::core::default::Default
        for $Name <$($gargs)*> where Self: Sized, $Inner: ::core::default::Default, $($($where)*)? {
            fn default() -> Self {
                <$Mapping<$($gargs)*> as $crate::custom_cast::TransparentMapping>::into_outer(
                    <$Inner as ::core::default::Default>::default()
                )
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _clone_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident) $($_rest:tt)*
    ) => {
        impl
        <$($gbounded)*>
        ::core::clone::Clone
        for $Name <$($gargs)*> where Self: Sized, $Inner: ::core::clone::Clone, $($($where)*)?
        {
            fn clone(&self) -> Self {
                <$Mapping<$($gargs)*> as $crate::custom_cast::TransparentMapping>::into_outer(
                    <$Inner as ::core::clone::Clone>::clone(&self.$inner)
                )
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _copy_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident) $($_rest:tt)*
    ) => {
        impl
        <$($gbounded)*>
        ::core::marker::Copy
        for $Name <$($gargs)*> where Self: Sized, $Inner: ::core::marker::Copy, $($($where)*)? {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _partial_eq_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident) $($_rest:tt)*
    ) => {
        impl
        <$($gbounded)*>
        ::core::cmp::PartialEq
        for $Name <$($gargs)*> where $Inner: ::core::cmp::PartialEq, $($($where)*)? {
            fn eq(&self, rhs: &Self) -> bool {
                self.$inner == rhs.$inner
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _eq_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident) $($_rest:tt)*
    ) => {
        impl
        <$($gbounded)*>
        ::core::cmp::Eq
        for $Name <$($gargs)*> where $Inner: ::core::cmp::Eq, $($($where)*)? {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _partial_ord_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident) $($_rest:tt)*
    ) => {
        impl
        <$($gbounded)*>
        ::core::cmp::PartialOrd
        for $Name <$($gargs)*> where $Inner: ::core::cmp::PartialOrd, $($($where)*)? {
            fn partial_cmp(&self, rhs: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::cmp::PartialOrd::partial_cmp(&self.$inner, &rhs.$inner)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _ord_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident) $($_rest:tt)*
    ) => {
        impl
        <$($gbounded)*>
        ::core::cmp::Ord
        for $Name <$($gargs)*> where $Inner: ::core::cmp::Ord, $($($where)*)? {
            fn cmp(&self, rhs: &Self) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.$inner, &rhs.$inner)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _hash_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident) $($_rest:tt)*
    ) => {
        impl
        <$($gbounded)*>
        ::core::hash::Hash
        for $Name <$($gargs)*> where $Inner: ::core::hash::Hash, $($($where)*)? {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                <$Inner as ::core::hash::Hash>::hash(&self.$inner, state)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _from_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident) $($_rest:tt)*
    ) => {
        impl
        <$($gbounded)*>
        ::core::convert::From<$Inner>
        for $Name <$($gargs)*> where Self: Sized, $Inner: Sized, $($($where)*)?
        {
            fn from(src: $Inner) -> Self {
                <$Mapping<$($gargs)*> as $crate::custom_cast::TransparentMapping>::into_outer(
                    src
                )
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _into_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident)
    ) => {
        impl
        <$($gbounded)*>
        ::core::convert::From<$Name <$($gargs)*>>
        for $Inner where $Name <$($gargs)*>: Sized, $Inner: Sized, $($($where)*)?
        {
            fn from(src: $Name <$($gargs)*>) -> Self {
                <$Mapping<$($gargs)*> as $crate::custom_cast::TransparentMapping>::into_inner(
                    src
                )
            }
        }
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! _deref_inner_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident)
    ) => {
        impl
        <$($gbounded)*>
        ::core::ops::Deref
        for $Name <$($gargs)*> $(where $($where)*)?
        {
            type Target = $Inner;
            fn deref(&self) -> &$Inner {
                return &self.$inner
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _deref_mut_inner_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident)
    ) => {
        impl
        <$($gbounded)*>
        ::core::ops::DerefMut
        for $Name <$($gargs)*> $(where $($where)*)?
        {
            fn deref_mut(&mut self) -> &mut $Inner {
                return &mut self.$inner
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _index_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident)
    ) => {
        impl
        <__Index, $($gbounded)*>
        ::core::ops::Index<__Index>
        for $Name <$($gargs)*> where $Inner: ::core::ops::Index<__Index> $($($where)*)?
        {
            type Output = <$Inner as ::core::ops::Index<__Index>>::Output;
            fn index(&self, idx: __Index) -> &<$Inner as ::core::ops::Index<__Index>>::Output {
                return <$Inner as ::core::ops::Index<__Index>>::index(&self.$inner, idx)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _index_mut_impl {
    (
        $attr:tt,
        (name = $vis:vis $Name:ident),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        (inner = $inner:ident: $Inner:ty),
        $fields:tt, (mapping = $mapping_vis:vis $Mapping:ident)
    ) => {
        impl
        <__Index, $($gbounded)*>
        ::core::ops::IndexMut<__Index>
        for $Name <$($gargs)*> where $Inner: ::core::ops::IndexMut<__Index> $($($where)*)?
        {
            fn index_mut(&mut self, idx: __Index) -> &mut <$Inner as ::core::ops::Index<__Index>>::Output {
                return <$Inner as ::core::ops::IndexMut<__Index>>::index_mut(&mut self.$inner, idx)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _proxy_impl {
    ((trait = Debug), $($x:tt)*) => { $crate::_debug_impl! { $($x)* } };
    ((trait = Display), $($x:tt)*) => { $crate::_display_impl! { $($x)* } };
    ((trait = Default), $($x:tt)*) => { $crate::_default_impl! { $($x)* } };
    ((trait = Clone), $($x:tt)*) => { $crate::_clone_impl! { $($x)* } };
    ((trait = Copy), $($x:tt)*) => { $crate::_copy_impl! { $($x)* } };
    ((trait = PartialEq), $($x:tt)*) => { $crate::_partial_eq_impl! { $($x)* } };
    ((trait = Eq), $($x:tt)*) => { $crate::_eq_impl! { $($x)* } };
    ((trait = PartialOrd), $($x:tt)*) => { $crate::_partial_ord_impl! { $($x)* } };
    ((trait = Ord), $($x:tt)*) => { $crate::_ord_impl! { $($x)* } };
    ((trait = Hash), $($x:tt)*) => { $crate::_hash_impl! { $($x)* } };
    ((trait = From), $($x:tt)*) => { $crate::_from_impl! { $($x)* } };
    ((trait = Into), $($x:tt)*) => { $crate::_into_impl! { $($x)* } };
    ((trait = DerefInner), $($x:tt)*) => { $crate::_deref_inner_impl! { $($x)* } };
    ((trait = DerefMutInner), $($x:tt)*) => { $crate::_deref_mut_inner_impl! { $($x)* } };
    ((trait = Index), $($x:tt)*) => { $crate::_index_impl! { $($x)* } };
    ((trait = IndexMut), $($x:tt)*) => { $crate::_index_mut_impl! { $($x)* } };
    ((trait = $impl_macro:path), $($x:tt)*) => { $impl_macro! { $($x)* } };
}
