/*
    Appellation: harmonic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::pitch::Pitch;
use num::traits::{Num, NumOps};

pub trait Frequency<T> {
    fn freq(self) -> T;

    fn period(self) -> T;
}

/// A tone is defined to be the pitch, quality, and strength of a particular
/// sound. Tone's are fundamental to the study of music and sound, providing
/// a standardized means of describing the characteristics of a sound.
pub trait MusicTone {
    type Freq: Frequency<f64>;

    fn pitch(&self) -> Pitch;
    /// The quality of a tone is a measure of the harmonic content of the sound.
    /// Often refered to as timbre, the quality of a tone is what allows us to
    /// distinguish a sound over time.
    fn quality(&self) -> f64;

    fn strength(&self) -> f64;
}

pub trait Timbre {}

pub trait Harmonic {
    type Tone: MusicTone;
}

pub struct Freq<T = f64> {
    hz: T,
}

/// A pure tone is a sine-wave of a constant amplitude, frequency, and phase shift.
pub struct PureTone<T = f64> {
    amplitude: T,
    freq: T,
    phase: T,
}
pub struct Tone<F> {
    pitch: Pitch,
    quality: f64,
    strength: F,
}

impl<T> Freq<T>
where
    T: Num + NumOps,
{
    pub fn period(self) -> T {
        T::one() / self.hz
    }
}
