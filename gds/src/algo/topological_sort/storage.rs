//! TopologicalSort Storage
//!
//! Stores in-degrees, sorted nodes, and optional longest path distances.

use crate::types::graph::NodeId;
use std::sync::atomic::{AtomicI64, Ordering};

/// Storage for topological sort computation
pub struct TopologicalSortStorageRuntime {
    /// In-degree for each node (updated during traversal)
    pub in_degrees: Vec<AtomicI64>,
    /// Sorted nodes in topological order
    pub sorted_nodes: Vec<AtomicI64>,
    /// Current position in sorted_nodes array
    pub add_index: AtomicI64,
    /// Optional longest path distances
    pub max_source_distances: Option<Vec<AtomicI64>>, // Stored as bits for atomic f64
}

impl TopologicalSortStorageRuntime {
    pub fn new(node_count: usize, compute_max_distance: bool) -> Self {
        Self {
            in_degrees: (0..node_count).map(|_| AtomicI64::new(0)).collect(),
            sorted_nodes: (0..node_count)
                .map(|_| AtomicI64::new(-1)) // Use -1 as sentinel instead of usize::MAX
                .collect(),
            add_index: AtomicI64::new(0),
            max_source_distances: if compute_max_distance {
                Some((0..node_count).map(|_| AtomicI64::new(0)).collect())
            } else {
                None
            },
        }
    }

    pub fn add_node(&self, node_id: NodeId) {
        let index = self.add_index.fetch_add(1, Ordering::SeqCst);
        self.sorted_nodes[index as usize].store(node_id, Ordering::SeqCst);
    }

    pub fn size(&self) -> usize {
        self.add_index.load(Ordering::SeqCst) as usize
    }
}
