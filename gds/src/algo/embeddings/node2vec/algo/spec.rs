//! Node2Vec Algorithm Specification
//!
//! Node2Vec is a **node embedding** algorithm that:
//! 1) generates a corpus of node sequences via **biased random walks**, and
//! 2) trains a **skip-gram** model with **negative sampling** (Word2Vec-style)
//!    to produce a dense vector per node.
//!
//! The key idea is that the walk bias can interpolate between:
//! - **BFS-like** walks (stay near the start node → community similarity)
//! - **DFS-like** walks (explore outward → structural similarity)
//!
//! In classic Node2Vec notation:
//! - `returnFactor` is $p$ (return/backtracking control)
//! - `inOutFactor` is $q$ (in-out / BFS-vs-DFS control)
//!
//! ## Example (Facade)
//! ```rust,no_run
//! # use std::sync::Arc;
//! # use gds::types::graph_store::DefaultGraphStore;
//! # use gds::procedures::GraphFacade;
//! # let store = Arc::new(DefaultGraphStore::empty());
//! let graph = Graph::new(store);
//! let result = graph
//!     .node2vec()
//!     .walks_per_node(5)
//!     .walk_length(40)
//!     .embedding_dimension(64)
//!     .window_size(10)
//!     .negative_sampling_rate(5)
//!     .iterations(1)
//!     .random_seed(Some(42))
//!     .run()?;
//! assert_eq!(result.embeddings[0].len(), 64);
//! # Ok::<(), gds::projection::eval::algorithm::AlgorithmError>(())
//! ```

use crate::config::validation::ConfigError;
use crate::core::LogLevel;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use serde::{Deserialize, Serialize};

use super::Node2VecComputationRuntime;
use super::Node2VecStorageRuntime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node2VecConfig {
    #[serde(default = "Node2VecConfig::default_walks_per_node")]
    /// How many random walks to start from each `sourceNodes` entry.
    ///
    /// More walks → better coverage / stability, but more work.
    pub walks_per_node: usize,

    #[serde(default = "Node2VecConfig::default_walk_length")]
    /// Length of each walk (number of visited nodes).
    ///
    /// Longer walks capture more distant relationships, but increase training cost.
    pub walk_length: usize,

    #[serde(default = "Node2VecConfig::default_return_factor")]
    /// Return factor ($p$ in the Node2Vec paper).
    ///
    /// Controls how much the walk **backtracks** to the previous node.
    /// - Larger $p$ → *discourage* immediate return (less backtracking)
    /// - Smaller $p$ → *encourage* immediate return (more backtracking)
    pub return_factor: f64,

    #[serde(default = "Node2VecConfig::default_in_out_factor")]
    /// In-out factor ($q$ in the Node2Vec paper).
    ///
    /// Controls BFS-vs-DFS character of the walk:
    /// - $q > 1$ tends to stay close (BFS-ish)
    /// - $q < 1$ tends to explore outward (DFS-ish)
    pub in_out_factor: f64,

    #[serde(default = "Node2VecConfig::default_positive_sampling_factor")]
    /// Subsampling factor for frequent nodes in the walk corpus.
    ///
    /// This is analogous to Word2Vec subsampling: it can down-weight extremely frequent
    /// nodes so they don't dominate training.
    pub positive_sampling_factor: f64,

    #[serde(default = "Node2VecConfig::default_negative_sampling_exponent")]
    /// Exponent applied to the node-frequency distribution for negative sampling.
    ///
    /// Word2Vec commonly uses 0.75.
    pub negative_sampling_exponent: f64,

    #[serde(default = "Node2VecConfig::default_initial_learning_rate")]
    /// Initial SGD learning rate.
    pub initial_learning_rate: f64,

    #[serde(default = "Node2VecConfig::default_min_learning_rate")]
    /// Minimum SGD learning rate (floor for decay schedule).
    pub min_learning_rate: f64,

    #[serde(default = "Node2VecConfig::default_iterations")]
    /// Number of training iterations over the walk corpus.
    pub iterations: usize,

    #[serde(default = "Node2VecConfig::default_window_size")]
    /// Skip-gram context window size.
    ///
    /// For each position in the walk, predicts nearby nodes within this window.
    pub window_size: usize,

    #[serde(default = "Node2VecConfig::default_negative_sampling_rate")]
    /// Number of negative samples per positive (center, context) pair.
    pub negative_sampling_rate: usize,

    #[serde(default = "Node2VecConfig::default_embedding_dimension")]
    /// Size of the embedding vector per node.
    pub embedding_dimension: usize,

    #[serde(default = "Node2VecConfig::default_embedding_initializer")]
    /// Embedding initialization strategy.
    pub embedding_initializer: EmbeddingInitializerConfig,

    /// Optional subset of nodes to start walks from.
    ///
    /// When empty/not provided, walks start from **all** nodes.
    /// When non-empty, only those nodes seed walks (useful for sampling or debugging).
    #[serde(default)]
    pub source_nodes: Vec<i64>,

    #[serde(default = "Node2VecConfig::default_concurrency")]
    /// Concurrency for walk generation + training.
    pub concurrency: usize,

    /// Controls how often termination is checked during walk generation.
    ///
    /// Larger values reduce overhead; smaller values make cancellation more responsive.
    #[serde(default = "Node2VecConfig::default_walk_buffer_size")]
    pub walk_buffer_size: usize,

    #[serde(default)]
    /// Optional random seed for determinism.
    pub random_seed: Option<u64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub enum EmbeddingInitializerConfig {
    #[default]
    Uniform,
    Normalized,
}

impl Node2VecConfig {
    fn default_walks_per_node() -> usize {
        10
    }
    fn default_walk_length() -> usize {
        80
    }
    fn default_return_factor() -> f64 {
        1.0
    }
    fn default_in_out_factor() -> f64 {
        1.0
    }
    fn default_positive_sampling_factor() -> f64 {
        0.001
    }
    fn default_negative_sampling_exponent() -> f64 {
        0.75
    }
    fn default_initial_learning_rate() -> f64 {
        0.025
    }
    fn default_min_learning_rate() -> f64 {
        0.0001
    }
    fn default_iterations() -> usize {
        1
    }
    fn default_window_size() -> usize {
        10
    }
    fn default_negative_sampling_rate() -> usize {
        5
    }
    fn default_embedding_dimension() -> usize {
        128
    }
    fn default_embedding_initializer() -> EmbeddingInitializerConfig {
        EmbeddingInitializerConfig::Uniform
    }
    fn default_concurrency() -> usize {
        num_cpus::get().max(1)
    }
    fn default_walk_buffer_size() -> usize {
        8_192
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.walks_per_node == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "walksPerNode".to_string(),
                reason: "walksPerNode must be > 0".to_string(),
            });
        }
        if self.walk_length == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "walkLength".to_string(),
                reason: "walkLength must be > 0".to_string(),
            });
        }
        if self.embedding_dimension == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "embeddingDimension".to_string(),
                reason: "embeddingDimension must be > 0".to_string(),
            });
        }
        if self.window_size == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "windowSize".to_string(),
                reason: "windowSize must be > 0".to_string(),
            });
        }
        if self.iterations == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "iterations".to_string(),
                reason: "iterations must be > 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for Node2VecConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        Node2VecConfig::validate(self)
    }
}

impl Default for Node2VecConfig {
    fn default() -> Self {
        Self {
            walks_per_node: Self::default_walks_per_node(),
            walk_length: Self::default_walk_length(),
            return_factor: Self::default_return_factor(),
            in_out_factor: Self::default_in_out_factor(),
            positive_sampling_factor: Self::default_positive_sampling_factor(),
            negative_sampling_exponent: Self::default_negative_sampling_exponent(),
            initial_learning_rate: Self::default_initial_learning_rate(),
            min_learning_rate: Self::default_min_learning_rate(),
            iterations: Self::default_iterations(),
            window_size: Self::default_window_size(),
            negative_sampling_rate: Self::default_negative_sampling_rate(),
            embedding_dimension: Self::default_embedding_dimension(),
            embedding_initializer: Self::default_embedding_initializer(),
            source_nodes: Vec::new(),
            concurrency: Self::default_concurrency(),
            walk_buffer_size: Self::default_walk_buffer_size(),
            random_seed: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node2VecResult {
    pub embeddings: Vec<Vec<f32>>,
    pub loss_per_iteration: Vec<f64>,
    pub embedding_dimension: usize,
    pub node_count: usize,
}

define_algorithm_spec! {
    name: "node2vec",
    output_type: Node2VecResult,
    projection_hint: Dense,
    modes: [Stream, Stats],

    execute: |_self, graph_store, config_input, context| {
        let config: Node2VecConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Failed to parse Node2Vec config: {e}")))?;

        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(e.to_string()))?;

        context.log(
            LogLevel::Info,
            &format!(
                "Running Node2Vec: nodes={}, dim={}, walksPerNode={}, walkLength={}, iters={}",
                graph_store.node_count(),
                config.embedding_dimension,
                config.walks_per_node,
                config.walk_length,
                config.iterations,
            ),
        );

        let rel_types: std::collections::HashSet<RelationshipType> = std::collections::HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to obtain graph view: {e}")))?;

        let storage = Node2VecStorageRuntime::new();
        let result = Node2VecComputationRuntime::run(graph_view, &config, &storage)?;
        Ok(result)
    }
}

// Macro generates NODE2VECAlgorithmSpec; provide stable alias.
pub type Node2VecAlgorithmSpec = NODE2VECAlgorithmSpec;
