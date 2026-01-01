//! Modularity specification: configuration and results

/// Configuration for modularity computation
#[derive(Clone, Debug)]
pub struct ModularityConfig {
    /// Community property name to evaluate
    pub community_property: String,
}

impl Default for ModularityConfig {
    fn default() -> Self {
        Self {
            community_property: "community".to_string(),
        }
    }
}

/// Modularity score for a single community
#[derive(Clone, Debug)]
pub struct CommunityModularity {
    pub community_id: u64,
    pub modularity: f64,
}

/// Result of modularity computation
#[derive(Clone, Debug)]
pub struct ModularityResult {
    /// Total modularity score across all communities
    pub total_modularity: f64,
    /// Number of communities
    pub community_count: usize,
    /// Modularity score per community
    pub community_modularities: Vec<CommunityModularity>,
}
