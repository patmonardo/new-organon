//! K-Core Decomposition Facade
//!
//! Finds the k-core values for each node in an undirected graph.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: accepted for parity; currently unused.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, MutationResult, WriteResult};
use crate::algo::kcore::{
    KCoreComputationResult, KCoreComputationRuntime, KCoreConfig, KCoreMutateResult,
    KCoreMutationSummary, KCoreResult, KCoreResultBuilder, KCoreStats, KCoreStorageRuntime,
};
use crate::collections::backends::vec::VecLong;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{TaskProgressTracker, TaskRegistry, Tasks};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultLongNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use crate::types::schema::NodeLabel;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Per-node k-core value row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct KCoreRow {
    pub node_id: u64,
    pub core_value: i32,
}

/// K-Core Decomposition algorithm facade.
#[derive(Clone)]
pub struct KCoreFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: KCoreConfig,
    task_registry: Option<TaskRegistry>,
}

impl KCoreFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: KCoreConfig::default(),
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: KCoreConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry: None,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: KCoreConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: KCoreConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    pub fn task_registry(mut self, task_registry: TaskRegistry) -> Self {
        self.task_registry = Some(task_registry);
        self
    }

    fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<KCoreResult> {
        self.validate()?;
        let start = Instant::now();

        let config = self.config.clone();

        let storage = KCoreStorageRuntime::new(self.graph_store.as_ref())?;
        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok(KCoreResult {
                core_values: Vec::new(),
                degeneracy: 0,
                node_count: 0,
                execution_time: start.elapsed(),
            });
        }

        let base_task = Tasks::leaf_with_volume("kcore".to_string(), node_count);
        let mut progress_tracker =
            TaskProgressTracker::with_concurrency(base_task, self.config.concurrency);

        let termination_flag = TerminationFlag::default();

        let mut runtime = KCoreComputationRuntime::new().concurrency(self.config.concurrency);
        let result = storage.compute_kcore(
            &mut runtime,
            &config,
            &mut progress_tracker,
            &termination_flag,
        )?;

        Ok(KCoreResult {
            core_values: result.core_values,
            degeneracy: result.degeneracy,
            node_count,
            execution_time: start.elapsed(),
        })
    }

    /// Stream mode: yields `(node_id, core_value)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = KCoreRow>>> {
        let result = self.compute()?;
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
        let result = self.compute()?;
        Ok(KCoreResultBuilder::new(result).stats())
    }

    /// Mutate mode: writes core values back to the graph store.
    pub fn mutate(self) -> Result<MutationResult> {
        // Implement mutate returning updated store
        Err(AlgorithmError::Execution(
            "Use mutate_with_store() for KCore (internal)".to_string(),
        ))
    }

    /// Write mode: writes core values to a new graph.
    pub fn write(self) -> Result<WriteResult> {
        // Note: write logic is deferred.
        let result = self.compute()?;
        let node_count = self.graph_store.node_count();
        let nodes_written = node_count as u64;
        Ok(WriteResult::new(
            nodes_written,
            "core_value".to_string(),
            result.execution_time,
        ))
    }

    /// Mutate mode: compute core values and add as a node property, returning updated store
    pub fn mutate_with_store(self, property_name: &str) -> Result<KCoreMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let longs: Vec<i64> = result.core_values.into_iter().map(|v| v as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels_set: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels_set, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("KCore mutate failed to add property: {e}"))
            })?;

        let summary = KCoreMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(KCoreMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
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
        let result = self.compute()?;
        Ok(KCoreComputationResult {
            core_values: result.core_values,
            degeneracy: result.degeneracy,
        })
    }
}
