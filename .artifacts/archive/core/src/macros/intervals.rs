/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! interval {
    ($(default: $variant:ident$(,)?)?; $vis:vis enum $name:ident {$($key:ident = $val:literal),* $(,)?}) => {
        unit_enum! {
            $vis enum $name {
                $($key = $val),*
            }
        }

        impl $name {
            pub fn new(src: $crate::Note, tgt: $crate::Note) -> Result<Self, $crate::Error> {
                Self::try_from((tgt - src).pitch.pymod())
            }

            pub fn from_i8(value: i8) -> Result<Self, $crate::Error> {
                Self::try_from(value)
            }

            pub fn into_interval(self) -> $crate::Intervals {
                self.into()
            }

            pub fn interval(&self) -> $crate::Intervals {
                (*self).into()
            }

            pub fn get(&self) -> $crate::intervals::IntervalTy {
                *self as $crate::intervals::IntervalTy
            }

            pub fn value(&self) -> $crate::intervals::IntervalTy {
                *self as $crate::intervals::IntervalTy
            }

            pub fn validate(value: i8) -> bool {
                Self::try_from(value).is_ok()
            }
        }

        enum_as!($name: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

        impl $crate::IntervalKind for $name {
            seal!();

            fn value(&self) -> i8 {
                $name::value(self)
            }
        }

        impl TryFrom<i8> for $name where {
            type Error = $crate::Error;

            fn try_from(interval: i8) -> Result<$name, Self::Error> {
                match interval {
                    $($val => Ok($name::$key)),*,
                    _ => Err($crate::Error::invalid_interval(format!("Invalid interval value: {}", interval))),
                }
            }
        }

        impl TryFrom<$crate::Pitch> for $name where {
            type Error = $crate::Error;

            fn try_from(interval: $crate::Pitch) -> Result<$name, Self::Error> {
                Self::try_from(interval.0)
            }
        }

        $(
            impl Default for $name {
                fn default() -> Self {
                    $name::$variant
                }
            }
        )?
    };
}

macro_rules! enum_from_value {
    ($T:ty => $name:ident {$($key:ident: $val:expr),* $(,)?}) => {

        impl From<$T> for $name {
            fn from(value: $T) -> Self {
                use strum::EnumCount;
                match value % Self::COUNT as $T {
                    $(x if x == $val => Self::$key,)*
                    _ => panic!("Invalid value {}", value),
                }
            }
        }
    };
}

macro_rules! enum_as {
    ($name:ident: $($T:ty),* $(,)?) => {
        $(
            impl From<$name> for $T {
                fn from(interval: $name) -> $T {
                    interval as $T
                }
            }
        )*
    };
}
