/*
    Appellation: notes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! notes {
    ($($suffix:literal)? $name:ident {$($cls:ident($value:expr)),* $(,)?} ) => {
        pub enum $name {
            $($cls = $value),*
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                match self {
                    $(Self::$cls => stringify!($cls)),*
                }
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {

                write!(f, "{}", self.as_ref())
            }
        }
    };
}

macro_rules! pitches {
    (@impl $class:ident($n:expr, flat: $flat:expr, sharp: $sharp:expr) ) => {
        pub struct $class;
    };
}


macro_rules! impl_pitch {
    ($class:ident($mud:expr, flat: $flat:expr, sharp: $sharp:expr) ) => {
        impl $class {
            pub fn mud() -> i8 {
                $mud
            }
            pub fn flat() -> i8 {
                $flat
            }
            pub fn sharp() -> i8 {
                $sharp
            }
        }
    };
}

notes! {
    Sample {
        C(0),
        D(2),
        E(4),
        F(5),
        G(7),
        A(9),
        B(11)
    }
}