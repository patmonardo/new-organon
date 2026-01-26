//! FastRP Algorithm Specification
//!
//! Implements the `AlgorithmSpec` contract for the executor runtime.

use crate::collections::backends::vec::VecFloatArray;
use crate::config::validation::ConfigError;
use crate::core::LogLevel;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::NodeLabel;
use crate::projection::RelationshipType;
use crate::types::properties::node::{DefaultFloatArrayNodePropertyValues, NodePropertyValues};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

use super::FastRPStorageRuntime;
use super::FastRPComputationRuntime;

// ============================================================================
// Configuration
// ============================================================================

/// FastRP configuration.
///
/// JSON field naming is Java GDS aligned (camelCase).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FastRPConfig {
    /// Node feature properties to include in the embedding tail (optional).
    #[serde(default)]
    pub feature_properties: Vec<String>,

    /// Iteration weights. Length controls number of propagation steps.
    #[serde(default = "FastRPConfig::default_iteration_weights")]
    pub iteration_weights: Vec<f32>,

    /// Total embedding dimension.
    #[serde(default = "FastRPConfig::default_embedding_dimension")]
    pub embedding_dimension: usize,

    /// Dimension reserved for projected property features at the tail.
    #[serde(default)]
    pub property_dimension: usize,

    /// Relationship weight property to use (optional).
    #[serde(default)]
    pub relationship_weight_property: Option<String>,

    /// Normalization exponent applied to degree during initialization.
    #[serde(default)]
    pub normalization_strength: f32,

    /// Contribution of the initial random vector into the final embedding.
    #[serde(default = "FastRPConfig::default_node_self_influence")]
    pub node_self_influence: f32,

    /// Concurrency hint (currently unused; computation runs single-threaded).
    #[serde(default = "FastRPConfig::default_concurrency")]
    pub concurrency: usize,

    /// Batch size hint (currently unused; kept for parity).
    #[serde(default = "FastRPConfig::default_min_batch_size")]
    pub min_batch_size: usize,

    /// Optional random seed.
    #[serde(default)]
    pub random_seed: Option<u64>,
}

impl FastRPConfig {
    fn default_embedding_dimension() -> usize {
        128
    }

    fn default_iteration_weights() -> Vec<f32> {
        // Keep this conservative/minimal: two propagation steps.
        vec![1.0, 1.0]
    }

    fn default_node_self_influence() -> f32 {
        1.0
    }

    fn default_concurrency() -> usize {
        num_cpus::get().max(1)
    }

    fn default_min_batch_size() -> usize {
        10_000
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.embedding_dimension == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "embeddingDimension".to_string(),
                reason: "embeddingDimension must be > 0".to_string(),
            });
        }
        if self.property_dimension > self.embedding_dimension {
            return Err(ConfigError::InvalidParameter {
                parameter: "propertyDimension".to_string(),
                reason: "propertyDimension must be <= embeddingDimension".to_string(),
            });
        }
        if self.iteration_weights.is_empty() {
            return Err(ConfigError::InvalidParameter {
                parameter: "iterationWeights".to_string(),
                reason: "iterationWeights must be non-empty".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for FastRPConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        FastRPConfig::validate(self)
    }
}

impl Default for FastRPConfig {
    fn default() -> Self {
        Self {
            feature_properties: Vec::new(),
            iteration_weights: Self::default_iteration_weights(),
            embedding_dimension: Self::default_embedding_dimension(),
            property_dimension: 0,
            relationship_weight_property: None,
            normalization_strength: 0.0,
            node_self_influence: Self::default_node_self_influence(),
            concurrency: Self::default_concurrency(),
            min_batch_size: Self::default_min_batch_size(),
            random_seed: None,
        }
    }
}

// ============================================================================
// Result
// ============================================================================

/// FastRP result: per-node embeddings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FastRPResult {
    pub embeddings: Vec<Vec<f32>>,
}

// ============================================================================
// Algorithm Spec
// ============================================================================

define_algorithm_spec! {
    name: "fastrp",
    output_type: FastRPResult,
    projection_hint: Dense,
    modes: [Stream, Stats, MutateNodeProperty],

    execute: |_self, graph_store, config_input, context| {
        let config: FastRPConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Failed to parse FastRP config: {e}")))?;

        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        context.log(
            LogLevel::Info,
            &format!(
                "Running FastRP: nodes={}, dim={}, iters={}",
                graph_store.node_count(),
                config.embedding_dimension,
                config.iteration_weights.len(),
            ),
        );

        let rel_types: std::collections::HashSet<RelationshipType> = std::collections::HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to obtain graph view: {e}")))?;

        let storage = FastRPStorageRuntime::new();
        let feature_extractors = storage
            .feature_extractors(graph_view.as_ref(), &config.feature_properties)
            .map_err(AlgorithmError::Execution)?;

        let result = FastRPComputationRuntime::run(
            graph_view,
            &config,
            feature_extractors,
        )?;

        Ok(result)
    },

    mutate_node_property: |_self, graph_store, config, result| {
        let mutate_property = config
            .get("mutateProperty")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AlgorithmError::Execution("Missing mutateProperty".to_string()))?;

        let labels: HashSet<NodeLabel> = config
            .get("nodeLabels")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| NodeLabel::of(s.to_string())))
                    .collect()
            })
            .unwrap_or_else(|| graph_store.node_labels());

        let node_count = graph_store.node_count();
        if result.embeddings.len() != node_count {
            return Err(AlgorithmError::Execution(format!(
                "fastrp returned {} embeddings for {} nodes",
                result.embeddings.len(),
                node_count
            )));
        }

        let dense: Vec<Option<Vec<f32>>> = result.embeddings.clone().into_iter().map(Some).collect();
        let backend = VecFloatArray::from(dense);
        let values = DefaultFloatArrayNodePropertyValues::<VecFloatArray>::from_collection(
            backend, node_count,
        );
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        graph_store
            .add_node_property(labels, mutate_property.to_string(), values)
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        Ok(node_count)
    }
}

// The `define_algorithm_spec!` macro generates `FASTRPAlgorithmSpec`.
// Provide a stable alias with conventional casing.
pub type FastRPAlgorithmSpec = FASTRPAlgorithmSpec;
