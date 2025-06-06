/*
    appellation: notes <module>
    authors: @FL03
*/
//! this modules implements the various representations of musical notes, octaves, and pitches.
#[doc(inline)]
pub use self::prelude::*;

pub mod note;
pub mod octave;
pub mod pitch;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::note::*;
    #[doc(inline)]
    pub use super::octave::*;
    #[doc(inline)]
    pub use super::pitch::*;
}
