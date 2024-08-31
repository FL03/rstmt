/*
    Appellation: note <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{IntoPitch, Octave, Pitch, Pitches};

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

    pub fn from_octave(octave: Octave) -> Self {
        Self {
            octave,
            pitch: Pitch::default(),
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
    /// Consumes the current note; returning a new instance with the given octave.
    pub fn with_octave(self, octave: Octave) -> Self {
        Self { octave, ..self }
    }
    /// Consumes the current note; returning a new instance with the given pitch.
    pub fn with_pitch(self, pitch: impl IntoPitch) -> Self {
        Self {
            pitch: pitch.into_pitch(),
            ..self
        }
    }
    /// Returns a string representation of the note consistent with the [American Standard Pitch Notation](https://en.wikipedia.org/wiki/Scientific_pitch_notation).
    pub fn aspn(&self) -> String {
        format!("{}.{}", self.class(), self.octave)
    }
    /// Returns an instance of the note's pitch class; each class is a symbolic representation
    /// of a group of frequencies separated by a factor of 2^(1/12).
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
    pub fn set_pitch(&mut self, Pitch(pitch): Pitch) {
        self.pitch.set(pitch);
    }
}

/*
 ************* Implementations *************
*/

impl core::fmt::Debug for Note {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.aspn().as_str())
    }
}

impl core::fmt::Display for Note {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(self.aspn().as_str())
    }
}

#[allow(unused)]
impl core::str::FromStr for Note {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let pitch = parts
            .next()
            .ok_or_else(|| crate::Error::music_error("Invalid note."))?;
        let octave = parts
            .next()
            .ok_or_else(|| crate::Error::music_error("Invalid note."))?;
        unimplemented!();
        // let pitch = pitch.parse::<Pitches>()?;
        // let octave = octave.parse::<Octave>()?;
        // Ok(Note { octave, pitch })
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

impl From<Pitch> for Note {
    fn from(pitch: Pitch) -> Self {
        Self {
            octave: Octave::default(),
            pitch,
        }
    }
}

impl From<Note> for Pitch {
    fn from(note: Note) -> Self {
        note.pitch
    }
}

impl From<Octave> for Note {
    fn from(octave: Octave) -> Self {
        Self {
            octave,
            pitch: Pitch::default(),
        }
    }
}

impl From<Note> for Octave {
    fn from(note: Note) -> Self {
        note.octave
    }
}
