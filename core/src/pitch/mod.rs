/*
    Appellation: pitches <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{kinds::*, pitch::Pitch};

pub(crate) mod kinds;
pub(crate) mod pitch;

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

pub enum SymbolCount {
    Double = 2,
    Single = 1,
}
pub struct FlatSymbol(SymbolCount);

pub struct SharpSym(SymbolCount);

impl SharpSym {
    pub fn symbol(&self) -> &str {
        match self.0 {
            SymbolCount::Double => "♯♯",
            SymbolCount::Single => "♯",
        }
    }
}

impl FlatSymbol {
    pub fn symbol(&self) -> &str {
        match self.0 {
            SymbolCount::Double => "♭♭",
            SymbolCount::Single => "♭",
        }
    }
}
