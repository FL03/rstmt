/*
    appellation: pitch <module>
    authors: @FL03
*/
//! this module implements various pitch-related items used throughout the library. Some of the
//! key abstractions defined here are the [`Pitch`] and [`PitchClass`] implementations designed
//! to generically represent these musical concepts. The [`PitchClass`] isn't necessarily
//! intended to be used directly, rather through type aliases such as [`C`], [`DSharp`],
//! [`EFlat`], etc. Additionally, the module provides various traits, types, and other
//! implementations aimed at facilitating the manipulation and representation of pitches in a
//! musical context.
#[doc(inline)]
pub use self::{pitch::*, pitch_class::*, traits::*, types::*};
// modules
mod pitch;
mod pitch_class;

mod impls {
    mod impl_pitch;
    mod impl_pitch_ext;
    mod impl_pitch_rand;
    mod impl_pitch_repr;

    mod impl_pclass;
    mod impl_pclass_ext;
    mod impl_pclass_ops;
}

mod traits {
    #[doc(inline)]
    pub use self::{accidental::*, classifiers::*, raw_pitch::*};

    mod accidental;
    mod classifiers;
    mod raw_pitch;
}

mod types {
    #[doc(inline)]
    pub use self::{class_enums::*, flags::*, pitch_reprs::*};

    mod class_enums;
    mod flags;
    mod pitch_reprs;
}
// prelude (local)
pub(crate) mod prelude {
    pub use super::pitch::*;
    pub use super::pitch_class::*;
    pub use super::traits::*;
    pub use super::types::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_class() {
        let c = C::new();
        // verify the type checkers
        assert! { c.is_natural() && !c.is_flat() && !c.is_sharp() }
        // check the index
        assert_eq! { c, 0 }
    }
}
