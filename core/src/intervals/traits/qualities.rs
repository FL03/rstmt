/*
    appellation: qualities <module>
    authors: @FL03
*/

/// [`Quality`] is a sealed marker trait used to define compatible intervallic qualities.
pub trait Quality
where
    Self: Send + Sync + AsRef<str> + core::fmt::Debug + core::fmt::Display,
{
    private! {}
    /// returns the name of the current quality
    fn name(&self) -> &str;
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

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", stringify!($name))
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", stringify!($name))
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                stringify!($name)
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

        unsafe impl Send for $name {}

        unsafe impl Sync for $name {}

        impl Quality for $name {
            seal! {}

            fn name(&self) -> &str {
                self.as_ref()
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
