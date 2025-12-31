/*
    Appellation: pitches <module>
    Created At: 2025.12.20:10:02:21
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

/// A trait for converting a reference into a [`Pitch`].
pub trait AsPitch<T>
where
    T: RawPitch,
{
    fn as_pitch(&self) -> Pitch<T>;
}
/// [`IntoPitch`] defines a consuming conversion from some type into a [`Pitch`].
pub trait IntoPitch<T>
where
    T: RawPitch,
{
    fn into_pitch(self) -> Pitch<T>;

    private! {}
}
/// The [`Pitched`] trait is used to denote objects that have an associated pitch.
pub trait Pitched<T>
where
    T: RawPitch,
{
    fn pitch(&self) -> &Pitch<T>;
}

/// The [`Pitch`] implementation is a generic wrapper used to represent a musical pitch. A
/// pitch is defined to be a perceptual property of sounds that enables one to define the
/// _highness_ or _lowness_ of a sound. In music, pitch is often associated with the
/// frequency of a sound wave, with higher frequencies corresponding to higher pitches.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct Pitch<T = f64>(pub T);

/*
 ************* Implementations *************
*/
impl<U, T> AsPitch<T> for U
where
    U: Clone + IntoPitch<T>,
    T: RawPitch,
{
    fn as_pitch(&self) -> Pitch<T> {
        self.clone().into_pitch()
    }
}

impl<U, T> IntoPitch<T> for U
where
    U: Into<Pitch<T>>,
    T: RawPitch,
{
    fn into_pitch(self) -> Pitch<T> {
        self.into()
    }

    seal! {}
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_creation() {
        let a4 = Pitch(440f64);
        assert_eq!(a4, 440.0);
    }
}
