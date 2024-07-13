/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{TriadId, TriadGraph,};
use super::state::{BinaryState, TriadState};

#[derive(Clone)]
pub struct Triad {
    pub(crate) id: TriadId,
    pub(crate) chord: TriadGraph,
    pub(crate) state: BinaryState<TriadState>,
    
}

impl Triad {
    pub fn new(id: TriadId) -> Self {
        let chord = TriadGraph::new();
        let state = BinaryState::default();
        Self {
            id,
            chord,
            state,
            
        }
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
