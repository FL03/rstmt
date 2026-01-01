/*
    Appellation: pitch_type <module>
    Created At: 2025.12.23:14:35:22
    Contrib: @FL03
*/

/// [`Accidental`] is a sealed marker trait used to designate various _kinds_ of musical notes,
/// i.e., sharp, flat, natural, etc.
pub trait Accidental: 'static + Default + Send + Sync + core::fmt::Debug {
    const NAME: &'static str;

    private! {}

    fn name(&self) -> &str {
        Self::NAME
    }

    fn symbol(&self) -> char;
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

        impl $crate::pitch::Accidental for $name {
            const NAME: &'static str = stringify!($name);
            seal! {}

            fn name(&self) -> &str {
                self.name()
            }

            fn symbol(&self) -> char {
                self.symbol()
            }
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
    pub struct Natural = '♮';
}

impl<T> Accidental for core::marker::PhantomData<T>
where
    T: Accidental,
{
    const NAME: &'static str = T::NAME;

    seal! {}

    fn name(&self) -> &str {
        <T>::NAME
    }

    fn symbol(&self) -> char {
        T::default().symbol()
    }
}
