/*
    Appellation: pitch_kinds <module>
    Created At: 2025.12.20:10:03:26
    Contrib: @FL03
*/

macro_rules! pitch_repr {
    [$vis:vis $i:ident {$( $(#[$meta:meta])* $name:ident($c:literal): $Tag:ty),* $(,)?}] => {
        $(pitch_repr! {@impl $(#[$meta])* $vis $i $name($c): $Tag })*
    };
    {@def $(#[$meta:meta])* $vis:vis struct $name:ident($c:literal): $Tag:ty} => {
        $(#[$meta])*
        $vis struct $name<const N: isize = $c>;
    };
    (@impl $(#[$meta:meta])* $vis:vis struct $name:ident($c:literal): $Tag:ty) => {
        pitch_repr! { @def
            $(#[$meta])*
            #[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = "UPPERCASE"))]
            #[repr(transparent)]
            pub struct $name($c): $Tag
        }

        impl<const N: isize> $name<N> {
            pub const C_MAJOR_ID: usize = $c;

            pub const fn new() -> Self {
                $name::<N>
            }
            #[inline]
            /// returns true if the pitch class corresponds to the given value
            pub fn is(value: isize) -> bool {
                rstmt_traits::PitchMod::pmod(value) == N
            }
            /// returns a reference to the assigned index
            pub const fn get(&self) -> &isize {
                &N
            }
            /// returns the assigned index value
            pub const fn value(self) -> isize {
                N
            }
            /// returns the name of the pitch class
            pub fn name(&self) -> &str {
                stringify!($name)
            }
        }

        impl $name<$c> {
            #[allow(clippy::should_implement_trait)]
            /// a constructor for the pitch class that uses the default index for the target
            pub const fn default() -> Self {
                Self::new()
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

        impl<const N: isize> $crate::pitch::RawPitchClass for $name<N> {
            type Tag = $Tag;

            seal! {}

            fn name(&self) -> &str {
                stringify!($name)
            }

            fn index(&self) -> isize {
                N
            }
        }

        impl<const N: isize> $crate::pitch::PitchClassRepr for $name<N> {
            const IDX: isize = N;

            seal! {}

            fn new() -> Self {
                Self::new()
            }
        }

        impl<const N: isize> PartialEq<str> for $name<N> {
            fn eq(&self, other: &str) -> bool {
                self.name().to_lowercase() == other.to_lowercase()
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

        impl<const N: isize> core::str::FromStr for $name<N> {
            type Err = crate::error::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s.eq_ignore_ascii_case(stringify!($name)) {
                    Ok(Self::new())
                } else {
                    Err(crate::error::Error::FromStrParseError)
                }
            }
        }

        impl<const N: isize> TryFrom<isize> for $name<N> {
            type Error = crate::error::Error;

            fn try_from(value: isize) -> Result<Self, Self::Error> {
                if Self::is(value) {
                    Ok(Self::new())
                } else {
                    Err(crate::error::Error::InvalidPitchClass(value))
                }
            }
        }
    };
}

pitch_repr! {
    pub struct {
        CNote(0): crate::Natural,
        CSharpNote(1): crate::Sharp,
        DFlatNote(1): crate::Flat,
        DNote(2): crate::Natural,
        DSharpNote(3): crate::Sharp,
        EFlatNote(3): crate::Flat,
        ENote(4): crate::Natural,
        FNote(5): crate::Natural,
        FSharpNote(6): crate::Sharp,
        GFlatNote(6): crate::Flat,
        GNote(7): crate::Natural,
        GSharpNote(8): crate::Sharp,
        AFlatNote(8): crate::Flat,
        ANote(9): crate::Natural,
        ASharpNote(10): crate::Sharp,
        BFlatNote(10): crate::Flat,
        BNote(11): crate::Natural,
    }
}
