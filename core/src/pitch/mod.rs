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
    pub use super::{Accidentals, PitchClass, PitchTy};
}

/// A type alias for an integer representing a particular pitch of a note
pub type PitchTy = i8;

pub trait IntoPitch {
    fn into_pitch(self) -> Pitch;
}

/// Used to denote a particular pitch class; pitch classes are symbolic
/// representations of pre-defined frequencies.
pub trait PitchClass {
    private!();

    fn pitch(&self) -> PitchTy;
}

pub trait Sharps {
    private!();
}

pub trait Flats {
    private!();
}

pub trait Accidentals: PitchClass {
    private!();
}

/*
 ************* Implementations *************
*/
impl<S> IntoPitch for S where S: Into<Pitch> {
    fn into_pitch(self) -> Pitch {
        self.into()
    }
}

impl Flats for Flat {
    seal!();
}

impl Sharps for Sharp {
    seal!();
}
impl Accidentals for Sharp {
    seal!();
}

impl Accidentals for Flat {
    seal!();
}
