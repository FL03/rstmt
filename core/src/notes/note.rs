/*
    Appellation: aspn <module>
    Contrib: @FL03
*/
use super::Octave;
use crate::PitchMod;

/// An american scientific pitch notation ([`Note`]) representation of a musical note.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize),
    serde(deny_unknown_fields, default, rename_all = "snake_case")
)]
#[repr(C)]
pub struct Note {
    pub(crate) class: usize,
    pub(crate) octave: Octave,
}

impl Note {
    pub fn new(class: usize, Octave(octave): Octave) -> Self {
        Self {
            class,
            octave: Octave(octave),
        }
    }
    /// returns a new note from a pitch value
    pub fn from_pitch(pitch: usize) -> Self {
        Self::new(pitch.pmod(), Octave(4))
    }
    /// returns a copy to the index of the note's class
    pub const fn class(&self) -> usize {
        self.class
    }
    /// returns a mutable reference to the index of the note's class
    pub fn class_mut(&mut self) -> &mut usize {
        &mut self.class
    }
    /// returns a copy to the octave of the note
    pub const fn octave(&self) -> Octave {
        self.octave
    }
    /// returns a mutable reference to the current octave
    pub const fn octave_mut(&mut self) -> &mut Octave {
        &mut self.octave
    }
    /// set the pitch class of the note
    pub fn set_class(&mut self, class: usize) -> &mut Self {
        self.class = class.pmod();
        self
    }
    /// set the octave of the note
    pub fn set_octave(&mut self, octave: Octave) -> &mut Self {
        self.octave = octave;
        self
    }
    /// consumes the current instance to create another with the given pitch class
    pub fn with_class(self, class: usize) -> Self {
        Self { class, ..self }
    }
    /// consumes the current instance to create another with the given octave
    pub fn with_octave(self, octave: Octave) -> Self {
        Self { octave, ..self }
    }
}

impl core::fmt::Display for Note {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}.{}", self.class, self.octave)
    }
}

impl PartialEq<usize> for Note {
    fn eq(&self, other: &usize) -> bool {
        self.class() == *other
    }
}

impl PartialEq<Note> for usize {
    fn eq(&self, other: &Note) -> bool {
        *self == other.class()
    }
}

impl PartialOrd<usize> for Note {
    fn partial_cmp(&self, other: &usize) -> Option<core::cmp::Ordering> {
        self.class().partial_cmp(other)
    }
}

impl PartialOrd<Note> for usize {
    fn partial_cmp(&self, other: &Note) -> Option<core::cmp::Ordering> {
        self.partial_cmp(&other.class())
    }
}

impl core::ops::Add<Note> for Note {
    type Output = Self;

    fn add(self, rhs: Note) -> Self::Output {
        let class = (self.class + rhs.class).pmod();
        let octave = self.octave + rhs.octave;
        
        Self::new(class, octave)
    }
}

impl core::ops::AddAssign<Note> for Note {
    fn add_assign(&mut self, rhs: Note) {
        self.class += rhs.class;
        self.octave += rhs.octave;
    }
}

impl core::ops::Add<usize> for Note {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self::new(self.class + rhs, self.octave)
    }
}

impl core::ops::AddAssign<usize> for Note {
    fn add_assign(&mut self, rhs: usize) {
        self.class = (self.class + rhs).pmod();
    }
}

impl core::ops::Sub<usize> for Note {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        let class = self.class as isize - rhs as isize;
        Self {
            class: class.pmod() as usize,
            ..self
        }
    }
}

impl core::ops::SubAssign<usize> for Note {
    fn sub_assign(&mut self, rhs: usize) {
        self.class = (self.class as isize - rhs as isize).pmod() as usize;
    }
}

macro_rules! impl_note_from {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Note {
                fn from(class: $t) -> Self {
                    Self::from_pitch(class.pmod() as usize)
                }
            }

            impl From<($t, Octave)> for Note {
                fn from((class, octave): ($t, Octave)) -> Self {
                    Self::new(class.pmod() as usize, octave)
                }
            }
        )*
    };
}

impl_note_from!(
    usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128
);
