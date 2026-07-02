/*
    Appellation: path <module>
    Contrib: @FL03
*/
use super::{ChainFeatures, SearchNode};
use crate::triad::Triad;
use crate::types::{LPR, Triads};
use rshyper::EdgeId;

/// Represents a sequence of transformations from one triad to another
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Path<T = usize> {
    /// Musical cost or distance metric (lower is better)
    pub(crate) cost: usize,
    /// Edge IDs in the tonnetz (if available)
    pub(crate) edges: Vec<Option<EdgeId>>,
    /// Path features for musical analysis
    pub(crate) features: ChainFeatures,
    /// The sequence of transformations to apply
    pub(crate) transforms: Vec<LPR>,
    /// The sequence of triads visited
    pub(crate) triads: Vec<Triad<Triads, T>>,
}
impl<T> Path<T> {
    pub fn new(
        transforms: Vec<LPR>,
        triads: Vec<Triad<Triads, T>>,
        edges: Vec<Option<EdgeId>>,
    ) -> Self {
        Path {
            transforms,
            triads,
            edges,
            cost: 0,
            features: ChainFeatures::default(),
        }
    }

    pub fn from_triads(triads: Vec<Triad<Triads, T>>) -> Self {
        Path {
            transforms: Vec::new(),
            triads,
            edges: Vec::new(),
            cost: 0,
            features: ChainFeatures::default(),
        }
    }

    pub fn from_node_with_features(node: SearchNode<T>, features: ChainFeatures) -> Self {
        let SearchNode {
            cost,
            edges: edge_ids,
            transforms,
            visited: triads,
            ..
        } = node;
        Path {
            transforms,
            triads,
            edges: edge_ids,
            cost,
            features,
        }
    }
    /// returns a copy of the path's cost
    pub const fn cost(&self) -> usize {
        self.cost
    }
    /// returns a mutable reference to the path's cost
    pub const fn cost_mut(&mut self) -> &mut usize {
        &mut self.cost
    }
    /// returns a reference to the path's edge IDs
    pub const fn edges(&self) -> &Vec<Option<EdgeId>> {
        &self.edges
    }
    /// returns a mutable reference to the path's edge IDs
    pub const fn edges_mut(&mut self) -> &mut Vec<Option<EdgeId>> {
        &mut self.edges
    }
    /// returns a reference to the path's features
    pub const fn features(&self) -> &ChainFeatures {
        &self.features
    }
    /// returns a mutable reference to the path's features
    pub const fn features_mut(&mut self) -> &mut ChainFeatures {
        &mut self.features
    }
    /// returns a reference to the path's transformations
    pub const fn transforms(&self) -> &Vec<LPR> {
        &self.transforms
    }
    /// returns a mutable reference to the path's transformations
    pub const fn transforms_mut(&mut self) -> &mut Vec<LPR> {
        &mut self.transforms
    }
    /// returns a reference to the triads in the path
    pub const fn triads(&self) -> &Vec<Triad<Triads, T>> {
        &self.triads
    }
    /// returns a mutable reference to the triads in the path
    pub const fn triads_mut(&mut self) -> &mut Vec<Triad<Triads, T>> {
        &mut self.triads
    }

    pub fn get_edge(&self, index: usize) -> Option<EdgeId> {
        self.edges.get(index).cloned().flatten()
    }

    pub fn push_transform(&mut self, transform: LPR) {
        self.transforms.push(transform);
    }

    pub fn push_triad(&mut self, triad: Triad<Triads, T>) {
        self.triads.push(triad);
    }
}
