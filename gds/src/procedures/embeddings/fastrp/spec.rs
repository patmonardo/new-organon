//! FastRP Algorithm Specification
//!
//! Implements the `AlgorithmSpec` contract for the executor runtime.

use crate::define_algorithm_spec;
use crate::projection::eval::procedure::{AlgorithmError, LogLevel};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use serde::{Deserialize, Serialize};

use super::computation::FastRPComputationRuntime;
use super::storage::FastRPStorageRuntime;

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
    modes: [Stream, Stats],

    execute: |_self, graph_store, config_input, context| {
        let config: FastRPConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Failed to parse FastRP config: {e}")))?;

        if config.embedding_dimension == 0 {
            return Err(AlgorithmError::Execution("embeddingDimension must be > 0".into()));
        }
        if config.property_dimension > config.embedding_dimension {
            return Err(AlgorithmError::Execution(
                "propertyDimension must be <= embeddingDimension".into(),
            ));
        }
        if config.iteration_weights.is_empty() {
            return Err(AlgorithmError::Execution("iterationWeights must be non-empty".into()));
        }

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
            .map_err(|e| AlgorithmError::Execution(e))?;

        let result = FastRPComputationRuntime::run(
            graph_view,
            &config,
            feature_extractors,
        )?;

        Ok(result)
    }
}

// The `define_algorithm_spec!` macro generates `FASTRPAlgorithmSpec`.
// Provide a stable alias with conventional casing.
pub type FastRPAlgorithmSpec = FASTRPAlgorithmSpec;
