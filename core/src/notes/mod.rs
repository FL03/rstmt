/*
    Appellation: notes <module>
    Created At: 2026.01.20:08:56:38
    Contrib: @FL03
*/
//! this module contains various implementations and traits for defining and manipulating
//! musical notes. The [`NoteBase`] struct provides a generic representation of a musical note
//! that is parameterized by a pitch class and an octave.
#[doc(inline)]
pub use self::{aspn::*, note_base::*, types::*};

mod aspn;
mod note_base;

mod impls {
    mod impl_aspn;

    mod impl_note_base;
    mod impl_note_ext;
    mod impl_note_repr;
}

mod traits {
    // #[doc(inline)]
    // pub use self::*;
}

mod types {
    #[doc(inline)]
    pub use self::{accents::*, notes::*};

    mod accents;
    mod notes;
}

pub(crate) mod prelude {
    pub use super::note_base::*;
    pub use super::types::*;
}

#[cfg(test)]
mod tests {
    use super::NoteBase;
    use crate::octave::Octave;
    use crate::pitch::{C, CNote};

    #[test]
    fn test_note_init() {
        assert_eq! { NoteBase::<CNote>::from_octave(Octave(4)), "C.4" }
    }

    #[test]
    fn test_note_parse_from_str() {
        let exp = NoteBase::new(C::default(), Octave(4));
        assert_eq! { "C.4".parse::<NoteBase<_, _>>().unwrap(), exp }
    }
}
