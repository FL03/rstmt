/*
    Appellation: motion <planner>
    Contrib: @FL03
*/
use crate::motion::config::MotionPlannerConfig;
use crate::motion::types::PathCache;
use crate::tonnetz::StdHyperTonnetz;
use core::hash::Hash;

/// The [`MotionPlanner`] is a pathfinding algorithm implementation for finding the chain of
/// transformations between two triads along the surface of the hyper-tonnetz.
pub struct MotionPlanner<'a, T = usize>
where
    T: Eq + Hash,
{
    /// Cache for storing computed paths
    pub(crate) cache: PathCache<T>,
    /// Reference to the tonnetz graph
    pub(crate) tonnetz: &'a StdHyperTonnetz<T>,
    pub(crate) config: MotionPlannerConfig,
}
