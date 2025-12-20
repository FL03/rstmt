/*
    Appellation: frequency <module>
    Contrib: @FL03
*/

mod impl_freq;
mod impl_freq_ops;
#[cfg(feature = "rand")]
mod impl_freq_rand;
mod impl_freq_repr;

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

/// The [`Frequency`] type is a generic wrapper around type `T` that implements the
/// [`RawFrequency`] trait. This implementation is designed to provide a consistent interface
/// for dealing with frequencies within the crate, enabling conversion, arithmetic operations,
/// and other utilities that are common to frequency values.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct Frequency<T = f64>(pub T);

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
