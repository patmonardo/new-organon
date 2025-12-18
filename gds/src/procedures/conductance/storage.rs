//! Conductance storage: accumulates edge counts per community

use std::collections::HashMap;
use std::sync::Mutex;

/// Storage for conductance computation
pub struct ConductanceStorageRuntime {
    /// Internal edge weight (edges within each community)
    internal_counts: Mutex<HashMap<u64, f64>>,
    /// External edge weight (edges crossing community boundaries)
    external_counts: Mutex<HashMap<u64, f64>>,
}

impl Default for ConductanceStorageRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ConductanceStorageRuntime {
    pub fn new() -> Self {
        Self {
            internal_counts: Mutex::new(HashMap::new()),
            external_counts: Mutex::new(HashMap::new()),
        }
    }

    /// Add internal edge weight for a community
    pub fn add_internal(&self, community_id: u64, weight: f64) {
        let mut counts = self.internal_counts.lock().unwrap();
        *counts.entry(community_id).or_insert(0.0) += weight;
    }

    /// Add external edge weight for a community
    pub fn add_external(&self, community_id: u64, weight: f64) {
        let mut counts = self.external_counts.lock().unwrap();
        *counts.entry(community_id).or_insert(0.0) += weight;
    }

    /// Compute conductances from accumulated counts
    pub fn compute_conductances(self) -> (HashMap<u64, f64>, f64) {
        let internal_counts = self.internal_counts.into_inner().unwrap();
        let external_counts = self.external_counts.into_inner().unwrap();

        let mut community_conductances = HashMap::new();
        let mut sum = 0.0;
        let mut count = 0;

        // Get all communities that have any edges
        let mut all_communities: Vec<u64> = internal_counts
            .keys()
            .chain(external_counts.keys())
            .copied()
            .collect();
        all_communities.sort_unstable();
        all_communities.dedup();

        for community_id in all_communities {
            let internal = internal_counts.get(&community_id).copied().unwrap_or(0.0);
            let external = external_counts.get(&community_id).copied().unwrap_or(0.0);

            // Conductance = external / (external + internal)
            let total = external + internal;
            if total > 0.0 {
                let conductance = external / total;
                community_conductances.insert(community_id, conductance);
                sum += conductance;
                count += 1;
            }
        }

        let average = if count > 0 { sum / count as f64 } else { 0.0 };

        (community_conductances, average)
    }
}
