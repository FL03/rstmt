/*
    Appellation: tonnetz <module>
    Contrib: @FL03
*/
use crate::{LPR, Triad};
use rshyper::idx::{EdgeId, VertexId};
use rshyper::{HyperMap, Weight};
use rstmt::{Aspn, Octave};
use std::collections::HashMap;

/// a type alias for a [`HashMap`] that maps an [`EdgeId`] to a [`Triad`]
pub(crate) type TriadMap<I = usize> = HashMap<EdgeId<I>, Triad>;
/// a type alias for a [`HashMap`] that maps an [`EdgeId`] to a [`HashMap`] of [`LPR`]
/// transformations.
pub(crate) type LprMap<I = usize> = HashMap<EdgeId<I>, HashMap<LPR, EdgeId<I>>>;

/// The tonnetz is a representation of tonal space in-which every facet is a valid triad.
/// Here, we use the tonnetz to define the topology of the runtime as well as the cluster.
/// Each instance of the runtime orchestrates a _fragment_ of the Tonnetz and glues it to the
/// cluster with various networking protocols.
#[derive(Clone, Debug)]
pub struct HyperTonnetz {
    /// The underlying hypergraph structure
    pub(crate) graph: HyperMap<Aspn>,
    /// Maps EdgeIds to Triad for efficient access
    pub(crate) triads: TriadMap,
    /// Tracks adjacency between triads via transformations
    pub(crate) transformations: LprMap,
}

impl Default for HyperTonnetz {
    fn default() -> Self {
        Self::new()
    }
}

impl HyperTonnetz {
    /// returns a new [`Tonnetz`] structure initialized with empty stores
    pub fn new() -> Self {
        HyperTonnetz {
            graph: HyperMap::new(),
            triads: TriadMap::new(),
            transformations: LprMap::new(),
        }
    }
    /// returns a new instance of the [`Tonnetz`] with a specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        // each edge has n vertices meaning we need to reserve space for n^2 edges
        HyperTonnetz {
            graph: HyperMap::with_capacity(capacity * capacity, capacity),
            triads: HashMap::with_capacity(capacity),
            transformations: HashMap::new(),
        }
    }
    /// returns a reference to the underlying graph
    pub const fn graph(&self) -> &HyperMap<Aspn> {
        &self.graph
    }
    /// returns a mutable reference to the underlying graph
    pub const fn graph_mut(&mut self) -> &mut HyperMap<Aspn> {
        &mut self.graph
    }
    /// returns a reference to the triads map
    pub const fn triads(&self) -> &TriadMap {
        &self.triads
    }
    /// returns a mutable reference to the triads map
    pub fn triads_mut(&mut self) -> &mut TriadMap {
        &mut self.triads
    }
    /// returns a reference to the transformations map
    pub const fn transformations(&self) -> &LprMap {
        &self.transformations
    }
    /// returns a mutable reference to the transformations map
    pub fn transformations_mut(&mut self) -> &mut LprMap {
        &mut self.transformations
    }
    /// overwrite the current graph and return a mutable reference to the instance
    pub fn set_graph(&mut self, graph: HyperMap<Aspn>) -> &mut Self {
        self.graph = graph;
        self
    }
    /// overwrite the current triads and return a mutable reference to the instance
    pub fn set_triads(&mut self, triads: TriadMap) -> &mut Self {
        self.triads = triads;
        self
    }
    /// overwrite the current transformations and return a mutable reference to the instance
    pub fn set_transformations(&mut self, transformations: LprMap) -> &mut Self {
        self.transformations = transformations;
        self
    }
    /// add a new note class vertex to the Tonnetz
    pub fn add_note(&mut self, note: Aspn) -> crate::Result<VertexId> {
        let id = self.graph_mut().add_node(Weight(note))?;
        Ok(id)
    }
    /// adds each note within the iterator to the Tonnetz
    pub fn add_notes<I>(&mut self, notes: I) -> Vec<VertexId>
    where
        I: IntoIterator<Item = Aspn>,
    {
        notes
            .into_iter()
            .filter_map(|note| self.add_note(note).ok())
            .collect::<Vec<_>>()
    }
    /// Add a new triad to the Tonnetz
    pub fn add_triad(&mut self, triad: Triad) -> crate::Result<EdgeId> {
        // Ensure we have vertices for all Note classes
        let vertices: Vec<VertexId> = triad
            .notes()
            .iter()
            .map(|&p| {
                // Try to find existing vertex with this Note class
                if let Some(v) = self.find_vertex_by_note(p) {
                    v
                } else {
                    // Add new vertex if not found
                    self.add_note(Aspn::from_pitch(p))
                        .expect("Failed to add note")
                }
            })
            .collect();

        // Add the hyperedge representing this triad
        let edge_id = self
            .graph
            .add_link(vertices)
            .expect("Failed to add hyperedge");
        // Store the triad data
        self.triads.insert(edge_id, triad);

        Ok(edge_id)
    }
    /// returns a reference to the triad associated with the given edge index
    pub fn get_triad(&self, edge_id: EdgeId) -> Option<&Triad> {
        self.triads().get(&edge_id)
    }
    /// returns a mutable reference to the triad associated with the given edge index
    pub fn get_triad_mut(&mut self, edge_id: EdgeId) -> Option<&mut Triad> {
        self.triads_mut().get_mut(&edge_id)
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

                    if let Some(b) = self.triads().get(&edge2) {
                        // Check if there's a transformation from triad1 to triad2
                        if let Some(transform) = a.is_neighbor(b) {
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
    /// initialize a complete layer of the Tonnetz at the given octave
    pub fn scaffold_layer(&mut self, octave: Octave) -> crate::Result<Vec<VertexId>> {
        // create an iterator over all 12 notes within the given octave
        let iter = (0..12).map(|i| Aspn::new(i, octave));
        // use the iterator to insert all the notes into the Tonnetz
        let res = self.add_notes(iter);
        // return the result
        Ok(res)
    }
}

/// private methods supporting the [`HashTonnetz`] structure
impl HyperTonnetz {
    /// Find a vertex by its associated pitch class
    pub(crate) fn find_vertex_by_note(&self, note: usize) -> Option<VertexId> {
        self.graph
            .nodes()
            .iter()
            .find(|(_, node)| node.weight().class() == note)
            .map(|(id, _)| *id)
    }
}
