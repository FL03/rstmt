/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{
    interval::*,
    kinds::*,
    qualities::{IntervalQuality, Quality},
};

pub(crate) mod interval;
pub(crate) mod kinds;

pub mod qualities;

pub(crate) mod prelude {
    pub use super::interval::*;
    pub use super::kinds::*;
    pub use super::qualities::{IntervalQuality, Quality};
    pub use super::IntervalKind;
}

pub(crate) type IntervalTy = i8;

/// [IntervalKind] denotes objects used to explicitly define the various
/// intervals in music theory.
pub trait IntervalKind {
    /// Returns the interval associated with the value
    fn kind(&self) -> Intervals {
        Intervals::from_value(self.value())
    }
    /// Returns the value associated with the interval
    fn value(&self) -> IntervalTy;
}

/*
 ************* Implementations *************
*/
impl IntervalKind for Intervals {
    fn value(&self) -> IntervalTy {
        self.value()
    }
}
