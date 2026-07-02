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
    mod impl_pclass_repr;
}

mod traits {
    #[doc(inline)]
    pub use self::{accidental::*, classifiers::*};

    mod accidental;
    mod classifiers;
}

mod types {
    #[doc(inline)]
    pub use self::pitch_reprs::*;

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
        let c = C::default();
        // verify the type checkers
        assert! { c.is_natural() && !c.is_flat() && !c.is_sharp() }
        // check the index
        assert_eq! { c, 0 }
    }

    #[test]
    fn test_parse_pitch_class() {
        let d_sharp: DSharp = "D#".parse().unwrap();
        assert! { d_sharp.is_sharp() }
        assert_eq! { d_sharp.index(), 3 }

        let e_flat: EFlat = "Eb".parse().unwrap();
        assert! { e_flat.is_flat() }
        assert_eq! { e_flat.index(), 3 }

        let f: F = "F".parse().unwrap();
        assert! { f.is_natural() }
        assert_eq! { f.index(), 5 }
    }
}
