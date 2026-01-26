//! Result Builders - Algorithm result construction and processing
//!
//! **Translation Source**: `org.neo4j.gds.result.Abstract*ResultBuilder` classes
//! **Key Features**: Result building, statistics integration, histogram generation
//!
//! This module provides result building capabilities for different algorithm types,
//! integrating with our statistics and progress tracking modules.

use crate::algo::algorithms::statistics::{Histogram, StatisticalSummary, StatisticsError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Base result builder trait
pub trait ResultBuilder<T> {
    /// Build the final result
    fn build(self) -> Result<T, ResultBuilderError>;

    /// Add statistics to the result
    fn with_statistics(self, stats: StatisticalSummary) -> Self;

    /// Add histogram to the result
    fn with_histogram(self, histogram: Option<Histogram>) -> Self;

    /// Add execution metadata
    fn with_metadata(self, metadata: ExecutionMetadata) -> Self;
}

/// Execution metadata for algorithm results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    /// Execution time
    pub execution_time: Duration,
    /// Number of iterations (if applicable)
    pub iterations: Option<u32>,
    /// Convergence status (if applicable)
    pub converged: Option<bool>,
    /// Additional metadata
    pub additional: HashMap<String, String>,
}

impl ExecutionMetadata {
    /// Create new execution metadata
    pub fn new(execution_time: Duration) -> Self {
        Self {
            execution_time,
            iterations: None,
            converged: None,
            additional: HashMap::new(),
        }
    }

    /// Add iteration count
    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.iterations = Some(iterations);
        self
    }

    /// Add convergence status
    pub fn with_convergence(mut self, converged: bool) -> Self {
        self.converged = Some(converged);
        self
    }

    /// Add additional metadata
    pub fn with_additional(mut self, key: String, value: String) -> Self {
        self.additional.insert(key, value);
        self
    }
}

// ============================================================================
// Mutation and Write Results
// ============================================================================

/// Result of a mutation operation
#[derive(Debug, Clone, Serialize)]
pub struct MutationResult {
    /// Number of nodes updated
    pub nodes_updated: u64,
    /// Property name created/updated
    pub property_name: String,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

impl MutationResult {
    /// Create a new mutation result
    pub fn new(nodes_updated: u64, property_name: String, execution_time: Duration) -> Self {
        Self {
            nodes_updated,
            property_name,
            execution_time_ms: execution_time.as_millis() as u64,
        }
    }

    /// Get execution time in milliseconds
    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time_ms
    }
}

/// Result of a write operation
#[derive(Debug, Clone, Serialize)]
pub struct WriteResult {
    /// Number of nodes written
    pub nodes_written: u64,
    /// Property name written
    pub property_name: String,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

impl WriteResult {
    /// Create a new write result
    pub fn new(nodes_written: u64, property_name: String, execution_time: Duration) -> Self {
        Self {
            nodes_written,
            property_name,
            execution_time_ms: execution_time.as_millis() as u64,
        }
    }

    /// Get execution time in milliseconds
    pub fn execution_time_ms(&self) -> u64 {
        self.execution_time_ms
    }
}

/// Result builder error
#[derive(Debug, thiserror::Error)]
pub enum ResultBuilderError {
    #[error("Statistics computation failed: {0}")]
    StatisticsError(#[from] StatisticsError),

    #[error("Invalid result data: {0}")]
    InvalidData(String),

    #[error("Builder configuration error: {0}")]
    ConfigurationError(String),

    #[error("Missing required data: {0}")]
    MissingData(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_metadata() {
        let metadata = ExecutionMetadata::new(Duration::from_secs(5))
            .with_iterations(100)
            .with_convergence(true)
            .with_additional("algorithm".to_string(), "pagerank".to_string());

        assert_eq!(metadata.execution_time, Duration::from_secs(5));
        assert_eq!(metadata.iterations, Some(100));
        assert_eq!(metadata.converged, Some(true));
        assert_eq!(
            metadata.additional.get("algorithm"),
            Some(&"pagerank".to_string())
        );
    }
}
