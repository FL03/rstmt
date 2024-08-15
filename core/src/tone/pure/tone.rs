/*
    Appellation: tone <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Frequency;


/// A pure tone is a sine-wave of a constant amplitude, frequency, and phase shift. []()
/// 
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PureTone<T = f64> {
    amplitude: T,
    freq: Frequency<T>,
    phase: T,
}

impl<T> PureTone<T> {
    pub fn new(amplitude: T, Frequency(freq): Frequency<T>, phase: T) -> Self {
        Self {
            amplitude,
            freq: Frequency(freq),
            phase,
        }
    }
    pub const fn amplitude(&self) -> &T {
        &self.amplitude
    }
    pub const fn freq(&self) -> &T {
        self.freq.get()
    }
    pub const fn phase(&self) -> &T {
        &self.phase
    }
    pub fn set_amplitude(&mut self, val: T) {
        self.amplitude = val;
    }
    pub fn set_freq(&mut self, val: T) {
        self.freq.set(val);
    }
    pub fn set_phase(&mut self, val: T) {
        self.phase = val;
    }
}