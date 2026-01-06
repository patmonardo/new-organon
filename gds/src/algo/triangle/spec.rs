use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleConfig {
    /// Reserved for future parallel implementation.
    pub concurrency: usize,
    /// Skip nodes with degree > max_degree (performance / approximation).
    pub max_degree: u64,
}

impl Default for TriangleConfig {
    fn default() -> Self {
        Self {
            concurrency: 4,
            max_degree: u64::MAX,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleResult {
    pub local_triangles: Vec<u64>,
    pub global_triangles: u64,
}

pub struct TriangleAlgorithmSpec {
    graph_name: String,
}

impl TriangleAlgorithmSpec {
    pub fn new(graph_name: String) -> Self {
        Self { graph_name }
    }

    pub fn graph_name(&self) -> &str {
        &self.graph_name
    }
}
