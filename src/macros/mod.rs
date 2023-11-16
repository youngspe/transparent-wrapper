mod parse;
mod proxy_impl;

#[macro_export]
macro_rules! transparent {
    (@impl_mapping
        ( attr = $(#[$attr:meta])* ),
        ( name = $vis:vis $Name:ident ),
        <{
            ( args = $($gargs:tt)* ),
            ( bounded = $($gbounded:tt)* ),
            ( lt = $({ $glt:lifetime: [$($glt_bnd:tt)*] $($glt_rest:tt)* })* ),
            ( ty = $({ $gty:ident: [$($gty_bnd:tt)*], [$(def = $gty_def:tt)?] $($gty_rest:tt)* })* ),
            ( const = $({ $gconst:ident: [$($gconst_bnd:tt)*], [$(def = $gconst_def:tt)?] $($gconst_rest:tt)* })* )
            $($grest:tt)*
        }>,
        { $( where = $($where:tt)* )? },
        ( inner = $inner:ident: $Inner:ty ),
        $fields:tt,
        (mapping = $mapping_vis:vis $Mapping:ident)
    ) => {
        $(#[$attr])*
        #[repr(transparent)]
        $vis struct $Name <
            $($glt: $($glt_bnd)*,)*
            $($gty: $($gty_bnd)* $(= $gty_def)?,)*
            $(const $gconst: $($gconst_bnd)* $(= $gconst_def)?,)*
        > $(where $($where)*)? $fields

        impl <$($gbounded)*>
        ::core::marker::Unpin
        for $Name <$($gargs)*>
        where
            for<'__unpin> $crate::helpers::UnpinGuardWrapper<'__unpin, $Inner>:
                $crate::helpers::UnpinGuard, $($($where)*)? {}

        unsafe impl <$($gbounded)*>
        $crate::custom_cast::TransparentMapping
        for $Mapping <$($gargs)*> $(where $($where)*)?
        {
            type Inner = $Inner;
            type Outer = $Name <$($gargs)*>;
            #[inline(always)]
            fn ptr_into_inner(outer: *mut Self::Outer) -> *mut Self::Inner {
                outer as *mut _ as *mut _
            }
            #[inline(always)]
            fn ptr_into_outer(inner: *mut Self::Inner) -> *mut Self::Outer {
                inner as *mut _ as *mut _
            }
        }

        unsafe impl <$($gbounded)*>
        $crate::custom_cast::SafeMappingToInner
        for $Mapping <$($gargs)*> $(where $($where)*)? {}

        unsafe impl <$($gbounded)*>
        $crate::custom_cast::SafeMappingToOuter
        for $Mapping <$($gargs)*> $(where $($where)*)? {}

        impl <$($gbounded)*>
        $crate::custom_cast::Wrapper
        for $Mapping <$($gargs)*> $(where $($where)*)? {
            type Wrapping = Self;
        }
    };
    (@pre_impl_mapping
        $attr:tt, $name:tt, <$Gen:tt>,
        $where:tt, $inner:tt, $fields:tt, $mapping:tt,
        ($(proxy_impl = $($proxy:ident),* $(,)?)?)
    ) => {
        $crate::transparent! { @impl_mapping
            $attr, $name, <$Gen>, $where, $inner, $fields, $mapping
        }
        $($($crate::_proxy_impl! { (trait = $proxy),
            $attr, $name, <$Gen>, $where, $inner, $fields, $mapping
        })*)?
    };
    (@define_mapping
        $attr:tt, $name:tt,
        <{
            ( args = $($gargs:tt)* ),
            $gbounded:tt,
            ( lt = $({ $glt:lifetime $($glt_rest:tt)* })* ),
            ( ty = $({ $gty:ident $($gty_rest:tt)* })* ),
            ( const = $({ $gconst:ident: [$($gconst_bnd:tt)*] $($gconst_rest:tt)* })* )
            $($grest:tt)*
        }>, $where:tt, $inner:tt, $fields:tt,
        (mapping = $mapping_vis:vis $Mapping:ident), $proxy_impl:tt
    ) => {
        #[non_exhaustive]
        $mapping_vis struct $Mapping <
        $($glt,)* $($gty: ?::core::marker::Sized,)* $(const $gconst: $($gconst_bnd)*,)*
        > (
            $(::core::marker::PhantomData<& $glt ()>,)*
            $(::core::marker::PhantomData<fn() -> ::core::marker::PhantomData<$gty>>,)*
        );

        #[allow(unused)]
        impl <
            $($glt,)* $($gty: ?::core::marker::Sized,)* $(const $gconst: $($gconst_bnd)*,)*
        > $Mapping <$($gargs)*>
        where Self: $crate::custom_cast::SafeMappingToInner + $crate::custom_cast::SafeMappingToOuter {
            fn as_inner<P: $crate::custom_cast::SafeMapIn<Self>>(outer: P) -> P::Converted {
                $crate::custom_cast::as_inner_by::<Self, P>(outer)
            }

            fn as_outer<P: $crate::custom_cast::SafeMapOut<Self>>(inner: P) -> P::Converted {
                $crate::custom_cast::as_outer_by::<Self, P>(inner)
            }

            fn into_inner(
                outer: <Self as $crate::custom_cast::TransparentMapping>::Outer,
            ) -> <Self as $crate::custom_cast::TransparentMapping>::Inner
            where
                <Self as $crate::custom_cast::TransparentMapping>::Inner: Sized,
                <Self as $crate::custom_cast::TransparentMapping>::Outer: Sized,
            {
                $crate::custom_cast::into_inner_by::<Self>(outer)
            }

            fn into_outer(
                inner: <Self as $crate::custom_cast::TransparentMapping>::Inner,
            ) -> <Self as $crate::custom_cast::TransparentMapping>::Outer
            where
                <Self as $crate::custom_cast::TransparentMapping>::Inner: Sized,
                <Self as $crate::custom_cast::TransparentMapping>::Outer: Sized,
            {
                $crate::custom_cast::into_outer_by::<Self>(inner)
            }
        }
    };
    (@define_mapping_if_needed
        $attr:tt, (name = $vis:vis $Name:ident), <$Gen:tt>,
        $where:tt, $inner:tt, $fields:tt,
        (), $proxy_impl:tt
    ) => {
        $crate::transparent! { @pre_impl_mapping
            $attr,
            (name = $vis $Name), <$Gen>,
            $where, $inner, $fields, (mapping = $vis $Name), $proxy_impl
        }
    };
    (@define_mapping_if_needed
        $attr:tt, $name:tt, <$Gen:tt>,
        $where:tt, $inner:tt, $fields:tt,
        $mapping:tt, $proxy_impl:tt
    ) => {
        $crate::transparent! { @define_mapping
            $attr,
            $name, <$Gen>,
            $where, $inner, $fields, $mapping, $proxy_impl
        }

        $crate::transparent! { @pre_impl_mapping
            $attr,
            $name, <$Gen>,
            $where, $inner, $fields, $mapping, $proxy_impl
        }
    };
    (@parse_attrs
        (attr =),
        (parsed_attr = $($parsed_attr:tt)*),
        $name:tt, <$Gen:tt>,
        $where:tt, $inner:tt, $fields:tt,
        $mapping:tt, $proxy_impl:tt
    ) => {
        $crate::transparent! { @define_mapping_if_needed
            (attr = $($parsed_attr)*),
            $name, <$Gen>,
            $where, $inner, $fields, $mapping, $proxy_impl
        }
    };
    (@parse_attrs
        (attr = #[proxy_impl($($proxy1:ident),* $(,)?)] $($attr2:tt)*),
        $parsed_attr:tt,
        $name:tt, <$Gen:tt>,
        $where:tt, $inner:tt, $fields:tt,
        $mapping:tt, ($(proxy_impl = $($proxy2:tt)*)?)
    ) => {
        $crate::transparent! { @parse_attrs
            (attr = $($attr2)*),
            $parsed_attr,
            $name, <$Gen>,
            $where, $inner, $fields,
            $mapping, (proxy_impl = $($($proxy2)*)? $($proxy1,)*)
        }
    };
    (@parse_attrs
        (attr = #[mapping = $mapping_vis:vis $Mapping:ident] $($attr2:tt)*),
        $parsed_attr:tt,
        $name:tt, <$Gen:tt>,
        $where:tt, $inner:tt, $fields:tt,
        (), $proxy_impl:tt
    ) => {
        $crate::transparent! { @parse_attrs
            (attr = $($attr2)*),
            $parsed_attr,
            $name, <$Gen>,
            $where, $inner, $fields,
            (mapping = $mapping_vis $Mapping), $proxy_impl
        }
    };
    (@parse_attrs
        (attr = #[mapping = $($mapping1:tt)*] $($attr2:tt)*),
        $parsed_attr:tt,
        $name:tt, <$Gen:tt>,
        $where:tt, $inner:tt, $fields:tt,
        (mapping = $($mapping2:tt)*), $proxy_impl:tt
    ) => {
        ::core::compile_error! { ::core::concat!(
            "Mapping ", ::core::stringify!($($mapping1)*),
            " conflicts with ", ::core::stringify!($($mapping2)*))
        }

        $crate::transparent! { @parse_attrs
            (attr = $($attr2)*),
            $parsed_attr,
            $name, <$Gen>,
            $where, $inner, $fields,
            (mapping = $($mapping2)*), $proxy_impl
        }
    };
    (@parse_attrs
        (attr = #$attr:tt $($attr2:tt)*),
        ($(parsed_attr = $($parsed_attr:tt)*)?),
        $name:tt, <$Gen:tt>,
        $where:tt, $inner:tt, $fields:tt,
        $mapping:tt, $proxy_impl:tt
    ) => {
        $crate::transparent! { @parse_attrs
            (attr = $($attr2)*),
            (parsed_attr = $($($parsed_attr)* #$attr)?),
            $name, <$Gen>,
            $where, $inner, $fields, $mapping, $proxy_impl
        }
    };
    (@process_fields
        $attr:tt,
        $name:tt, <$Gen:tt>,
        $where:tt, (),
        $fields:tt, {}
    ) => {
        ::core::compile_error! { "1 non-PhantomData field expected." }
    };
    (@process_fields
        $attr:tt,
        $name:tt, <$Gen:tt>,
        $where:tt, $inner:tt,
        $fields:tt, {}
    ) => {
        $crate::transparent! { @parse_attrs
            $attr, (parsed_attr =),
            $name, <$Gen>,
            $where, $inner, $fields,
            (), (proxy_impl =)
        }
    };
    (@process_fields
        $attr:tt,
        $name:tt, <{ $gargs:tt, (bounded = $($bounded:tt)* ) $($grest:tt)* }>,
        {$(where = $($where:tt)*)?}, $inner:tt,
        $fields:tt, {
            $(#[$field_attr:meta])*
            $field_vis:vis $field:ident: $($($Field1:ident)?::)* PhantomData<$Field2:ty>
            $(, $($proc_fields:tt)*)?
        }
    ) => {
        const _: () = {
            fn<$($gbounded)*> _check_phantom_data(
                p: ::core::marker::PhantomData<$Field2>,
            ) $(where $($where)*)? {
                let $field: ::core::marker::PhantomData<$Field2> = p;
                let _ = $field;
            }
        };

        $crate::transparent! { @process_fields
            $attr, $name, <{$gargs, (bounded = $($bounded)* ) $($grest)* }>,
            {$(where = $($where)*)?}, $inner, $fields, { $($($proc_fields)*)? }
        }
    };
    (@process_fields
        $attr:tt,
        $name:tt, <$Gen:tt>,
        $where:tt, (),
        $fields:tt, {
            $(#[$field_attr:meta])*
            $field_vis:vis $field:ident: $Field:ty
            $(, $($proc_fields:tt)*)?
        }
    ) => {
        $crate::transparent! { @process_fields
            $attr, $name, <$Gen>,
            $where, (inner = $field: $Field), $fields, { $($($proc_fields)*)? }
        }
    };
    (@process_fields
        $attr:tt,
        $name:tt, <$Gen:tt>,
        $where:tt, (inner = $($inner:tt)*),
        $fields:tt, {
            $(#[$field_attr:meta])*
            $field:ident: $Field:ty
            $(, $($proc_fields:tt)*)?
        }
    ) => {
        ::core::compile_error! { "Can only have 1 non-PhantomData field" }
        $crate::transparent! { @process_fields
            $attr,
            $name, <$Gen>,
            $where, (inner = $($inner)*), $fields, { $($($proc_fields)*)? }
        }
    };
    (@parsed_where
        $attr:tt,
        $name:tt,
        <$Gen:tt>,
        $where:tt,
        { $($fields:tt)* }
        $($rest:tt)*
    ) => {
        $crate::transparent! { @process_fields
            $attr, $name, <$Gen>,
            $where, (),
            { $($fields)* },
            { $($fields)* }
        }

        $crate::transparent! { $($rest)* }
    };
    (
        $(#[$($attr:tt)*])*
        $vis:vis struct $Name:ident { $($fields:tt)* } $($rest:tt)*
    ) => {
        $crate::transparent! { @parsed_where
            (attr = $(#[$($attr)*])*),
            (name = $vis $Name),
            <{ (args =), (bounded =), (lt =), (ty =), (const =) }>,
            {},
            { $($fields)* }
            $($rest)*
        }
    };
    (@parsed_generics
        $attr:tt, $name:tt, <$Gen:tt>, $($rest:tt)*
    ) => {
        $crate::_parse_where! {
            (callback = $crate::transparent) (pre = @parsed_where
                $attr, $name, <$Gen>,
            )
            $($rest)*
        }
    };
    (@$($other:tt)*) => {
        compile_error! {
            ::core::concat!("Bad intermediate input: @", ::core::stringify!($($other)*))
        }
    };
    ($(#[$($attr:tt)*])* $vis:vis struct $Name:ident $($rest:tt)*) => {
        $crate::_parse_generics! {
            (callback = $crate::transparent) (pre = @parsed_generics
                (attr = $(#[$($attr)*])*),
                (name = $vis $Name),
            ) $($rest)*
        }
    };
    ({$($inner:tt)*}) => { $crate::transparent! { $($inner)* } };
    () => {};
}
