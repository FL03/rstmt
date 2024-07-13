/*
    Appellation: pitches <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{kinds::*, pitch::Pitch};

pub(crate) mod kinds;
pub(crate) mod pitch;

pub(crate) mod prelude {
    pub use super::kinds::*;
    pub use super::{PitchClass, PitchTy};
}


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


pub trait SharpPitch {
    private!();
}

pub trait FlatPitch {
    private!();
}

pub trait AccidentalPitch: PitchClass {
    private!();
}


impl FlatPitch for Flat {
    seal!();
}

impl SharpPitch for Sharp {
    seal!();
}
impl AccidentalPitch for Sharp {
    seal!();
}

impl AccidentalPitch for Flat {
    seal!();
}

mod impl_pitches {
    use super::*;

    impl PitchT for Pitch {
        fn class(&self) -> Pitches {
            self.class()
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
