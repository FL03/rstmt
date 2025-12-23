/*
    Appellation: pitches <module>
    Created At: 2025.12.20:10:02:21
    Contrib: @FL03
*/

/// A trait for converting a reference into a [`Pitch`].
pub trait AsPitch<T> {
    fn as_pitch(&self) -> Pitch<T>;
}
/// [`IntoPitch`] defines a consuming conversion from some type into a [`Pitch`].
pub trait IntoPitch<T> {
    fn into_pitch(self) -> Pitch<T>;

    private! {}
}

/// Musically, a pitch is defined to be a discrete frequency that may be symbolically
/// represented via a pitch class.
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
{
    fn as_pitch(&self) -> Pitch<T> {
        self.clone().into_pitch()
    }
}

impl<U, T> IntoPitch<T> for U
where
    U: Into<Pitch<T>>,
{
    fn into_pitch(self) -> Pitch<T> {
        self.into()
    }

    seal! {}
}
