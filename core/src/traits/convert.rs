/*
    Appellation: convert <module>
    Contrib: @FL03
*/
use crate::{Note, Octave, Pitch};

/// The [`AsNote`] trait is used to convert a reference into a [`Note`].
pub trait AsNote {
    fn as_note(&self) -> Note;
}
/// A trait for converting a type into a [`Note`]
pub trait IntoNote {
    fn into_note(self) -> Note;
}
/// A trait for converting a reference into an [`Octave`].
pub trait AsOctave {
    fn as_octave(&self) -> Octave;
}
/// A trait for converting a type into an [`Octave`].
pub trait IntoOctave {
    fn into_octave(self) -> Octave;
}
/// A trait for converting a reference into a [`Pitch`].
pub trait AsPitch {
    fn as_pitch(&self) -> Pitch;
}
/// A trait for converting a type into a [`Pitch`].
pub trait IntoPitch {
    fn into_pitch(self) -> Pitch;
}

/*
 ************* Implementations *************
*/
impl<T> IntoNote for T
where
    T: Into<Note>,
{
    fn into_note(self) -> Note {
        self.into()
    }
}

impl<T> IntoOctave for T
where
    T: Into<Octave>,
{
    fn into_octave(self) -> Octave {
        self.into()
    }
}

impl<T> IntoPitch for T
where
    T: Into<Pitch>,
{
    fn into_pitch(self) -> Pitch {
        self.into()
    }
}

impl<T> AsNote for T
where
    T: Clone + IntoNote,
{
    fn as_note(&self) -> Note {
        self.clone().into_note()
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

impl<T> AsPitch for T
where
    T: Clone + IntoPitch,
{
    fn as_pitch(&self) -> Pitch {
        self.clone().into_pitch()
    }
}
