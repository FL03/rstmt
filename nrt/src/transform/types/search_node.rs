/*
    Appellation: search_node <module>
    Contrib: @FL03
*/
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::{LPR, Triad};
use core::cmp::Ordering;
use rshyper::EdgeId;

/// A* search node with priority
#[derive(Clone, Debug)]
pub struct SearchNode {
    /// The actual cost so far (g)
    pub(crate) cost: usize,
    /// The edge IDs so far (None for virtual triads)
    pub(crate) edges: Vec<Option<EdgeId>>,
    /// The estimated total cost (f = g + h)
    pub(crate) priority: i32,
    /// The current triad
    pub(crate) triad: Triad,
    /// The transformations applied so far
    pub(crate) transforms: Vec<LPR>,
    /// The triads visited so far
    pub(crate) visited: Vec<Triad>,
}

impl SearchNode {
    /// returns a copy of the current cost of the node
    pub const fn cost(&self) -> usize {
        self.cost
    }
    /// returns a mutable reference to the current cost
    pub const fn cost_mut(&mut self) -> &mut usize {
        &mut self.cost
    }
    /// returns a reference to the edge IDs
    pub const fn edges(&self) -> &Vec<Option<EdgeId>> {
        &self.edges
    }
    /// returns a mutable reference to the edge IDs
    pub const fn edges_mut(&mut self) -> &mut Vec<Option<EdgeId>> {
        &mut self.edges
    }
    /// returns a copy to the current priority
    pub const fn priority(&self) -> i32 {
        self.priority
    }
    /// returns a mutable reference to the current priority
    pub const fn priority_mut(&mut self) -> &mut i32 {
        &mut self.priority
    }
    /// returns a reference to the transformations
    pub const fn transforms(&self) -> &Vec<LPR> {
        &self.transforms
    }
    /// returns a mutable reference to the transformations
    pub const fn transforms_mut(&mut self) -> &mut Vec<LPR> {
        &mut self.transforms
    }
    /// returns a copy of the traid
    pub const fn triad(&self) -> Triad {
        self.triad
    }
    /// returns a mutable reference to the current triad
    pub const fn triad_mut(&mut self) -> &mut Triad {
        &mut self.triad
    }
    /// returns a reference to the triads visited so far
    pub const fn visited(&self) -> &Vec<Triad> {
        &self.visited
    }
    /// returns a mutable reference to the triads visited so far
    pub const fn visited_mut(&mut self) -> &mut Vec<Triad> {
        &mut self.visited
    }
    /// set the current cost and return a mutable reference to the node
    pub fn set_cost(&mut self, cost: usize) -> &mut Self {
        self.cost = cost;
        self
    }
    /// set the current priority and return a mutable reference to the node
    pub fn set_priority(&mut self, priority: i32) -> &mut Self {
        self.priority = priority;
        self
    }
    /// set the current triad and return a mutable reference to the node
    pub fn set_triad(&mut self, triad: Triad) -> &mut Self {
        self.triad = triad;
        self
    }
    /// set the current edges and return a mutable reference to the node
    pub fn set_edges(&mut self, edges: Vec<Option<EdgeId>>) -> &mut Self {
        self.edges = edges;
        self
    }
    /// set the current transformations and return a mutable reference to the node
    pub fn set_transforms(&mut self, transforms: Vec<LPR>) -> &mut Self {
        self.transforms = transforms;
        self
    }
    /// set the visited triads and return a mutable reference to the node
    pub fn set_visited(&mut self, visited: Vec<Triad>) -> &mut Self {
        self.visited = visited;
        self
    }
    /// consumes the current instance to create another with the given cost
    pub fn with_cost(self, cost: usize) -> Self {
        Self { cost, ..self }
    }
    /// consumes the current instance to create another with the given priority
    pub fn with_priority(self, priority: i32) -> Self {
        Self { priority, ..self }
    }
    /// consumes the current instance to create another with the given triad
    pub fn with_triad(self, triad: Triad) -> Self {
        Self { triad, ..self }
    }
    /// consumes the current instance to create another with the given edges
    pub fn with_edges(self, edges: Vec<Option<EdgeId>>) -> Self {
        Self { edges, ..self }
    }
    /// consumes the current instance to create another with the given transformations
    pub fn with_transforms(self, transforms: Vec<LPR>) -> Self {
        Self { transforms, ..self }
    }

    /// extend the current visited triads with an iterator and return a mutable reference to
    /// the node
    pub fn extend_visited<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = Triad>,
    {
        self.visited.extend(iter);
        self
    }
    /// extend the current edges with an iterator and return a mutable reference to the node
    pub fn extend_edges<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = Option<EdgeId>>,
    {
        self.edges.extend(iter);
        self
    }
    /// extend the current transformations with an iterator and return a mutable reference to
    /// the node
    pub fn extend_transforms<I>(&mut self, iter: I) -> &mut Self
    where
        I: IntoIterator<Item = LPR>,
    {
        self.transforms.extend(iter);
        self
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
