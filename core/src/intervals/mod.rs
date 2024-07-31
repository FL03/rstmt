/*
    Appellation: intervals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::kinds::*;

pub(crate) mod kinds;

#[doc(hidden)]
pub mod qualities;

pub(crate) mod prelude {
    pub use super::kinds::*;
    pub use super::IntervalKind;
}

pub(crate) type IntervalTy = i8;

pub trait MusicalInterval {
    fn as_i8(&self) -> i8;
}

/// [IntoInterval] is a trait describing a method which consumes the current type,
/// converting it into an [Intervals]
pub trait IntoInterval {
    fn into_interval(self) -> Interval;
}

/// [IntervalKind] denotes objects used to explicitly define the various
/// intervals in music theory.
pub trait IntervalKind {
    private!();
    /// Returns the interval associated with the value
    fn kind(&self) -> Interval {
        Interval::from_value(self.value())
    }
    /// Returns the value associated with the interval
    fn value(&self) -> IntervalTy;
}

/*
 ************* Implementations *************
*/

impl<I> IntoInterval for I
where
    I: Into<Interval>,
{
    fn into_interval(self) -> Interval {
        self.into()
    }
}

impl IntervalKind for Interval {
    seal!();
    fn value(&self) -> IntervalTy {
        self.value()
    }
}
