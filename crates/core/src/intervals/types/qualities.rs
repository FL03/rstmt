/*
    appellation: qualities <module>
    authors: @FL03
*/

/// [`Quality`] is a sealed marker trait used to define compatible intervallic qualities.
pub trait RawQuality
where
    Self: Send
        + Sync
        + AsRef<str>
        + core::borrow::Borrow<str>
        + core::fmt::Debug
        + core::fmt::Display,
{
    private! {}

    /// returns the name of the current quality
    fn name(&self) -> &str;
}

pub trait Quality: RawQuality
where
    Self: Clone + Copy + Default,
{
    /// initialize a new instance of the interval quality.
    fn new() -> Self;
}

/*
 ************* Implementations *************
*/
#[allow(unused_macros)]
macro_rules! impl_raw_quality {
    (@def $(#[$meta:meta])* $vis:vis struct $name:ident $(;)?) => {
        $(#[$meta])*
        #[derive(Default)]
        $vis struct $name;
    };
    (@impl $(#[$meta:meta])* $vis:vis $kind:ident $name:ident) => {
        impl_raw_quality! { @def
            $(#[$meta])?
            #[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            #[repr(transparent)]
            $vis $kind $name;
        }

        impl $name {
            /// initialize a new instance of the quality
            pub const fn new() -> Self {
                Self
            }
            /// returns the name of the interval quality
            pub const fn name(&self) -> &str {
                stringify!($name)
            }
            /// returns true if the two qualities are of the same type
            pub fn of<T: 'static>() -> bool {
                core::any::TypeId::of::<Self>() == core::any::TypeId::of::<T>()
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.name())
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.name())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.name()
            }
        }

        impl ::core::borrow::Borrow<str> for $name {
            fn borrow(&self) -> &str {
                self.name()
            }
        }

        impl ::core::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                self.name()
            }
        }

        impl core::str::FromStr for $name {
            type Err = crate::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s.to_lowercase() == stringify!($name).to_lowercase() {
                    return Ok(Self);
                }
                Err(crate::Error::FromStrParseError)
            }
        }

        impl<Q> PartialEq<Q> for $name
        where
            str: PartialEq<Q>,
        {
            fn eq(&self, other: &Q) -> bool {
                self.name() == other
            }
        }

        impl<'a> PartialEq<$name> for &'a str {
            fn eq(&self, other: &$name) -> bool {
                *self == other.name()
            }
        }

        unsafe impl Send for $name {}

        unsafe impl Sync for $name {}

        impl $crate::intervals::RawQuality for $name {
            seal! {}

            fn name(&self) -> &str {
                self.name()
            }
        }

        impl $crate::intervals::Quality for $name {
            fn new() -> Self {
                Self
            }
        }
    };
    ($($(#[$meta:meta])* $vis:vis $kind:ident $name:ident),* $(,)?) => {
        $(impl_raw_quality! { @impl $(#[$meta])* $vis $kind $name })*
    };
}

impl_raw_quality! {
    pub struct Major,
    pub struct Minor,
    pub struct Augmented,
    pub struct Diminished,
    pub struct Perfect,
}
