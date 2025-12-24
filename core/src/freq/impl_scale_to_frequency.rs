/*
    Appellation: impl_freq_to_scale <module>
    Created At: 2025.12.21:08:58:02
    Contrib: @FL03
*/
use super::{Frequency, ScaleToFrequency, classify_freq_by_scale, compute_freq_of_pitch};
use num_traits::{Float, FromPrimitive};

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
    pub fn from_scale_degree(&self, freq: Frequency<T>) -> Option<isize>
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
        compute_freq_of_pitch(n, Some(anchor))
    }
}
