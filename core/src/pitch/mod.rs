/*
    Appellation: pitches <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{kinds::*, pitch::Pitch, utils::*};

pub(crate) mod kinds;
pub(crate) mod pitch;

/// A type alias for an integer representing a particular pitch of a note
pub type PitchTy = i8;

pub trait PitchClass {
    fn pitch(&self) -> PitchTy;
}

pub trait PitchT: Copy + Sized + core::fmt::Display {
    /// Classify the pitch into a pitch class
    fn class(&self) -> Pitches {
        self.value().into()
    }
    /// Find the modular index of the given pitch
    fn value(&self) -> PitchTy;
}

mod impl_pitches {
    use super::*;

    impl PitchT for Pitch {
        fn class(&self) -> Pitches {
            self.as_class()
        }

        fn value(&self) -> PitchTy {
            self.value()
        }
    }

    impl PitchT for Pitches {
        fn class(&self) -> Pitches {
            self.clone()
        }

        fn value(&self) -> PitchTy {
            self.class().into()
        }
    }

    impl PitchT for PitchTy {
        fn class(&self) -> Pitches {
            Pitches::try_from_value(*self).unwrap()
        }

        fn value(&self) -> PitchTy {
            *self
        }
    }
}
pub(crate) mod prelude {
    pub use super::kinds::*;
    pub use super::{PitchClass, PitchTy};
}

pub(crate) mod utils {

    /// [absmod] is short for the absolute value of a modular number;
    pub fn absmod(a: i64, m: i64) -> i64 {
        (((a % m) + m) % m).abs()
    }
}
