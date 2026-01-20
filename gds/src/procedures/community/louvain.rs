//! Louvain Facade
//!
//! Louvain is a modularity-optimization community detection algorithm.
//!
//! Note: the underlying Louvain implementation in this crate is currently a
//! placeholder. The facade is wired to the procedure module and supports seeding
//! to keep API parity and determinism while the full modularity optimization
//! runtime is built out.
//!
//! Parameters:
//! - `concurrency`

use crate::algo::louvain::{
    LouvainComputationRuntime, LouvainConfig, LouvainResult, LouvainStorageRuntime,
};
use crate::collections::backends::vec::VecLong;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{TaskProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::Result;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::impls::default_node_property_values::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node Louvain assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct LouvainRow {
    pub node_id: u64,
    pub community_id: u64,
}

/// Aggregated Louvain stats.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct LouvainStats {
    pub community_count: usize,
    pub execution_time_ms: u64,
}

/// Louvain algorithm facade.
#[derive(Clone)]
pub struct LouvainFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: LouvainConfig,
    concurrency: usize,
    task_registry: Option<TaskRegistry>,
}

/// Mutate result for Louvain: summary + updated graph store
#[derive(Debug, Clone)]
pub struct LouvainMutateResult {
    pub summary: MutationResult,
    pub updated_store: Arc<DefaultGraphStore>,
}

impl LouvainFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: LouvainConfig { concurrency: 4 },
            concurrency: 4,
            task_registry: None,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self.concurrency = concurrency;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = LouvainRow>>> {
        let (result, _elapsed) = self.compute()?;
        let iter = result
            .data
            .into_iter()
            .enumerate()
            .map(|(node_id, community_id)| LouvainRow {
                node_id: node_id as u64,
                community_id,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(self) -> Result<LouvainStats> {
        let (result, elapsed) = self.compute()?;
        let community_count = result
            .data
            .iter()
            .copied()
            .collect::<std::collections::HashSet<u64>>()
            .len();
        Ok(LouvainStats {
            community_count,
            execution_time_ms: elapsed,
        })
    }

    pub fn mutate(self, property_name: &str) -> Result<LouvainMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let start = Instant::now();
        let (result, _elapsed) = self.compute()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        // Convert community ids (u64) into i64 backend for VecLong
        let longs: Vec<i64> = result.data.into_iter().map(|c| c as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "Louvain mutate failed to add property: {e}"
                ))
            })?;

        let execution_time = start.elapsed();
        let summary = MutationResult::new(nodes_updated, property_name.to_string(), execution_time);

        Ok(LouvainMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        // For Louvain, write is the same as mutate since it's node properties
        let res = self.mutate(property_name)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            property_name.to_string(),
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        // Estimate memory for Louvain computation
        // - Community assignments: node_count * 8 bytes
        // - Modularity calculations: node_count * 8 bytes
        // - Graph view overhead: roughly node_count * 16 bytes
        let node_count = self.graph_store.node_count();
        let assignment_memory = node_count * 8;
        let modularity_memory = node_count * 8;
        let graph_memory = node_count * 16;

        let total = assignment_memory + modularity_memory + graph_memory;
        MemoryRange::of_range(total, total * 2) // Conservative upper bound
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(
            self.config.concurrency as f64,
            1.0,
            1_000_000.0,
            "concurrency",
        )?;
        Ok(())
    }

    fn compute(&self) -> Result<(LouvainResult, u64)> {
        self.validate()?;
        let start = Instant::now();

        let storage =
            LouvainStorageRuntime::new(self.graph_store.as_ref(), self.config.concurrency)?;
        let mut computation = LouvainComputationRuntime::new();
        let termination_flag = TerminationFlag::default();

        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume("louvain".to_string(), storage.node_count()),
            self.config.concurrency,
        );

        let result =
            storage.compute_louvain(&mut computation, &mut progress_tracker, &termination_flag)?;
        Ok((result, start.elapsed().as_millis() as u64))
    }

    /// Full result: returns the procedure-level Louvain result.
    pub fn run(&self) -> Result<LouvainResult> {
        let (result, _elapsed) = self.compute()?;
        Ok(result)
    }
}
