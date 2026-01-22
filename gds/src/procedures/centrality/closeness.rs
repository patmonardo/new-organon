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

use crate::algo::closeness::computation::ClosenessCentralityComputationRuntime;
use crate::algo::closeness::ClosenessCentralityStorageRuntime;
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, Task, TaskProgressTracker, TaskRegistryFactory, Tasks,
};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::{CentralityScore, Result};
use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::orientation::Orientation;
use crate::projection::NodeLabel;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::impls::default_node_property_values::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

/// Statistics about closeness centrality.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ClosenessCentralityStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub stddev: f64,
    pub p50: f64,
    pub p90: f64,
    pub p99: f64,
    pub isolated_nodes: u64,
    pub execution_time_ms: u64,
}

/// Mutation result for closeness centrality, including the updated graph store.
#[derive(Debug, Clone)]
pub struct ClosenessCentralityMutateResult {
    pub summary: MutationResult,
    pub updated_store: Arc<DefaultGraphStore>,
}

/// Closeness centrality facade/builder bound to a live graph store.
#[derive(Clone)]
pub struct ClosenessCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    wasserman_faust: bool,
    direction: String,
    concurrency: usize,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl ClosenessCentralityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            wasserman_faust: false,
            direction: "both".to_string(),
            concurrency: 4,
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
    }

    /// Enable/disable Wasserman–Faust normalization.
    pub fn wasserman_faust(mut self, enabled: bool) -> Self {
        self.wasserman_faust = enabled;
        self
    }

    /// Direction of traversal: "outgoing", "incoming", or "both".
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    /// Set concurrency level for parallel computation.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set the task registry factory for progress tracking and concurrency control.
    pub fn task_registry(mut self, task_registry: Arc<dyn TaskRegistryFactory>) -> Self {
        self.task_registry = task_registry;
        self
    }

    fn orientation(&self) -> Orientation {
        match self.direction.as_str() {
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
        if self.concurrency == 0 {
            return Err(AlgorithmError::Execution(
                "concurrency must be positive".to_string(),
            ));
        }
        Ok(())
    }

    fn compute_scores(&self) -> Result<(Vec<f64>, std::time::Duration)> {
        let start = Instant::now();

        let storage =
            ClosenessCentralityStorageRuntime::new(self.graph_store.as_ref(), self.orientation())?;
        let node_count = storage.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), start.elapsed()));
        }

        let computation = ClosenessCentralityComputationRuntime::new();

        let farness_task =
            std::sync::Arc::new(Task::leaf("Farness computation".to_string(), node_count));
        let closeness_task =
            std::sync::Arc::new(Task::leaf("Closeness computation".to_string(), node_count));
        let root_task = Tasks::task("closeness".to_string(), vec![farness_task, closeness_task]);

        let mut progress_tracker = TaskProgressTracker::with_registry(
            root_task,
            Concurrency::of(self.concurrency.max(1)),
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
                self.wasserman_faust,
                self.concurrency,
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

        Ok((centralities, start.elapsed()))
    }

    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
        self.validate()?;
        let (scores, _elapsed) = self.compute_scores()?;
        let iter = scores
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
        let (scores, elapsed) = self.compute_scores()?;
        if scores.is_empty() {
            return Ok(ClosenessCentralityStats {
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                stddev: 0.0,
                p50: 0.0,
                p90: 0.0,
                p99: 0.0,
                isolated_nodes: 0,
                execution_time_ms: elapsed.as_millis() as u64,
            });
        }

        let isolated_nodes = scores.iter().filter(|v| **v == 0.0).count() as u64;

        let mut sorted = scores.clone();
        sorted.sort_by(|a, b| a.total_cmp(b));
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let var = scores
            .iter()
            .map(|x| {
                let d = x - mean;
                d * d
            })
            .sum::<f64>()
            / scores.len() as f64;
        let stddev = var.sqrt();

        let percentile = |p: f64| -> f64 {
            let idx =
                ((p.clamp(0.0, 100.0) / 100.0) * (sorted.len() as f64 - 1.0)).round() as usize;
            sorted[idx]
        };

        Ok(ClosenessCentralityStats {
            min,
            max,
            mean,
            stddev,
            p50: percentile(50.0),
            p90: percentile(90.0),
            p99: percentile(99.0),
            isolated_nodes,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Mutate mode: compute scores, write them to a new graph store, and return it.
    pub fn mutate(self, property_name: &str) -> Result<ClosenessCentralityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let start_time = Instant::now();
        let (scores, _elapsed) = self.compute_scores()?;

        let nodes_updated = scores.len() as u64;

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

        let execution_time = start_time.elapsed();
        let summary = MutationResult::new(nodes_updated, property_name.to_string(), execution_time);

        Ok(ClosenessCentralityMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode is not implemented yet for closeness.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let start_time = Instant::now();
        let (scores, _elapsed) = self.compute_scores()?;

        let nodes_written = scores.len() as u64;

        let execution_time = start_time.elapsed();
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

        let concurrency = self.concurrency.max(1);

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
