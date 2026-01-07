//! Strongly Connected Components (SCC) Facade
//!
//! Finds SCCs in a directed graph and returns:
//! - per-node component assignment
//! - component count and execution time stats
//!
//! Parameters:
//! - `concurrency`: accepted for Java GDS alignment; currently unused.

use crate::algo::scc::{SccComputationRuntime, SccResult, SccStorageRuntime};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, TaskProgressTracker, TaskRegistry, TaskRegistryFactory, Tasks,
};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
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

        let leaf = Tasks::leaf_with_volume("scc".to_string(), self.graph_store.node_count());
        let base_task = leaf.base().clone();
        let registry_factory = self.registry_factory();
        let mut progress_tracker = TaskProgressTracker::with_registry(
            base_task,
            Concurrency::of(self.concurrency.max(1)),
            JobId::new(),
            registry_factory.as_ref(),
        );
        let termination_flag = TerminationFlag::default();

        let result = storage
            .compute_scc(
                &mut computation,
                self.graph_store.as_ref(),
                &mut progress_tracker,
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
        // SCC typically uses several per-node arrays (index/lowlink/stack flags) and a stack.
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        // Per node: multiple u64/usize arrays + stack membership.
        let per_node = 128usize;
        // Per relationship: traversal over outgoing edges.
        let per_relationship = 8usize;

        let base: usize = 64 * 1024;
        let total = base
            .saturating_add(node_count.saturating_mul(per_node))
            .saturating_add(relationship_count.saturating_mul(per_relationship));

        Ok(MemoryRange::of_range(total, total.saturating_mul(2)))
    }

    /// Full result: returns the procedure-level SCC result.
    pub fn run(&self) -> Result<SccResult> {
        self.compute()
    }

    fn registry_factory(&self) -> Box<dyn TaskRegistryFactory> {
        struct PrebuiltTaskRegistryFactory(TaskRegistry);

        impl TaskRegistryFactory for PrebuiltTaskRegistryFactory {
            fn new_instance(&self, _job_id: JobId) -> TaskRegistry {
                self.0.clone()
            }
        }

        if let Some(registry) = &self.task_registry {
            Box::new(PrebuiltTaskRegistryFactory(registry.clone()))
        } else {
            Box::new(EmptyTaskRegistryFactory)
        }
    }
}
