//! Strongly Connected Components (SCC) Facade
//!
//! Finds SCCs in a directed graph and returns:
//! - per-node component assignment
//! - component count and execution time stats
//!
//! Parameters:
//! - `concurrency`: accepted for Java GDS alignment; currently unused.

use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{ProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::algo::scc::{SccComputationRuntime, SccResult, SccStorageRuntime};
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::Arc;

/// Per-node SCC assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct SccRow {
    pub node_id: u64,
    pub component_id: u64,
}

/// Aggregated SCC stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct SccStats {
    pub component_count: usize,
    pub execution_time_ms: u64,
}

/// SCC algorithm facade.
#[derive(Clone)]
pub struct SccFacade {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    task_registry: Option<TaskRegistry>,
}

impl SccFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            concurrency: 4,
            task_registry: None,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        Ok(())
    }

    fn compute(&self) -> Result<SccResult> {
        self.validate()?;

        let mut computation = SccComputationRuntime::new();
        let storage = SccStorageRuntime::new(self.concurrency);

        let progress_tracker = ProgressTracker::new(Tasks::Leaf(
            "SCC".to_string(),
            self.graph_store.node_count(),
        ));
        let termination_flag = TerminationFlag::default();

        let result = storage
            .compute_scc(
                &mut computation,
                self.graph_store.as_ref(),
                &progress_tracker,
                &termination_flag,
            )
            .map_err(crate::projection::eval::procedure::AlgorithmError::Execution)?;

        Ok(SccResult::new(
            result.components,
            result.component_count,
            result.computation_time_ms,
        ))
    }

    /// Stream mode: yields `(node_id, component_id)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = SccRow>>> {
        let result = self.compute()?;
        let iter = result
            .components
            .into_iter()
            .enumerate()
            .map(|(node_id, component_id)| SccRow {
                node_id: node_id as u64,
                component_id,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields component count and execution time.
    pub fn stats(&self) -> Result<SccStats> {
        let result = self.compute()?;
        Ok(SccStats {
            component_count: result.component_count,
            execution_time_ms: result.computation_time_ms,
        })
    }

    /// Mutate mode: writes component assignments back to the graph store.
    pub fn mutate(self) -> Result<MutationResult> {
        // Note: mutation logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes component assignments to a new graph.
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

    /// Full result: returns the procedure-level SCC result.
    pub fn run(&self) -> Result<SccResult> {
        self.compute()
    }
}
