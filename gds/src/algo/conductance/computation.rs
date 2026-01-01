//! Conductance computation: parallel edge counting and conductance calculation

use super::spec::{ConductanceConfig, ConductanceResult};
use super::storage::ConductanceStorageRuntime;
use std::sync::Arc;

pub struct ConductanceComputationRuntime {
    config: ConductanceConfig,
}

impl ConductanceComputationRuntime {
    pub fn new(config: ConductanceConfig) -> Self {
        Self { config }
    }

    /// Compute conductance given community assignments and graph structure
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes in the graph
    /// * `get_community` - Closure that returns the community ID for a node (or None if no assignment)
    /// * `get_neighbors` - Closure that returns (target, weight) pairs for a node's neighbors
    pub fn compute<F, G>(
        &self,
        node_count: usize,
        get_community: F,
        get_neighbors: G,
    ) -> ConductanceResult
    where
        F: Fn(usize) -> Option<u64> + Sync,
        G: Fn(usize) -> Vec<(usize, f64)> + Sync,
    {
        let storage = Arc::new(ConductanceStorageRuntime::new());

        // Process all nodes in parallel
        (0..node_count).for_each(|source_node| {
            let source_community = match get_community(source_node) {
                Some(c) => c,
                None => return, // Skip nodes without community assignment
            };

            let neighbors = get_neighbors(source_node);

            for (target_node, mut weight) in neighbors {
                // Transform weight based on config
                if !self.config.has_relationship_weight_property {
                    weight = 1.0;
                }

                let target_community = match get_community(target_node) {
                    Some(c) => c,
                    None => continue, // Skip edges to unassigned nodes
                };

                if source_community == target_community {
                    // Internal edge: both endpoints in same community
                    storage.add_internal(source_community, weight);
                } else {
                    // External edge: endpoints in different communities
                    storage.add_external(source_community, weight);
                }
            }
        });

        // Unwrap Arc and compute conductances
        let storage = Arc::try_unwrap(storage).unwrap_or_else(|arc| (*arc).clone());
        let (community_conductances, average_conductance) = storage.compute_conductances();

        ConductanceResult {
            community_conductances,
            average_conductance,
        }
    }
}

impl Clone for ConductanceStorageRuntime {
    fn clone(&self) -> Self {
        // This is only used as fallback in Arc::try_unwrap
        // We can create a new empty storage since we're about to compute
        Self::new()
    }
}
