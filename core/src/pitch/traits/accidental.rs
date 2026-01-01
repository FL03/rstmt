/*
    Appellation: pitch_type <module>
    Created At: 2025.12.23:14:35:22
    Contrib: @FL03
*/

/// [`Accidental`] is a sealed marker trait used to designate various _kinds_ of musical notes,
/// i.e., sharp, flat, natural, etc.
pub trait RawAccidental:
    'static + AsRef<str> + Send + Sync + core::fmt::Debug + core::fmt::Display
{
    private! {}

    fn name(&self) -> &str;

    fn symbol(&self) -> char;
}

pub trait Accidental: RawAccidental
where
    Self: Default + core::str::FromStr<Err = crate::error::Error>,
{
}
/*
 ************* Implementations *************
*/
macro_rules! accidental {
    (@impl $(#[$meta:meta])* $vis:vis $type:ident $name:ident = $sym:literal $(;)?) => {
        unit_type! {
            $(#[$meta])*
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            $vis $type $name
        }

        impl $name {
            pub const NAME: &'static str = stringify!($name);
            /// compares some static type `T` against `Self`
            pub fn of<T>() -> bool
            where
                T: 'static,
            {
                ::core::any::TypeId::of::<T>() == ::core::any::TypeId::of::<Self>()
            }
            /// returns the symbol of the accidental
            pub const fn symbol(&self) -> char {
                $sym
            }
            /// returns the name of the accidental
            pub fn name(&self) -> &str {
                stringify!($name)
            }
        }

        impl $crate::pitch::RawAccidental for $name {
            seal! {}

            fn name(&self) -> &str {
                self.name()
            }

            fn symbol(&self) -> char {
                self.symbol()
            }
        }

        impl $crate::pitch::Accidental for $name {

        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                stringify!($name)
            }
        }

        #[cfg(feature = "alloc")]
        impl ::core::str::FromStr for $name {
            type Err = $crate::error::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if s == stringify!($name) || s == $sym.to_string() {
                    Ok(Self::default())
                } else {
                    Err($crate::error::Error::FromStrParseError)
                }
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.symbol())
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.symbol())
            }
        }
    };
    ($($vis:vis $type:ident $name:ident $(= $sym:literal)?);* $(;)?) => {
        $(accidental! { @impl $vis $type $name $(= $sym)? })*
    };
}

accidental! {
    pub struct Flat = '♭';
    pub struct Sharp = '♯';
}

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(transparent)]
pub struct Natural;

impl Natural {
    pub const fn new() -> Self {
        Self
    }
    /// compares some static type `T` against `Self`
    pub fn of<T>() -> bool
    where
        T: 'static,
    {
        ::core::any::TypeId::of::<T>() == ::core::any::TypeId::of::<Self>()
    }

    pub const fn name(&self) -> &str {
        "Natural"
    }

    pub const fn symbol(&self) -> char {
        '♮'
    }
}

impl crate::pitch::RawAccidental for Natural {
    seal! {}

    fn name(&self) -> &str {
        self.name()
    }

    fn symbol(&self) -> char {
        self.symbol()
    }
}

impl crate::pitch::Accidental for Natural {}

impl AsRef<str> for Natural {
    fn as_ref(&self) -> &str {
        "Natural"
    }
}

impl AsRef<char> for Natural {
    fn as_ref(&self) -> &char {
        &'♮'
    }
}

impl core::borrow::Borrow<str> for Natural {
    fn borrow(&self) -> &str {
        self.as_ref()
    }
}

impl core::borrow::Borrow<char> for Natural {
    fn borrow(&self) -> &char {
        self.as_ref()
    }
}

impl core::ops::Deref for Natural {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl core::str::FromStr for Natural {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.to_lowercase() == "natural" || s == '♮'.to_string() || s == "" {
            Ok(Natural)
        } else {
            Err(crate::error::Error::FromStrParseError)
        }
    }
}

impl core::fmt::Debug for Natural {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.name())
    }
}

impl core::fmt::Display for Natural {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}
