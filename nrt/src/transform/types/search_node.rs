/*
    Appellation: search_node <module>
    Contrib: @FL03
*/
use crate::{LPR, Triad};
use rshyper::EdgeId;
use std::cmp::Ordering;

/// A* search node with priority
#[derive(Clone, Debug)]
pub struct SearchNode {
    /// The estimated total cost (f = g + h)
    pub(crate) priority: i32,
    /// The actual cost so far (g)
    pub(crate) cost: usize,
    /// The current triad
    pub(crate) triad: Triad,
    /// The transformations applied so far
    pub(crate) transforms: Vec<LPR>,
    /// The triads visited so far
    pub(crate) triads: Vec<Triad>,
    /// The edge IDs so far (None for virtual triads)
    pub(crate) edge_ids: Vec<Option<EdgeId>>,
}

impl SearchNode {
    scsys::gsw! {
        cost: usize,
        priority: i32,
        triad: Triad
    }

    scsys::gsw! {
        edge_ids: &Vec<Option<EdgeId>>,
        transforms: &Vec<LPR>,
        triads: &Vec<Triad>
    }
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for SearchNode {}


impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse ordering for max-heap to act as a min-heap
        other.priority.partial_cmp(&self.priority)
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for max-heap to act as a min-heap
        other.priority.cmp(&self.priority)
    }
}
