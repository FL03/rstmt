/*
    appellation: pitch <module>
    authors: @FL03
*/
//! this module implements the [`Pitch`] type and its associated traits and types.
#[doc(inline)]
pub use self::{pitch_base::*, pitch_class::*};
// modules
mod pitch_base;
mod pitch_class;
mod wrapper;

// prelude (local)
pub(crate) mod prelude {
    pub use super::pitch_class::*;
    pub use super::wrapper::*;
    pub use super::{AsPitch, IntoPitch, PitchNum, RawPitch};
}

/// A trait for converting a reference into a [`Pitch`].
pub trait AsPitch {
    fn as_pitch(&self) -> ClassifiedPitch;
}
/// A trait for converting a type into a [`Pitch`].
pub trait IntoPitch {
    fn into_pitch(self) -> ClassifiedPitch;
}

/// [`RawPitch`] defines an interface for all raw pitch types.
///
/// **note:** This trait is sealed and cannot be implemented outside of this crate.
pub trait RawPitch:
    'static + Default + Send + Sync + core::fmt::Debug + core::fmt::Display
{
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

/*
 ************* Implementations *************
*/
impl<T> AsPitch for T
where
    T: Clone + IntoPitch,
{
    fn as_pitch(&self) -> ClassifiedPitch {
        self.clone().into_pitch()
    }
}

impl<T> IntoPitch for T
where
    T: Into<ClassifiedPitch>,
{
    fn into_pitch(self) -> ClassifiedPitch {
        self.into()
    }
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
