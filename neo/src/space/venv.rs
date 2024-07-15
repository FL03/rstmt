/*
    Appellation: store <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::state::{BinaryState, TriadState};
use super::{TriadGraph, TriadId};

#[derive(Clone)]
pub struct TopoVenv {
    pub(crate) id: TriadId,
    pub(crate) chord: TriadGraph,
    pub(crate) state: BinaryState<TriadState>,
}

impl TopoVenv {
    pub fn new(id: TriadId) -> Self {
        let chord = TriadGraph::new();
        let state = BinaryState::default();
        Self { id, chord, state }
    }

    pub fn id(&self) -> &TriadId {
        &self.id
    }

    pub fn is_valid(&self) -> bool {
        self.state().is_valid()
    }

    pub fn chord(&self) -> &TriadGraph {
        &self.chord
    }

    pub const fn state(&self) -> &BinaryState<TriadState> {
        &self.state
    }
}
