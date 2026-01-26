//! Indirect Exposure specification.
//! Translation source: `org.neo4j.gds.indirectExposure.IndirectExposureConfig`.

use crate::config::validation::ConfigError;
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

impl IndirectExposureConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.sanctioned_property.trim().is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "sanctioned_property".to_string(),
                reason: "sanctioned_property must be provided".to_string(),
            });
        }
        if self.max_iterations == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "max_iterations".to_string(),
                reason: "max_iterations must be greater than 0".to_string(),
            });
        }
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be greater than 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for IndirectExposureConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        IndirectExposureConfig::validate(self)
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
