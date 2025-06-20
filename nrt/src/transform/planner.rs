/*
    Appellation: motion <planner>
    Contrib: @FL03
*/
use super::types::{Path, PathFeatures, SearchNode};
use super::{PathCache, PathFinderConfig};
use crate::tonnetz::HyperTonnetz;
use crate::{LPR, Triad};

use rshyper::EdgeId;
use rstmt::PitchMod;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

/// Motion planning algorithm for finding optimal paths in the Tonnetz
pub struct MotionPlanner<'a> {
    /// Cache for storing computed paths
    pub(crate) cache: PathCache,
    /// Reference to the tonnetz graph
    pub(crate) tonnetz: &'a HyperTonnetz,
    pub(crate) config: PathFinderConfig,
}

impl<'a> MotionPlanner<'a> {
    /// Create a new motion planner for the given tonnetz
    pub fn new(tonnetz: &'a HyperTonnetz) -> Self {
        let capacity = 1000; // Default cache capacity
        MotionPlanner {
            cache: PathCache::new(capacity),
            tonnetz,
            config: PathFinderConfig::default(),
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
    pub const fn config(&self) -> &PathFinderConfig {
        &self.config
    }
    /// returns a mutable reference to the configuration of the planner
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
    /// returns an immutable reference to the tonnetz
    pub const fn tonnetz(&self) -> &HyperTonnetz {
        self.tonnetz
    }
    /// updates the current configuration and returns a mutable reference to the instance.
    pub fn set_config(&mut self, config: PathFinderConfig) -> &mut Self {
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
    /// find a set of paths from one triad to one that contains the target pitch
    pub fn find_paths_to_pitch(&mut self, start_edge: EdgeId, target_pitch: usize) -> Vec<Path> {
        // Better heuristic function that is admissible for A*
        fn improved_heuristic(triad: &Triad, target: usize) -> usize {
            if triad.contains(&target) {
                return 0;
            }

            // For voice-leading distance, we use max of 1 as estimate
            // This ensures heuristic is admissible (never overestimates)
            1
        }
        // Check cache first
        if let Some(paths) = self.cache.get(&[*start_edge, 0, 0], target_pitch).cloned() {
            return paths;
        }

        // Get the starting triad
        let start_triad = match self.tonnetz().get_triad(start_edge) {
            Some(&triad) => triad,
            None => return Vec::new(),
        };

        // Check if the starting triad already contains the target pitch
        if start_triad.contains(&target_pitch) {
            let features = PathFeatures::default();
            let path = Path {
                transforms: Vec::new(),
                triads: vec![start_triad],
                edge_ids: vec![Some(start_edge)],
                cost: 0,
                features,
            };

            self.cache
                .insert([*start_edge, 0, 0], target_pitch, vec![path.clone()]);
            return vec![path];
        }

        let mut result_paths = Vec::new();
        let mut open_set = BinaryHeap::new();
        let mut visited = HashMap::<[usize; 3], usize>::new(); // Track visited triads with their path length

        // Start with initial node
        open_set.push(SearchNode {
            priority: 0, // -heuristic for min-heap
            cost: 0,
            triad: start_triad,
            transforms: Vec::new(),
            visited: vec![start_triad],
            edges: vec![Some(start_edge)],
        });

        visited.insert(start_triad.notes(), 0);

        while let Some(node) = open_set.pop() {
            // Skip if we've found a shorter path to this triad
            if let Some(&prev_cost) = visited.get(&node.triad.notes()) {
                if prev_cost < node.cost {
                    continue;
                }
            }

            // Check depth limit
            if node.transforms.len() >= self.max_depth() {
                continue;
            }

            // Try each transformation
            for transform in LPR::iter() {
                // Apply transformation
                let next_triad = node.triad.transform(transform);
                let new_cost = node.cost + 1;

                // Skip if we've found a shorter path to this triad
                if let Some(&prev_cost) = visited.get(&next_triad.notes()) {
                    if prev_cost <= new_cost {
                        continue;
                    }
                }

                // Update visited with this triad's path length
                visited.insert(next_triad.notes(), new_cost);

                // Find edge ID if this exists in the tonnetz
                let next_edge_id = self.tonnetz.triads.iter().find_map(|(&id, facet)| {
                    if facet.notes() == next_triad.notes() {
                        Some(id)
                    } else {
                        None
                    }
                });

                // Build new path
                let mut new_transforms = node.transforms.clone();
                new_transforms.push(transform);

                let mut new_triads = node.visited.clone();
                new_triads.push(next_triad);

                let mut new_edge_ids = node.edges.clone();
                new_edge_ids.push(next_edge_id);

                // Check if this triad contains our target pitch
                if next_triad.contains(&target_pitch) {
                    // Calculate path features
                    let features = self.analyze_path_features(&new_triads);

                    // Found a path
                    let path = Path {
                        transforms: new_transforms.clone(),
                        triads: new_triads.clone(),
                        edge_ids: new_edge_ids.clone(),
                        cost: new_cost,
                        features,
                    };

                    result_paths.push(path);

                    // Check if we've found enough paths
                    if result_paths.len() >= self.max_paths() {
                        // Sort paths by cost
                        result_paths.sort_by_key(|p| p.cost);

                        // Cache the result
                        self.cache
                            .insert([*start_edge, 0, 0], target_pitch, result_paths.clone());

                        return result_paths;
                    }
                }

                // Continue search - use admissible heuristic
                let h = improved_heuristic(&next_triad, target_pitch);
                let priority = -((new_cost as i32) + (h as i32)); // Negative for min-heap

                let next_node = SearchNode {
                    priority,
                    cost: new_cost,
                    triad: next_triad,
                    transforms: new_transforms,
                    visited: new_triads,
                    edges: new_edge_ids,
                };

                open_set.push(next_node);
            }
        }

        // Sort by cost
        result_paths.sort_by_key(|p| p.cost);

        // Cache results
        self.cache
            .insert([*start_edge, 0, 0], target_pitch, result_paths.clone());

        result_paths
    }
    /// Search for paths between two specific edges in the tonnetz
    pub fn find_paths_between_edges(&mut self, start_edge: EdgeId, goal_edge: EdgeId) -> Vec<Path> {
        // Check cache first
        if let Some(paths) = self
            .cache_mut()
            .get(&[*start_edge, *goal_edge, 1], 0)
            .cloned()
        {
            return paths;
        }

        // Get the starting and goal triads
        let start_triad = match self.tonnetz.get_triad(start_edge) {
            Some(triad) => *triad,
            None => return Vec::new(),
        };

        // let goal_triad = match self.tonnetz.get_triad(goal_edge) {
        //     Some(triad) => triad.clone(),
        //     None => return Vec::new(),
        // };

        // Check if start and goal are the same
        if start_edge == goal_edge {
            let features = PathFeatures::default();
            let path = Path {
                transforms: Vec::new(),
                triads: vec![start_triad],
                edge_ids: vec![Some(start_edge)],
                cost: 0,
                features,
            };

            // Cache the result
            self.cache
                .insert([*start_edge, *goal_edge, 1], 0, vec![path.clone()]);

            return vec![path];
        }

        // Use BFS with iterative deepening to find paths to goal edge
        let mut all_paths = Vec::new();

        // Iterative deepening
        for depth in 1..=self.max_depth() {
            let paths = self.bfs_between_edges(start_edge, goal_edge, depth);

            if !paths.is_empty() {
                all_paths = paths;
                break;
            }
        }

        // Sort by cost and limit to max_paths
        all_paths.sort_by_key(|p| p.cost);
        if all_paths.len() > self.max_paths() {
            all_paths.truncate(self.max_paths());
        }

        // Cache the result
        self.cache
            .insert([*start_edge, *goal_edge, 1], 0, all_paths.clone());

        all_paths
    }

    /// BFS to find paths between two edges with a depth limit
    fn bfs_between_edges(
        &self,
        start_edge: EdgeId,
        goal_edge: EdgeId,
        max_depth: usize,
    ) -> Vec<Path> {
        let mut result_paths = Vec::new();

        // Get the starting triad
        let start_triad = match self.tonnetz.get_triad(start_edge) {
            Some(&triad) => triad,
            None => return Vec::new(),
        };

        // Initialize BFS queue
        let mut queue = VecDeque::new();
        queue.push_back((
            start_triad,            // Current triad
            Vec::<LPR>::new(),      // Transformation path
            vec![start_triad],      // Triad history
            vec![Some(start_edge)], // Edge IDs
            0usize,                 // Current depth
        ));

        // Track visited triads at each depth
        let mut visited = HashMap::<[usize; 3], HashSet<usize>>::new();
        visited.entry(start_triad.notes()).or_default().insert(0);

        while let Some((current_triad, transforms, triads, edge_ids, depth)) = queue.pop_front() {
            // If we've reached max depth, skip this path
            if depth >= max_depth {
                continue;
            }

            // Try each transformation
            for transform in LPR::iter() {
                // Apply transformation
                let next_triad = current_triad.transform(transform);
                let next_depth = depth + 1;

                // Check if this triad+depth combination has been visited before
                let depths = visited.entry(next_triad.notes()).or_default();
                if depths.contains(&next_depth) {
                    continue;
                }

                // Mark as visited at this depth
                depths.insert(next_depth);

                // Find edge ID if this triad exists in the tonnetz
                let next_edge_id = self.tonnetz.triads.iter().find_map(|(&id, facet)| {
                    if facet.notes() == next_triad.notes() {
                        Some(id)
                    } else {
                        None
                    }
                });

                // Create new path components
                let mut new_transforms = transforms.clone();
                new_transforms.push(transform);

                let mut new_triads = triads.clone();
                new_triads.push(next_triad);

                let mut new_edge_ids = edge_ids.clone();
                new_edge_ids.push(next_edge_id);

                // Check if we've reached the goal edge
                if next_edge_id == Some(goal_edge) {
                    // Calculate path features
                    let features = self.analyze_path_features(&new_triads);
                    let cost = features.distance + new_transforms.len();

                    // Create path
                    let path = Path {
                        transforms: new_transforms,
                        triads: new_triads,
                        edge_ids: new_edge_ids,
                        cost,
                        features,
                    };

                    result_paths.push(path);

                    // If we've found max_paths, return early
                    if result_paths.len() >= self.max_paths() {
                        return result_paths;
                    }
                } else if next_depth < max_depth {
                    // If we haven't reached the goal and haven't reached max depth,
                    // add to queue for further exploration
                    queue.push_back((
                        next_triad,
                        new_transforms,
                        new_triads,
                        new_edge_ids,
                        next_depth,
                    ));
                }
            }
        }

        result_paths
    }
    #[allow(clippy::too_many_arguments)]
    /// Find paths from a specified edge by continuing search from a given state
    /// Used for parallel search implementations
    pub fn search_from(
        &self,
        start_triad: Triad,
        target_pitch: usize,
        transforms: Vec<LPR>,
        triads: Vec<Triad>,
        edge_ids: Vec<Option<EdgeId>>,
        max_paths: usize,
        remaining_depth: usize,
    ) -> Vec<Path> {
        let mut result_paths = Vec::new();

        // Check if we're already at a triad containing the target pitch
        if start_triad.contains(&target_pitch) {
            let features = self.analyze_path_features(&triads);
            let cost = features.distance + transforms.len();

            result_paths.push(Path {
                transforms,
                triads,
                edge_ids,
                cost,
                features,
            });

            return result_paths;
        }

        // If we've reached max depth, return empty results
        if remaining_depth == 0 {
            return result_paths;
        }

        // Use BFS for continuation of search
        let mut queue = VecDeque::new();
        queue.push_back((
            start_triad,
            transforms.clone(),
            triads.clone(),
            edge_ids.clone(),
            0usize,
        ));

        // Track visited triads at each depth
        let mut visited = HashMap::<[usize; 3], HashSet<usize>>::new();
        visited.entry(start_triad.notes()).or_default().insert(0);

        while let Some((
            current_triad,
            current_transforms,
            current_triads,
            current_edge_ids,
            depth,
        )) = queue.pop_front()
        {
            // If we've reached max depth, skip this path
            if depth >= remaining_depth {
                continue;
            }

            // Try each transformation
            for transform in LPR::iter() {
                // Apply transformation
                let next_triad = current_triad.transform(transform);
                let next_depth = depth + 1;

                // Check if this triad+depth combination has been visited before
                let depths = visited.entry(next_triad.notes()).or_default();
                if depths.contains(&next_depth) {
                    continue;
                }

                // Mark as visited at this depth
                depths.insert(next_depth);

                // Find edge ID if this triad exists in the tonnetz
                let next_edge_id = self.tonnetz.triads.iter().find_map(|(&id, facet)| {
                    if facet.notes() == next_triad.notes() {
                        Some(id)
                    } else {
                        None
                    }
                });

                // Create new path components
                let mut new_transforms = current_transforms.clone();
                new_transforms.push(transform);

                let mut new_triads = current_triads.clone();
                new_triads.push(next_triad);

                let mut new_edge_ids = current_edge_ids.clone();
                new_edge_ids.push(next_edge_id);

                // Check if this triad contains the target pitch
                if next_triad.contains(&target_pitch) {
                    // Calculate path features
                    let features = self.analyze_path_features(&new_triads);
                    let cost = features.distance + new_transforms.len();

                    // Create path
                    let path = Path {
                        transforms: new_transforms,
                        triads: new_triads,
                        edge_ids: new_edge_ids,
                        cost,
                        features,
                    };

                    result_paths.push(path);

                    // If we've found max_paths, return early
                    if result_paths.len() >= max_paths {
                        result_paths.sort_by_key(|p| p.cost);
                        return result_paths;
                    }
                } else if next_depth < remaining_depth {
                    // If we haven't reached the target and haven't reached max depth,
                    // add to queue for further exploration
                    queue.push_back((
                        next_triad,
                        new_transforms,
                        new_triads,
                        new_edge_ids,
                        next_depth,
                    ));
                }
            }
        }

        // Sort and return paths
        result_paths.sort_by_key(|p| p.cost);
        result_paths
    }

    /// Run searches in parallel from initial transformations
    #[cfg(feature = "rayon")]
    pub fn find_paths_parallel(&self, start_edge: EdgeId, target_pitch: usize) -> Vec<Path> {
        use rayon::prelude::*;

        // Get the starting triad
        let start_triad = match self.tonnetz.get_triad(start_edge) {
            Some(triad) => *triad,
            None => return Vec::new(),
        };

        // Check if starting triad already contains the target pitch
        if start_triad.contains(&target_pitch) {
            let features = PathFeatures::default();
            let path = Path {
                transforms: Vec::new(),
                triads: vec![start_triad],
                edge_ids: vec![Some(start_edge)],
                cost: 0,
                features,
            };

            return vec![path];
        }

        // Initial transformations for parallel searches
        let results: Vec<Vec<Path>> = LPR::iter()
            .par_bridge()
            .filter_map(|transform| {
                // Try applying the transformation
                match transform.try_apply(&start_triad) {
                    Ok(next_triad) => {
                        // Find edge ID if it exists
                        let next_edge_id = self.tonnetz.triads.iter().find_map(|(&id, facet)| {
                            if facet.notes() == next_triad.notes() {
                                Some(id)
                            } else {
                                None
                            }
                        });

                        let transforms = vec![transform];
                        let triads = vec![start_triad, next_triad];
                        let edge_ids = vec![Some(start_edge), next_edge_id];

                        // Start search from this branch
                        Some(self.search_from(
                            next_triad,
                            target_pitch,
                            transforms,
                            triads,
                            edge_ids,
                            self.max_paths(),
                            self.max_depth() - 1,
                        ))
                    }
                    Err(_) => None,
                }
            })
            .collect();

        // Combine and sort results
        let mut all_paths = Vec::new();
        for paths in results {
            all_paths.extend(paths);
        }

        // Sort by cost and take max_paths
        all_paths.sort_by_key(|p| p.cost);
        if all_paths.len() > self.max_paths() {
            all_paths.truncate(self.max_paths());
        }

        all_paths
    }

    /// Analyze musical features of a transformation path
    fn analyze_path_features(&self, triads: &[Triad]) -> PathFeatures {
        let mut features = PathFeatures::default();

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
                // Relative transform preserves two notes()
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
            for prev_note in prev.notes() {
                // Find the minimum distance to move from prev_note to any note in curr
                let min_distance = curr
                    .notes()
                    .iter()
                    .map(|&curr_note| {
                        let dist = (curr_note as isize - prev_note as isize).abs().pmod();
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
