//! ToUndirected specification and configuration.
//!
//! Translation source: `org.neo4j.gds.undirected.ToUndirectedConfig`.

use serde::{Deserialize, Serialize};

/// Configuration for creating an undirected projection from a single relationship type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToUndirectedConfig {
    /// The relationship type to symmetrize.
    pub relationship_type: String,
    /// Relationship type name to use for the undirected output.
    pub mutate_relationship_type: String,
    /// Concurrency hint (currently unused by the sequential runtime).
    pub concurrency: usize,
}

impl Default for ToUndirectedConfig {
    fn default() -> Self {
        Self {
            relationship_type: "".to_string(),
            mutate_relationship_type: "undirected".to_string(),
            concurrency: 4,
        }
    }
}

/// Result of the undirected projection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToUndirectedResult {
    /// Relationship type assigned to the undirected edges.
    pub mutate_relationship_type: String,
    /// Unique directed edges (source, target) representing the undirected view.
    pub edges: Vec<(u64, u64)>,
}

/// Marker used by the executor catalog.
pub struct ToUndirectedAlgorithmSpec {
    graph_name: String,
}

impl ToUndirectedAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
