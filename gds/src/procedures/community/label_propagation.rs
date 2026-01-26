//! Label Propagation Facade
//!
//! Community detection by iterative label voting.
//!
//! Parameters (Java GDS aligned):
//! - `concurrency`: accepted for parity; currently unused.
//! - `max_iterations`: max number of propagation iterations (must be >= 1).
//! - `node_weight_property`: optional node weight property (defaults to 1.0).
//! - `seed_property`: optional seed labels property.

use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::label_propagation::computation::LabelPropComputationRuntime;
use crate::algo::label_propagation::spec::{
    LabelPropConfig, LabelPropMutateResult, LabelPropMutationSummary, LabelPropResult,
    LabelPropResultBuilder, LabelPropStats,
};
use crate::algo::label_propagation::storage::LabelPropStorageRuntime;
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

/// Per-node label assignment row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct LabelPropagationRow {
    pub node_id: u64,
    pub label_id: u64,
}

/// Label Propagation algorithm facade.
#[derive(Clone)]
pub struct LabelPropagationFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: LabelPropConfig,
    task_registry: Option<TaskRegistry>,
}

impl LabelPropagationFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: LabelPropConfig::default(),
            task_registry: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: LabelPropConfig,
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
        let parsed: LabelPropConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: LabelPropConfig) -> Result<Self> {
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

    pub fn max_iterations(mut self, max_iterations: u64) -> Self {
        self.config.max_iterations = max_iterations;
        self
    }

    pub fn node_weight_property(mut self, property: &str) -> Self {
        self.config.node_weight_property = Some(property.to_string());
        self
    }

    pub fn seed_property(mut self, property: &str) -> Self {
        self.config.seed_property = Some(property.to_string());
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

    fn compute(&self) -> Result<LabelPropResult> {
        self.validate()?;
        let start = Instant::now();

        let config = self.config.clone();

        let storage = LabelPropStorageRuntime::new(self.graph_store.as_ref())?;
        let node_count = storage.node_count();

        let base_task = Tasks::leaf_with_volume(
            "label_propagation".to_string(),
            node_count.saturating_add(self.config.max_iterations as usize),
        );
        let mut progress_tracker =
            TaskProgressTracker::with_concurrency(base_task, self.config.concurrency);

        let termination_flag = TerminationFlag::default();

        let runtime = LabelPropComputationRuntime::new(node_count, self.config.max_iterations)
            .concurrency(self.config.concurrency);

        let result = storage.compute_label_propagation(
            runtime,
            &config,
            &mut progress_tracker,
            &termination_flag,
        )?;

        Ok(LabelPropResult {
            labels: result.labels,
            did_converge: result.did_converge,
            ran_iterations: result.ran_iterations,
            node_count,
            execution_time: start.elapsed(),
        })
    }

    /// Stream mode: yields `(node_id, label_id)` for every node.
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = LabelPropagationRow>>> {
        let result = self.compute()?;
        let iter = result
            .labels
            .into_iter()
            .enumerate()
            .map(|(node_id, label_id)| LabelPropagationRow {
                node_id: node_id as u64,
                label_id,
            });
        Ok(Box::new(iter))
    }

    /// Stats mode: yields convergence info + community count.
    pub fn stats(&self) -> Result<LabelPropStats> {
        let result = self.compute()?;
        Ok(LabelPropResultBuilder::new(result).stats())
    }

    /// Mutate mode: writes labels back to the graph store.
    pub fn mutate(self, property_name: &str) -> Result<LabelPropMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let result = self.compute()?;

        let node_count = self.graph_store.node_count();
        let nodes_updated = node_count as u64;

        let longs: Vec<i64> = result.labels.into_iter().map(|l| l as i64).collect();
        let backend = VecLong::from(longs);
        let values = DefaultLongNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        let mut new_store = self.graph_store.as_ref().clone();
        let labels_set: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels_set, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!(
                    "Label Propagation mutate failed to add property: {e}"
                ))
            })?;

        let summary = LabelPropMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: result.execution_time.as_millis() as u64,
        };

        Ok(LabelPropMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode: writes labels to a new graph.
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
        // Label Propagation keeps two label buffers and scans relationships for message passing.
        let node_count = self.graph_store.node_count();
        let relationship_count = self.graph_store.relationship_count();

        // Per node: current+next labels (u64) + convergence bookkeeping.
        let per_node = 96usize;
        // Per relationship: transient accumulation while streaming.
        let per_relationship = 8usize;

        let base: usize = 64 * 1024;
        let total = base
            .saturating_add(node_count.saturating_mul(per_node))
            .saturating_add(relationship_count.saturating_mul(per_relationship));

        Ok(MemoryRange::of_range(total, total.saturating_mul(2)))
    }

    /// Full result: returns the procedure-level Label Propagation result.
    pub fn run(&self) -> Result<LabelPropResult> {
        let result = self.compute()?;
        Ok(result)
    }
}
