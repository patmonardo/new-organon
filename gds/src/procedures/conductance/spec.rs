//! Conductance specification: configuration and results

use std::collections::HashMap;

/// Configuration for conductance computation
#[derive(Clone, Debug)]
#[derive(Default)]
pub struct ConductanceConfig {
    /// Whether to use relationship weights (true) or treat all edges as weight 1.0 (false)
    pub has_relationship_weight_property: bool,
}


/// Result of conductance computation
#[derive(Clone, Debug)]
pub struct ConductanceResult {
    /// Conductance per community (sparse: only communities with nodes)
    pub community_conductances: HashMap<u64, f64>,
    /// Global average conductance across all communities
    pub average_conductance: f64,
}
