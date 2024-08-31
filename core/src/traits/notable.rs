/*
    Appellation: notable <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::pitch::{Pitch, Pitches};
use crate::Octave;

/// The American Standard Pitch Notation ([`ASPN`]) is a system popularized for its ability to
/// simplify the representation of musical notes. The ASPN representation simply modifies the
/// traditional symbolic framework by including a subscript used to denote the octave of the
/// note.
pub trait ASPN {
    fn aspn(&self) -> String {
        format!("{}.{}", self.pitch_class(), self.octave())
    }

    fn octave(&self) -> Octave;

    fn pitch(&self) -> Pitch;

    fn pitch_class(&self) -> Pitches {
        self.pitch().class()
    }
}

/// [Notable] is used to describe objects capable of being represented as a pitch
pub trait Notable: Copy + Sized + core::fmt::Display {
    /// Find the modular index of the given pitch
    fn pitch(&self) -> Pitch;
    /// Classify the pitch into a pitch class
    fn pitch_class(&self) -> Pitches {
        self.pitch().class()
    }
}

/*
 ************* Implementations *************
*/

impl ASPN for crate::Note {
    fn octave(&self) -> Octave {
        self.octave
    }

    fn pitch(&self) -> Pitch {
        self.pitch
    }
}

impl Notable for Pitch {
    fn pitch_class(&self) -> Pitches {
        self.class()
    }

    fn pitch(&self) -> Pitch {
        *self
    }
}

impl Notable for Pitches {
    fn pitch_class(&self) -> Pitches {
        *self
    }

    fn pitch(&self) -> Pitch {
        Pitch(self.pitch_class().get())
    }
}

impl Notable for crate::Note {
    fn pitch(&self) -> Pitch {
        *self.pitch()
    }
}

impl Notable for crate::PitchTy {
    fn pitch(&self) -> Pitch {
        Pitch(*self)
    }
}
