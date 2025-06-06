/*
    appellation: notes <module>
    authors: @FL03
*/
//! this modules implements the various representations of musical notes, octaves, and pitches.
#[doc(inline)]
pub use self::{aspn::Aspn, note_base::NoteBase, types::prelude::*};

pub mod aspn;
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
    pub use super::aspn::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
    #[doc(inline)]
    pub use super::{AsAspn, IntoAspn};
}

/// The [`AsNote`] trait is used to convert a reference into a [`Note`].
pub trait AsAspn {
    fn as_aspn(&self) -> Aspn;
}
/// A trait for converting a type into a [`Note`]
pub trait IntoAspn {
    fn into_aspn(self) -> Aspn;
}

/*
 ************* Implementations *************
*/
impl<T> AsAspn for T
where
    T: Clone + IntoAspn,
{
    fn as_aspn(&self) -> Aspn {
        self.clone().into_aspn()
    }
}

impl<T> IntoAspn for T
where
    T: Into<Aspn>,
{
    fn into_aspn(self) -> Aspn {
        self.into()
    }
}
