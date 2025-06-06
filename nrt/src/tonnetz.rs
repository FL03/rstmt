/*
    Appellation: tonnetz <module>
    Contrib: @FL03
*/
use crate::{LPR, Triad, utils};
use rshyper::prelude::{EdgeId, HashGraph, VertexId};
use rstmt::{Note, Octave};
use std::collections::HashMap;

/// The tonnetz is a representation of tonal space in-which every facet is a valid triad.
/// Here, we use the tonnetz to define the topology of the runtime as well as the cluster.
/// Each instance of the runtime orchestrates a _fragment_ of the Tonnetz and glues it to the
/// cluster with various networking protocols.
#[derive(Clone, Debug)]
pub struct Tonnetz {
    /// The underlying hypergraph structure
    pub(crate) graph: HashGraph<Note>,
    /// Maps EdgeIds to Triad for efficient access
    pub(crate) triads: HashMap<EdgeId, Triad>,
    /// Tracks adjacency between triads via transformations
    pub(crate) transformations: HashMap<EdgeId, HashMap<LPR, EdgeId>>,
}

impl Default for Tonnetz {
    fn default() -> Self {
        Self::new()
    }
}

impl Tonnetz {
    /// returns a new [`Tonnetz`] structure initialized with empty stores
    pub fn new() -> Self {
        Tonnetz {
            graph: HashGraph::new(),
            triads: HashMap::new(),
            transformations: HashMap::new(),
        }
    }
    /// returns a new instance of the [`Tonnetz`] with a specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        // each edge has n vertices meaning we need to reserve space for n^2 edges
        Tonnetz {
            graph: HashGraph::with_capacity(capacity * capacity, capacity),
            triads: HashMap::with_capacity(capacity),
            transformations: HashMap::new(),
        }
    }
    /// returns a reference to the underlying graph
    pub const fn graph(&self) -> &HashGraph<Note> {
        &self.graph
    }
    /// returns a mutable reference to the underlying graph
    pub const fn graph_mut(&mut self) -> &mut HashGraph<Note> {
        &mut self.graph
    }
    /// Add a new note class vertex to the Tonnetz
    pub fn add_note(&mut self, note: Note) -> crate::Result<VertexId> {
        let id = self.graph_mut().add_node(note)?;
        Ok(id)
    }
    /// Add a new triad to the Tonnetz
    pub fn add_triad(&mut self, triad: Triad) -> crate::Result<EdgeId> {
        // Ensure we have vertices for all pitch classes
        let vertices: Vec<VertexId> = triad
            .notes()
            .iter()
            .map(|&p| {
                // Try to find existing vertex with this pitch class
                if let Some(v) = self.find_vertex_by_pitch(p) {
                    v
                } else {
                    // Add new vertex if not found
                    self.add_note(Note::from_pitch(p))
                        .expect("Failed to add note")
                }
            })
            .collect();

        // Add the hyperedge representing this triad
        let edge_id = self
            .graph
            .add_edge(vertices)
            .expect("Failed to add hyperedge");
        // Store the triad data
        self.triads.insert(edge_id, triad);

        Ok(edge_id)
    }
    /// initialize a complete layer of the Tonnetz at the given octave
    pub fn scaffold_layer(&mut self, Octave(octave): Octave) -> crate::Result<Vec<VertexId>> {
        // iterate over all pitch classes in the octave
        let res = (0..12)
            .filter_map(|i| {
                let note = Note::new(i, Octave(octave));
                self.add_note(note).ok()
            })
            .collect::<Vec<_>>();
        Ok(res)
    }
    /// Get a triad by its edge id
    pub fn get_triad(&self, edge_id: EdgeId) -> Option<&Triad> {
        self.triads.get(&edge_id)
    }

    /// Compute and store all possible transformations between triads
    pub fn compute_transformations(&mut self) {
        // Clear existing transformations
        self.transformations.clear();

        // For each pair of triads, check if there's a transformation between them
        let edges: Vec<EdgeId> = self.triads.keys().cloned().collect();

        for &edge1 in &edges {
            self.transformations.insert(edge1, HashMap::new());

            if let Some(a) = self.triads.get(&edge1) {
                for &edge2 in &edges {
                    if edge1 == edge2 {
                        continue;
                    }

                    if let Some(b) = self.triads.get(&edge2) {
                        // Check if there's a transformation from triad1 to triad2
                        if let Some(transform) = utils::get_transformation(a, b) {
                            self.transformations
                                .get_mut(&edge1)
                                .unwrap()
                                .insert(transform, edge2);
                        }
                    }
                }
            }
        }
    }
    /// Find a vertex by its pitch class value
    fn find_vertex_by_pitch(&self, pitch: usize) -> Option<VertexId> {
        self.graph
            .nodes()
            .iter()
            .find(|(_, node)| node.weight().class() == pitch)
            .map(|(id, _)| *id)
    }
}
