/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{octave::*, qualities::*, steps::*};

pub mod octave;
pub mod qualities;
pub mod steps;

///
pub type Tuple3<A = f64, B = A, C = B> = (A, B, C);

pub(crate) mod prelude {
    pub use super::octave::Octave;
    pub use super::qualities::*;
    pub use super::steps::*;
    pub use super::Tuple3;
}
