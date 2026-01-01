//! TopologicalSort Storage
//!
//! Stores in-degrees, sorted nodes, and optional longest path distances.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Storage for topological sort computation
pub struct TopologicalSortStorageRuntime {
    /// In-degree for each node (updated during traversal)
    pub in_degrees: Vec<AtomicUsize>,
    /// Sorted nodes in topological order
    pub sorted_nodes: Vec<AtomicUsize>,
    /// Current position in sorted_nodes array
    pub add_index: AtomicUsize,
    /// Optional longest path distances
    pub max_source_distances: Option<Vec<AtomicUsize>>, // Stored as bits for atomic f64
}

impl TopologicalSortStorageRuntime {
    pub fn new(node_count: usize, compute_max_distance: bool) -> Self {
        Self {
            in_degrees: (0..node_count).map(|_| AtomicUsize::new(0)).collect(),
            sorted_nodes: (0..node_count)
                .map(|_| AtomicUsize::new(usize::MAX))
                .collect(),
            add_index: AtomicUsize::new(0),
            max_source_distances: if compute_max_distance {
                Some((0..node_count).map(|_| AtomicUsize::new(0)).collect())
            } else {
                None
            },
        }
    }

    pub fn add_node(&self, node_id: usize) {
        let index = self.add_index.fetch_add(1, Ordering::SeqCst);
        self.sorted_nodes[index].store(node_id, Ordering::SeqCst);
    }

    pub fn size(&self) -> usize {
        self.add_index.load(Ordering::SeqCst)
    }
}
