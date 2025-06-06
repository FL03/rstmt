/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{octave::*, pair::Pair, steps::*};

pub mod octave;
pub mod pair;
pub mod steps;

///
pub type Tuple2<A = f64, B = A> = (A, B);
///
pub type Tuple3<A = f64, B = A, C = B> = (A, B, C);

pub(crate) mod prelude {
    pub use super::octave::Octave;
    pub use super::pair::Pair;
    #[cfg(feature = "std")]
    pub use super::std_types::*;
    pub use super::steps::Step;
    pub use super::{Tuple2, Tuple3};
}

#[cfg(feature = "std")]
pub(crate) mod std_types {

    ///
    pub type BoxError = Box<dyn std::error::Error>;
    ///
    pub type BoxResult<T = ()> = Result<T, BoxError>;
}
