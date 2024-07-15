/*
    Appellation: notable <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::pitch::{Pitch, PitchTy, Pitches};

pub trait Notable: Copy + Sized + core::fmt::Display {
    /// Classify the pitch into a pitch class
    fn class(&self) -> Pitches {
        self.pitch().class()
    }
    /// Find the modular index of the given pitch
    fn pitch(&self) -> Pitch;
}

/*
 ************* Implementations *************
*/
impl Notable for Pitch {
    fn class(&self) -> Pitches {
        self.class()
    }

    fn pitch(&self) -> Pitch {
        *self
    }
}

impl Notable for Pitches {
    fn class(&self) -> Pitches {
        *self
    }

    fn pitch(&self) -> Pitch {
        Pitch(self.class().pitch())
    }
}

impl Notable for PitchTy {
    fn pitch(&self) -> Pitch {
        Pitch(*self)
    }
}
