/*
    Appellation: convert <module>
    Created At: 2025.12.20:09:12:05
    Contrib: @FL03
*/
use crate::pitch::Pitch;
/// A trait for converting a reference into a [`Pitch`].
pub trait AsPitch<T> {
    fn as_pitch(&self) -> Pitch<T>;
}
/// [`IntoPitch`] defines a consuming conversion from some type into a [`Pitch`].
pub trait IntoPitch<T> {
    fn into_pitch(self) -> Pitch<T>;

    private! {}
}

/*
 ************* Implementations *************
*/

impl<T> AsPitch<T> for T
where
    T: Clone + IntoPitch<T>,
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
