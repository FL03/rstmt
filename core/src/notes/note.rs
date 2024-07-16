/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{IntoPitch, Octave, Pitch, Pitches};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Note {
    pub(crate) octave: Octave,
    pub(crate) pitch: Pitch,
}

impl Note {
    pub fn new(octave: Octave, pitch: impl IntoPitch) -> Self {
        Self {
            octave,
            pitch: pitch.into_pitch(),
        }
    }
    /// Returns a new instance of the note with the given pitch;
    /// the note's octave is set to the default octave (4).
    pub fn from_pitch(pitch: impl IntoPitch) -> Self {
        Self {
            octave: Octave::default(),
            pitch: pitch.into_pitch(),
        }
    }
    /// Returns an instance of the note's [PitchClass](crate::pitch::PitchCalss).
    /// Each pitch class is a synmoblic representation of a group of frequencies,
    /// which are separated by a factor of 2^(1/12).
    ///
    pub fn class(&self) -> Pitches {
        self.pitch.class()
    }

    /// Returns an owned instance of the note's octave
    pub const fn octave(&self) -> &Octave {
        &self.octave
    }
    /// Returns a mutable reference to the note's octave
    pub fn octave_mut(&mut self) -> &mut Octave {
        &mut self.octave
    }
    /// Returns an owned instance of the note's pitch
    pub const fn pitch(&self) -> &Pitch {
        &self.pitch
    }
    /// Returns a mutable reference to the note's pitch
    pub fn pitch_mut(&mut self) -> &mut Pitch {
        &mut self.pitch
    }
    /// Sets the note's octave
    pub fn set_octave(&mut self, octave: Octave) {
        self.octave = octave;
    }
    /// Sets the note's pitch
    pub fn set_pitch(&mut self, pitch: Pitch) {
        self.pitch = pitch;
    }
    /// Returns a new instance of the note with the given octave
    pub fn with_octave(self, octave: Octave) -> Self {
        Self { octave, ..self }
    }
    /// Returns a new instance of the note with the given pitch
    pub fn with_pitch(self, pitch: impl IntoPitch) -> Note {
        Note {
            octave: self.octave,
            pitch: pitch.into_pitch(),
        }
    }
}

/*
 ************* Implementations *************
*/

impl core::fmt::Display for Note {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}.{}", self.class(), self.octave)
    }
}

unsafe impl Send for Note {}

unsafe impl Sync for Note {}

impl From<(Octave, Pitch)> for Note {
    fn from((octave, pitch): (Octave, Pitch)) -> Self {
        Self { octave, pitch }
    }
}

impl From<Note> for (Octave, Pitch) {
    fn from(note: Note) -> Self {
        (note.octave, note.pitch)
    }
}

impl From<Note> for Pitch {
    fn from(note: Note) -> Self {
        note.pitch
    }
}

impl From<Pitch> for Note {
    fn from(pitch: Pitch) -> Self {
        Self {
            octave: Octave::default(),
            pitch,
        }
    }
}

macro_rules! impl_std_ops {
    (@impl $trait:ident.$call:ident($rhs:ty) -> $out:ty) => {
        impl core::ops::$trait<$rhs> for Note {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a> core::ops::$trait<&'a $rhs> for Note {
            type Output = $out;

            fn $call(self, rhs: &'a $rhs) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a> core::ops::$trait<$rhs> for &'a Note {
            type Output = $out;

            fn $call(self, rhs: $rhs) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }

        impl<'a> core::ops::$trait<&'a $rhs> for &'a Note {
            type Output = $out;

            fn $call(self, rhs: &'a $rhs) -> Self::Output {
                Note {
                    octave: ::core::ops::$trait::$call(self.octave, rhs.octave),
                    pitch: ::core::ops::$trait::$call(self.pitch, rhs.pitch)
                }
            }
        }
    };

    (@impl $trait:ident.$call:ident($rhs:ty)) => {
        impl_std_ops!(@impl $trait.$call($rhs) -> Note);
    };
    (@impl $trait:ident.$call:ident $(-> $out:ty)?) => {
        impl_std_ops!(@impl $trait.$call(Note) $(-> $out)?);
    };
    ($($trait:ident.$call:ident$(($rhs:ty))? $(-> $out:ty)?),* $(,)?) => {
        $(
            impl_std_ops!(@impl $trait.$call$(($rhs))? $(-> $out)?);
        )*
    };
}

impl_std_ops!(Add.add, Div.div, Mul.mul, Rem.rem, Sub.sub);
