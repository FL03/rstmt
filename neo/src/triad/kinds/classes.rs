/*
    Appellation: classes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Class, Kind, Relative, TriadCls, TriadKind};
use crate::triad::Triads;

macro_rules! class {
    (@impl $name:ident::$call:ident(relative: $relative:ident)) => {
        #[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize),)]
        #[repr(transparent)]
        pub struct $name;

        impl $name {
            pub fn class() -> Triads {
                Triads::$name
            }

            pub fn as_class(&self) -> Triads {
                Triads::$name
            }

            pub fn into_class(self) -> Triads {
                Triads::$name
            }

            pub fn name(&self) -> &'static str {
                stringify!($call)
            }
        }

        impl Relative for $name {
            type Rel = $relative;

            fn relative(&self) -> Self::Rel {
                $relative
            }
        }

        impl Class for $name {}

        impl Kind for $name {
            type Class = Triads;

            fn class() -> Triads where Self: Sized {
                Triads::$name
            }

            fn name() -> &'static str where Self: Sized {
                $name.as_ref()
            }
        }



        impl TriadCls for $name {
            seal!();

            fn named(&self) -> &'static str {
                stringify!($call)
            }
        }

        impl TriadKind for $name {
            type Rel = $relative;

            seal!();
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                stringify!($call)
            }
        }

        impl core::ops::Deref for $name {
            type Target = Triads;

            fn deref(&self) -> &Self::Target {
                &Triads::$name
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(stringify!($call))
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(stringify!($call))
            }
        }

        unsafe impl Send for $name {}

        unsafe impl Sync for $name {}
    };
    ($($name:ident::$call:ident(relative: $rel:ident)),* $(,)?) => {
        $(
            class!(@impl $name::$call(relative: $rel));
        )*
    };
}

class!(
    Augmented::augmented(relative: Diminished),
    Diminished::diminished(relative: Augmented),
    Major::major(relative: Minor),
    Minor::minor(relative: Major),
);
