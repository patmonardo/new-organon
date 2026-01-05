//! Closeness Centrality Algorithm Specification
//!
//! This mirrors the Java GDS layering used elsewhere in this crate:
//! - `spec` defines the config and result shapes.
//! - `computation` contains the pure compute runtime.
//! - `storage` is the (optional) persistence/mutate/write layer.

use serde::{Deserialize, Serialize};

/// Configuration for closeness centrality.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosenessCentralityConfig {
    /// Enable Wasserman–Faust normalization.
    #[serde(default)]
    pub wasserman_faust: bool,

    /// Requested parallelism.
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
}

fn default_concurrency() -> usize {
    4
}

impl Default for ClosenessCentralityConfig {
    fn default() -> Self {
        Self {
            wasserman_faust: false,
            concurrency: default_concurrency(),
        }
    }
}

/// Result of closeness centrality computation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClosenessCentralityResult {
    pub centralities: Vec<f64>,
}

pub struct ClosenessCentralityAlgorithmSpec {
    graph_name: String,
}

impl ClosenessCentralityAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
