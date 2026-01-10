/*
    Appellation: chain_features <module>
    Created At: 2026.01.09:08:42:28
    Contrib: @FL03
*/
use crate::LPR;
use hashbrown::HashMap;

/// The [`ChainFeatures`] implementation defines various characteristics of a transformation
/// chain, including the total distance, number of class or modality changes, and a map for
/// tracking the number of each transformation applied.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ChainFeatures {
    /// Smoothness of voice leading (sum of semitone movements)
    pub(crate) distance: usize,
    /// Changes in modality (major to minor or vice versa)
    pub(crate) modality_changes: usize,
    /// Count of each transformation type in the path
    pub(crate) transformations: HashMap<LPR, usize>,
}

impl ChainFeatures {
    /// initialize a new instance of the [`ChainFeatures`] implementation
    pub fn new() -> Self {
        Self {
            distance: 0,
            modality_changes: 0,
            transformations: HashMap::new(),
        }
    }
    /// returns the voice leading distance of the chain features
    pub const fn distance(&self) -> usize {
        self.distance
    }
    /// returns the number of modality changes in the chain features
    pub const fn modality_changes(&self) -> usize {
        self.modality_changes
    }
    /// returns a reference to a map used to track the number of transformations of each type
    pub const fn transformations(&self) -> &HashMap<LPR, usize> {
        &self.transformations
    }
    /// returns a mutable reference to a map used to track the number of transformations of each
    pub const fn transformations_mut(&mut self) -> &mut HashMap<LPR, usize> {
        &mut self.transformations
    }
    #[inline]
    /// sets the transform counts of the chain features
    pub fn set_transform_counts(&mut self, counts: HashMap<LPR, usize>) -> &mut Self {
        self.transformations = counts;
        self
    }
    /// sets the modality changes of the chain features
    pub const fn set_modality_changes(&mut self, changes: usize) -> &mut Self {
        self.modality_changes = changes;
        self
    }
    /// sets the distance of the chain features
    pub const fn set_distance(&mut self, distance: usize) -> &mut Self {
        self.distance = distance;
        self
    }
    /// consumes the instance to create another with the given distance
    pub fn with_distance(self, distance: usize) -> Self {
        Self { distance, ..self }
    }
    /// consumes the instance to create another with the given modality changes
    pub fn with_modality_changes(self, changes: usize) -> Self {
        Self {
            modality_changes: changes,
            ..self
        }
    }
    /// consumes the instance to create another with the given transformation counts
    pub fn with_transform_counts(self, counts: HashMap<LPR, usize>) -> Self {
        Self {
            transformations: counts,
            ..self
        }
    }
    /// returns true if there are no transformations, modality changes, or distance recorded
    pub fn is_empty(&self) -> bool {
        self.transformations().is_empty() && self.modality_changes() == 0 && self.distance() == 0
    }
    /// the total number of transformations recorded
    pub fn len(&self) -> usize {
        self.transformations().len()
    }
}
