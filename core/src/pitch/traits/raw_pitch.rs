/*
    Appellation: raw_pitch <module>
    Created At: 2025.12.20:09:10:45
    Contrib: @FL03
*/
use rstmt_traits::Numerical;

/// [`RawPitch`] defines an interface for all raw pitch types.
///
/// **note:** This trait is sealed and cannot be implemented outside of this crate.
pub trait RawPitch
where
    Self: Send + Sync + core::fmt::Debug + core::fmt::Display + PartialEq + PartialOrd,
{
    private! {}
}
/// [`NumPitch`] extends the `RawPitch` trait with additional capabilities for numerical types.
/// The trait is automatically implemented for all
pub trait NumPitch
where
    Self: RawPitch + Numerical,
{
}
/*
 ************* Implementations *************
*/

impl<T> NumPitch for T where T: RawPitch + Numerical {}

impl<T> RawPitch for &T
where
    T: RawPitch,
{
    seal! {}
}

impl<T> RawPitch for &mut T
where
    T: RawPitch,
{
    seal! {}
}

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
