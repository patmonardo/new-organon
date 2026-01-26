use crate::config::validation::ConfigError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleConfig {
    /// Reserved for future parallel implementation.
    pub concurrency: usize,
    /// Skip nodes with degree > max_degree (performance / approximation).
    pub max_degree: u64,
}

impl TriangleConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.concurrency == 0 {
            return Err(ConfigError::InvalidParameter {
                parameter: "concurrency".to_string(),
                reason: "concurrency must be > 0".to_string(),
            });
        }
        Ok(())
    }
}

impl crate::config::ValidatedConfig for TriangleConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        TriangleConfig::validate(self)
    }
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
    pub node_count: usize,
    pub execution_time: Duration,
}

/// Aggregated triangle count statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriangleStats {
    pub global_triangles: u64,
    pub execution_time_ms: u64,
}

/// Summary of a mutate operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriangleWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Triangle: summary + updated store.
#[derive(Debug, Clone)]
pub struct TriangleMutateResult {
    pub summary: TriangleMutationSummary,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

/// Triangle result builder (facade adapter).
pub struct TriangleResultBuilder {
    result: TriangleResult,
}

impl TriangleResultBuilder {
    pub fn new(result: TriangleResult) -> Self {
        Self { result }
    }

    pub fn stats(&self) -> TriangleStats {
        TriangleStats {
            global_triangles: self.result.global_triangles,
            execution_time_ms: self.result.execution_time.as_millis() as u64,
        }
    }

    pub fn execution_time_ms(&self) -> u64 {
        self.result.execution_time.as_millis() as u64
    }
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
