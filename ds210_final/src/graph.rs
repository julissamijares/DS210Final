use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Graph {
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, source: usize, target: usize) {
        self.edges.entry(source).or_insert(Vec::new()).push(target);
        self.edges.entry(target).or_insert(Vec::new()).push(source);
    }

    pub fn degree(&self, node: usize) -> usize {
        self.edges.get(&node).map_or(0, |neighbors| neighbors.len())
    }

    pub fn neighbors_at_distance_2(&self, node: usize) -> HashSet<usize> {
        let mut neighbors = HashSet::new();
        if let Some(adjacent_nodes) = self.edges.get(&node) {
            for &adjacent_node in adjacent_nodes {
                if let Some(second_order_nodes) = self.edges.get(&adjacent_node) {
                    neighbors.extend(second_order_nodes.iter().filter(|&&n| n != node));
                }
            }
        }
        neighbors
    }

    pub fn nodes(&self) -> Vec<usize> {
        self.edges.keys().cloned().collect()
    }
}