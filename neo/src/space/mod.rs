/*
    Appellation: space <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{
    state::{BinaryState, BinaryStates, TriadState},
    venv::TopoVenv,
};

pub mod state;
pub mod venv;

///
pub type TriadGraph = petgraph::Graph<String, u8, petgraph::Directed>;
///
pub type TriadId = String;

pub(crate) mod prelude {
    pub use super::state::{BinaryState, BinaryStates, TriadState};
    pub use super::venv::TopoVenv;
    pub use super::TriadGraph;
    pub use super::TriadId;
}
