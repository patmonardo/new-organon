use std::collections::HashMap;

/// Configuration for conductance.
///
/// Java parity reference: `ConductanceParameters` / `ConductanceConfigTransformer`.
#[derive(Debug, Clone)]
pub struct ConductanceConfig {
    /// Concurrency used for relationship counting.
    pub concurrency: usize,

    /// Minimum batch size for degree partitioning.
    pub min_batch_size: usize,

    /// When `true`, relationship weights are taken from the projected graph.
    /// When `false`, every relationship contributes weight `1.0`.
    pub has_relationship_weight_property: bool,

    /// Node property key storing community IDs (non-negative long values).
    pub community_property: String,
}

impl Default for ConductanceConfig {
    fn default() -> Self {
        Self {
            concurrency: 4,
            min_batch_size: 10_000,
            has_relationship_weight_property: false,
            community_property: String::new(),
        }
    }
}

/// Result of conductance computation.
#[derive(Debug, Clone, PartialEq)]
pub struct ConductanceResult {
    /// Per-community conductance values.
    pub community_conductances: HashMap<u64, f64>,

    /// Global average conductance over communities with counts.
    pub global_average_conductance: f64,
}
