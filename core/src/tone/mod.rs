/*
    Appellation: tone <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(dead_code)]
#[doc(inline)]
pub use self::{pure::PureTone, tone::Tone};

pub(crate) mod pure;
pub(crate) mod tone;

pub(crate) mod prelude {
    pub use super::pure::*;
    pub use super::tone::*;
}

use crate::{Frequency, Pitch};

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
