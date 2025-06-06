/*
    Appellation: tone <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Frequency;

/// [PureTone] is defined to be a sine-wave of a constant amplitude, frequency, and phase shift.
///
///
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PureTone<T = f64> {
    /// The amplitude of the tone; the height or maximum value of the wave.
    amplitude: T,
    /// The frequency of the tone; typically measured in hertz [Hz]
    freq: Frequency<T>,
    /// The phase shift of the tone;
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

    pub fn from_freq(Frequency(freq): Frequency<T>) -> Self
    where
        T: Default,
    {
        Self {
            amplitude: T::default(),
            freq: Frequency(freq),
            phase: T::default(),
        }
    }

    pub fn with_amplitude(self, amplitude: T) -> Self {
        Self { amplitude, ..self }
    }

    pub fn with_freq(self, freq: Frequency<T>) -> Self {
        Self { freq, ..self }
    }

    pub fn with_phase(self, phase: T) -> Self {
        Self { phase, ..self }
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
