//! K-Core Decomposition Facade
//!
//! Finds the k-core values for each node in an undirected graph.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: accepted for parity; currently unused.

use crate::core::utils::progress::{TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::algo::kcore::{
    KCoreComputationResult, KCoreComputationRuntime, KCoreConfig, KCoreStorageRuntime,
};
use crate::concurrency::TerminationFlag;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::Arc;
use std::time::Instant;

/// Per-node k-core value row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct KCoreRow {
    pub node_id: u64,
    pub core_value: i32,
}

/// Aggregated k-core stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct KCoreStats {
    pub degeneracy: i32,
    pub execution_time_ms: u64,
}

/// K-Core Decomposition algorithm facade.
#[derive(Clone)]
pub struct KCoreFacade {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    task_registry: Option<TaskRegistry>,
}

impl KCoreFacade {
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

    fn compute(&self) -> Result<(KCoreComputationResult, u64)> {
        self.validate()?;
        let start = Instant::now();

        let config = KCoreConfig {
            concurrency: self.concurrency,
        };

        let storage = KCoreStorageRuntime::new(self.graph_store.as_ref())?;
        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok((
                KCoreComputationResult {
                    core_values: Vec::new(),
                    degeneracy: 0,
                },
                start.elapsed().as_millis() as u64,
            ));
        }

        let base_task = Tasks::leaf_with_volume("kcore".to_string(), node_count);
        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_concurrency(
            base_task,
            self.concurrency,
        );

        let termination_flag = TerminationFlag::default();

        let mut runtime = KCoreComputationRuntime::new().concurrency(self.concurrency);
        let result = storage.compute_kcore(
            &mut runtime,
            &config,
            &mut progress_tracker,
            &termination_flag,
        )?;

        let elapsed_ms = start.elapsed().as_millis() as u64;

        Ok((result, elapsed_ms))
    }

    /// Stream mode: yields `(node_id, core_value)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = KCoreRow>>> {
        let (result, _elapsed) = self.compute()?;
        let iter = result
            .core_values
            .into_iter()
            .enumerate()
            .map(|(node_id, core_value)| KCoreRow {
                node_id: node_id as u64,
                core_value,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields degeneracy and execution time.
    pub fn stats(&self) -> Result<KCoreStats> {
        let (result, elapsed_ms) = self.compute()?;

        Ok(KCoreStats {
            degeneracy: result.degeneracy,
            execution_time_ms: elapsed_ms,
        })
    }

    /// Mutate mode: writes core values back to the graph store.
    pub fn mutate(self) -> Result<MutationResult> {
        // Note: mutation logic is deferred.
        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "mutate not yet implemented".to_string(),
            ),
        )
    }

    /// Write mode: writes core values to a new graph.
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
        // K-Core keeps per-node degree/core arrays and uses relationship streaming.
        let node_count = GraphStore::node_count(self.graph_store.as_ref());
        let relationship_count = GraphStore::relationship_count(self.graph_store.as_ref());

        // Per node: degree (usize) + core (u64) + bucket/work queues.
        let per_node = 96usize;
        // Per relationship: one pass over edges.
        let per_relationship = 8usize;

        let base: usize = 32 * 1024;
        let total = base
            .saturating_add(node_count.saturating_mul(per_node))
            .saturating_add(relationship_count.saturating_mul(per_relationship));

        Ok(MemoryRange::of_range(total, total.saturating_mul(2)))
    }

    /// Full result: returns the procedure-level k-core result.
    pub fn run(&self) -> Result<KCoreComputationResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
