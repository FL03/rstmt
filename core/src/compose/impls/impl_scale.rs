/*
    Appellation: impl_freq_to_scale <module>
    Created At: 2025.12.21:08:58:02
    Contrib: @FL03
*/
use crate::compose::Scale;
use crate::freq::{
    Frequency, IntoFrequency, RawFrequency, classify_freq_with_scale, get_frequency_of_pitch,
};
use num_traits::{Float, FromPrimitive};

impl<T> Scale<T> {
    pub const fn new(root: T) -> Self {
        Self {
            root: Frequency(root),
        }
    }
    /// creates a new [`Scale`] from the given root frequency
    pub const fn from_freq(root: Frequency<T>) -> Self {
        Self { root }
    }
    /// returns a reference to the base frequency
    pub const fn root(&self) -> &Frequency<T> {
        &self.root
    }
    /// returns a mutable reference to the base frequency
    pub const fn root_mut(&mut self) -> &mut Frequency<T> {
        &mut self.root
    }
    /// classifies the given frequency as a pitch class `n`, using the formula:
    ///
    /// ```math
    /// n=\text{round}\Bigg(12\cdot\log_{2}\bigg(\frac{F}{\beta}\bigg)\Bigg)
    /// ```
    pub fn classify(&self, Frequency(freq): Frequency<T>) -> Option<isize>
    where
        T: RawFrequency + Float + FromPrimitive,
    {
        // Ensure frequency is positive
        if freq <= T::zero() {
            return None;
        }
        classify_freq_with_scale(freq, self.root().value())
    }
    /// returns the frequency [Hz] of the given pitch class `n`, using the formula:
    ///
    /// ```math
    /// F= \beta \cdot 2^{\frac{n}{12}}
    /// ```
    pub fn get_freq_of_class(&self, n: isize) -> Frequency<T>
    where
        T: RawFrequency + Float + FromPrimitive,
    {
        get_frequency_of_pitch(n, self.root().value()).into_frequency()
    }
}
