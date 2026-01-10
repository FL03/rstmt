/*
    Appellation: chain <module>
    Created At: 2026.01.09:11:04:36
    Contrib: @FL03
*/
use super::ChainFeatures;
use crate::LPR;
use alloc::vec::Vec;

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Chain {
    /// Musical cost or distance metric (lower is better)
    pub(crate) cost: usize,
    /// Path features for musical analysis
    pub(crate) features: ChainFeatures,
    /// The sequence of transformations to apply
    pub(crate) path: Vec<LPR>,
}

impl Chain {
    /// initialize a new, empty transformation [`Chain`]
    pub fn new() -> Self {
        Chain {
            cost: 0,
            features: ChainFeatures::new(),
            path: Vec::new(),
        }
    }
    /// returns a new instance of the [`Chain`] using the given features
    pub fn from_features(features: ChainFeatures) -> Self {
        Chain {
            cost: 0,
            features,
            path: Vec::new(),
        }
    }
    /// returns a new instance of the [`Chain`] from an iterator over transformations
    pub fn from_path<I>(path: I) -> Self
    where
        I: IntoIterator<Item = LPR>,
    {
        let path = Vec::from_iter(path);
        Chain {
            cost: 0,
            features: ChainFeatures::new(),
            path,
        }
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
    /// update the cost of the chain
    pub fn set_cost(&mut self, cost: usize) {
        self.cost = cost;
    }
    /// update the features of the chain
    pub fn set_features(&mut self, features: ChainFeatures) {
        self.features = features
    }

    /// consumes the instance to create another with the given features
    pub fn with_features(self, features: ChainFeatures) -> Self {
        Self { features, ..self }
    }
    /// consumes the instance to create another with the given cost
    pub fn with_cost(self, cost: usize) -> Self {
        Self { cost, ..self }
    }
    /// consumes the instance to create another with the given path
    pub fn with_path<I>(self, path: I) -> Self
    where
        I: IntoIterator<Item = LPR>,
    {
        let path = Vec::from_iter(path);
        Self { path, ..self }
    }
    /// returns an iterator over the transformations in the chain
    pub fn iter(&self) -> impl Iterator<Item = &LPR> {
        self.path.iter()
    }
}

impl IntoIterator for Chain {
    type Item = LPR;
    type IntoIter = alloc::vec::IntoIter<LPR>;

    fn into_iter(self) -> Self::IntoIter {
        self.path.into_iter()
    }
}

impl<'a> IntoIterator for &'a Chain {
    type Item = &'a LPR;
    type IntoIter = core::slice::Iter<'a, LPR>;

    fn into_iter(self) -> Self::IntoIter {
        self.path.iter()
    }
}
