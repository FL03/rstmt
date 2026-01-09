/*
    Appellation: path <module>
    Contrib: @FL03
*/
use super::ChainFeatures;
use crate::LPR;
use crate::triad::DynTriad;
use alloc::vec::Vec;
use rshyper::EdgeId;

/// Represents a sequence of transformations from one triad to another
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TransformationChain<T = isize> {
    /// Musical cost or distance metric (lower is better)
    pub(crate) cost: usize,
    /// The edge id of the path
    pub(crate) edges: Vec<Option<EdgeId>>,
    /// Path features for musical analysis
    pub(crate) features: ChainFeatures,
    /// The sequence of transformations to apply
    pub(crate) path: Vec<LPR>,
    /// The sequence of triads visited
    pub(crate) visited: Vec<DynTriad<T>>,
}

/*
 ************* Implementations *************
*/

impl<T> TransformationChain<T> {
    pub fn new(path: Vec<LPR>, visited: Vec<DynTriad<T>>) -> Self {
        TransformationChain {
            cost: 0,
            edges: Vec::new(),
            features: ChainFeatures::new(),
            path,
            visited,
        }
    }

    pub fn from_visited<I>(visited: I) -> Self
    where
        I: IntoIterator<Item = DynTriad<T>>,
    {
        let visited = Vec::from_iter(visited);
        TransformationChain::new(Vec::new(), visited)
    }
    /// returns a copy of the cost of the transformation chain
    pub const fn cost(&self) -> usize {
        self.cost
    }
    /// returns a copy of the features of the transformation chain
    pub const fn features(&self) -> &ChainFeatures {
        &self.features
    }
    /// returns a reference to the path of transformations
    pub const fn path(&self) -> &Vec<LPR> {
        &self.path
    }
    /// returns an immutable reference to the visited triads
    pub const fn visited(&self) -> &Vec<DynTriad<T>> {
        &self.visited
    }
}
