/*
    appellation: convert <module>
    authors: @FL03
*/
use crate::freq::Frequency;

/// [`AsFrequency`] is a trait enabling the conversion of a reference to a type into a
/// [`Frequency`] instance.
pub trait AsFrequency<T> {
    /// Converts the current value into a [`Frequency`] instance.
    fn as_frequency(&self) -> Frequency<T>;
}
/// The [`IntoFrequency`] trait consumes the value and converts it into a [`Frequency`].
pub trait IntoFrequency<T> {
    /// Converts the current value into a [`Frequency`] instance.
    fn into_frequency(self) -> Frequency<T>;
}

/*
 ************* Implementations *************
*/
impl<T> IntoFrequency<T> for T
where
    T: Into<Frequency<T>>,
{
    fn into_frequency(self) -> Frequency<T> {
        self.into()
    }
}
