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
pub struct VisitedChain<T = isize, Ix = usize> {
    pub(crate) chain: Chain,
    pub(crate) edges: Vec<EdgeId<Ix>>,
    /// The sequence of triads visited
    pub(crate) visited: Vec<DynTriad<T>>,
}

/*
 ************* Implementations *************
*/

impl<T, Ix> VisitedChain<T, Ix> {
    pub fn new(path: Vec<LPR>, visited: Vec<DynTriad<T>>) -> Self {
        VisitedChain {
            chain: Chain::from_path(path),
            edges: Vec::new(),
            visited,
        }
    }
    /// returns a new transformation chain using the visited triads
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
    /// returns a reference to the edges
    pub const fn edges(&self) -> &Vec<EdgeId<Ix>> {
        &self.edges
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

impl<T, Ix> AsRef<Chain> for VisitedChain<T, Ix> {
    fn as_ref(&self) -> &Chain {
        self.chain()
    }
}

impl<T, Ix> AsMut<Chain> for VisitedChain<T, Ix> {
    fn as_mut(&mut self) -> &mut Chain {
        self.chain_mut()
    }
}

impl<T, Ix> core::borrow::Borrow<Chain> for VisitedChain<T, Ix> {
    fn borrow(&self) -> &Chain {
        self.chain()
    }
}

impl<T, Ix> core::borrow::BorrowMut<Chain> for VisitedChain<T, Ix> {
    fn borrow_mut(&mut self) -> &mut Chain {
        self.chain_mut()
    }
}

impl<T, Ix> core::ops::Deref for VisitedChain<T, Ix> {
    type Target = Chain;

    fn deref(&self) -> &Self::Target {
        self.chain()
    }
}

impl<T, Ix> core::ops::DerefMut for VisitedChain<T, Ix> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.chain_mut()
    }
}

impl<T, Ix> IntoIterator for VisitedChain<T, Ix> {
    type Item = LPR;
    type IntoIter = alloc::vec::IntoIter<LPR>;

    fn into_iter(self) -> Self::IntoIter {
        self.chain.into_iter()
    }
}

impl<'a, T, Ix> IntoIterator for &'a VisitedChain<T, Ix> {
    type Item = &'a LPR;
    type IntoIter = core::slice::Iter<'a, LPR>;

    fn into_iter(self) -> Self::IntoIter {
        self.chain.path.iter()
    }
}
