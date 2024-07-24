/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! impl_binop_method {
    (@impl $trait:ident.$call:ident($rhs:ty) -> $out:ty) => {
        paste::paste! {
            pub fn $call(&self, rhs: $rhs) -> $out {
                let p = core::ops::$trait::$call(self.value(), rhs.value());
                Self::new(p)
            }
        }
    };
    (suffix: $s:ident; $($trait:ident.$call:ident($rhs:ty) -> $out:ty),* $(,)?) => {
        paste::paste! {
            $(
                impl_binop_method!(@impl $trait.[<$call $s>]($rhs) -> $out);
            )*
        }
    };
}

macro_rules! opper {
    ($($trait:ident)::*$call:ident($a:ident: $lhs:ty, $b:ident: $rhs:ty) -> $out:ty $blk:block) => {
        impl $trait<$rhs> for $lhs {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output $blk
        }

        impl<'a> $trait<&'a $rhs> for $lhs {
            type Output = $out;

            fn $call(self, rhs: &'a $rhs) -> Self::Output $blk
        }

        impl<'a> $trait<&'a $rhs> for &'a $lhs {
            type Output = $out;

            fn $call(self, rhs: &'a $rhs) -> Self::Output $blk
        }

        impl<'a> $trait<$rhs> for &'a $lhs {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output $blk
        }
    };
    (@impl $($trait:ident)::*$call:ident($a:ident: $lhs:ty, $b:ident: $rhs:ty) -> $out:ty {$($rest:tt)*}) => {
        impl $trait<$rhs> for $lhs {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                $($rest)*
            }
        }

        impl<'a> $trait<&'a $rhs> for $lhs {
            type Output = $out;

            fn $call(self, rhs: &'a $rhs) -> Self::Output {
                $($rest)*
            }
        }

        impl<'a> $trait<&'a $rhs> for &'a $lhs {
            type Output = $out;

            fn $call(self, rhs: &'a $rhs) -> Self::Output {
                $($rest)*
            }
        }

        impl<'a> $trait<$rhs> for &'a $lhs {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                $($rest)*
            }
        }
    };
    (@base $trait:ident::$call:ident($(&$lt:lifetime)? $lhs:ty, $(&$rt:lifetime)?$rhs:ty) -> $out:ty {$($rest:tt)*}) => {
        impl $trait<$(&$rt)? $rhs> for $(&$lt)?$lhs {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                $($rest)*
            }
        }
    };
}

macro_rules! wrapper_ops {
    (@impl $name:ident::<$type:ty>::$trait:ident.$call:ident) => {
        impl core::ops::$trait<$name> for $name {
            type Output = $name;

            fn $call(self, rhs: $name) -> Self::Output {
                let p = ::core::ops::$trait::$call(self.0, rhs.0);
                $name(p)
            }
        }

        impl<'a> core::ops::$trait<&'a $name> for $name {
            type Output = $name;

            fn $call(self, rhs: &'a $name) -> Self::Output {
                let p = ::core::ops::$trait::$call(self.0, rhs.0);
                $name(p)
            }
        }

        impl<'a> core::ops::$trait<$name> for &'a $name {
            type Output = $name;

            fn $call(self, rhs: $name) -> Self::Output {
                let p = ::core::ops::$trait::$call(self.0, rhs.0);
                $name(p)
            }
        }

        impl<'a> core::ops::$trait<&'a $name> for &'a $name {
            type Output = $name;

            fn $call(self, rhs: &'a $name) -> Self::Output {
                let p = ::core::ops::$trait::$call(self.0, rhs.0);
                $name(p)
            }
        }

        paste::paste! {
            impl<T> core::ops::$trait<T> for $name
            where
                $type: core::ops::$trait<T, Output = $type>,
            {
                type Output = $name;

                fn $call(self, rhs: T) -> Self::Output {
                    let p = ::core::ops::$trait::$call(self.0, rhs);
                    $name(p)
                }
            }

            impl<'a, T> core::ops::$trait<T> for &'a $name
            where
                $type: core::ops::$trait<T, Output = $type>,
            {
                type Output = $name;

                fn $call(self, rhs: T) -> Self::Output {
                    let p = ::core::ops::$trait::$call(self.0, rhs);
                    $name(p)
                }
            }
        }
    };
    ($name:ident::<$type:ty>: $($trait:ident.$call:ident),* $(,)?) => {
        $(
            wrapper_ops!(@impl $name::<$type>::$trait.$call);
        )*
    };
}

macro_rules! wrapper_unop {
    (@impl $name:ident impls $trait:ident.$call:ident) => {
        impl ::core::ops::$trait for $name {
            type Output = $name;

            fn $call(self) -> Self::Output {
                $name::new(::core::ops::$trait::$call(self.0))
            }
        }

        impl<'a> ::core::ops::$trait for &'a $name {
            type Output = $name;

            fn $call(self) -> Self::Output {
                $name::new(::core::ops::$trait::$call(self.0))
            }
        }
    };

    ($name:ident impls $($trait:ident.$call:ident),* $(,)?) => {
        $(wrapper_unop!(@impl $name impls $trait.$call);)*
    };

}
