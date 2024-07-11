/*
    Appellation: pitches <macros>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! pitch {
    ($vis:vis enum $class:ident {$($name:ident = $value:expr),* $(,)?}) => {

        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, strum::AsRefStr, strum::Display, strum::EnumIs, strum::EnumString, strum::VariantNames)]
        #[repr(i8)]
        #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(rename_all = "lowercase"))]
        #[strum(serialize_all = "lowercase")]
        $vis enum $class {
            $($name = $value),*
        }

        impl $class {
            pub fn new(value: $crate::pitches::PitchTy) -> $crate::Result<Self> {
                match value.abs() % $crate::MODULUS as $crate::pitches::PitchTy {
                    $(x if x == $value => Ok(Self::$name),)*
                    _ => Err($crate::Error::invalid_pitch("Invalid pitch value."))
                }
            }

            pub fn as_class(&self) -> $crate::pitches::Pitches {
                $crate::pitches::Pitches::$class(*self)
            }

            pub fn pitch(&self) -> $crate::pitches::PitchTy {
                *self as $crate::pitches::PitchTy
            }
        }

        impl $crate::pitches::PitchClass for $class {
            fn pitch(&self) -> $crate::pitches::PitchTy {
                *self as $crate::pitches::PitchTy
            }
        }

        impl From<$class> for $crate::pitches::PitchTy {
            fn from(pitch: $class) -> $crate::pitches::PitchTy {
                pitch as $crate::pitches::PitchTy
            }
        }

        impl TryFrom<$crate::pitches::PitchTy> for $class {
            type Error = $crate::Error;

            fn try_from(value: PitchTy) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}
