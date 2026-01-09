/*
    Appellation: config <module>
    Created At: 2026.01.09:11:31:54
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
    pub max_depth: usize,
    /// Maximum number of paths to find
    pub max_paths: usize,
}

impl PathFinderConfig {
    /// the default maximum search depth for pathfinding
    pub const DEFAULT_MAX_DEPTH: usize = 5;
    /// the default maximum number of paths to find
    pub const DEFAULT_MAX_PATHS: usize = 5;

    /// returns a new instance of the [`PathFinderConfig`] with the given values
    pub const fn new(depth: usize, paths: usize) -> Self {
        Self {
            max_depth: depth, // Default search depth
            max_paths: paths, // default number of paths to find
        }
    }
    /// returns a new instance of the [`PathFinderConfig`] with the given depth and default
    /// [`paths`](Self::DEFAULT_MAX_PATHS)
    pub const fn from_depth(depth: usize) -> Self {
        Self {
            max_depth: depth,                   // Default search depth
            max_paths: Self::DEFAULT_MAX_PATHS, // default number of paths to find
        }
    }
    /// returns a new instance of the [`PathFinderConfig`] with the given paths and default
    /// [`depth`](Self::DEFAULT_MAX_DEPTH)
    pub const fn from_paths(paths: usize) -> Self {
        Self {
            max_depth: Self::DEFAULT_MAX_DEPTH, // Default search depth
            max_paths: paths,                   // default number of paths to find
        }
    }
    /// returns the maximum depth for pathfinding
    pub const fn max_depth(&self) -> usize {
        self.max_depth
    }
    /// returns a mutable reference to the maximum depth for pathfinding
    pub const fn max_depth_mut(&mut self) -> &mut usize {
        &mut self.max_depth
    }
    /// returns the maximum number of paths to find
    pub const fn max_paths(&self) -> usize {
        self.max_paths
    }
    /// returns a mutable reference to the maximum number of paths to find
    pub const fn max_paths_mut(&mut self) -> &mut usize {
        &mut self.max_paths
    }
    /// [`replace`](core::mem::replace) the maximum depth for pathfinding, returning the previous value
    pub const fn replace_max_depth(&mut self, depth: usize) -> usize {
        core::mem::replace(self.max_depth_mut(), depth)
    }
    /// [`replace`](core::mem::replace) the maximum number of paths to find, returning the previous value
    pub const fn replace_max_paths(&mut self, paths: usize) -> usize {
        core::mem::replace(self.max_paths_mut(), paths)
    }
    /// [`replace`](core::mem::replace) the current values with that of another instance
    pub const fn replace(&mut self, other: Self) -> Self {
        core::mem::replace(self, other)
    }
    /// set the maximum depth for pathfinding
    pub const fn set_max_depth(&mut self, depth: usize) {
        self.max_depth = depth
    }
    /// set the maximum number of paths to find
    pub const fn set_max_paths(&mut self, paths: usize) {
        self.max_paths = paths
    }
    #[inline]
    /// consumes the current instance to create another with the given maximum depth
    pub fn with_max_depth(self, depth: usize) -> Self {
        Self {
            max_depth: depth,
            ..self
        }
    }
    #[inline]
    /// consumes the current instance to create another with the given maximum number of paths
    pub fn with_max_paths(self, paths: usize) -> Self {
        Self {
            max_paths: paths,
            ..self
        }
    }
}

impl Default for PathFinderConfig {
    fn default() -> Self {
        Self {
            max_depth: Self::DEFAULT_MAX_DEPTH,
            max_paths: Self::DEFAULT_MAX_PATHS,
        }
    }
}

impl core::fmt::Display for PathFinderConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ depth: {}, paths: {} }}",
            self.max_depth, self.max_paths
        )
    }
}
