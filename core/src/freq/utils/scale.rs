/*
    appellation: frequency <module>
    authors: @FL03
*/
use crate::PitchMod;
use num_traits::{Float, FromPrimitive};

/// the [`ScaleToFrequency`] struct provides a way to convert between musical scale degrees
/// (in semitones) and their corresponding frequencies (in hertz) based on a given
/// anchor frequency. This is useful for applications such as music synthesis, audio processing,
/// and musical analysis, where it's important to relate musical notes to their physical
/// properties.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(default, rename_all = "snake_case"))]
pub struct ScaleToFrequency<T = f64> {
    /// the `anchor` frequency is one that we can use as a reference point for calculating
    /// other frequencies within the scale
    pub anchor: T,
}

impl<T> ScaleToFrequency<T> {
    pub const fn new(base_freq: T) -> Self {
        Self { anchor: base_freq }
    }
    /// calculate the position (in semitones) of a given frequency, using the formula:
    ///
    /// ```math
    /// n = 12 * log2(f / base)
    /// ```
    pub fn from_scale_degree(&self, freq: T) -> Option<isize>
    where
        T: Copy + Float + FromPrimitive,
    {
        // Ensure frequency is positive
        if freq <= T::zero() {
            return None;
        }
        // Calculate pitch class: round(12 * log2(frequency / tune))
        let log2 = T::from(2.0).unwrap();
        let semitones = T::from(12.0).unwrap() * (freq / self.anchor).log(log2);
        let pitch_class = semitones.round().to_i32()?.pmod();

        isize::from_i32(pitch_class)
    }
    /// returns a reference to the base frequency
    pub const fn base_freq(&self) -> &T {
        &self.anchor
    }
    /// returns a mutable reference to the base frequency
    pub fn base_freq_mut(&mut self) -> &mut T {
        &mut self.anchor
    }
    /// calculate the frequency (in hertz) of a given pitch class, using the formula:
    ///
    /// ```math
    /// f = base * 2^(n/12)
    /// ```
    pub fn compute(&self, n: i32) -> Option<T>
    where
        T: Float + FromPrimitive,
    {
        // get the base "tuning" frequency
        let exp = T::from_f64(n as f64 / 12.0)?;
        let res = self.anchor * T::from_f64(2.0)?.powf(exp);
        Some(res)
    }
}

/// calculate the frequency (in hertz) of a given pitch class, using the formula:
///
/// ```math
/// f = base * 2^(n/12)
/// ```
pub fn compute_freq_from_scale<T>(n: isize, base: Option<f64>) -> Option<T>
where
    T: Float + FromPrimitive,
{
    // get the base "tuning" frequency
    let base = T::from_f64(base.unwrap_or(440.0))?;
    let exp = T::from_f64(n as f64 / 12.0)?;
    let res = base * T::from_f64(2.0)?.powf(exp);
    Some(res)
}
/// Compute the pitch class of a frequency (in hertz), using the formula:
///
/// ```math
/// n = 12 * log2(f / base)
/// ```
pub fn get_scale_of_freq<T>(freq: T, base: Option<T>) -> Option<isize>
where
    T: Float + FromPrimitive,
{
    // Ensure frequency is positive
    if freq <= T::zero() {
        return None;
    }
    // Reference frequency (A4 = 440 Hz)
    let ref_freq = base.unwrap_or(T::from(440.0)?);

    // Calculate pitch class: round(12 * log2(frequency / 440))
    let log2 = T::from(2.0).unwrap();
    let semitones = T::from(12.0).unwrap() * (freq / ref_freq).log(log2);
    let pitch_class = semitones.round().to_i32()?.pmod();

    isize::from_i32(pitch_class)
}
