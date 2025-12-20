/*
    appellation: notes <module>
    authors: @FL03
*/
//! this modules implements the various representations of musical notes, octaves, and pitches.
#[doc(inline)]
pub use self::{aspn::*, note_base::*};

/// an implementation of the American Scientific Pitch Notation (ASPN) for musical notes.
pub mod aspn;
mod note_base;

pub(crate) mod prelude {
    pub use super::aspn::*;
    pub use super::note_base::*;
}
