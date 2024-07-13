/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{builder::TriadBuilder, classes::Triads, factors::ChordFactor, state::*, triad::Triad};

pub(crate) mod builder;
pub(crate) mod triad;

pub mod classes;
pub mod factors;
pub mod state;

pub type TriadGraph = petgraph::Graph<String, u8, petgraph::Directed>;

pub type TriadId = String;

pub(crate) mod prelude {
    pub use super::builder::TriadBuilder;
    pub use super::classes::*;
    pub use super::factors::ChordFactor;
    pub use super::state::{BinaryState, BinaryStates, TriadState};
    pub use super::triad::Triad;
    pub use super::{TriadGraph, TriadId};

}