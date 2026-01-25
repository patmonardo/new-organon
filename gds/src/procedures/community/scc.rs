//! Strongly Connected Components (SCC) Facade
//!
//! Finds SCCs in a directed graph and returns:
//! - per-node component assignment
//! - component count and execution time stats
//!
//! Parameters:
//! - `concurrency`: accepted for Java GDS alignment; currently unused.

use crate::algo::scc::{SccComputationRuntime, SccResult, SccStorageRuntime};
use crate::collections::backends::vec::VecLong;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, TaskProgressTracker, TaskRegistry, TaskRegistryFactory, Tasks,
};
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

/// Mutate result for SCC: summary + updated store
#[derive(Debug, Clone)]
pub struct SccMutateResult {
    pub summary: MutationResult,
    pub updated_store: Arc<DefaultGraphStore>,
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
            .map_err(AlgorithmError::Execution)?;

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
    pub fn mutate(self, property_name: &str) -> Result<SccMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let longs: Vec<i64> = result.components.into_iter().map(|c| c as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels_set: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels_set, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("SCC mutate failed to add property: {e}"))
            })?;

        let execution_time = std::time::Duration::from_millis(result.computation_time_ms as u64);
        let summary = MutationResult::new(nodes_updated, property_name.to_string(), execution_time);

        Ok(SccMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: writes component assignments to a new graph.
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
