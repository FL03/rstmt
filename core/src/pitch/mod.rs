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
    use crate::Notable;

    impl Notable for Pitch {
        fn class(&self) -> Pitches {
            self.class()
        }

        fn pitch(&self) -> Pitch {
            *self
        }
    }

    impl Notable for Pitches {
        fn class(&self) -> Pitches {
            *self
        }

        fn pitch(&self) -> Pitch {
            Pitch(self.class().pitch())
        }
    }

    impl Notable for PitchTy {
        fn pitch(&self) -> Pitch {
            Pitch(*self)
        }
    }
}
