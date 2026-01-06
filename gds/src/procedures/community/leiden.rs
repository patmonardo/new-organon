//! Leiden Facade
//!
//! Leiden is a state-of-the-art community detection algorithm that improves
//! upon Louvain by preventing disconnected communities through a refinement phase.
//!
//! Parameters:
//! - `gamma`: Resolution parameter (default: 1.0)
//! - `theta`: Randomness parameter for refinement (default: 0.01)
//! - `tolerance`: Convergence tolerance (default: 0.0001)
//! - `max_iterations`: Maximum iterations (default: 10)
//! - `random_seed`: Random seed for reproducibility (default: 42)

use crate::core::utils::progress::{TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::algo::leiden::{LeidenComputationRuntime, LeidenConfig, LeidenResult, LeidenStorageRuntime};
use crate::concurrency::TerminationFlag;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::Arc;
use std::time::Instant;

/// Per-node Leiden assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct LeidenRow {
    pub node_id: u64,
    pub community_id: u64,
}

/// Aggregated Leiden stats.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct LeidenStats {
    pub community_count: u64,
    pub modularity: f64,
    pub levels: usize,
    pub converged: bool,
    pub execution_time_ms: u64,
}

/// Leiden algorithm facade.
#[derive(Clone)]
pub struct LeidenFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: LeidenConfig,
    task_registry: Option<TaskRegistry>,
}

impl LeidenFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: LeidenConfig::default(),
            task_registry: None,
        }
    }

    /// Set the resolution parameter (gamma)
    ///
    /// Higher values lead to more, smaller communities.
    /// Default: 1.0
    pub fn gamma(mut self, gamma: f64) -> Self {
        self.config.gamma = gamma;
        self
    }

    /// Set the randomness parameter (theta)
    ///
    /// Controls randomness in the refinement phase.
    /// Range: [0.0, 1.0] where 0.0 is deterministic.
    /// Default: 0.01
    pub fn theta(mut self, theta: f64) -> Self {
        self.config.theta = theta;
        self
    }

    /// Set the convergence tolerance
    ///
    /// Algorithm stops when modularity improvement < tolerance.
    /// Default: 0.0001
    pub fn tolerance(mut self, tolerance: f64) -> Self {
        self.config.tolerance = tolerance;
        self
    }

    /// Set the maximum number of iterations/levels
    ///
    /// Default: 10
    pub fn max_iterations(mut self, max_iterations: usize) -> Self {
        self.config.max_iterations = max_iterations;
        self
    }

    /// Set random seed for reproducibility
    ///
    /// Default: 42
    pub fn random_seed(mut self, seed: u64) -> Self {
        self.config.random_seed = seed;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.config.gamma, 0.0, 100.0, "gamma")?;
        ConfigValidator::in_range(self.config.theta, 0.0, 1.0, "theta")?;
        ConfigValidator::in_range(self.config.tolerance, 0.0, 1.0, "tolerance")?;
        ConfigValidator::in_range(
            self.config.max_iterations as f64,
            1.0,
            1000.0,
            "max_iterations",
        )?;
        Ok(())
    }

    fn compute(&self) -> Result<(LeidenResult, u64)> {
        self.validate()?;
        let start = Instant::now();

        let storage = LeidenStorageRuntime::new(self.graph_store.as_ref())?;
        let node_count = storage.node_count();

        let base_task = Tasks::leaf_with_volume(
            "leiden".to_string(),
            node_count.saturating_add(self.config.max_iterations),
        );
        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::new(base_task);

        let termination_flag = TerminationFlag::default();

        let mut computation = LeidenComputationRuntime::new();

        let result = storage.compute_leiden(
            &mut computation,
            &self.config,
            &mut progress_tracker,
            &termination_flag,
        )?;

        Ok((result, start.elapsed().as_millis() as u64))
    }

    /// Stream mode: yields `(node_id, community_id)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = LeidenRow>>> {
        let (result, _elapsed) = self.compute()?;
        let iter = result
            .communities
            .into_iter()
            .enumerate()
            .map(|(node_id, community_id)| LeidenRow {
                node_id: node_id as u64,
                community_id,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: returns aggregated statistics.
    pub fn stats(&self) -> Result<LeidenStats> {
        let (result, elapsed) = self.compute()?;
        Ok(LeidenStats {
            community_count: result.community_count,
            modularity: result.modularity,
            levels: result.levels,
            converged: result.converged,
            execution_time_ms: elapsed,
        })
    }

    /// Mutate mode: writes labels back to the graph store.
    pub fn mutate(self) -> Result<MutationResult> {
        // Note: mutation logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes labels to a new graph.
    pub fn write(self) -> Result<WriteResult> {
        // Note: write logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "write not yet implemented".to_string(),
            ),
        )
    }

    /// Estimate memory usage.
    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        // Leiden maintains community assignments and modularity-related working state.
        // Estimate is dominated by per-node arrays; relationship count affects traversal.
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        // Per node: community id + per-level bookkeeping (conservative).
        let per_node = 128usize;
        // Per relationship: scan/aggregation.
        let per_relationship = 8usize;

        let base: usize = 128 * 1024;
        let total = base
            .saturating_add(node_count.saturating_mul(per_node))
            .saturating_add(relationship_count.saturating_mul(per_relationship));

        Ok(MemoryRange::of_range(total, total.saturating_mul(3)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leiden_builder() {
        // Test that builder creates correct config (without graph store for now)
        let config = LeidenConfig {
            gamma: 1.5,
            theta: 0.05,
            tolerance: 0.001,
            max_iterations: 20,
            random_seed: 123,
            seed_communities: None,
        };

        assert_eq!(config.gamma, 1.5);
        assert_eq!(config.theta, 0.05);
        assert_eq!(config.tolerance, 0.001);
        assert_eq!(config.max_iterations, 20);
        assert_eq!(config.random_seed, 123);
        assert!(config.seed_communities.is_none());
    }
}
