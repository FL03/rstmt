/*
    Appellation: motion <planner>
    Contrib: @FL03
*/
use crate::motion::config::PathfinderConfig;
use crate::motion::types::PathCache;
use crate::tonnetz::HyperTonnetz;

/// The [`MotionPlanner`] is a pathfinding algorithm implementation for finding the chain of
/// transformations between two triads along the surface of the hyper-tonnetz.
pub struct MotionPlanner<'a> {
    /// Cache for storing computed paths
    pub(crate) cache: PathCache,
    /// Reference to the tonnetz graph
    pub(crate) tonnetz: &'a HyperTonnetz,
    pub(crate) config: PathfinderConfig,
}

impl<'a> MotionPlanner<'a> {
    /// Create a new motion planner for the given tonnetz
    pub fn new(tonnetz: &'a HyperTonnetz) -> Self {
        let capacity = 1000; // Default cache capacity
        MotionPlanner {
            cache: PathCache::new(capacity),
            tonnetz,
            config: PathfinderConfig::default(),
        }
    }
    /// returns an immutable reference to the cache
    pub const fn cache(&self) -> &PathCache {
        &self.cache
    }
    /// returns a mutable reference to the cache
    pub const fn cache_mut(&mut self) -> &mut PathCache {
        &mut self.cache
    }
    /// returns an immutable reference to the configuration of the planner
    pub const fn config(&self) -> &PathfinderConfig {
        &self.config
    }
    /// returns a mutable reference to the configuration of the planner
    pub const fn config_mut(&mut self) -> &mut PathfinderConfig {
        &mut self.config
    }
    /// returns the maximum depth for pathfinding
    pub const fn max_depth(&self) -> usize {
        self.config().max_depth()
    }
    /// returns the maximum number of paths to find
    pub const fn max_paths(&self) -> usize {
        self.config().max_paths()
    }
    /// returns an immutable reference to the tonnetz
    pub const fn tonnetz(&self) -> &HyperTonnetz {
        self.tonnetz
    }
    /// updates the current configuration and returns a mutable reference to the instance.
    pub fn set_config(&mut self, config: PathfinderConfig) -> &mut Self {
        self.config = config;
        self
    }
    /// set the maximum depth for pathfinding
    pub fn set_max_depth(&mut self, depth: usize) -> &mut Self {
        self.config_mut().set_max_depth(depth);
        self
    }
    /// set the maximum number of paths to find
    pub fn set_max_paths(&mut self, paths: usize) -> &mut Self {
        self.config_mut().set_max_paths(paths);
        self
    }
    /// consumes the current instance to create another with the given maximum depth
    pub fn with_max_depth(self, depth: usize) -> Self {
        Self {
            config: self.config.with_max_depth(depth),
            ..self
        }
    }
    /// consumes the current instance to create another with the given maximum number of paths
    pub fn with_max_paths(self, paths: usize) -> Self {
        Self {
            config: self.config.with_max_paths(paths),
            ..self
        }
    }
}
