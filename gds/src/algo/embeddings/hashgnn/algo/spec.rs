//! HashGNN Algorithm Specification
//!
//! HashGNN produces either:
//! - a **binary** embedding (bitset per node), or
//! - a **dense** embedding (vector per node) when `outputDimension` is set.
//!
//! This spec exposes a canonical config/result surface and delegates execution
//! to the computation runtime.

use crate::config::validation::ConfigError;
use crate::core::LogLevel;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use serde::{Deserialize, Serialize};

use super::HashGNNStorageRuntime;
use super::HashGNNComputationRuntime;

// ============================================================================
// Configuration
// ============================================================================

/// Config for generating random base features when no `featureProperties` are provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateFeaturesConfig {
    pub dimension: usize,
    pub density_level: usize,
}

/// Config for binarizing scalar/array properties into a binary embedding.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BinarizeFeaturesConfig {
    pub dimension: usize,
    pub threshold: f64,
}

/// HashGNN configuration.
///
/// JSON field naming is Java GDS aligned (camelCase).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashGNNConfig {
    #[serde(default = "HashGNNConfig::default_iterations")]
    pub iterations: usize,

    #[serde(default = "HashGNNConfig::default_embedding_density")]
    pub embedding_density: usize,

    #[serde(default = "HashGNNConfig::default_neighbor_influence")]
    pub neighbor_influence: f64,

    #[serde(default)]
    pub feature_properties: Vec<String>,

    #[serde(default)]
    pub heterogeneous: bool,

    /// If set, densifies to a dense embedding of this dimension.
    #[serde(default)]
    pub output_dimension: Option<usize>,

    #[serde(default)]
    pub binarize_features: Option<BinarizeFeaturesConfig>,

    #[serde(default)]
    pub generate_features: Option<GenerateFeaturesConfig>,

    #[serde(default = "HashGNNConfig::default_concurrency")]
    pub concurrency: usize,

    #[serde(default)]
    pub random_seed: Option<u64>,
}

impl HashGNNConfig {
    fn default_iterations() -> usize {
        2
    }

    fn default_embedding_density() -> usize {
        4
    }

    fn default_neighbor_influence() -> f64 {
        1.0
    }

    fn default_concurrency() -> usize {
        num_cpus::get().max(1)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.iterations == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "iterations".to_string(),
                reason: "iterations must be > 0".to_string(),
            });
        }
        if self.embedding_density == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "embeddingDensity".to_string(),
                reason: "embeddingDensity must be > 0".to_string(),
            });
        }
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be > 0".to_string(),
            });
        }
        if let Some(out) = self.output_dimension {
            if out == 0 {
                return Err(ConfigError::InvalidParameter {
                    parameter: "outputDimension".to_string(),
                    reason: "outputDimension must be > 0".to_string(),
                });
            }
        }
        if self.feature_properties.is_empty() && self.generate_features.is_none() {
            return Err(ConfigError::InvalidParameter {
                parameter: "featureProperties/generateFeatures".to_string(),
                reason: "HashGNN requires either featureProperties or generateFeatures".to_string(),
            });
        }
        if let Some(cfg) = &self.generate_features {
            if cfg.dimension == 0 {
                return Err(ConfigError::InvalidParameter {
                    parameter: "generateFeatures.dimension".to_string(),
                    reason: "generateFeatures.dimension must be > 0".to_string(),
                });
            }
            if cfg.density_level == 0 {
                return Err(ConfigError::InvalidParameter {
                    parameter: "generateFeatures.densityLevel".to_string(),
                    reason: "generateFeatures.densityLevel must be > 0".to_string(),
                });
            }
        }
        if let Some(cfg) = &self.binarize_features {
            if cfg.dimension == 0 {
                return Err(ConfigError::InvalidParameter {
                    parameter: "binarizeFeatures.dimension".to_string(),
                    reason: "binarizeFeatures.dimension must be > 0".to_string(),
                });
            }
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for HashGNNConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        HashGNNConfig::validate(self)
    }
}

impl Default for HashGNNConfig {
    fn default() -> Self {
        Self {
            iterations: Self::default_iterations(),
            embedding_density: Self::default_embedding_density(),
            neighbor_influence: Self::default_neighbor_influence(),
            feature_properties: Vec::new(),
            heterogeneous: false,
            output_dimension: None,
            binarize_features: None,
            generate_features: Some(GenerateFeaturesConfig {
                dimension: 64,
                density_level: 3,
            }),
            concurrency: Self::default_concurrency(),
            random_seed: Some(42),
        }
    }
}

// ============================================================================
// Result
// ============================================================================

/// HashGNN embeddings output.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "mode")]
pub enum HashGNNEmbeddings {
    /// Binary embedding represented as **set bit indices** per node.
    BinaryIndices {
        embedding_dimension: usize,
        embeddings: Vec<Vec<u32>>,
    },
    /// Dense embedding represented as f32 vectors.
    Dense { embeddings: Vec<Vec<f32>> },
}

/// HashGNN result wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashGNNResult {
    pub embeddings: HashGNNEmbeddings,
}

// ============================================================================
// Algorithm Spec
// ============================================================================

define_algorithm_spec! {
    name: "hashgnn",
    output_type: HashGNNResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config_input, context| {
        let config: HashGNNConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Failed to parse HashGNN config: {e}")))?;

        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        context.log(
            LogLevel::Info,
            &format!(
                "Running HashGNN: nodes={}, iters={}, density={}, out_dim={:?}",
                graph_store.node_count(),
                config.iterations,
                config.embedding_density,
                config.output_dimension,
            ),
        );

        let rel_types: std::collections::HashSet<RelationshipType> = std::collections::HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to obtain graph view: {e}")))?;

        let storage = HashGNNStorageRuntime::new();
        let result = HashGNNComputationRuntime::run(graph_view, &config, &storage)?;

        Ok(result)
    }
}

// The `define_algorithm_spec!` macro generates `HASHGNNAlgorithmSpec`.
pub type HashGNNAlgorithmSpec = HASHGNNAlgorithmSpec;
