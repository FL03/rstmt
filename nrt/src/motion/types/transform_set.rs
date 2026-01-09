/*
    Appellation: path <module>
    Contrib: @FL03
*/
use super::{Chain, ChainFeatures};
use crate::LPR;
use crate::triad::DynTriad;
use alloc::vec::Vec;
use rshyper::EdgeId;

/// An extended transformation chain that records the visited triads and path features
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct VisitedChain<T = isize> {
    pub(crate) chain: Chain,
    pub(crate) edges: Vec<EdgeId>,
    /// The sequence of triads visited
    pub(crate) visited: Vec<DynTriad<T>>,
}

/*
 ************* Implementations *************
*/

impl<T> VisitedChain<T> {
    pub fn new(path: Vec<LPR>, visited: Vec<DynTriad<T>>) -> Self {
        VisitedChain {
            chain: Chain::from_path(path),
            edges: Vec::new(),
            visited,
        }
    }

    pub fn from_visited<I>(visited: I) -> Self
    where
        I: IntoIterator<Item = DynTriad<T>>,
    {
        let visited = Vec::from_iter(visited);
        VisitedChain::new(Vec::new(), visited)
    }
    /// returns a reference to the chain
    pub const fn chain(&self) -> &Chain {
        &self.chain
    }
    /// returns a mutable reference to the chain
    pub const fn chain_mut(&mut self) -> &mut Chain {
        &mut self.chain
    }
    /// returns a copy of the cost of the transformation chain
    pub const fn cost(&self) -> usize {
        self.chain().cost()
    }
    /// returns a copy of the features of the transformation chain
    pub const fn features(&self) -> &ChainFeatures {
        self.chain().features()
    }
    /// returns a reference to the path of transformations
    pub const fn path(&self) -> &Vec<LPR> {
        self.chain().path()
    }
    /// returns an immutable reference to the visited triads
    pub const fn visited(&self) -> &Vec<DynTriad<T>> {
        &self.visited
    }
    /// returns an iterator over the transformations in the chain
    pub fn iter_path(&self) -> impl Iterator<Item = &LPR> {
        self.path().iter()
    }
}

impl<T> core::ops::Deref for VisitedChain<T> {
    type Target = Chain;

    fn deref(&self) -> &Self::Target {
        self.chain()
    }
}
impl<T> core::ops::DerefMut for VisitedChain<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.chain_mut()
    }
}
