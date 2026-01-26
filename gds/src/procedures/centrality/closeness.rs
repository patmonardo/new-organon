//! Closeness Centrality Facade
//!
//! **What is it?**: Measures how close a node is to all other nodes
//! **Why care?**: Finds nodes that can reach others quickly (good broadcasters)
//! **Complexity**: O(V*(V+E)) in the worst case (all-pairs BFS)
//!
//! This implementation follows the Neo4j GDS behavior:
//! - Uses MSBFS-style aggregation to compute farness and component size
//! - Centrality formula: `componentSize / farness`
//! - Optional Wasserman–Faust normalization

use crate::algo::algorithms::{CentralityScore, Result};
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::closeness::ClosenessCentralityStorageRuntime;
use crate::algo::closeness::{
    ClosenessCentralityComputationRuntime, ClosenessCentralityConfig,
    ClosenessCentralityMutateResult, ClosenessCentralityMutationSummary, ClosenessCentralityResult,
    ClosenessCentralityResultBuilder, ClosenessCentralityStats,
};
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, Task, TaskProgressTracker, TaskRegistryFactory, Tasks,
};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::NodeLabel;
use crate::projection::Orientation;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Closeness centrality facade/builder bound to a live graph store.
#[derive(Clone)]
pub struct ClosenessCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: ClosenessCentralityConfig,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl ClosenessCentralityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: ClosenessCentralityConfig::default(),
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: ClosenessCentralityConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: ClosenessCentralityConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: ClosenessCentralityConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    /// Enable/disable Wasserman–Faust normalization.
    pub fn wasserman_faust(mut self, enabled: bool) -> Self {
        self.config.wasserman_faust = enabled;
        self
    }

    /// Direction of traversal: "outgoing", "incoming", or "both".
    pub fn direction(mut self, direction: &str) -> Self {
        self.config.direction = direction.to_string();
        self
    }

    /// Set concurrency level for parallel computation.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
        self
    }

    /// Set the task registry factory for progress tracking and concurrency control.
    pub fn task_registry(mut self, task_registry: Arc<dyn TaskRegistryFactory>) -> Self {
        self.task_registry = task_registry;
        self
    }

    fn orientation(&self) -> Orientation {
        match self.config.direction.as_str() {
            "incoming" => Orientation::Reverse,
            "outgoing" => Orientation::Natural,
            _ => Orientation::Undirected,
        }
    }

    /// Validate the facade configuration.
    ///
    /// # Returns
    /// Ok(()) if configuration is valid, Err otherwise
    ///
    /// # Errors
    /// Returns an error if concurrency is not positive
    pub fn validate(&self) -> Result<()> {
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<ClosenessCentralityResult> {
        let start = Instant::now();

        let storage =
            ClosenessCentralityStorageRuntime::new(self.graph_store.as_ref(), self.orientation())?;
        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok(ClosenessCentralityResult {
                centralities: Vec::new(),
                node_count,
                execution_time: start.elapsed(),
            });
        }

        let computation = ClosenessCentralityComputationRuntime::new();

        let farness_task =
            std::sync::Arc::new(Task::leaf("Farness computation".to_string(), node_count));
        let closeness_task =
            std::sync::Arc::new(Task::leaf("Closeness computation".to_string(), node_count));
        let root_task = Tasks::task("closeness".to_string(), vec![farness_task, closeness_task]);

        let mut progress_tracker = TaskProgressTracker::with_registry(
            root_task,
            Concurrency::of(self.config.concurrency.max(1)),
            JobId::new(),
            self.task_registry.as_ref(),
        );

        // Start root then the farness leaf.
        progress_tracker.begin_subtask();
        progress_tracker
            .begin_subtask_with_description_and_volume("Farness computation", node_count);

        let termination = TerminationFlag::running_true();

        let farness_progress_handle = progress_tracker.clone();
        let on_sources_done = Arc::new(move |sources_done: usize| {
            let mut tracker = farness_progress_handle.clone();
            tracker.log_progress(sources_done);
        });

        let on_closeness_done = Arc::new(|_nodes_done: usize| {});

        let centralities = storage
            .compute_parallel(
                &computation,
                self.config.wasserman_faust,
                self.config.concurrency,
                &termination,
                on_sources_done,
                on_closeness_done,
            )
            .map_err(|e| AlgorithmError::Execution(format!("Closeness terminated: {e}")))?;

        progress_tracker.end_subtask_with_description("Farness computation");

        // The runtime computes scores in parallel already; keep a second task for Java parity.
        progress_tracker
            .begin_subtask_with_description_and_volume("Closeness computation", node_count);
        progress_tracker.log_progress(node_count);
        progress_tracker.end_subtask_with_description("Closeness computation");

        // End root.
        progress_tracker.end_subtask();
        progress_tracker.release();

        Ok(ClosenessCentralityResult {
            centralities,
            node_count,
            execution_time: start.elapsed(),
        })
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
        self.validate()?;
        let result = self.compute()?;
        let iter = result
            .centralities
            .into_iter()
            .enumerate()
            .map(|(node_id, score)| CentralityScore {
                node_id: node_id as u64,
                score,
            });
        Ok(Box::new(iter))
    }

    pub fn stats(&self) -> Result<ClosenessCentralityStats> {
        self.validate()?;
        let result = self.compute()?;
        Ok(ClosenessCentralityResultBuilder::new(result).stats())
    }

    /// Mutate mode: compute scores, write them to a new graph store, and return it.
    pub fn mutate(self, property_name: &str) -> Result<ClosenessCentralityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let result = self.compute()?;
        let scores = result.centralities.clone();
        let nodes_updated = scores.len() as u64;
        let builder = ClosenessCentralityResultBuilder::new(result);

        // Build property values
        let node_count = scores.len();
        let backend = VecDouble::from(scores);
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn NodePropertyValues> = Arc::new(values);

        // Clone store, add property, and return updated store
        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                AlgorithmError::Execution(format!("Closeness mutate failed to add property: {e}"))
            })?;

        let summary = ClosenessCentralityMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: builder.execution_time_ms(),
        };

        Ok(ClosenessCentralityMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode is not implemented yet for closeness.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let result = self.compute()?;
        let nodes_written = result.centralities.len() as u64;
        let execution_time = result.execution_time;
        Ok(WriteResult::new(
            nodes_written,
            property_name.to_string(),
            execution_time,
        ))
    }

    /// Estimate memory requirements for closeness centrality computation.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::ClosenessCentralityFacade;
    /// let facade = ClosenessCentralityFacade::new(graph);
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min(), memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        let concurrency = self.config.concurrency.max(1);

        // Memory for closeness scores (one f64 per node)
        let scores_memory = node_count * std::mem::size_of::<f64>();

        // Atomic accumulation arrays for farness/component (i64 per node each)
        let farness_memory = node_count * std::mem::size_of::<i64>();
        let component_memory = node_count * std::mem::size_of::<i64>();

        // Per-worker MSBFS bitsets: visit, visit_next, seen (u64 per node each)
        let msbfs_per_worker = 3 * node_count * std::mem::size_of::<u64>();
        let msbfs_memory = msbfs_per_worker * concurrency;

        // Additional overhead for executor + temporary vectors.
        let overhead = 1024 * 1024; // 1MB

        let total = scores_memory + farness_memory + component_memory + msbfs_memory + overhead;
        let total_with_overhead = total + (total / 5);

        MemoryRange::of_range(total, total_with_overhead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(7),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_stream_returns_node_count_rows() {
        let facade = ClosenessCentralityFacade::new(store());
        let rows: Vec<_> = facade.stream().unwrap().collect();
        assert_eq!(rows.len(), 8);
    }

    #[test]
    fn test_stats_shape() {
        let facade = ClosenessCentralityFacade::new(store());
        let stats = facade.stats().unwrap();
        assert!(stats.max >= stats.min);
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let facade = ClosenessCentralityFacade::new(store());
        assert!(facade.clone().mutate("").is_err());
        let result = facade.mutate("closeness");
        assert!(result.is_ok());
        let mutation_result = result.unwrap();
        assert_eq!(mutation_result.summary.property_name, "closeness");
        assert!(mutation_result.updated_store.has_node_property("closeness"));
    }
}
