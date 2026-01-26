//! Common traits for algorithm facades
//!
//! Defines the contract that all algorithm facades must implement,
//! ensuring consistent API across all algorithms while allowing
//! algorithms to customize their behavior.

use crate::projection::eval::algorithm::AlgorithmError;

/// Result type for facade operations
pub type Result<T> = std::result::Result<T, AlgorithmError>;

/// Core trait for any algorithm facade
pub trait AlgorithmRunner {
    /// Algorithm name (e.g., "pagerank", "louvain")
    fn algorithm_name(&self) -> &'static str;

    /// Human-readable description
    fn description(&self) -> &'static str;
}

// ============================================================================
// Builder Pattern Utilities
// ============================================================================

/// Common configuration validation
pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate that a value is positive
    pub fn positive(value: f64, field_name: &str) -> Result<()> {
        if value <= 0.0 {
            return Err(AlgorithmError::Execution(format!(
                "{} must be positive, got {}",
                field_name, value
            )));
        }
        Ok(())
    }

    /// Validate that a value is in range [min, max]
    pub fn in_range(value: f64, min: f64, max: f64, field_name: &str) -> Result<()> {
        if value < min || value > max {
            return Err(AlgorithmError::Execution(format!(
                "{} must be in range [{}, {}], got {}",
                field_name, min, max, value
            )));
        }
        Ok(())
    }

    /// Validate that an iteration count is reasonable
    pub fn iterations(value: u32, field_name: &str) -> Result<()> {
        if value == 0 || value > 1_000_000 {
            return Err(AlgorithmError::Execution(format!(
                "{} must be > 0 and <= 1_000_000, got {}",
                field_name, value
            )));
        }
        Ok(())
    }

    /// Validate that a property name is non-empty
    pub fn non_empty_string(value: &str, field_name: &str) -> Result<()> {
        if value.is_empty() {
            return Err(AlgorithmError::Execution(format!(
                "{} cannot be empty",
                field_name
            )));
        }
        Ok(())
    }
}

/// Result type for centrality algorithms: (node_id, score)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize)]
pub struct CentralityScore {
    pub node_id: u64,
    pub score: f64,
}
