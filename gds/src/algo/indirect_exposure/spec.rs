//! Indirect Exposure specification.
//! Translation source: `org.neo4j.gds.indirectExposure.IndirectExposureConfig`.

use serde::{Deserialize, Serialize};

/// Configuration for indirect exposure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndirectExposureConfig {
    /// Name of the boolean/flag node property indicating sanctioned nodes (1 = sanctioned).
    pub sanctioned_property: String,
    /// Optional relationship weight property name. When absent, unweighted degrees are used.
    pub relationship_weight_property: Option<String>,
    /// Maximum Pregel iterations (supersteps).
    pub max_iterations: usize,
    /// Concurrency hint for Pregel execution.
    pub concurrency: usize,
}

impl Default for IndirectExposureConfig {
    fn default() -> Self {
        Self {
            sanctioned_property: "sanctioned".to_string(),
            relationship_weight_property: None,
            max_iterations: 20,
            concurrency: 4,
        }
    }
}

/// Result of indirect exposure computation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndirectExposureResult {
    pub exposures: Vec<f64>,
    pub roots: Vec<i64>,
    pub parents: Vec<i64>,
    pub hops: Vec<i64>,
    pub iterations_ran: usize,
    pub did_converge: bool,
}

/// Catalog marker.
pub struct IndirectExposureAlgorithmSpec {
    graph_name: String,
}

impl IndirectExposureAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
