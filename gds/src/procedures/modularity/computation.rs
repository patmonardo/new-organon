//! Modularity computation: parallel relationship counting and modularity calculation

use super::spec::{CommunityModularity, ModularityResult};
use super::storage::ModularityStorageRuntime;
use std::sync::Arc;

pub struct ModularityComputationRuntime;

impl Default for ModularityComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ModularityComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Compute modularity given community assignments and graph structure
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
    ) -> ModularityResult
    where
        F: Fn(usize) -> Option<u64> + Sync,
        G: Fn(usize) -> Vec<(usize, f64)> + Sync,
    {
        let storage = Arc::new(ModularityStorageRuntime::new());

        // Process all nodes in parallel
        (0..node_count).for_each(|source_node| {
            let source_community = match get_community(source_node) {
                Some(c) => c,
                None => return, // Skip nodes without community assignment
            };

            let neighbors = get_neighbors(source_node);

            for (target_node, weight) in neighbors {
                let target_community = match get_community(target_node) {
                    Some(c) => c,
                    None => continue, // Skip edges to unassigned nodes
                };

                // Track inside relationships (both endpoints in same community)
                if source_community == target_community {
                    storage.add_inside(source_community, weight);
                }

                // Track total community relationships (all edges from this community)
                storage.add_total_community(source_community, weight);

                // Track total weight
                storage.add_total_weight(weight);
            }
        });

        // Unwrap Arc and compute modularity
        let storage = Arc::try_unwrap(storage).unwrap_or_else(|arc| (*arc).clone());
        let (total_modularity, community_scores) = storage.compute_modularity();

        let community_modularities: Vec<CommunityModularity> = community_scores
            .into_iter()
            .map(|(community_id, modularity)| CommunityModularity {
                community_id,
                modularity,
            })
            .collect();

        ModularityResult {
            total_modularity,
            community_count: community_modularities.len(),
            community_modularities,
        }
    }
}

impl Clone for ModularityStorageRuntime {
    fn clone(&self) -> Self {
        // Fallback for Arc::try_unwrap
        Self::new()
    }
}
