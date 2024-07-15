/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{intervals::*, octave::*};

pub mod intervals;
pub mod octave;

///
pub type Tuple3<A = f64, B = A, C = B> = (A, B, C);

pub(crate) mod prelude {
    pub use super::intervals::*;
    pub use super::{Octave, Tuple3};
}
