/*
    Appellation: path <module>
    Contrib: @FL03
*/
use crate::transform::SearchNode;
use crate::{LPR, Triad};
use hashbrown::HashMap;
use rshyper::EdgeId;

/// Represents a sequence of transformations from one triad to another
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Path {
    /// Musical cost or distance metric (lower is better)
    pub(crate) cost: usize,
    /// Edge IDs in the tonnetz (if available)
    pub(crate) edge_ids: Vec<Option<EdgeId>>,
    /// Path features for musical analysis
    pub(crate) features: PathFeatures,
    /// The sequence of transformations to apply
    pub(crate) transforms: Vec<LPR>,
    /// The sequence of triads visited
    pub(crate) triads: Vec<Triad>,
}

/// Features describing musical characteristics of a transformation path
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PathFeatures {
    /// Smoothness of voice leading (sum of semitone movements)
    pub(crate) distance: usize,
    /// Changes in modality (i.e. major/minor or any other changes in classification)
    pub(crate) modality_changes: usize,
    /// Count of each transformation type in the path
    pub(crate) transform_counts: HashMap<LPR, usize>,
}

impl Path {
    pub fn new(transforms: Vec<LPR>, triads: Vec<Triad>, edge_ids: Vec<Option<EdgeId>>) -> Self {
        Path {
            transforms,
            triads,
            edge_ids,
            cost: 0,
            features: PathFeatures::default(),
        }
    }

    pub fn from_triads(triads: Vec<Triad>) -> Self {
        Path {
            transforms: Vec::new(),
            triads,
            edge_ids: Vec::new(),
            cost: 0,
            features: PathFeatures::default(),
        }
    }

    pub fn from_node_with_features(node: SearchNode, features: PathFeatures) -> Self {
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
            edge_ids,
            cost,
            features,
        }
    }
    /// returns a copy of the path's cost
    pub const fn cost(&self) -> usize {
        self.cost
    }
    #[inline]
    pub fn cost_mut(&mut self) -> &mut usize {
        &mut self.cost
    }
    /// returns a reference to the path's edge IDs
    pub const fn edges(&self) -> &Vec<Option<EdgeId>> {
        &self.edge_ids
    }
    #[inline]
    pub fn edges_mut(&mut self) -> &mut Vec<Option<EdgeId>> {
        &mut self.edge_ids
    }
    /// returns a reference to the path's features
    pub const fn features(&self) -> &PathFeatures {
        &self.features
    }
    #[inline]
    pub fn features_mut(&mut self) -> &mut PathFeatures {
        &mut self.features
    }

    pub const fn transforms(&self) -> &Vec<LPR> {
        &self.transforms
    }
    #[inline]
    pub fn transforms_mut(&mut self) -> &mut Vec<LPR> {
        &mut self.transforms
    }

    pub const fn triads(&self) -> &Vec<Triad> {
        &self.triads
    }

    #[inline]
    pub fn triads_mut(&mut self) -> &mut Vec<Triad> {
        &mut self.triads
    }

    pub fn get_edge(&self, index: usize) -> Option<EdgeId> {
        self.edge_ids.get(index).cloned().flatten()
    }

    pub fn push_transform(&mut self, transform: LPR) {
        self.transforms.push(transform);
    }

    pub fn push_triad(&mut self, triad: Triad) {
        self.triads.push(triad);
    }
}
