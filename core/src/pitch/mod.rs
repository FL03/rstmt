/*
    Appellation: pitches <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//!
//! 
//! ### Pitch Classes
//! 
//! Every pitch class has a natural reprsentation and depending on the scale, may have either a
//! sharp or flat representation and in some cases, both. The following table shows the 
//! assigned value for the natural pitch classes:
//! 
//! C = 0
//! D = 2
//! E = 4
//! F = 5
//! G = 7
//! A = 9
//! B = 11
//! 
//! If there is 'space' between two natural pitch classes than each class will have a sharp and
//! flat representation, respectively. For example, the pitches C# and Db are enharmonic;
//! C# is a semitone above `C` while Db is a half-step below its natural counterpart. 
//! 
//! Using this logic allows for 17 unique pitches to represent 12 different tones.
//! The definition above allows for 17 unique pitches to represent 12 different tones. 
//! 
#[doc(inline)]
pub use self::{kinds::*, pitch::Pitch, types::prelude::*};

pub(crate) mod kinds;
pub(crate) mod pitch;

pub mod types {
    #[doc(inline)]
    pub use self::signs::*;

    pub mod signs;
    
    #[doc(hidden)]
    pub(crate) mod prelude {
        pub use super::signs::*;
    }
}

mod impls {
    mod pitch_ops;
}

pub(crate) mod prelude {
    pub use super::kinds::{Flat, Natural, Pitches, Sharp};
    pub use super::pitch::Pitch;
    pub use super::{IntoPitch, PitchClass, PitchTy};
}

lazy_static::lazy_static! {
    static ref FLAT_SYMBOLS: [char; 3] = ['♭', 'f', 'F'];
    static ref SHARP_SYMBOLS: [char; 3] = ['#', 's', 'S'];
}
/// A type alias for an integer representing a particular pitch of a note
pub type PitchTy = i8;
/// A trait for converting a type into a [Pitch](Pitch) instance.
pub trait IntoPitch {
    fn into_pitch(self) -> Pitch;
}

pub trait PitchT {
    private!();

}
/// Used to denote a particular pitch class; pitch classes are symbolic
/// representations of pre-defined frequencies.
pub trait PitchClass {
    private!();

    fn pitch(&self) -> PitchTy;
}

/*
 ************* Implementations *************
*/
impl<S> IntoPitch for S
where
    S: Into<Pitch>,
{
    fn into_pitch(self) -> Pitch {
        self.into()
    }
}

macro_rules! impl_pitch_t {
    (@impl $t:ty) => {
        impl PitchT for $t {
            seal!();
        }
    };
    ($($t:ty),* $(,)?) => {
        $(impl_pitch_t!(@impl $t);)*
    };
}

impl_pitch_t!(i8);