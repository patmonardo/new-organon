use crate::api::Graph;
use crate::core::utils::progress::ProgressTracker;
use crate::concurrency::TerminationFlag;
use crate::config::base_types::Config;
use crate::procedures::bfs::{BfsStorageRuntime, BfsComputationRuntime, BfsConfig as ProceduresBfsConfig};
use crate::types::graph::id_map::NodeId;

// Make ProceduresBfsConfig implement Config trait
impl Config for ProceduresBfsConfig {}

/// Placeholder for HugeLongArray - represents a large array of long values
#[derive(Debug, Clone)]
pub struct HugeLongArray {
    data: Vec<i64>,
}

impl HugeLongArray {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }

    pub fn get(&self, index: usize) -> i64 {
        self.data[index]
    }

    pub fn set(&mut self, index: usize, value: i64) {
        self.data[index] = value;
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn from_distances(node_count: usize, distances: &[(NodeId, u32)]) -> Self {
        let mut data = vec![-1i64; node_count];
        for &(node_id, distance) in distances {
            if node_id >= 0 && (node_id as usize) < node_count {
                data[node_id as usize] = distance as i64;
            }
        }
        Self { data }
    }
}

/// Base configuration for BFS algorithm
pub trait BfsBaseConfig: Config {
    fn has_target_nodes(&self) -> bool;
    fn target_nodes(&self) -> Vec<i64>;
    fn has_max_depth(&self) -> bool;
    fn max_depth(&self) -> Option<i32>;
    fn source_node(&self) -> i64;
    fn concurrency(&self) -> usize;
}

// Make ProceduresBfsConfig implement BfsBaseConfig trait
impl BfsBaseConfig for ProceduresBfsConfig {
    fn has_target_nodes(&self) -> bool {
        !self.target_nodes.is_empty()
    }

    fn target_nodes(&self) -> Vec<i64> {
        self.target_nodes.iter().map(|&n| n as i64).collect()
    }

    fn has_max_depth(&self) -> bool {
        self.max_depth.is_some()
    }

    fn max_depth(&self) -> Option<i32> {
        self.max_depth.map(|d| d as i32)
    }

    fn source_node(&self) -> i64 {
        self.source_node as i64
    }

    fn concurrency(&self) -> usize {
        self.concurrency
    }
}

/// Breadth-First Search algorithm implementation
pub struct BreadthFirstSearch;

impl BreadthFirstSearch {
    pub fn compute<C: BfsBaseConfig>(
        &self,
        graph: &Graph,
        configuration: &C,
        _progress_tracker: ProgressTracker,
        _termination_flag: TerminationFlag,
    ) -> HugeLongArray {
        // Convert trait-based config to concrete ProceduresBfsConfig
        let bfs_config = ProceduresBfsConfig {
            source_node: configuration.source_node() as NodeId,
            target_nodes: configuration.target_nodes().iter().map(|&n| n as NodeId).collect(),
            max_depth: configuration.max_depth().map(|d| d as u32),
            track_paths: false, // For now, just distances
            concurrency: configuration.concurrency(),
            delta: 64, // Default delta
        };

        // Create storage and computation runtimes
        let storage = BfsStorageRuntime::new(
            bfs_config.source_node,
            bfs_config.target_nodes.clone(),
            bfs_config.max_depth,
            bfs_config.track_paths,
            bfs_config.concurrency,
            bfs_config.delta,
        );

        let mut computation = BfsComputationRuntime::new(
            bfs_config.source_node,
            bfs_config.track_paths,
            bfs_config.concurrency,
        );

        // Execute BFS algorithm
        let result = storage.compute_bfs(&mut computation, Some(graph))
            .unwrap_or_else(|_e| {
                // On error, return empty result
                crate::procedures::bfs::BfsResult {
                    visited_nodes: vec![],
                    paths: vec![],
                    nodes_visited: 0,
                    computation_time_ms: 0,
                }
            });

        // Extract distances from visited_nodes and create HugeLongArray
        let node_count = graph.node_count();
        HugeLongArray::from_distances(node_count, &result.visited_nodes)
    }
}
