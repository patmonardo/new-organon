//! Triangle Facade
//!
//! Counts triangles in an undirected graph and returns per-node and global counts.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: reserved for future parallel implementation
//! - `max_degree`: filter to skip high-degree nodes (performance / approximation)

use crate::algo::triangle::{TriangleComputationRuntime, TriangleConfig, TriangleStorageRuntime};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, LeafTask, ProgressTracker, TaskProgressTracker, Tasks,
};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Per-node triangle count row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct TriangleRow {
    pub node_id: u64,
    pub triangles: u64,
}

/// Aggregated triangle count statistics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct TriangleStats {
    pub global_triangles: u64,
    pub execution_time_ms: u64,
}

/// Triangle algorithm facade.
#[derive(Clone)]
pub struct TriangleFacade {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    max_degree: u64,
}

impl TriangleFacade {
    /// Create a new facade bound to a live graph store.
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            concurrency: TriangleConfig::default().concurrency,
            max_degree: TriangleConfig::default().max_degree,
        }
    }

    /// Concurrency hint (reserved for future parallel implementation).
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Skip nodes with degree > max_degree.
    pub fn max_degree(mut self, max_degree: u64) -> Self {
        self.max_degree = max_degree;
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        Ok(())
    }

    fn compute(&self) -> Result<(Vec<u64>, u64, Duration)> {
        self.validate()?;
        let start = Instant::now();

        let node_count = self.graph_store.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), 0, start.elapsed()));
        }

        let leaf: LeafTask = Tasks::leaf_with_volume("triangle".to_string(), node_count);
        let base_task = leaf.base().clone();
        let registry_factory = EmptyTaskRegistryFactory;
        let mut progress_tracker: Box<dyn ProgressTracker> =
            Box::new(TaskProgressTracker::with_registry(
                base_task,
                Concurrency::of(self.concurrency.max(1)),
                JobId::new(),
                &registry_factory,
            ));

        progress_tracker.begin_subtask_with_volume(node_count);

        let config = TriangleConfig {
            concurrency: self.concurrency,
            max_degree: self.max_degree,
        };

        let termination_flag = TerminationFlag::default();
        let storage = TriangleStorageRuntime::new();
        let mut runtime = TriangleComputationRuntime::new();
        let result = storage
            .compute(
                &mut runtime,
                self.graph_store.as_ref(),
                &config,
                progress_tracker.as_mut(),
                &termination_flag,
            )
            .map_err(crate::projection::eval::procedure::AlgorithmError::Execution)?;

        Ok((
            result.local_triangles,
            result.global_triangles,
            start.elapsed(),
        ))
    }

    /// Stream mode: yields `(node_id, triangles)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = TriangleRow>>> {
        let (local, _global, _elapsed) = self.compute()?;
        let iter = local
            .into_iter()
            .enumerate()
            .map(|(node_id, triangles)| TriangleRow {
                node_id: node_id as u64,
                triangles,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields global triangle count.
    pub fn stats(&self) -> Result<TriangleStats> {
        let (_local, global, elapsed) = self.compute()?;
        Ok(TriangleStats {
            global_triangles: global,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Mutate mode: writes triangle counts back to the graph store.
    pub fn mutate(self) -> Result<MutationResult> {
        // Note: mutation logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes triangle counts to a new graph.
    pub fn write(self) -> Result<WriteResult> {
        // Note: write logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "write not yet implemented".to_string(),
            ),
        )
    }

    /// Full result: returns both local and global counts.
    pub fn run(&self) -> Result<crate::algo::triangle::TriangleResult> {
        let (local, global, _elapsed) = self.compute()?;
        Ok(crate::algo::triangle::TriangleResult {
            local_triangles: local,
            global_triangles: global,
        })
    }

    /// Estimate memory usage.
    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        // Triangle count stores per-node local counts and uses neighbor traversal.
        // Some implementations keep adjacency lists; we estimate proportional to (n + m).
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        // Per node: u64 local triangle count + temporary counters.
        let per_node = 64usize;
        // Per relationship: potential adjacency materialization (conservative).
        let per_relationship = 24usize;

        let base: usize = 64 * 1024;
        let total = base
            .saturating_add(node_count.saturating_mul(per_node))
            .saturating_add(relationship_count.saturating_mul(per_relationship));

        Ok(MemoryRange::of_range(total, total.saturating_mul(3)))
    }
}
