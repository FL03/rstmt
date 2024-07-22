/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::machine::TuringMachine;

pub(crate) mod machine;
pub mod utm;

pub(crate) mod prelude {
    pub use super::machine::TuringMachine;
    pub use super::utm::*;
}

use crate::prelude::{Direction, State};
///
pub type Registry<Q = String> =
    std::collections::HashMap<(State<Q>, char), (State<Q>, char, Direction)>;

pub trait Turing {}

pub trait Alphabet {
    fn symbols(&self) -> Vec<char>;
}
