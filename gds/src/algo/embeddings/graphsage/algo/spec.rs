//! GraphSAGE Algorithm Specification
//!
//! GraphSAGE produces dense embeddings by sampling and aggregating
//! neighborhood features through trained neural network layers.
//!
//! This spec exposes a canonical config/result surface and delegates execution
//! to the computation runtime.

use crate::collections::HugeObjectArray;
use crate::config::validation::ConfigError;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use serde::{Deserialize, Serialize};

use super::GraphSageStorageRuntime;

// ============================================================================
// Configuration
// ============================================================================

/// Config for GraphSAGE inference/embeddings generation.
///
/// JSON field naming is Java GDS aligned (camelCase).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphSageConfig {
    /// Model username
    pub model_user: String,

    /// Model name
    pub model_name: String,

    /// Batch size for inference
    #[serde(default = "GraphSageConfig::default_batch_size")]
    pub batch_size: usize,

    /// Concurrency level
    #[serde(default = "GraphSageConfig::default_concurrency")]
    pub concurrency: usize,
}

impl GraphSageConfig {
    fn default_batch_size() -> usize {
        100
    }

    fn default_concurrency() -> usize {
        num_cpus::get().max(1)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.model_name.is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "modelName".to_string(),
                reason: "modelName must be specified".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for GraphSageConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        GraphSageConfig::validate(self)
    }
}

impl Default for GraphSageConfig {
    fn default() -> Self {
        Self {
            model_user: "anonymous".to_string(),
            model_name: "".to_string(),
            batch_size: Self::default_batch_size(),
            concurrency: Self::default_concurrency(),
        }
    }
}

// ============================================================================
// Result
// ============================================================================

/// GraphSAGE result containing node embeddings.
pub struct GraphSageResult {
    pub embeddings: HugeObjectArray<Vec<f64>>,
    pub embedding_dimension: usize,
    pub node_count: usize,
}

// ============================================================================
// Algorithm Spec
// ============================================================================

define_algorithm_spec! {
    name: "graphsage",
    output_type: GraphSageResult,
    projection_hint: Dense,
    modes: [Stream],

    execute: |_self, graph_store, config_input, _context| {
        let config: GraphSageConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Failed to parse GraphSAGE config: {e}")))?;

        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        // Load graph
        let rel_types = std::collections::HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        // Run computation
        Ok(GraphSageStorageRuntime::new()
            .compute(graph.as_ref(), &config))
    }
}
