/*
    appellation: frequency <module>
    authors: @FL03
*/
use super::Frequency;
use num_traits::{Float, FromPrimitive};
use rstmt_traits::PitchMod;

/// Given some pitch class $`n`$ (in semitones) and an optional base frequency $`\beta`$ (in hertz),
/// calculate the corresponding frequency $`f`$.
///
/// ```math
/// F=\beta\cdot{2^{\frac{n}{12}}}
/// ```
pub fn get_freq_from_scale<T>(n: isize, base: Option<T>) -> Option<T>
where
    T: Float + FromPrimitive,
{
    // get the base "tuning" frequency
    let base = match base {
        Some(v) => v,
        None => T::from_f32(440.0)?,
    };

    let res = base * T::from_f32(2f32.powf(n as f32 / 12f32))?;
    Some(res)
}
/// Compute the pitch class of a frequency (in hertz), using the formula:
///
/// ```math
/// n = 12\cdot\log_{2}(\frac{F}{\beta})
/// ```
pub fn classify_freq_by_scale<T>(freq: T, base: Option<T>) -> Option<isize>
where
    T: Float + FromPrimitive + PitchMod<Output = T>,
{
    // Ensure frequency is positive
    debug_assert! { freq <= T::zero(), "Frequency must be positive" }
    // Reference frequency (A4 = 440 Hz)
    let base = base.unwrap_or(T::from_u16(440)?);
    // Calculate pitch class: round(12 * log2(frequency / 440))
    let two = T::from_u8(2)?;
    let modulo = T::from_u8(12)?;
    let semitones = modulo * (freq / base).log(two);
    semitones.round().pmod().to_isize()
}

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
pub struct ScaleToFrequency<T = f64> {
    /// the `anchor` frequency is one that we can use as a reference point for calculating
    /// other frequencies within the scale
    pub anchor: Frequency<T>,
}

impl<T> ScaleToFrequency<T> {
    pub const fn new(anchor: T) -> Self {
        Self {
            anchor: Frequency(anchor),
        }
    }
    /// calculate the position (in semitones) of a given frequency, using the formula:
    ///
    /// ```math
    /// n = 12 * log2(f / base)
    /// ```
    pub fn from_scale_degree(&self, freq: T) -> Option<isize>
    where
        T: Float + FromPrimitive,
    {
        // Ensure frequency is positive
        if freq <= T::zero() {
            return None;
        }
        let anchor = self.anchor().get();
        classify_freq_by_scale(freq, Some(*anchor))
    }
    /// returns a reference to the base frequency
    pub const fn anchor(&self) -> &Frequency<T> {
        &self.anchor
    }
    /// returns a mutable reference to the base frequency
    pub const fn anchor_mut(&mut self) -> &mut Frequency<T> {
        &mut self.anchor
    }
    /// calculate the frequency (in hertz) of a given pitch class, using the formula:
    ///
    /// ```math
    /// f = base * 2^(n/12)
    /// ```
    pub fn compute(&self, n: isize) -> Option<T>
    where
        T: Float + FromPrimitive,
    {
        let anchor = **self.anchor();
        get_freq_from_scale(n, Some(anchor))
    }
}
