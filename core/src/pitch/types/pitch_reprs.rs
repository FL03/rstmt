/*
    Appellation: pitch_kinds <module>
    Created At: 2025.12.20:10:03:26
    Contrib: @FL03
*/

macro_rules! pitch_repr {
    [$vis:vis $i:ident {$( $(#[$meta:meta])* $name:ident($repr:literal, $sym:literal): $Tag:ty = $idx:literal ),* $(,)?}] => {
        $(pitch_repr! {@impl $(#[$meta])* $vis $i $name($repr, $sym): $Tag = $idx })*
    };
    {@def $(#[$meta:meta])* $vis:vis struct $name:ident = $idx:literal $(;)?} => {
        $(#[$meta])*
        $vis struct $name<const N: isize = $idx>;
    };
    (@impl $(#[$meta:meta])* $vis:vis struct $name:ident($r:literal, $sym:literal): $Tag:ty = $idx:literal) => {
        pitch_repr! { @def
            $(#[$meta])*
            #[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = "UPPERCASE"))]
            #[repr(transparent)]
            pub struct $name = $idx;
        }

        impl<const N: isize> $name<N> {
            pub const C_MAJOR_ID: usize = $idx;

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
            pub const fn name(&self) -> &str {
                $r
            }
            /// returns the symbol associated with the pitch class
            pub const fn symbol(&self) -> char {
                $sym
            }
        }


        impl $name<$idx> {
            #[allow(clippy::should_implement_trait)]
            /// a constructor for the pitch class that uses the default index for the target
            pub const fn default() -> Self {
                Self::new()
            }
        }

        impl<const N: isize> $crate::pitch::RawPitchClass for $name<N> {
            type Tag = $Tag;

            seal! {}

            fn new() -> Self {
                Self::new()
            }

            fn name(&self) -> &str {
                self.name()
            }

            fn index(&self) -> isize {
                N
            }
        }

        impl<const N: isize> $crate::pitch::PitchClassRepr for $name<N> {
            const IDX: isize = N;
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
                if s.eq_ignore_ascii_case($r) || s.eq_ignore_ascii_case(&$sym.to_string()) {
                    Ok(Self::new())
                } else {
                    Err(anyhow::anyhow!("Invalid pitch class string: {}", s).into())
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

        impl<const N: isize> AsRef<isize> for $name<N> {
            fn as_ref(&self) -> &isize {
                self.get()
            }
        }

        impl<const N: isize> AsRef<str> for $name<N> {
            fn as_ref(&self) -> &str {
                self.name()
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
                f.write_str(self.name())
            }
        }

        impl<const N: isize> ::core::fmt::Display for $name<N> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.name())
            }
        }
    };
}

pitch_repr! {
    pub struct {
        CNote("C", 'C'): crate::Natural = 0,
        CSharpNote("C#", 'C'): crate::Sharp = 1,
        DFlatNote("D♭", 'D'): crate::Flat = 1,
        DNote("D", 'D'): crate::Natural = 2,
        DSharpNote("D#", 'D'): crate::Sharp = 3,
        EFlatNote("E♭", 'E'): crate::Flat = 3,
        ENote("E", 'E'): crate::Natural = 4,
        FNote("F", 'F'): crate::Natural = 5,
        FSharpNote("F", 'F'): crate::Sharp = 6,
        GFlatNote("G", 'G'): crate::Flat = 6,
        GNote("G", 'G'): crate::Natural = 7,
        GSharpNote("G#", 'G'): crate::Sharp = 8,
        AFlatNote("A♭", 'A'): crate::Flat = 8,
        ANote("A", 'A'): crate::Natural = 9,
        ASharpNote("A#", 'A'): crate::Sharp = 10,
        BFlatNote("B♭", 'B'): crate::Flat = 10,
        BNote("B", 'B'): crate::Natural = 11,
    }
}
