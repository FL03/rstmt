/*
    appellation: notes <module>
    authors: @FL03
*/
//! this modules implements the various representations of musical notes, octaves, and pitches.
#[doc(inline)]
pub use self::{aspn::Aspn, note_base::NoteBase, traits::*, types::*};

/// an implementation of the American Scientific Pitch Notation (ASPN) for musical notes.
pub mod aspn;
mod note_base;

mod traits {
    //! this module provides additional traits for the [`notes`](crate::notes) module.
    #[doc(inline)]
    pub use self::prelude::*;

    mod convert;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
    }
}

mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    mod flags;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::flags::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::aspn::*;
    #[doc(inline)]
    pub use super::traits::*;
    #[doc(inline)]
    pub use super::types::*;
}
