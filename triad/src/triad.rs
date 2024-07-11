/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{builder::TriadBuilder, classes::*, factors::*, state::*};

pub(crate) mod builder;
pub(crate) mod classes;
pub(crate) mod factors;
pub(crate) mod state;

pub type TriadGraph = petgraph::Graph<String, u8, petgraph::Directed>;

pub type TriadId = String;

#[derive(Clone)]
pub struct Triad {
    pub(crate) id: TriadId,
    pub(crate) state: BinaryState<TriadState>,
    pub(crate) store: TriadGraph,
}

impl Triad {
    pub fn new(id: TriadId) -> Self {
        Self {
            id,
            state: BinaryState::default(),
            store: TriadGraph::new(),
        }
    }
}
