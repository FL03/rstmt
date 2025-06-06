/*
    Appellation: convert <module>
    Contrib: @FL03
*/
use crate::{Note, Octave};

/// The [`AsNote`] trait is used to convert a reference into a [`Note`].
pub trait AsNote {
    fn as_note(&self) -> Note;
}

pub trait IntoNote {
    fn into_note(self) -> Note;
}

/// A trait for converting a reference into an octave
pub trait AsOctave {
    fn as_octave(&self) -> Octave;
}

/// A trait for converting a type into an octave
pub trait IntoOctave {
    fn into_octave(self) -> Octave;
}

/*
 ************* Implementations *************
*/
impl<T> IntoOctave for T
where
    T: Into<Octave>,
{
    fn into_octave(self) -> Octave {
        self.into()
    }
}

impl<T> IntoNote for T
where
    T: Into<Note>,
{
    fn into_note(self) -> Note {
        self.into()
    }
}

impl<T> AsOctave for T
where
    T: Clone + IntoOctave,
{
    fn as_octave(&self) -> Octave {
        self.clone().into_octave()
    }
}
