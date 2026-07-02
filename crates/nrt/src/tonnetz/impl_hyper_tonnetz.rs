/*
    Appellation: impl_hyper_tonnetz <module>
    Created At: 2025.12.28:10:45:40
    Contrib: @FL03
*/
use crate::tonnetz::{EdgeMap, HyperTonnetz, LprMap, NoteGraph};
use crate::traits::{TriadRepr, TriadType};
use crate::triad::TriadBase;
use core::hash::Hash;
use hashbrown::HashMap;
use num_traits::FromPrimitive;
use rshyper::{AddStep, EdgeId, RawIndex, VertexId, Weight};
use rstmt_core::Octave;

impl<S, K, T, Ix> HyperTonnetz<S, K, T, Ix>
where
    K: TriadType,
    S: TriadRepr<Elem = T>,
    Ix: RawIndex,
{
    /// initialize a new, empty instance of the [`HyperTonnetz`]
    pub fn new() -> Self
    where
        Ix: Default,
    {
        HyperTonnetz {
            graph: NoteGraph::new(),
            triads: EdgeMap::new(),
            transformations: LprMap::new(),
        }
    }
    /// returns a new instance of the [`HyperTonnetz`] with a specified capacity
    pub fn with_capacity(capacity: usize) -> Self
    where
        Ix: Default,
    {
        // each edge has n vertices meaning we need to reserve space for n^2 edges
        HyperTonnetz {
            graph: NoteGraph::with_capacity(capacity * capacity, capacity),
            triads: HashMap::with_capacity(capacity),
            transformations: HashMap::new(),
        }
    }
    /// returns a reference to the underlying graph
    pub const fn graph(&self) -> &NoteGraph<T, (), Ix> {
        &self.graph
    }
    /// returns a mutable reference to the underlying graph
    pub const fn graph_mut(&mut self) -> &mut NoteGraph<T, (), Ix> {
        &mut self.graph
    }
    /// returns a reference to the triads map
    pub const fn triads(&self) -> &EdgeMap<S, K, T, Ix> {
        &self.triads
    }
    /// returns a mutable reference to the triads map
    pub const fn triads_mut(&mut self) -> &mut EdgeMap<S, K, T, Ix> {
        &mut self.triads
    }
    /// returns a reference to the transformations map
    pub const fn transformations(&self) -> &LprMap<Ix> {
        &self.transformations
    }
    /// returns a mutable reference to the transformations map
    pub const fn transformations_mut(&mut self) -> &mut LprMap<Ix> {
        &mut self.transformations
    }
    #[inline]
    /// clears the current instance, removing any existing data
    pub fn clear(&mut self)
    where
        Ix: Default + Eq + Hash,
    {
        self.graph_mut().clear();
        self.triads_mut().clear();
        self.transformations_mut().clear();
    }
    /// update the current graph with the given instance
    pub fn set_graph(&mut self, graph: NoteGraph<T, (), Ix>) {
        self.graph = graph
    }
    #[inline]
    /// update the triad map
    pub fn set_triads(&mut self, triads: EdgeMap<S, K, T, Ix>) {
        self.triads = triads
    }
    #[inline]
    /// overwrite the current transformations and return a mutable reference to the instance
    pub fn set_transformations(&mut self, transformations: LprMap<Ix>) {
        self.transformations = transformations
    }
    /// add a new note class vertex to the Tonnetz
    pub fn add_note(&mut self, note: T) -> crate::Result<VertexId<Ix>>
    where
        Ix: Copy + Eq + Hash + AddStep<Output = Ix>,
    {
        self.graph_mut()
            .add_node(Weight(note))
            .map_err(|e| e.into())
    }
    /// adds each note within the iterator to the Tonnetz
    pub fn add_notes<I>(&mut self, notes: I) -> Vec<VertexId<Ix>>
    where
        I: IntoIterator<Item = T>,
        Ix: Copy + Eq + Hash + AddStep<Output = Ix>,
    {
        notes
            .into_iter()
            .filter_map(|note| self.add_note(note).ok())
            .collect::<Vec<_>>()
    }
    /// returns a reference to the triad associated with the given edge index
    pub fn get_triad<Q>(&self, key: &Q) -> Option<&TriadBase<S, K, T>>
    where
        Q: Eq + Hash,
        Ix: Eq + Hash,
        EdgeId<Ix>: core::borrow::Borrow<Q>,
    {
        self.triads().get(key)
    }
    /// returns a mutable reference to the triad associated with the given edge index
    pub fn get_triad_mut<Q>(&mut self, key: &Q) -> Option<&mut TriadBase<S, K, T>>
    where
        Q: Eq + Hash,
        Ix: Eq + Hash,
        EdgeId<Ix>: core::borrow::Borrow<Q>,
    {
        self.triads_mut().get_mut(key)
    }
}
/// private methods supporting the [`HyperTonnetz`] structure
impl<S, K, T, Ix> HyperTonnetz<S, K, T, Ix>
where
    K: TriadType,
    S: TriadRepr<Elem = T>,
    T: Eq + Hash,
    Ix: Copy + Eq + Hash + RawIndex,
{
    /// initialize a complete layer of the Tonnetz at the given octave
    pub fn scaffold_layer(&mut self, octave: Octave) -> crate::Result<Vec<VertexId<Ix>>>
    where
        Ix: AddStep<Output = Ix>,
        T: Copy + FromPrimitive + core::ops::Add<Output = T>,
    {
        let octave = T::from_isize(octave.value()).unwrap();
        // create an iterator over all 12 notes within the given octave
        let iter = (0..12).map(|i| T::from_usize(i).unwrap() + octave);
        // use the iterator to insert all the notes into the Tonnetz
        Ok(self.add_notes(iter))
    }
    /// Add a new triad to the Tonnetz
    pub fn add_triad(&mut self, triad: TriadBase<S, K, T>) -> crate::Result<EdgeId<Ix>>
    where
        Ix: AddStep<Output = Ix>,
        T: Copy,
        S: Clone + IntoIterator<Item = T>,
    {
        // Ensure we have vertices for all Note classes
        let vertices: Vec<VertexId<Ix>> = triad
            .chord()
            .clone()
            .into_iter()
            .map(|p| {
                // Try to find existing vertex with this Note class
                if let Some(v) = self.find_vertex_by_note(p) {
                    v
                } else {
                    // Add new vertex if not found
                    self.add_note(p).expect("Failed to add note")
                }
            })
            .collect();

        // Add the hyperedge representing this triad
        let edge_id = self
            .graph_mut()
            .add_link(vertices)
            .expect("Failed to add hyperedge");
        // Store the triad data
        self.triads.insert(edge_id, triad);

        Ok(edge_id)
    }
    /// Find a vertex by its associated pitch class
    pub(crate) fn find_vertex_by_note<Q>(&self, note: Q) -> Option<VertexId<Ix>>
    where
        Weight<T>: PartialEq<Q>,
    {
        self.graph
            .nodes()
            .iter()
            .find(|(_, node)| *node.weight() == note)
            .map(|(id, _)| *id)
    }
}

impl HyperTonnetz {
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
}

/// private methods supporting the [`HashTonnetz`] structure
impl HyperTonnetz {}

impl Default for HyperTonnetz {
    fn default() -> Self {
        Self::new()
    }
}
