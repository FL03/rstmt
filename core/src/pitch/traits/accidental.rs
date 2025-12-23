/*
    Appellation: pitch_type <module>
    Created At: 2025.12.23:14:35:22
    Contrib: @FL03
*/

/// [`Accidental`] is a sealed marker trait used to designate various _kinds_ of musical notes,
/// i.e., sharp, flat, natural, etc.
pub trait Accidental: 'static + AsRef<str> + Default + Send + Sync + core::fmt::Debug + core::fmt::Display {
    const NAME: &'static str;

    private! {}

    fn name(&self) -> &str {
        Self::NAME
    }
}

/*
 ************* Implementations *************
*/
macro_rules! accidental {
    (@impl $(#[$meta:meta])* $vis:vis $type:ident $name:ident $(;)?) => {
        unit_type! { 
            $(#[$meta])*            
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            $vis $type $name;
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
        }

        impl $crate::pitch::Accidental for $name {
            const NAME: &'static str = stringify!($name);
            seal! {}

            fn name(&self) -> &str {
                Self::NAME
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                stringify!($name)
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_ref())
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(self.as_ref())
            }
        }

    };
    ($($vis:vis $type:ident $name:ident);* $(;)?) => {
        $(accidental! { @impl $vis $type $name })*
    };
}

accidental! {
    pub struct Flat;
    pub struct Sharp;
    pub struct Natural;
}
