/*
    Appellation: notes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::note::*;

pub(crate) mod impls {
    pub mod impl_note_ops;
}
pub(crate) mod note;

pub(crate) mod prelude {
    pub use super::note::*;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Octave, Pitch, Pitches};

    #[test]
    fn test_note() {
        let note = Note::from_pitch(0);
        // check the note's octave
        assert_eq!(note.octave(), &Octave(4));
        // check the note's pitch
        assert_eq!(note.pitch(), &Pitch(0));
        // check the note's pitch class
        assert_eq!(note.class(), Pitches::c());
        // assert the note's string representation; aspn: C.4
        assert_eq!(note.to_string(), "C.4");
    }
}
