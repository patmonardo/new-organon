//! Modularity storage: accumulates relationship counts per community

use std::collections::HashMap;
use std::sync::Mutex;

/// Storage for modularity computation
pub struct ModularityStorageRuntime {
    /// Inside relationships (edges within each community)
    inside_relationships: Mutex<HashMap<u64, f64>>,
    /// Total relationships touching each community
    total_community_relationships: Mutex<HashMap<u64, f64>>,
    /// Total relationship weight in the graph
    total_relationship_weight: Mutex<f64>,
}

impl Default for ModularityStorageRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ModularityStorageRuntime {
    pub fn new() -> Self {
        Self {
            inside_relationships: Mutex::new(HashMap::new()),
            total_community_relationships: Mutex::new(HashMap::new()),
            total_relationship_weight: Mutex::new(0.0),
        }
    }

    /// Add inside relationship weight for a community
    pub fn add_inside(&self, community_id: u64, weight: f64) {
        let mut counts = self.inside_relationships.lock().unwrap();
        *counts.entry(community_id).or_insert(0.0) += weight;
    }

    /// Add total community relationship weight
    pub fn add_total_community(&self, community_id: u64, weight: f64) {
        let mut counts = self.total_community_relationships.lock().unwrap();
        *counts.entry(community_id).or_insert(0.0) += weight;
    }

    /// Add to total relationship weight
    pub fn add_total_weight(&self, weight: f64) {
        let mut total = self.total_relationship_weight.lock().unwrap();
        *total += weight;
    }

    /// Compute modularity scores from accumulated counts
    pub fn compute_modularity(self) -> (f64, Vec<(u64, f64)>) {
        let inside = self.inside_relationships.into_inner().unwrap();
        let total_community = self.total_community_relationships.into_inner().unwrap();
        let total_weight = *self.total_relationship_weight.lock().unwrap();

        if total_weight == 0.0 {
            return (0.0, Vec::new());
        }

        let mut community_modularities = Vec::new();
        let mut total_modularity = 0.0;

        // Get all communities
        let mut all_communities: Vec<u64> = inside
            .keys()
            .chain(total_community.keys())
            .copied()
            .collect();
        all_communities.sort_unstable();
        all_communities.dedup();

        for community_id in all_communities {
            let ec = inside.get(&community_id).copied().unwrap_or(0.0); // Inside edges
            let kc = total_community.get(&community_id).copied().unwrap_or(0.0); // Total edges

            // Modularity formula: (ec - kc*kc/total_weight) / total_weight
            let modularity = (ec - kc * kc / total_weight) / total_weight;

            total_modularity += modularity;
            community_modularities.push((community_id, modularity));
        }

        (total_modularity, community_modularities)
    }
}
