/*
    Appellation: freq <module>
    Created At: 2025.12.29:17:13:00
    Contrib: @FL03
*/
mod impl_freq;
mod impl_freq_ext;
#[cfg(feature = "rand")]
mod impl_freq_rand;
mod impl_freq_repr;

/// [`RawFrequency`] is a marker trait denoting objects capable of representing a frequency
pub trait RawFrequency {
    private! {}
}

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
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
#[repr(transparent)]
pub struct Frequency<T = f64>(pub T);

/*
 ************* Implementations *************
*/
impl<U, T> IntoFrequency<T> for U
where
    U: Into<Frequency<T>>,
{
    fn into_frequency(self) -> Frequency<T> {
        self.into()
    }
}

macro_rules! raw_frequency {
    (@impl $T:ty) => {
        impl RawFrequency for $T {
            seal! {}
        }
    };
    {$($T:ty),* $(,)?} => {
        $(raw_frequency!(@impl $T);)*
    };
}

raw_frequency! {
    f32, f64,
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize
}

#[cfg(feature = "complex")]
impl<T> RawFrequency for num_complex::Complex<T>
where
    T: RawFrequency,
{
    seal! {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const A4: f64 = 440.0;
    const C4: f64 = 261.6255653005986;

    #[test]
    fn test_freq_classification() {
        let f = Frequency(C4);
        let p_class = f.classify_by(A4);
        assert_eq!(p_class, Some(-9));
    }
}
