/*
    Appellation: tonnetz-graph <module>
    Contrib: @FL03
*/
use core::str::FromStr;
use rstmt_core::prelude::{Pitch, Pitches};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeState {
    class: Pitches,
    pitch: Pitch,
    index: usize,
}

impl NodeState {
    pub fn new(class: &str, weight: i8, index: usize) -> Self {
        Self {
            class: Pitches::from_str(class).unwrap(),
            pitch: Pitch(weight),
            index,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TonnetzNode {
    pitch_class: NodeState,
    neighbors: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct TonnetzGraph {
    nodes: HashMap<usize, TonnetzNode>,
}

impl TonnetzGraph {
    pub fn new<I: IntoIterator<Item = NodeState>>(scale: I) -> Self {
        let mut nodes = HashMap::new();
        for (i, pitch_class) in scale.into_iter().enumerate() {
            nodes.insert(i, TonnetzNode {
                pitch_class: pitch_class.clone(),
                neighbors: Vec::new(),
            });
        }
        TonnetzGraph { nodes }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        if let Some(node) = self.nodes.get_mut(&from) {
            node.neighbors.push(to);
        }
    }

    pub fn get_neighbors(&self, index: usize) -> Option<&Vec<usize>> {
        self.nodes.get(&index).map(|node| &node.neighbors)
    }

    pub fn resolve_pitch_class(&self, index: usize) -> Option<&NodeState> {
        self.nodes.get(&index).map(|node| &node.pitch_class)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;

    lazy_static! {
        static ref C_MAJOR_SCALE: [NodeState; 7] = [
            NodeState::new("C", 0, 0),
            NodeState::new("D", 2, 1),
            NodeState::new("E", 4, 2),
            NodeState::new("F", 5, 3),
            NodeState::new("G", 7, 4),
            NodeState::new("A", 9, 5),
            NodeState::new("B", 11, 6),
        ];
    }
    #[test]
    fn test_tgraph() {
        let mut graph = TonnetzGraph::new(C_MAJOR_SCALE.to_vec());

        // Example of adding edges
        graph.add_edge(0, 2);
        graph.add_edge(2, 4);
        graph.add_edge(4, 5);

        // Example of resolving pitch class
        if let Some(pitch_class) = graph.resolve_pitch_class(0) {
            println!("Pitch class at index 0: {:?}", pitch_class);
        }

        // Example of getting neighbors
        if let Some(neighbors) = graph.get_neighbors(0) {
            println!("Neighbors of node 0: {:?}", neighbors);
        }
    }
}
