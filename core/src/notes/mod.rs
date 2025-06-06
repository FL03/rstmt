/*
    appellation: notes <module>
    authors: @FL03
*/
//! this modules implements the various representations of musical notes, octaves, and pitches.
#[doc(inline)]
pub use self::{note::Note, note_base::NoteBase, types::prelude::*};

pub mod note;
pub mod note_base;

pub mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod flags;
    pub mod octave;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::flags::*;
        #[doc(inline)]
        pub use super::octave::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::note::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
    #[doc(inline)]
    pub use super::{AsNote, IntoNote};
}

/// The [`AsNote`] trait is used to convert a reference into a [`Note`].
pub trait AsNote {
    fn as_note(&self) -> Note;
}
/// A trait for converting a type into a [`Note`]
pub trait IntoNote {
    fn into_note(self) -> Note;
}

/*
    ************* Implementations *************
*/
impl<T> AsNote for T
where
    T: Clone + IntoNote,
{
    fn as_note(&self) -> Note {
        self.clone().into_note()
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
