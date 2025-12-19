//! HITS Facade - Bidirectional Pregel implementation

use crate::procedures::hits::computation::run_hits;
use crate::projection::RelationshipType;
use crate::types::graph_store::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
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

/// Builder for HITS Pregel execution
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
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = self
            .store
            .get_graph_with_types_and_orientation(
                &rel_types,
                crate::projection::Orientation::Natural,
            )
            .map_err(|e| format!("Failed to get graph: {}", e))?;

        let result = run_hits(graph, self.max_iterations, self.tolerance);

        let node_count = result.hub_scores.len();
        let mut rows = Vec::with_capacity(node_count);

        for node_id in 0..node_count {
            rows.push(HitsRow {
                node_id: node_id as i64,
                hub_score: result.hub_scores[node_id],
                authority_score: result.authority_scores[node_id],
            });
        }

        Ok(rows)
    }

    /// Get statistics about the algorithm execution
    pub fn stats(&self) -> Result<HitsStats, String> {
        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = self
            .store
            .get_graph_with_types_and_orientation(
                &rel_types,
                crate::projection::Orientation::Natural,
            )
            .map_err(|e| format!("Failed to get graph: {}", e))?;

        let result = run_hits(graph, self.max_iterations, self.tolerance);
        let elapsed = start.elapsed();

        Ok(HitsStats {
            iterations: result.iterations_ran,
            converged: result.did_converge,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Run the algorithm and return hub and authority scores
    pub fn run(&self) -> Result<(Vec<f64>, Vec<f64>), String> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = self
            .store
            .get_graph_with_types_and_orientation(
                &rel_types,
                crate::projection::Orientation::Natural,
            )
            .map_err(|e| format!("Failed to get graph: {}", e))?;

        let result = run_hits(graph, self.max_iterations, self.tolerance);
        Ok((result.hub_scores, result.authority_scores))
    }
}
