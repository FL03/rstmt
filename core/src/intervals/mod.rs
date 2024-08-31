/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{kinds::*, types::*};

#[macro_use]
pub(crate) mod kinds;
pub(crate) mod types;

#[doc(hidden)]
pub mod interval;

pub(crate) mod prelude {
    pub use super::kinds::*;
    pub use super::types::IntervalLevel;
    pub use super::IntervalKind;
}

pub(crate) type IntervalTy = i8;

pub trait Intervallic {
    private!();

    fn as_i8(&self) -> i8;

    fn level(&self) -> IntervalLevel;

    fn kind(&self) -> Intervals {
        Intervals::from_value(self.as_i8())
    }
}

/// [IntoInterval] is a trait describing a method which consumes the current type,
/// converting it into an [Intervals]
pub trait IntoInterval {
    fn into_interval(self) -> Intervals;
}

/// [IntervalKind] denotes objects used to explicitly define the various
/// intervals in music theory.
pub trait IntervalKind {
    private!();
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

impl<I> IntoInterval for I
where
    I: Into<Intervals>,
{
    fn into_interval(self) -> Intervals {
        self.into()
    }
}

impl IntervalKind for Intervals {
    seal!();
    fn value(&self) -> IntervalTy {
        self.value()
    }
}
