/*
    Appellation: config <module>
    Contrib: @FL03
*/

/// The [`PathFinderConfig`] object provides a standard interface for configuring various
/// implemented transformers.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default, rename_all = "snake_case")
)]
pub struct PathFinderConfig {
    /// Maximum search depth for pathfinding
    pub(crate) depth: usize,
    /// Maximum number of paths to find
    pub(crate) paths: usize,
}

impl PathFinderConfig {
    /// the default maximum search depth for pathfinding
    pub const DEFAULT_MAX_DEPTH: usize = 5;
    /// the default maximum number of paths to find
    pub const DEFAULT_MAX_PATHS: usize = 5;

    /// returns a new instance of the [`PathFinderConfig`] with the given values
    pub fn new(depth: usize, paths: usize) -> Self {
        Self {
            depth, // Default search depth
            paths, // default number of paths to find
        }
    }
    /// returns a new instance of the [`PathFinderConfig`] with the given depth and default
    /// [`paths`](Self::DEFAULT_MAX_PATHS)
    pub fn from_depth(depth: usize) -> Self {
        Self {
            depth,                          // Default search depth
            paths: Self::DEFAULT_MAX_PATHS, // default number of paths to find
        }
    }
    /// returns a new instance of the [`PathFinderConfig`] with the given paths and default
    /// [`depth`](Self::DEFAULT_MAX_DEPTH)
    pub fn from_paths(paths: usize) -> Self {
        Self {
            depth: Self::DEFAULT_MAX_DEPTH, // Default search depth
            paths,                          // default number of paths to find
        }
    }
    /// returns the maximum depth for pathfinding
    pub const fn max_depth(&self) -> usize {
        self.depth
    }
    /// returns a mutable reference to the maximum depth for pathfinding
    pub const fn max_depth_mut(&mut self) -> &mut usize {
        &mut self.depth
    }
    /// returns the maximum number of paths to find
    pub const fn max_paths(&self) -> usize {
        self.paths
    }
    /// returns a mutable reference to the maximum number of paths to find
    pub const fn max_paths_mut(&mut self) -> &mut usize {
        &mut self.paths
    }
    /// set the maximum depth for pathfinding
    pub fn set_max_depth(&mut self, depth: usize) -> &mut Self {
        self.depth = depth;
        self
    }
    /// set the maximum number of paths to find
    pub fn set_max_paths(&mut self, paths: usize) -> &mut Self {
        self.paths = paths;
        self
    }
    /// consumes the current instance to create another with the given maximum depth
    pub fn with_max_depth(self, depth: usize) -> Self {
        Self { depth, ..self }
    }
    /// consumes the current instance to create another with the given maximum number of paths
    pub fn with_max_paths(self, paths: usize) -> Self {
        Self { paths, ..self }
    }
}

impl Default for PathFinderConfig {
    fn default() -> Self {
        Self {
            depth: Self::DEFAULT_MAX_DEPTH,
            paths: Self::DEFAULT_MAX_PATHS,
        }
    }
}

impl core::fmt::Display for PathFinderConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ depth: {}, paths: {} }}", self.depth, self.paths)
    }
}
