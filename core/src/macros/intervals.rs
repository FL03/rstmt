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
            pub fn value(&self) -> i8 {
                *self as i8
            }

            pub fn validate(value: i8) -> bool {
                Self::try_from(value).is_ok()
            }


        }

        enum_as!($name: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

        impl From<i8> for $name where {
            fn from(interval: i8) -> $name {
                use strum::EnumCount;
                match interval % Self::COUNT as i8 {
                    $($val => $name::$key),*,
                    _ => panic!("Invalid interval value: {}", interval),
                }
            }
        }
        enum_from_value!(u8 => $name {$($key: $val),*});
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
