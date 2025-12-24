/*
    Appellation: frequency <module>
    Contrib: @FL03
*/

mod impl_freq;
mod impl_freq_ext;
#[cfg(feature = "rand")]
mod impl_freq_rand;
mod impl_freq_repr;
mod impl_scale_to_frequency;

use num_traits::{Float, FromPrimitive};

/// Given some pitch class $`n`$ (in semitones) and an optional base frequency $`\beta`$ (in hertz),
/// calculate the corresponding frequency $`f`$.
///
/// ```math
/// F=\beta\cdot{2^{\frac{n}{12}}}
/// ```
pub(crate) fn compute_freq_of_pitch<T>(n: isize, base: Option<T>) -> Option<T>
where
    T: Float + FromPrimitive,
{
    // get the base "tuning" frequency
    let base = base.unwrap_or(T::from_u16(440)?);
    let exp = T::from_f32(2f32.powf(n as f32 / 12f32))?;
    Some(base * exp)
}
/// Compute the pitch class of a frequency (in hertz), using the formula:
///
/// ```math
/// n = 12\cdot\log_{2}(\frac{F}{\beta})
/// ```
pub(crate) fn classify_freq_with_scale<T>(
    Frequency(freq): Frequency<T>,
    base: Option<T>,
) -> Option<isize>
where
    T: Float + FromPrimitive,
{
    // Ensure frequency is positive
    debug_assert! { freq.is_sign_negative(), "Frequency must be positive" }
    // Reference frequency (A4 = 440 Hz)
    let base = base.unwrap_or(T::from_u16(440)?);
    // Calculate pitch class: round(12 * log2(frequency / 440))
    let semitones = T::from_u8(12)? * (freq / base).log(T::from_u8(2)?);
    semitones.round().to_isize()
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

/// the [`ScaleToFrequency`] struct provides a way to convert between musical scale degrees
/// (in semitones) and their corresponding frequencies (in hertz) based on a given
/// anchor frequency. This is useful for applications such as music synthesis, audio processing,
/// and musical analysis, where it's important to relate musical notes to their physical
/// properties.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default, rename_all = "snake_case")
)]
#[repr(transparent)]
pub struct ScaleToFrequency<T = f64> {
    /// the `anchor` frequency is one that we can use as a reference point for calculating
    /// other frequencies within the scale
    pub anchor: Frequency<T>,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    const A4: f64 = 440.0;
    const C4: f64 = 261.6255653005986;

    #[test]
    fn test_freq_classification() {
        let f = Frequency(C4);
        let p_class = f.classify_by(Some(A4));
        assert_eq!(p_class, Some(-9));
    }

    #[test]
    fn test_freq_converter() {
        let n: isize = -9; // C4;
        let f_exp: f64 = 261.6255653005986;
        let base = ScaleToFrequency::new(440f64); // A4

        let res = base.compute(n).unwrap();

        assert!((res - f_exp).abs() < 1e-5);
        assert_eq!(base.from_scale_degree(res.into_frequency()), Some(n));
    }
}
