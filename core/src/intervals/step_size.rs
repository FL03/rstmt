/*
    Appellation: step_size <module>
    Created At: 2025.12.31:16:35:29
    Contrib: @FL03
*/

/// [`StepSize`] provides an enumeration for musical step sizes, namely semitones and tones.
/// The implementation primarily works to provide a generic unit of measurement for musical
/// intervals.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(untagged, rename_all = "snake_case")
)]
#[strum(serialize_all = "snake_case")]
pub enum StepSize {
    #[default]
    Semitone = 1,
    Tone = 2,
}

/*
 ************* Implementations *************
*/

impl StepSize {
    /// A functional constructor for the [`Semitone`](StepSize::Semitone) variant.
    pub const fn semitone() -> Self {
        StepSize::Semitone
    }
    /// A functional constructor for the [`Tone`](StepSize::Tone) variant.
    pub const fn tone() -> Self {
        StepSize::Tone
    }
}

macro_rules! impl_step_size_binary {
    (@impl $trait:ident::$method:ident for $T:ty ) => {
        impl ::core::ops::$trait<$T> for StepSize {
            type Output = $T;

            fn $method(self, rhs: $T) -> Self::Output {
                ::core::ops::$trait::$method(self as $T, rhs)
            }
        }

        impl ::core::ops::$trait<&$T> for StepSize {
            type Output = $T;

            fn $method(self, rhs: &$T) -> Self::Output {
                ::core::ops::$trait::$method(self as $T, *rhs)
            }
        }

        impl ::core::ops::$trait<&mut $T> for StepSize {
            type Output = $T;

            fn $method(self, rhs: &mut $T) -> Self::Output {
                ::core::ops::$trait::$method(self as $T, *rhs)
            }
        }

        impl ::core::ops::$trait<StepSize> for $T {
            type Output = $T;

            fn $method(self, rhs: StepSize) -> Self::Output {
                ::core::ops::$trait::$method(self, rhs as $T)
            }
        }

        impl ::core::ops::$trait<StepSize> for &$T {
            type Output = $T;

            fn $method(self, rhs: StepSize) -> Self::Output {
                ::core::ops::$trait::$method(*self, rhs as $T)
            }
        }

        impl ::core::ops::$trait<StepSize> for &mut $T {
            type Output = $T;

            fn $method(self, rhs: StepSize) -> Self::Output {
                ::core::ops::$trait::$method(*self, rhs as $T)
            }
        }
    };
    ($($ty:ty),* $(,)? ) => {
        $(
            impl_step_size_binary! { @impl Add::add for $ty }
            impl_step_size_binary! { @impl Sub::sub for $ty }
            impl_step_size_binary! { @impl Mul::mul for $ty }
            impl_step_size_binary! { @impl Div::div for $ty }
            impl_step_size_binary! { @impl Rem::rem for $ty }
        )*
    };
}

impl_step_size_binary! { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }

macro_rules! impl_convert_step_size {
    ($($ty:ty),+) => {
        $(
            impl From<StepSize> for $ty {
                fn from(value: StepSize) -> Self {
                    value as $ty
                }
            }

            impl From<$ty> for StepSize {
                fn from(value: $ty) -> Self {
                    match value % 2 {
                        x if x == StepSize::Semitone as $ty => StepSize::Semitone,
                        x if x == StepSize::Tone as $ty => StepSize::Tone,
                        _ => unreachable!("invalid value for StepSize conversion"),
                    }
                }
            }
        )+
    };
}

impl_convert_step_size! { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }
