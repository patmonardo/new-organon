//! K1-Coloring Facade
//!
//! Greedy iterative graph coloring.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: accepted for parity; currently unused.
//! - `max_iterations`: maximum number of coloring/validation iterations (must be >= 1).
//! - `batch_size`: accepted for parity; currently unused.

use crate::algo::k1coloring::{
    K1ColoringComputationRuntime, K1ColoringConfig, K1ColoringResult, K1ColoringStorageRuntime,
};
use crate::collections::backends::vec::VecLong;
use crate::concurrency::TerminationFlag;
use crate::core::utils::partition::DEFAULT_BATCH_SIZE;
use crate::core::utils::progress::{TaskProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::Result;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node color assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct K1ColoringRow {
    pub node_id: u64,
    pub color_id: u64,
}

/// Aggregated K1-Coloring stats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct K1ColoringStats {
    pub did_converge: bool,
    pub ran_iterations: u64,
    pub color_count: usize,
    pub execution_time_ms: u64,
}

/// K1-Coloring algorithm facade.
#[derive(Clone)]
pub struct K1ColoringFacade {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    max_iterations: u64,
    batch_size: usize,
    task_registry: Option<TaskRegistry>,
}

/// Mutate result for K1Coloring: summary + updated store
#[derive(Debug, Clone)]
pub struct K1ColoringMutateResult {
    pub summary: MutationResult,
    pub updated_store: Arc<DefaultGraphStore>,
}

impl K1ColoringFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            concurrency: 4,
            max_iterations: 10,
            batch_size: DEFAULT_BATCH_SIZE,
            task_registry: None,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn max_iterations(mut self, max_iterations: u64) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        ConfigValidator::in_range(
            self.max_iterations as f64,
            1.0,
            1_000_000_000.0,
            "max_iterations",
        )?;
        ConfigValidator::in_range(self.batch_size as f64, 1.0, 1_000_000_000.0, "batch_size")?;
        Ok(())
    }

    fn compute(&self) -> Result<(K1ColoringResult, u64)> {
        self.validate()?;
        let start = Instant::now();

        let config = K1ColoringConfig {
            concurrency: self.concurrency,
            max_iterations: self.max_iterations,
            min_batch_size: self.batch_size,
        };

        let storage = K1ColoringStorageRuntime::new(self.graph_store.as_ref())?;
        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok((
                K1ColoringResult {
                    colors: Vec::new(),
                    ran_iterations: 0,
                    did_converge: true,
                },
                start.elapsed().as_millis() as u64,
            ));
        }

        let base_task =
            Tasks::leaf_with_volume("k1coloring".to_string(), self.max_iterations as usize);
        let mut progress_tracker =
            TaskProgressTracker::with_concurrency(base_task, self.concurrency);

        let termination_flag = TerminationFlag::default();

        let mut runtime = K1ColoringComputationRuntime::new(node_count, self.max_iterations)
            .concurrency(self.concurrency);

        let result = storage.compute_k1coloring(
            &mut runtime,
            &config,
            &mut progress_tracker,
            &termination_flag,
        )?;

        let elapsed_ms = start.elapsed().as_millis() as u64;

        Ok((result, elapsed_ms))
    }

    /// Stream mode: yields `(node_id, color_id)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = K1ColoringRow>>> {
        let (result, _elapsed) = self.compute()?;
        let iter = result
            .colors
            .into_iter()
            .enumerate()
            .map(|(node_id, color_id)| K1ColoringRow {
                node_id: node_id as u64,
                color_id,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields convergence info + number of distinct colors used.
    pub fn stats(&self) -> Result<K1ColoringStats> {
        let (result, elapsed_ms) = self.compute()?;
        let color_count = result
            .colors
            .iter()
            .copied()
            .collect::<HashSet<u64>>()
            .len();

        Ok(K1ColoringStats {
            did_converge: result.did_converge,
            ran_iterations: result.ran_iterations,
            color_count,
            execution_time_ms: elapsed_ms,
        })
    }

    /// Mutate mode: writes color assignments back to the graph store.
    pub fn mutate(self, property_name: &str) -> Result<K1ColoringMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let (result, elapsed_ms) = self.compute()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let longs: Vec<i64> = result.colors.into_iter().map(|c| c as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels_set: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels_set, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("K1Coloring mutate failed to add property: {e}"))
            })?;

        let summary = MutationResult::new(
            nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(elapsed_ms),
        );

        Ok(K1ColoringMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: writes color assignments to a new graph.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    /// Estimate memory usage.
    pub fn estimate_memory(&self) -> Result<MemoryRange> {
        // K1Coloring maintains a color assignment per node and iterates neighbors.
        let node_count = GraphStore::node_count(self.graph_store.as_ref());
        let relationship_count = GraphStore::relationship_count(self.graph_store.as_ref());

        // Per node: u64 color + temporary neighbor set buffers.
        let per_node = 80usize;
        // Per relationship: transient traversal, usually streamed from store.
        let per_relationship = 8usize;

        let base: usize = 32 * 1024;
        let total = base
            .saturating_add(node_count.saturating_mul(per_node))
            .saturating_add(relationship_count.saturating_mul(per_relationship));

        Ok(MemoryRange::of_range(total, total.saturating_mul(2)))
    }

    /// Full result: returns the procedure-level K1Coloring result.
    pub fn run(&self) -> Result<K1ColoringResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
