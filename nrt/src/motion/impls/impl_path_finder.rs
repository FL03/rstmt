/*
    Appellation: impl_path_finder <module>
    Created At: 2026.01.01:09:17:02
    Contrib: @FL03
*/
use crate::motion::config::PathFinderConfig;
use crate::motion::path_finder::PathFinder;
use crate::motion::types::{Chain, ChainFeatures, Visited};
use crate::traits::{TriadRepr, TriadType};
use crate::triad::{DynTriad, TriChord, TriadBase};
use crate::types::{LPR, Triads};
use alloc::collections::VecDeque;
use hashbrown::{HashMap, HashSet};
use num_traits::{FromPrimitive, One, ToPrimitive, Zero};
use rstmt_core::PitchMod;

impl<'a, S, K, T> PathFinder<'a, S, K, T>
where
    K: TriadType,
    S: TriadRepr<Elem = T>,
{
    pub const DEFAULT_MAX_DEPTH: usize = 5;
    pub const DEFAULT_MAX_PATHS: usize = 5;

    pub(crate) fn new(triad: &'a TriadBase<S, K, T>) -> Self {
        Self {
            triad,
            config: PathFinderConfig::new(Self::DEFAULT_MAX_DEPTH, Self::DEFAULT_MAX_PATHS),
        }
    }
    /// returns a reference to the pathfinder configuration
    pub const fn config(&self) -> &PathFinderConfig {
        &self.config
    }
    /// returns a mutable reference to the pathfinder configuration
    pub const fn config_mut(&mut self) -> &mut PathFinderConfig {
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
    /// returns a copy of the triad being navigated
    pub const fn triad(&self) -> &TriadBase<S, K, T> {
        self.triad
    }
    /// set the maximum depth for pathfinding
    pub fn set_max_depth(&mut self, depth: usize) {
        self.config_mut().set_max_depth(depth)
    }
    /// set the maximum number of paths to find
    pub fn set_max_paths(&mut self, paths: usize) {
        self.config_mut().set_max_paths(paths)
    }
    #[inline]
    /// consumes the current instance to create another with the given maximum depth
    pub fn with_max_depth(self, max_depth: usize) -> Self {
        Self {
            config: self.config.with_max_depth(max_depth),
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

impl<'a, T> PathFinder<'a, TriChord<T>, Triads, T>
where
    T: Copy
        + Eq
        + Ord
        + FromPrimitive
        + ToPrimitive
        + One
        + Zero
        + PitchMod<Output = T>
        + core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>
        + core::hash::Hash
        + core::fmt::Debug
        + core::ops::AddAssign,
{
    /// find all possible chains that are capable of transforming the given instance to the target symbol
    pub fn find_paths_to_target(&self, target: T) -> crate::Result<Vec<Visited<T>>> {
        // initialize an empty vector to hold the result paths
        let mut result_paths = Vec::new();
        // get a copy of the starting point
        let source = *self.triad();

        // Check if the starting triad already contains the target pitch
        if source.contains(&target) {
            result_paths.push(Visited::<T> {
                edges: Vec::new(),
                visited: vec![source],
                chain: Chain::new(),
            });

            return Ok(result_paths);
        }

        // For BFS: (current_triad, transforms_so_far, triads_so_far)
        let mut queue = VecDeque::new();
        queue.push_back((source, Vec::new(), vec![source]));

        // Use a hash set to track visited triads and avoid cycles
        // We'll hash based on the triad's notes, not its edge ID, since we might explore virtual triads
        let mut visited = HashSet::new();
        // make sure the source chord is included
        visited.insert(*source.chord());

        while let Some((current_triad, tchain, triads)) = queue.pop_front() {
            // ensure the depth and path limits are not exceeded
            if tchain.len() >= self.max_depth() || result_paths.len() >= self.max_paths() {
                break;
            }
            // iter over the LPR variants (leading / parallel / relative)
            for dirac in LPR::iter() {
                // apply the transformation
                let next_triad = current_triad.transform(dirac);
                // check that we haven't already visited this triad
                if visited.contains(next_triad.chord()) {
                    continue;
                }
                // mark the triad as visited
                visited.insert(*next_triad.chord());
                // append the new transformation and triad to the respective chains
                let mut new_transforms = tchain.clone();
                new_transforms.push(dirac);

                let mut new_triads = triads.clone();
                new_triads.push(next_triad);

                // Check if this triad contains our target pitch
                if next_triad.contains(&target) {
                    // Calculate path features
                    let features = self.analyze_path_features(&new_triads);
                    let cost = features.distance + new_transforms.len();
                    // create the chain
                    let visited = Visited {
                        visited: new_triads.clone(),
                        edges: Vec::new(),
                        chain: Chain {
                            cost,
                            features,
                            path: new_transforms.clone(),
                        },
                    };
                    // Found a path
                    result_paths.push(visited);
                    // Check if we've found enough paths
                    if result_paths.len() >= self.max_paths() {
                        break;
                    }
                }
                // Continue the search
                queue.push_back((next_triad, new_transforms, new_triads));
            }
        }

        // Sort paths by cost (lower is better)
        result_paths.sort_by_key(|p| p.cost);
        Ok(result_paths)
    }

    /// Analyze musical features of a transformation path using triads
    fn analyze_path_features(&self, triads: &[DynTriad<T>]) -> ChainFeatures {
        use LPR::*;
        let two = T::from_u8(2).unwrap();
        let mut features = ChainFeatures::default();

        // Count transforms (infer from triad progression)
        let mut transform_counts = HashMap::new();

        let mut modality_changes = 0;
        let mut voice_leading_distance = 0;

        // Analyze modality changes and voice leading
        for i in 1..triads.len() {
            let prev = &triads[i - 1];
            let curr = &triads[i];

            if prev.is_major() && curr.is_major() || prev.is_minor() && curr.is_minor() {
                #[cfg(feature = "tracing")]
                tracing::error!("No modality change between {:?} and {:?}", prev, curr);
                continue; // No transform if modality is unchanged
            }

            // Determine which transform was applied (approximate)
            let transform = if prev.is_major() != curr.is_major() {
                // Parallel transform affects the "third" chord factor
                if prev.root() == curr.root() {
                    Parallel
                } else if prev.is_major() && (*curr.root() - two).pmod() == *prev.fifth()
                    || prev.is_minor() && (*curr.fifth() + two).pmod() == *prev.root()
                {
                    Relative
                } else {
                    Leading
                }
            } else {
                #[cfg(feature = "tracing")]
                tracing::error!(
                    "Unable to resolve transform between {:?} and {:?}",
                    prev,
                    curr
                );
                panic!("Unable to determine transform between triads")
            };

            *transform_counts.entry(transform).or_insert(0) += 1;

            // Check for modality change
            if prev.is_major() != curr.is_major() {
                modality_changes += 1;
            }

            // Calculate voice leading distance (semitone movement between triads)
            for &prev_note in &prev.chord {
                // Find the minimum distance to move from prev_note to any note in curr
                let min_distance = curr
                    .chord
                    .iter()
                    .map(|&curr_note| {
                        let dist = (curr_note - prev_note).pmod();
                        core::cmp::min(dist, T::from_u8(12).unwrap() - dist)
                            .to_usize()
                            .unwrap()
                    })
                    .min()
                    .unwrap_or(0);

                voice_leading_distance += min_distance;
            }
        }

        features.set_transform_counts(transform_counts);
        features.set_modality_changes(modality_changes);
        features.set_distance(voice_leading_distance);

        features
    }
}
