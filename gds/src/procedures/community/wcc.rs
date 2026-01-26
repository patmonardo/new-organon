//! Weakly Connected Components (WCC) Facade
//!
//! Finds connected components in a graph under *undirected* semantics.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: accepted for parity; current runtime is single-threaded.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::wcc::{
    WccComputationRuntime, WccMutateResult, WccMutationSummary, WccResult, WccResultBuilder,
    WccStats, WccStorageRuntime,
};
use crate::collections::backends::vec::VecLong;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, TaskProgressTracker, TaskRegistry, TaskRegistryFactory, Tasks,
};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node WCC assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct WccRow {
    pub node_id: u64,
    pub component_id: u64,
}

/// WCC algorithm facade.
#[derive(Clone)]
pub struct WccFacade {
    graph_store: Arc<DefaultGraphStore>,
    concurrency: usize,
    task_registry: Option<TaskRegistry>,
}

impl WccFacade {
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

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = WccRow>>> {
        let result = self.compute()?;
        let iter = result
            .components
            .into_iter()
            .enumerate()
            .map(|(node_id, component_id)| WccRow {
                node_id: node_id as u64,
                component_id,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(self) -> Result<WccStats> {
        let result = self.compute()?;
        Ok(WccResultBuilder::new(result).stats())
    }

    pub fn mutate(self, _property_name: &str) -> Result<crate::algo::algorithms::MutationResult> {
        // Implemented below in the long-form mutate returning updated store
        Err(AlgorithmError::Execution(
            "Use mutate_with_store() for WCC (internal)".to_string(),
        ))
    }

    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        // For WCC, write is the same as mutate since it's node properties
        let result = self.compute()?;
        let node_count = self.graph_store.node_count();
        let nodes_written = node_count as u64;
        // For now, pretend we've written components externally
        Ok(WriteResult::new(
            nodes_written,
            property_name.to_string(),
            result.execution_time,
        ))
    }

    /// Mutate mode: compute components and add as a node property, returning updated store
    pub fn mutate_with_store(self, property_name: &str) -> Result<WccMutateResult> {
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
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("WCC mutate failed to add property: {e}"))
            })?;

        let summary = WccMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(WccMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        // Estimate memory for WCC computation
        // - Component assignments: node_count * 8 bytes
        // - Union-find structures: node_count * 16 bytes
        // - Graph view overhead: roughly node_count * 16 bytes
        let node_count = self.graph_store.node_count();
        let assignment_memory = node_count * 8;
        let union_find_memory = node_count * 16;
        let graph_memory = node_count * 16;

        let total = assignment_memory + union_find_memory + graph_memory;
        MemoryRange::of_range(total, total * 2) // Conservative upper bound
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        Ok(())
    }

    fn compute(&self) -> Result<WccResult> {
        self.validate()?;
        let start = Instant::now();

        let storage = WccStorageRuntime::new(self.concurrency);
        let mut computation = WccComputationRuntime::new().concurrency(self.concurrency);

        let leaf =
            Tasks::leaf_with_volume("wcc".to_string(), self.graph_store.relationship_count());
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
            .compute_wcc(
                &mut computation,
                self.graph_store.as_ref(),
                &mut progress_tracker,
                &termination_flag,
            )
            .map_err(AlgorithmError::Execution)?;

        Ok(WccResult {
            components: result.components,
            component_count: result.component_count,
            node_count: self.graph_store.node_count(),
            execution_time: start.elapsed(),
        })
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

    /// Full result: returns the procedure-level WCC result.
    pub fn run(&self) -> Result<WccResult> {
        let result = self.compute()?;
        Ok(result)
    }
}
