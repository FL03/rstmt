/*
    Appellation: pitches <module>
    Created At: 2025.12.20:10:02:21
    Contrib: @FL03
*/
use super::RawPitch;

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
pub trait Pitched<T> {
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
