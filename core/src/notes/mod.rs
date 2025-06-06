/*
    appellation: notes <module>
    authors: @FL03
*/
//! this modules implements the various representations of musical notes, octaves, and pitches.
#[doc(inline)]
pub use self::{note::Note, pitch::Pitch, types::prelude::*};

pub mod note;
pub mod pitch;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod aspn;
    pub mod flags;
    pub mod octave;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::aspn::*;
        #[doc(inline)]
        pub use super::flags::*;
        #[doc(inline)]
        pub use super::octave::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::note::*;
    #[doc(inline)]
    pub use super::pitch::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
    #[doc(inline)]
    pub use super::{PitchNum, RawPitch};
}
/// [`RawPitch`] defines an interface for all raw pitch types.
///
/// **note:** This trait is sealed and cannot be implemented outside of this crate.
pub trait RawPitch: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();
}
/// The [`PitchNum`] trait extends the [`RawPitch`] trait with additional numeric operations
/// and traits.
pub trait PitchNum: RawPitch + Sized
where
    Self: Copy
        + Eq
        + PartialEq
        + PartialOrd
        + core::ops::Add<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::Neg<Output = Self>
        + core::ops::AddAssign
        + core::ops::SubAssign
        + core::ops::MulAssign
        + core::ops::DivAssign
        + core::ops::RemAssign
        + num_traits::FromPrimitive
        + num_traits::ToPrimitive
        + num_traits::Zero
        + num_traits::One,
{
}

macro_rules! impl_raw_pitch {
    ($($t:ty),* $(,)?) => {
        $(
            impl_raw_pitch!(@impl $t);
        )*
    };
    (@impl $t:ty) => {
        impl RawPitch for $t {
            seal!();
        }
    };
}

impl_raw_pitch!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl<T> PitchNum for T where
    T: RawPitch
        + Copy
        + Eq
        + PartialEq
        + PartialOrd
        + core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>
        + core::ops::Mul<Output = T>
        + core::ops::Div<Output = T>
        + core::ops::Rem<Output = T>
        + core::ops::Neg<Output = T>
        + core::ops::AddAssign
        + core::ops::SubAssign
        + core::ops::MulAssign
        + core::ops::DivAssign
        + core::ops::RemAssign
        + num_traits::FromPrimitive
        + num_traits::ToPrimitive
        + num_traits::Zero
        + num_traits::One
{
}
