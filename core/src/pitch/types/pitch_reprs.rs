/*
    Appellation: pitch_kinds <module>
    Created At: 2025.12.20:10:03:26
    Contrib: @FL03
*/

macro_rules! pitch_class {
    {$($(#[$meta:meta])* $vis:vis $i:ident $name:ident<$tag:ty>::($c:literal));* $(;)?} => {
        $(pitch_class! {@impl $(#[$meta])* $vis $i $name<$tag>::($c) })*
    };
    {@def $(#[$meta:meta])* $vis:vis struct $name:ident<$tag:ty>::($c:literal)} => {
        $(#[$meta])*
        $vis struct $name<const N: isize = $c>;
    };
    (@impl $(#[$meta:meta])* $vis:vis struct $name:ident<$tag:ty>::($c:literal)) => {

        pitch_class! { @def
            #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = "UPPERCASE"))]
            #[repr(transparent)]
            pub struct $name<$tag>::($c)
        }

        impl<const N: isize> $name<N> {
            pub const C_MAJOR_ID: usize = $c;
            /// returns a new instance of the pitch class
            pub const fn new() -> Self {
                $name::<N>
            }
            /// returns a reference to the assigned index
            pub const fn get(&self) -> &isize {
                &N
            }
            /// returns the assigned index value
            pub const fn value(&self) -> isize {
                N
            }
        }

        impl $name<$c> {
            /// returns a new instance of the pitch class
            pub const fn new_c_major_scale() -> Self {
                $name::<$c>
            }
        }

        impl<const N: isize> AsRef<isize> for $name<N> {
            fn as_ref(&self) -> &isize {
                self.get()
            }
        }

        impl<const N: isize> AsRef<str> for $name<N> {
            fn as_ref(&self) -> &str {
                stringify!($name)
            }
        }

        impl<const N: isize> core::borrow::Borrow<isize> for $name<N> {
            fn borrow(&self) -> &isize {
                self.get()
            }
        }

        impl<const N: isize> core::ops::Deref for $name<N> {
            type Target = isize;

            fn deref(&self) -> &Self::Target {
                self.get()
            }
        }

        impl<const N: isize> Default for $name<N> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<const N: isize> ::core::fmt::Debug for $name<N> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_ref())
            }
        }

        impl<const N: isize> ::core::fmt::Display for $name<N> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(stringify!($name))
            }
        }

        impl<const N: isize> $crate::pitch::PitchClassRepr for $name<N> {
            const IDX: isize = N;
            type Tag = $tag;

            seal! {}

            fn new() -> Self {
                Self::new()
            }
        }

        impl<const N: isize> PartialEq<isize> for $name<N> {
            fn eq(&self, other: &isize) -> bool {
                self.get() == other
            }
        }

        impl<const N: isize> PartialEq<$name<N>> for isize {
            fn eq(&self, other: &$name<N>) -> bool {
                self == other.get()
            }
        }

        impl<const N: isize> PartialOrd<isize> for $name<N> {
            fn partial_cmp(&self, other: &isize) -> Option<core::cmp::Ordering> {
                Some(self.get().cmp(other))
            }
        }

        impl<const N: isize> PartialOrd<$name<N>> for isize {
            fn partial_cmp(&self, other: &$name<N>) -> Option<core::cmp::Ordering> {
                Some(self.cmp(&other.get()))
            }
        }
    };
}

pitch_class! {
    pub struct CNote<crate::Natural>::(0);
    pub struct DNote<crate::Natural>::(2);
    pub struct ENote<crate::Natural>::(4);
    pub struct FNote<crate::Natural>::(5);
    pub struct GNote<crate::Natural>::(7);
    pub struct ANote<crate::Natural>::(9);
    pub struct BNote<crate::Natural>::(11);
    pub struct CSharpNote<crate::Sharp>::(1);
    pub struct DSharpNote<crate::Sharp>::(3);
    pub struct FSharpNote<crate::Sharp>::(6);
    pub struct GSharpNote<crate::Sharp>::(8);
    pub struct ASharpNote<crate::Sharp>::(10);
    pub struct DFlatNote<crate::Flat>::(1);
    pub struct EFlatNote<crate::Flat>::(3);
    pub struct GFlatNote<crate::Flat>::(6);
    pub struct AFlatNote<crate::Flat>::(8);
    pub struct BFlatNote<crate::Flat>::(10);
}
