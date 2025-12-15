//! HITS Facade - User-facing API for HITS algorithm

use crate::procedures::hits::{HitsComputationResult, HitsComputationRuntime};
use crate::types::graph_store::DefaultGraphStore;
use std::sync::Arc;
use std::time::Instant;

/// Result row for HITS stream operation
#[derive(Debug, Clone)]
pub struct HitsRow {
    pub node_id: i64,
    pub hub_score: f64,
    pub authority_score: f64,
}

/// Statistics for HITS algorithm
#[derive(Debug, Clone)]
pub struct HitsStats {
    pub iterations: usize,
    pub converged: bool,
    pub execution_time_ms: u64,
}

/// Builder for HITS algorithm execution
pub struct HitsBuilder {
    store: Arc<DefaultGraphStore>,
    max_iterations: usize,
    tolerance: f64,
    concurrency: usize,
}

impl HitsBuilder {
    pub fn new(store: Arc<DefaultGraphStore>) -> Self {
        Self {
            store,
            max_iterations: 20,
            tolerance: 1e-4,
            concurrency: 1,
        }
    }

    /// Set maximum number of iterations
    pub fn max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Set convergence tolerance
    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = tolerance;
        self
    }

    /// Set concurrency level (currently unused, single-threaded)
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Stream results as rows (node_id, hub_score, authority_score)
    pub fn stream(&self) -> Result<Vec<HitsRow>, String> {
        let result = self.run()?;

        let rows = (0..result.hub_scores.len())
            .map(|internal_id| HitsRow {
                node_id: internal_id as i64,
                hub_score: result.hub_scores[internal_id],
                authority_score: result.authority_scores[internal_id],
            })
            .collect();

        Ok(rows)
    }

    /// Get statistics about the algorithm execution
    pub fn stats(&self) -> Result<HitsStats, String> {
        let start = Instant::now();
        let result = self.run()?;
        let elapsed = start.elapsed();

        Ok(HitsStats {
            iterations: result.iterations,
            converged: result.converged,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Run the algorithm and return the full result
    pub fn run(&self) -> Result<HitsComputationResult, String> {
        let runtime = HitsComputationRuntime::new(self.max_iterations, self.tolerance);
        Ok(runtime.compute(&self.store))
    }
}
