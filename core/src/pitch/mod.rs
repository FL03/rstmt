/*
    appellation: pitch <module>
    authors: @FL03
*/
//! this module implements the [`Pitch`] type and its associated traits and types.
#[doc(inline)]
pub use self::{pitch_class::*, pitches::*, traits::*, types::*};
// modules
mod pitch_class;
mod pitches;

mod impls {
    mod impl_pitch;
    mod impl_pitch_ext;
    mod impl_pitch_rand;
    mod impl_pitch_repr;

    mod impl_pitch_class;
    mod impl_pitch_classified;
}

mod traits {
    #[doc(inline)]
    pub use self::{convert::*, raw_pitch::*};

    mod convert;
    mod raw_pitch;
}

mod types {
    #[doc(inline)]
    pub use self::{classes::*, kinds::*};

    mod classes;
    mod kinds;
}
// prelude (local)
pub(crate) mod prelude {
    pub use super::pitch_class::*;
    pub use super::pitches::*;
    pub use super::traits::*;
    pub use super::types::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_class() {
        let c = C::default();
        assert_eq!(c.index(), 0);
    }
}
