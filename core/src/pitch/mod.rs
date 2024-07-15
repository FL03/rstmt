/*
    Appellation: pitches <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{kinds::*, pitch::Pitch};

pub(crate) mod kinds;
pub(crate) mod pitch;

pub(crate) mod prelude {
    pub use super::kinds::{Flat, Natural, Pitches, Sharp};
    pub use super::pitch::Pitch;
    pub use super::{AccidentalPitch, PitchClass, PitchTy};
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

/*
 ************* Implementations *************
*/
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
