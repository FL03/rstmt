/*
    Appellation: navigator <module>
    Contrib: @FL03
*/
use super::{ChainFeatures, PathFinderConfig, TransformationChain};
use crate::traits::{TriadRepr, TriadType};
use crate::triad::{Triad, TriadBase};
use crate::types::LPR;
use alloc::collections::VecDeque;
use hashbrown::{HashMap, HashSet};
use rspace_traits::RawSpace;
/// The transformer allows one triad to find valid transformation chains capable of taking the
/// instance to another based on some critieria.
#[derive(Debug)]
pub struct TriadNavigator<'a, S, K, T = <S as RawSpace>::Elem>
where
    K: TriadType,
    S: TriadRepr<Elem = T>,
{
    triad: &'a TriadBase<S, K, T>,
    config: PathFinderConfig,
}

impl<'a, S, K, T> TriadNavigator<'a, S, K, T>
where
    K: TriadType,
    S: TriadRepr<Elem = T>,
{
    pub(crate) fn new(triad: &'a TriadBase<S, K, T>) -> Self {
        Self {
            triad,
            config: PathFinderConfig::default(),
        }
    }
    pub const fn config(&self) -> &PathFinderConfig {
        &self.config
    }
    /// returns a mutable reference to the configuration of the navigator
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

impl<'a> TriadNavigator<'a, [usize; 3], crate::TriadClass, usize> {
    /// find all possible chains that are capable of transforming the given instance to the target symbol
    pub fn find_paths_to_target(&self, target: usize) -> crate::Result<Vec<TransformationChain>> {
        let mut result_paths = Vec::new();

        let start_triad = *self.triad();

        // Check if the starting triad already contains the target pitch
        if start_triad.contains(&target) {
            let features = ChainFeatures {
                transform_counts: HashMap::new(),
                modality_changes: 0,
                distance: 0,
            };

            result_paths.push(TransformationChain {
                cost: 0,
                edges: Vec::new(),
                features,
                path: Vec::new(),
                visited: vec![start_triad],
            });

            return Ok(result_paths);
        }

        // For BFS: (current_triad, transforms_so_far, triads_so_far, edge_ids_so_far)
        let mut queue = VecDeque::new();
        queue.push_back((start_triad, Vec::new(), vec![start_triad]));

        // Use a hash set to track visited triads and avoid cycles
        // We'll hash based on the triad's notes, not its edge ID, since we might explore virtual triads
        let mut visited_triads = HashSet::new();
        visited_triads.insert(start_triad.chord);

        while let Some((current_triad, transforms, triads)) = queue.pop_front() {
            // Don't exceed maximum depth
            if transforms.len() >= self.max_depth() {
                continue;
            }

            // Try each transformation: Leading, Parallel, Relative
            for transform in LPR::iter() {
                // Apply the transformation to get a new triad
                let next_triad = current_triad.transform(transform)?;

                // Skip if we've already visited this triad
                if visited_triads.contains(&next_triad.chord) {
                    continue;
                }

                // Mark as visited
                visited_triads.insert(next_triad.chord);

                // Build new path
                let mut new_transforms = transforms.clone();
                new_transforms.push(transform);

                let mut new_triads = triads.clone();
                new_triads.push(next_triad);

                // Check if this triad contains our target pitch
                if next_triad.contains(&target) {
                    // Calculate path features
                    let features = self.analyze_path_features(&new_triads);
                    let cost = features.distance + new_transforms.len();

                    // Found a path
                    result_paths.push(TransformationChain {
                        path: new_transforms.clone(),
                        visited: new_triads.clone(),
                        cost,
                        edges: Vec::new(),
                        features,
                    });

                    // Check if we've found enough paths
                    if result_paths.len() >= self.max_paths() {
                        // Sort paths by cost (lower is better)
                        result_paths.sort_by_key(|p| p.cost);
                        return Ok(result_paths);
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
    fn analyze_path_features(&self, triads: &[Triad]) -> ChainFeatures {
        let mut features = ChainFeatures::default();

        // Count transforms (infer from triad progression)
        let mut transform_counts = HashMap::new();

        let mut modality_changes = 0;
        let mut voice_leading_distance = 0;

        // Analyze modality changes and voice leading
        for i in 1..triads.len() {
            let prev = &triads[i - 1];
            let curr = &triads[i];

            // Determine which transform was applied (approximate)
            let transform = if prev.is_major() != curr.is_major() {
                // Parallel transform changes mode while preserving root
                if prev.root() == curr.root() {
                    LPR::Parallel
                }
                // Relative transform preserves two notes
                else if prev.common_tones(curr).len() == 2 {
                    LPR::Relative
                }
                // Leading transform if no better match
                else {
                    LPR::Leading
                }
            } else {
                // If mode is preserved, likely Leading transform
                LPR::Leading
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
                        let dist = (curr_note as isize - prev_note as isize).abs() % 12;
                        std::cmp::min(dist, 12 - dist) as usize
                    })
                    .min()
                    .unwrap_or(0);

                voice_leading_distance += min_distance;
            }
        }

        features.transform_counts = transform_counts;
        features.modality_changes = modality_changes;
        features.distance = voice_leading_distance;

        features
    }
}
