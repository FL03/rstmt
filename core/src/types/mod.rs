/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{intervals::*, octave::*, qualities::*};

pub mod intervals;
pub mod octave;
pub mod qualities;

///
pub type Tuple3<A = f64, B = A, C = B> = (A, B, C);

pub(crate) mod prelude {
    pub use super::intervals::*;
    pub use super::qualities::*;
    pub use super::{Octave, Tuple3};
}
