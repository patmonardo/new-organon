//! RandomWalk Specification
//!
//! **Translation Source**: `org.neo4j.gds.traversal.RandomWalkBaseConfig`

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomWalkConfig {
    /// Number of walks to perform per node
    pub walks_per_node: usize,
    /// Length of each walk (number of steps)
    pub walk_length: usize,
    /// Return factor for node2vec (probability to return to previous node)
    pub return_factor: f64,
    /// In-out factor for node2vec (probability to explore vs exploit)
    pub in_out_factor: f64,
    /// Optional list of source nodes (if empty, walks from all nodes)
    pub source_nodes: Vec<u64>,
    /// Random seed for reproducibility
    pub random_seed: Option<u64>,
}

impl Default for RandomWalkConfig {
    fn default() -> Self {
        Self {
            walks_per_node: 10,
            walk_length: 80,
            return_factor: 1.0,
            in_out_factor: 1.0,
            source_nodes: Vec::new(),
            random_seed: None,
        }
    }
}

/// Result of random walk computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomWalkResult {
    /// Collection of walks (each walk is a sequence of node IDs)
    pub walks: Vec<Vec<u64>>,
}

pub struct RandomWalkAlgorithmSpec {
    graph_name: String,
}

impl RandomWalkAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
