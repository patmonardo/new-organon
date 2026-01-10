use crate::types::graph_store::DefaultGraphStore;
use std::collections::HashMap;

use crate::types::graph::Degrees as _;
use crate::types::graph::IdMap as _;

/// DegreeDistributionApplier
///
/// Small utility that computes a simple degree histogram for a `DefaultGraphStore`.
pub struct DegreeDistributionApplier;

impl DegreeDistributionApplier {
    pub fn new() -> Self {
        Self
    }

    /// Compute a degree histogram: map degree -> count
    pub fn compute_histogram(&self, store: &DefaultGraphStore) -> HashMap<u32, u64> {
        let graph = store.graph();
        let node_count = graph.node_count();
        let mut hist: HashMap<u32, u64> = HashMap::new();

        for mapped in 0..node_count {
            let deg = graph.degree(mapped as i64);
            let key = deg as u32;
            *hist.entry(key).or_insert(0) += 1;
        }

        hist
    }
}

impl Default for DegreeDistributionApplier {
    fn default() -> Self {
        Self::new()
    }
}
