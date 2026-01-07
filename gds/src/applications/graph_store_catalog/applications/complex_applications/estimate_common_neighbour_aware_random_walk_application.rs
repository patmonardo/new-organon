use crate::applications::graph_store_catalog::results::MemoryEstimateResult;
use crate::types::graph_store::GraphStore;

use serde_json::json;
use std::collections::HashMap;

/// Estimates memory usage for Common Neighbour Aware Random Walk.
///
/// Java parity: returns `MemoryEstimateResult`.
/// Rust pass-1: deterministic estimate based on graph size and configuration.
pub struct EstimateCommonNeighbourAwareRandomWalkApplication;

impl EstimateCommonNeighbourAwareRandomWalkApplication {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(
        &self,
        graph_store: &impl GraphStore,
        configuration: &serde_json::Value,
    ) -> MemoryEstimateResult {
        let n = graph_store.node_count() as u64;
        let r = graph_store.relationship_count() as u64;

        // Very simple model: base + per-node + per-relationship.
        let base = 1_000_000u64;
        let per_node = 64u64;
        let per_rel = 48u64;
        let estimate = base + (n * per_node) + (r * per_rel);

        let mut details: HashMap<String, serde_json::Value> = HashMap::new();
        details.insert("nodeCount".to_string(), json!(n));
        details.insert("relationshipCount".to_string(), json!(r));
        details.insert("configuration".to_string(), configuration.clone());

        MemoryEstimateResult::new(estimate, details)
    }
}

impl Default for EstimateCommonNeighbourAwareRandomWalkApplication {
    fn default() -> Self {
        Self::new()
    }
}
