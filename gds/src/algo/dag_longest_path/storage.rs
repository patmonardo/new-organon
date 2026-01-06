//! DagLongestPath Storage
//!
//! Stores tentative distances and predecessors for longest path computation.

use std::sync::atomic::{AtomicI64, Ordering};

/// Storage for dag longest path computation
pub struct DagLongestPathStorageRuntime {
    /// In-degree for each node (updated during traversal)
    pub in_degrees: Vec<AtomicI64>,
    /// Best distances found to each node (stored as bits for atomic f64)
    pub distances: Vec<AtomicI64>,
    /// Predecessor for each node in the longest path
    pub predecessors: Vec<AtomicI64>,
}

impl DagLongestPathStorageRuntime {
    pub fn new(node_count: usize) -> Self {
        // Initialize distances to -infinity (worst possible for maximization)
        let neg_infinity_bits = f64::NEG_INFINITY.to_bits() as i64;

        Self {
            in_degrees: (0..node_count).map(|_| AtomicI64::new(0)).collect(),
            distances: (0..node_count)
                .map(|_| AtomicI64::new(neg_infinity_bits))
                .collect(),
            predecessors: (0..node_count)
                .map(|_| AtomicI64::new(-1)) // Use -1 as sentinel instead of usize::MAX
                .collect(),
        }
    }

    pub fn get_distance(&self, node: usize) -> f64 {
        let bits = self.distances[node].load(Ordering::SeqCst);
        f64::from_bits(bits as u64)
    }

    pub fn set_distance(&self, node: usize, distance: f64) {
        self.distances[node].store(distance.to_bits() as i64, Ordering::SeqCst);
    }

    pub fn compare_and_update_distance(&self, node: usize, new_distance: f64, predecessor: usize) -> bool {
        loop {
            let current_bits = self.distances[node].load(Ordering::SeqCst);
            let current = f64::from_bits(current_bits as u64);

            if new_distance > current {
                let new_bits = new_distance.to_bits() as i64;
                match self.distances[node].compare_exchange(
                    current_bits,
                    new_bits,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    Ok(_) => {
                        // Successfully updated distance, also set predecessor
                        self.predecessors[node].store(predecessor as i64, Ordering::SeqCst);
                        return true;
                    }
                    Err(_) => continue,
                }
            } else {
                return false;
            }
        }
    }

    pub fn get_predecessor(&self, node: usize) -> Option<usize> {
        let pred = self.predecessors[node].load(Ordering::SeqCst);
        if pred == -1 {
            None
        } else {
            Some(pred as usize)
        }
    }

    pub fn set_predecessor(&self, node: usize, predecessor: usize) {
        self.predecessors[node].store(predecessor as i64, Ordering::SeqCst);
    }
}
