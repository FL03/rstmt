/*
    Appellation: pitches <macros>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! pitch_class {
    ($(#[derive($($derive:ident),* $(,)?)])? $(#[default($default:ident)])? $(#[rename($rename:literal)])? $vis:vis enum $name:ident $($rest:tt)*) => {
        pitch_class!(@impl $(#[derive($($derive),*)])? $(#[default($default)])? $(#[rename($rename)])? $vis enum $name $($rest)*);
    };
    (@impl $(#[derive($($derive:ident),* $(,)?)])? $(#[default($default:ident)])? $vis:vis enum $name:ident $($rest:tt)*) => {
        pitch_class!($(#[derive($($derive),*)])? $(#[default($default)])? #[rename("UPPERCASE")] $vis enum $name $($rest)*);
    };
    (@impl $(#[derive($($derive:ident),* $(,)?)])? $(#[default($default:ident)])? #[rename($rename:literal)] $vis:vis enum $name:ident $($rest:tt)*) => {

        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            strum::AsRefStr,
            strum::Display,
            strum::EnumCount,
            strum::EnumIs,
            strum::VariantNames,
            $($($derive),*)?
        )]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = $rename))]
        #[strum(serialize_all = $rename)]
        pub enum $name $($rest)*

        impl_pitch!($name $($rest)*);

        $(
            impl Default for $name {
                fn default() -> Self {
                    Self::$default
                }
            }
        )?

    };
}

macro_rules! impl_pitch {
    ($group:ident {$($class:ident = $value:expr),* $(,)?}) => {
        impl $group {
            pub fn new(value: $crate::PitchTy) -> Option<Self> {
                $group::try_from(value).ok()
            }
            pub fn try_from_value(value: $crate::PitchTy) -> $crate::Result<Self> {
                match $crate::PitchMod::pitchmod(&value) {
                    $(x if x == $value => Ok(Self::$class),)*
                    _ => Err($crate::Error::invalid_pitch("Invalid pitch value."))
                }
            }

            pub fn as_class(&self) -> $crate::pitch::Pitches {
                $crate::pitch::Pitches::$group(*self)
            }

            pub fn pitch(&self) -> $crate::PitchTy {
                *self as $crate::PitchTy
            }
        }

        impl $crate::pitch::PitchClass for $group {
            fn pitch(&self) -> $crate::PitchTy {
                *self as $crate::PitchTy
            }
        }

        impl From<$group> for $crate::PitchTy {
            fn from(pitch: $group) -> $crate::PitchTy {
                pitch as $crate::PitchTy
            }
        }

        impl From<$group> for $crate::pitch::Pitch {
            fn from(pitch: $group) -> $crate::pitch::Pitch {
                $crate::pitch::Pitch::new(pitch as $crate::PitchTy)
            }
        }

        impl TryFrom<$crate::pitch::PitchTy> for $group {
            type Error = $crate::Error;

            fn try_from(p: $crate::pitch::PitchTy) -> Result<Self, Self::Error> {
                match $crate::PitchMod::pitchmod(&p) {
                    $(x if x == $value => Ok(Self::$class),)*
                    _ => Err($crate::Error::invalid_pitch("Invalid pitch value."))
                }
            }
        }

        impl TryFrom<$crate::pitch::Pitch> for $group {
            type Error = $crate::Error;

            fn try_from(p: $crate::pitch::Pitch) -> Result<Self, Self::Error> {
                Self::try_from(p.value())
            }
        }
    };
}
