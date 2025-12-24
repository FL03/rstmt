/*
    Appellation: raw_pitch <module>
    Created At: 2025.12.20:09:10:45
    Contrib: @FL03
*/

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

macro_rules! impl_raw_pitch {
    ($($tgt:ty),* $(,)?) => {
        $(
            impl RawPitch for $tgt {
                seal! {}
            }
        )*
    };
}

impl_raw_pitch! {
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
}

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
