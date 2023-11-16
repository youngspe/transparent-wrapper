#[doc(hidden)]
#[macro_export]
macro_rules! _parse_generics {
    (@exit (callback = $cb:path) ( pre = $($pre:tt)* ) <{
        (lt = $({ $glt:lifetime : [$($glt_bnd:tt)*] })* ),
        (ty = $({ $gty:ident : [$($gty_bnd:tt)*], [$(def = $gty_def:ty)?] })* ),
        (const = $({ $gconst:ident : [$($gconst_bnd:tt)*], [$(def = $gconst_def:expr)?] })* )
    }> $($rest:tt)* ) => {
        $cb! { $($pre)* <{
            (args = $($glt,)* $($gty,)* $($gconst,)* ),
            (bounded = $($glt: $($glt_bnd)*,)* $($gty: $($gty_bnd)*,)* $(const $gconst: $($gconst_bnd)*,)* ),
            (lt = $({ $glt : [$($glt_bnd)*] })* ),
            (ty = $({ $gty : [$($gty_bnd)*], [$(def = $gty_def)?] })* ),
            (const = $({ $gconst : [$($gconst_bnd)*], [$(def = $gconst_def)?] })* )
        }>, $($rest)* }
    };
    (@parse_generics (callback = $cb:path) ( pre = $($pre:tt)* ) $nest:tt <{}> <> $($rest:tt)*) => {
        $cb! { $($pre)* <{ (args =), (bounded =), (lt =), (ty =), (const =) }>, $($rest)* }
    };
    (@parse_generics $cb:tt $pre:tt $nest:tt <{}> < $($rest:tt)*) => {
        $crate::_parse_generics! { @parse_generics
            $cb $pre $nest <{
                (lt =), (ty =), (const =)
            }> $($rest)*
        }
    };
    (@parse_generics (callback = $cb:path) ( pre = $($pre:tt)* ) $nest:tt <{}> $($rest:tt)* ) => {
        $cb! { $($pre)* <{ (args =), (bounded =), (lt =), (ty =), (const =) }>, $($rest)* }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest =) <$Gen:tt>
        > $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @exit $cb $pre <$Gen> $($rest)* }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest =) <{
            (lt = $($glt:tt)*),
            (ty = $($gty:tt)*)
            $($grest:tt)*
        }>
        $($($lt:lifetime $( : $bound1a:lifetime $( + $bound2a:lifetime )* )? ),+ $(,)?)?
        $($($T:ident $( : $bound1b:tt $( + $bound2b:tt )* )? $(= $ty_def:ty )? ),+ $(,)?)?
        > $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @exit $cb $pre <{
            ( lt = $($glt)* $($( { $lt: [$($bound1a $(+ $bound2a)*)?] } )*)? ),
            ( ty = $($gty)* $($( {
                $T: [$($bound1b $(+ $bound2b)*)?], [$(def = $ty_def)?]
            } )*)? )
            $($grest)*
        }> $($rest)*}
    };
    (@parse_generics
        $cb:tt $pre:tt (nest =) <{
            $glt:tt, $gty:tt, (const = $($gconst:tt)*) $($grest:tt)*
        }>
        $(
            const $N:ident $(: $Bound:ty)? $(= $($def_lit:literal)? $($def_block:block)?)?
         ),* $(,)?> $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @exit
            $cb $pre <{
                $glt, $gty, (const = $($gconst)* $({
                    $N: [$($Bound)?], [$(def = $($def_lit)? $($def_block)?)?]
                })*) $($grest)*
            }> $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest =) <{
            $glt:tt, $gty:tt, (const = $($gconst:tt)*) $($grest:tt)*
        }>
        const $N:ident $(: $Bound:ty)? $(= $($def_lit:literal)? $($def_block:block)?)?, $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @parse_generics
            $cb $pre (nest =) <{
                $glt, $gty, (const = $($gconst)* {
                    $N: [$($Bound)?], [$(def = $($def_lit)? $($def_block)?)?]
                }) $($grest)*
            }> $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest =) <{
            (lt = $($glt:tt)*), (ty = $($gty:tt)*) $($grest:tt)*
        }>
        $( $($lt:lifetime $( : $bound1a:lifetime $( + $bound2a:lifetime )* )? ),+ $(,)? )?
        $( $($T:ident $( : $($bound1b:lifetime)? $($bound2b:path)? )? $(= $ty_def:ty)? ),+ $(,)? )?
        > $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @parse_generics $cb $pre (nest =) <{
            ( lt = $($glt)* $($( { $lt: [$($bound1a $(+ $bound2a)*)?] } )*)? ),
            ( ty = $($gty)* $($( {
                $T: [$($bound1b)? $($bound2b)?], [$(def = $ty_def)?]
            } )*)? )
            $($grest)*
        }> $($rest)*}
    };
    (@parse_generics
        $cb:tt $pre:tt (nest =) <{ (lt = $($glt:tt)*) $($grest:tt)* }>
        $(
            $lt:lifetime $(: $($bound1:lifetime $(+ $bound2:lifetime)* $(+)?)?)?
        ),* $(,)? > $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @exit
            $cb $pre <{
                (lt = $($glt)* $({ $lt: [$($($bound1 $(+ $bound2)*)?)?] })* ) $($grest)*
            }> $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest =) <{ (lt = $($glt:tt)*) $($grest:tt)* }>
        $lt:lifetime $(: $($bound1:lifetime $(+ $bound2:lifetime)* $(+)?)?)?, $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @parse_generics
            $cb $pre (nest =) <{
                (lt = $($glt)* { $lt: [$($($bound1 $(+ $bound2)*)?)?] }) $($grest)*
            }> $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest =) <{ $glt:tt, (ty = $($gty:tt)*) $($grest:tt)* }>
        $T:ident $(= $def:ty)?, $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @parse_generics
            $cb $pre (nest =) <{
                $glt, (ty = $($gty)* { $T: [], [$(def = $def)?] }) $($grest)*
            }> $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest = $($nest:tt)*) <$Gen:tt>
        $T:ident: @{$($bound:tt)*}@ < $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @parse_generics
            $cb $pre (nest = < $($nest)*) <$Gen> $T: @{$($bound)* <}@ $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest = $($nest:tt)*) <$Gen:tt>
        $T:ident: @{$($bound:tt)*}@ << $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @parse_generics
            $cb $pre (nest = < $($nest)*) <$Gen> $T: @{$($bound)* <}@ < $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest = < $($nest:tt)*) <$Gen:tt>
        $T:ident: @{$($bound:tt)*}@ > $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @parse_generics
            $cb $pre (nest = $($nest)*) <$Gen> $T: @{$($bound)* >}@ $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest = < $($nest:tt)*) <$Gen:tt>
        $T:ident: @{$($bound:tt)*}@ >> $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @parse_generics
            $cb $pre (nest = $($nest)*) <$Gen> $T: @{$($bound)* >}@ > $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest =) <{
            $glt:tt, (ty = $($gty:tt)*) $($grest:tt)*
        }>
        $T:ident: @{$($bound:tt)*}@ $(= $def:ty)? , $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @parse_generics
            $cb $pre (nest =) <{
                $glt, (ty = $($gty)* { $T: [$($bound)*], [$(def = $def)?] })
                $($grest)*
            }> $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt (nest =) <{
            $glt:tt, (ty = $($gty:tt)*) $($grest:tt)*
        }>
        $T:ident: @{$($bound:tt)*}@ $(= $def:ty)? > $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @exit
            $cb $pre <{
                $glt, (ty = $($gty)* { $T: [$($bound)*], [$(def = $def)?] })
                $($grest)*
            }> $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt $nest:tt <$Gen:tt>
        $T:ident: @{$($bound:tt)*}@ $next:tt $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @parse_generics
            $cb $pre $nest <$Gen> $T: @{$($bound)* $next}@ $($rest)*
        }
    };
    (@parse_generics
        $cb:tt $pre:tt $nest:tt <$Gen:tt>
        $T:ident: $($rest:tt)*
    ) => {
        $crate::_parse_generics! { @parse_generics
            $cb $pre $nest <$Gen> $T: @{}@ $($rest)*
        }
    };
    ((callback = $cb:path) ( pre = $($pre:tt)* ) $($input:tt)* ) => {
        $crate::_parse_generics! { @parse_generics
            (callback = $cb) (pre = $($pre)*) (nest =) <{}> $($input)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _parse_where {
    (@parse_where $cb:tt $pre:tt () where $($rest2:tt)*) => {
        $crate::_parse_where! { @parse_where
            $cb $pre (where = ) $($rest2)*
        }
    };
    (@parse_where (callback = $cb:path) (pre = $($pre:tt)*) () $($rest2:tt)* ) => {
        $cb! {
            $($pre)* {}, $($rest2)*
        }
    };
    (@parse_where (callback = $cb:path) (pre = $($pre:tt)*) ($($where:tt)*) { $($rest1:tt)* } $($rest2:tt)* ) => {
        $cb! {
            $($pre)*
            { $($where)* },
            { $($rest1)* }
            $($rest2)*
        }
    };
    (@parse_where (callback = $cb:path) (pre = $($pre:tt)*) ($($where:tt)*) $(; $($rest2:tt)*)? ) => {
        $cb! {
            $($pre)*
            { $($where)* },
            $(; $($rest2)*)?
        }
    };
    (@parse_where $cb:tt $pre:tt ($($where:tt)*)
        $lt1:lifetime: $lt2:lifetime $(+ $lt3:lifetime)*
        $(, $(@$comma:tt@)? $($rest2:tt)*)?
    ) => {
        $crate::_parse_where! { @parse_where
            $cb $pre ( $($where)*
                $lt1: $lt2 $(+ $lt3)*
                $(, $($comma)?)?
            ) $($($rest2)*)?
        }
    };
    (@parse_where $cb:tt $pre:tt ($($where:tt)*)
        for $($rest:tt)*
    ) => {
        $crate::_parse_where! { @parse_where
            $cb $pre ($($where)*
                for
            ) $($rest)*
        }
    };
    (@parse_where $cb:tt $pre:tt ($($where:tt)*)
        < $($Arg:tt),*> $($rest:tt)*
    ) => {
        $crate::_parse_where! { @parse_where
            $cb $pre ($($where)*
                <$($Arg),*>
            ) $($rest)*
        }
    };
    (@parse_where $cb:tt $pre:tt ($($where:tt)*)
        $(
            $($lt:lifetime)? $($T:ty)?:
            $($lt2:lifetime)? $($Bound:path)?
        ),* $(,)? $( { $($rest:tt)* } $($rest2:tt)* )?
    ) => {
        $crate::_parse_where! { @parse_where
            $cb $pre ( $($where)* $(
                $($lt)? $($T)? : $($lt2)? $($Bound)? ,
            )* )
            $( {$($rest1)*} $($($rest2)*)? )?
        }
    };
    (@parse_where $cb:tt $pre:tt ($($where:tt)*)
        $T:ty: $($rest:tt)*
    ) => {
        $crate::_parse_where! { @parse_where
            $cb $pre ($($where)*
                $T:
            ) $($rest)*
        }
    };
    (@parse_where $cb:tt $pre:tt ($($where:tt)*)
        $Bound:path
        $(, $(@$comma:tt@)? $($rest2:tt)*)?
    ) => {
        $crate::_parse_where! { @parse_where
            $cb $pre ( $($where)*
                $Bound
                $(, $($comma)?)?
            ) $($($rest2)*)?
        }
    };
    (@parse_where $cb:tt $pre:tt ($($where:tt)*) $rest1:tt $($rest2:tt)*) => {
        $crate::_parse_where! { @parse_where
            $cb $pre ($($where)* $rest1) $($rest2)*
        }
    };
    ((callback = $cb:path) (pre = $($pre:tt)*) $($input:tt)*) => {
        $crate::_parse_where! { @parse_where
            (callback = $cb) (pre = $($pre)*) () $($input)*
        }
    };
}
