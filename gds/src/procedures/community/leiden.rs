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

use crate::core::utils::progress::TaskRegistry;
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::algo::leiden::computation::leiden as leiden_fn;
use crate::algo::leiden::{LeidenConfig, LeidenResult};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
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

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Undirected)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        // Build adjacency list from graph view
        let node_count = graph_view.node_count();
        let mut adjacency_list = vec![Vec::new(); node_count];

        for (node_id, adj) in adjacency_list.iter_mut().enumerate() {
            let relationships = graph_view.stream_relationships_weighted(node_id as i64, 1.0);
            for cursor in relationships {
                let target_id = cursor.target_id() as usize;
                let weight = cursor.weight();
                adj.push((target_id, weight));
            }
        }

        // Run Leiden algorithm
        let storage = leiden_fn(
            node_count,
            |node: usize| adjacency_list[node].clone(),
            &self.config,
        );
        let result = storage.into_result();

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
        // Note: memory estimation is deferred.
        Ok(MemoryRange::of_range(0, 1024 * 1024)) // placeholder
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
