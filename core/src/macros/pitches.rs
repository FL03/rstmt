/*
    Appellation: pitches <macros>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! pitch {
    ($name:ident: $($class:ident = $val:expr),* $(,)?) => {

    };
    ($vis:vis enum $class:ident {$($name:ident = $value:expr),* $(,)?}) => {

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, strum::AsRefStr, strum::Display, strum::EnumIs, strum::EnumString, strum::VariantNames)]
        #[repr(i8)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = "UPPERCASE"))]
        #[strum(serialize_all = "UPPERCASE")]
        $vis enum $class {
            $($name = $value),*
        }

        impl $class {
            pub fn try_from_value(value: $crate::PitchTy) -> $crate::Result<Self> {
                match $crate::traits::PitchMod::pitchmod(&value) {
                    $(x if x == $value => Ok(Self::$name),)*
                    _ => Err($crate::Error::invalid_pitch("Invalid pitch value."))
                }
            }

            pub fn as_class(&self) -> $crate::pitch::Pitches {
                $crate::pitch::Pitches::$class(*self)
            }

            pub fn pitch(&self) -> $crate::PitchTy {
                *self as $crate::PitchTy
            }
        }

        impl $crate::pitch::PitchClass for $class {
            fn pitch(&self) -> $crate::PitchTy {
                *self as $crate::PitchTy
            }
        }

        impl From<$class> for $crate::PitchTy {
            fn from(pitch: $class) -> $crate::PitchTy {
                pitch as $crate::PitchTy
            }
        }

        impl TryFrom<$crate::PitchTy> for $class {
            type Error = $crate::Error;

            fn try_from(value: $crate::PitchTy) -> Result<Self, Self::Error> {
                Self::try_from_value(value)
            }
        }
    };
}
