//! IndexInverse specification and configuration.
//! Mirrors the Java `InverseRelationshipsConfig`/parameters.

use crate::types::graph_store::DefaultGraphStore;
use serde::{Deserialize, Serialize};

/// Configuration for building inverse relationship indices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInverseConfig {
    /// Relationship types to process ("*" or empty = all types).
    pub relationship_types: Vec<String>,
    /// Concurrency hint (currently unused by the sequential runtime).
    pub concurrency: usize,
    /// Target graph name for the output store.
    pub mutate_graph_name: String,
}

impl Default for IndexInverseConfig {
    fn default() -> Self {
        Self {
            relationship_types: vec!["*".to_string()],
            concurrency: 4,
            mutate_graph_name: "index_inverse".to_string(),
        }
    }
}

/// Result of building inverse relationship indices.
#[derive(Debug, Clone)]
pub struct IndexInverseResult {
    /// Name assigned to the output graph.
    pub graph_name: String,
    /// Node count of the output graph.
    pub node_count: u64,
    /// Relationship count of the output graph.
    pub relationship_count: u64,
    /// Graph store containing inverse indices.
    pub graph_store: DefaultGraphStore,
}

/// Statistics for IndexInverse computation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInverseStats {
    pub graph_name: String,
    pub node_count: u64,
    pub relationship_count: u64,
}

/// Catalog marker for executor wiring.
pub struct IndexInverseAlgorithmSpec {
    graph_name: String,
}

impl IndexInverseAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
