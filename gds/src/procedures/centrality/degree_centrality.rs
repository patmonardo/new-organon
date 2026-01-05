//! Degree Centrality Facade
//!
//! **What is it?**: Counts the number of connections (edges) each node has
//! **Why care?**: Identifies highly-connected nodes (hubs) in the network
//! **Complexity**: O(V + E) - linear in graph size
//! **Best for**: Quick identification of important nodes by connectivity
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # use std::sync::Arc;
//! # use gds::types::prelude::DefaultGraphStore;
//! # let graph = Graph::new(Arc::new(DefaultGraphStore::empty()));
//! // Get degree scores for all nodes
//! let results = graph.degree_centrality()
//!     .stream()?
//!     .collect::<Vec<_>>();
//!
//! // Get statistics
//! let stats = graph.degree_centrality().stats()?;
//! println!("Average degree: {}", stats.mean);
//!
//! // Store as node property for use in other algorithms
//! graph.degree_centrality().mutate("degree")?;
//! ```

use crate::mem::MemoryRange;
use crate::algo::degree_centrality::{
    DegreeCentralityComputationRuntime, DegreeCentralityStorageRuntime,
};
pub use crate::algo::degree_centrality::storage::Orientation;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::{CentralityScore, Result};
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::sync::{Arc, Mutex};
use std::time::Instant;

// Import upgraded systems
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, ProgressTracker, TaskRegistryFactory, Tasks,
};

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about degree distribution in the graph
#[derive(Debug, Clone, serde::Serialize)]
pub struct DegreeCentralityStats {
    /// Minimum degree found
    pub min: f64,
    /// Maximum degree found
    pub max: f64,
    /// Average degree
    pub mean: f64,
    /// Standard deviation
    pub stddev: f64,
    /// Median degree (50th percentile)
    pub p50: f64,
    /// 90th percentile degree
    pub p90: f64,
    /// 99th percentile degree
    pub p99: f64,
    /// Number of nodes with degree 0
    pub isolated_nodes: u64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

// ============================================================================
// Facade Type
// ============================================================================

/// DegreeCentrality algorithm facade/builder bound to a live graph store.
pub struct DegreeCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    normalize: bool,
    orientation: Orientation,
    has_relationship_weight_property: bool,
    concurrency: usize,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

impl DegreeCentralityFacade {
    /// Create a new DegreeCentrality facade bound to a graph store.
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            normalize: false,
            orientation: Orientation::Natural,
            has_relationship_weight_property: false,
            concurrency: 4,
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Normalize scores by max degree.
    pub fn normalize(mut self, normalize: bool) -> Self {
        self.normalize = normalize;
        self
    }

    /// Set orientation for degree computation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Use relationship weights when computing degree (sum of weights).
    pub fn weighted(mut self, weighted: bool) -> Self {
        self.has_relationship_weight_property = weighted;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Degree centrality benefits from parallelism in large graphs.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set task registry factory for progress tracking
    pub fn task_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.task_registry_factory = Some(factory);
        self
    }

    /// Set user log registry factory for progress tracking
    pub fn user_log_registry_factory(mut self, factory: Box<dyn TaskRegistryFactory>) -> Self {
        self.user_log_registry_factory = Some(factory);
        self
    }

    fn validate(&self) -> Result<()> {
        if self.concurrency == 0 {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "concurrency must be > 0".to_string(),
                ),
            );
        }

        Ok(())
    }

    fn compute_scores(self) -> Result<(Vec<f64>, std::time::Duration)> {
        self.validate()?;

        let start = Instant::now();

        let storage = DegreeCentralityStorageRuntime::with_settings(
            self.graph_store.as_ref(),
            self.orientation,
            self.has_relationship_weight_property,
        )?;

        let node_count = storage.node_count();

        let empty_factory = EmptyTaskRegistryFactory;
        let registry_factory: &dyn TaskRegistryFactory = match self.task_registry_factory.as_deref()
        {
            Some(factory) => factory,
            None => &empty_factory,
        };

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_registry(
            Tasks::leaf_with_volume("degree_centrality".to_string(), node_count)
                .base()
                .clone(),
            crate::concurrency::Concurrency::of(self.concurrency.max(1)),
            crate::core::utils::progress::JobId::new(),
            registry_factory,
        );
        progress_tracker.begin_subtask_with_volume(node_count);

        let tracker = Arc::new(Mutex::new(progress_tracker));
        let on_nodes_done = {
            let tracker = Arc::clone(&tracker);
            Arc::new(move |n: usize| {
                tracker.lock().unwrap().log_progress(n);
            })
        };

        let termination = crate::concurrency::TerminationFlag::default();
        let mut scores = match storage.compute_parallel(
            self.concurrency,
            &termination,
            on_nodes_done,
        ) {
            Ok(scores) => scores,
            Err(e) => {
                tracker.lock().unwrap().end_subtask_with_failure();
                return Err(crate::projection::eval::procedure::AlgorithmError::Execution(
                    format!("Degree centrality terminated: {e}"),
                ));
            }
        };

        if self.normalize {
            DegreeCentralityComputationRuntime::normalize_scores(&mut scores);
        }

        tracker.lock().unwrap().end_subtask();

        Ok((scores, start.elapsed()))
    }

    /// Stream mode: Get degree for each node
    ///
    /// Returns an iterator over (node_id, degree) tuples.
    /// Degrees are raw counts by default (not normalized).
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new();
    /// for (node_id, degree) in facade.stream()? {
    ///     if degree > 100.0 {
    ///         println!("Hub: node {} has {} connections", node_id, degree);
    ///     }
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
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

    /// Stats mode: Get aggregated statistics about degree distribution
    ///
    /// Returns min, max, mean, stddev, and percentiles of the degree distribution.
    /// This is useful for understanding the overall graph structure without
    /// needing to process individual scores.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new();
    /// let stats = facade.stats()?;
    /// println!("Graph has average degree: {}", stats.mean);
    /// println!("Max degree (highest hub): {}", stats.max);
    /// println!("Isolated nodes: {}", stats.isolated_nodes);
    /// ```
    pub fn stats(self) -> Result<DegreeCentralityStats> {
        let (scores, elapsed) = self.compute_scores()?;
        if scores.is_empty() {
            return Ok(DegreeCentralityStats {
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

        Ok(DegreeCentralityStats {
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

    /// Mutate mode: Compute and store degree as a node property
    ///
    /// Stores the degree of each node as a property in the graph.
    /// This allows other algorithms to use the degree as input.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new();
    /// let result = facade.mutate("degree")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let start_time = Instant::now();
        let (scores, _elapsed) = self.compute_scores()?;

        // For now, use placeholder mutation - just count the nodes that would be updated
        // TODO: Implement actual graph mutation using the mutation machinery
        let nodes_updated = scores.len() as u64;

        let execution_time = start_time.elapsed();
        Ok(MutationResult::new(nodes_updated, property_name.to_string(), execution_time))
    }

    /// Write mode: Compute and write results to external storage
    ///
    /// Writes the degree centrality scores to an external data store.
    /// This is useful for persisting results for later analysis.
    ///
    /// # Arguments
    /// * `property_name` - Name of the property to store the centrality scores
    ///
    /// # Returns
    /// Result containing write statistics
    ///
    /// # Example
    /// ```ignore
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new();
    /// let result = facade.write("degree_centrality")?;
    /// println!("Wrote {} records", result.records_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let start_time = Instant::now();
        let (scores, _elapsed) = self.compute_scores()?;

        // For now, use placeholder write - just count the nodes that would be written
        // TODO: Implement actual persistence to external storage
        let nodes_written = scores.len() as u64;

        let execution_time = start_time.elapsed();
        Ok(WriteResult::new(nodes_written, property_name.to_string(), execution_time))
    }

    /// Estimate memory usage for this algorithm execution
    ///
    /// Provides an estimate of the memory required to run this algorithm
    /// with the current configuration. This is useful for capacity planning
    /// and preventing out-of-memory errors.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new();
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min_bytes, memory.max_bytes);
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Memory for centrality scores (one f64 per node)
        let scores_memory = node_count * std::mem::size_of::<f64>();

        // Additional overhead for computation
        let computation_overhead = 1024 * 1024; // 1MB for temporary structures

        let total_memory = scores_memory + computation_overhead;
        let total_with_overhead = total_memory + (total_memory / 5); // Add 20% overhead

        MemoryRange::of_range(total_memory, total_with_overhead)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(3),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_facade_creation() {
        let _facade = DegreeCentralityFacade::new(store());
        // Smoke test - just verify it creates without panic
    }

    #[test]
    fn test_stream_returns_iterator() {
        let facade = DegreeCentralityFacade::new(store());
        let result = facade.stream();
        assert!(result.is_ok());
    }

    #[test]
    fn test_stats_returns_valid_structure() {
        let facade = DegreeCentralityFacade::new(store());
        let stats = facade.stats().unwrap();
        assert!(stats.max >= stats.min);
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let facade = DegreeCentralityFacade::new(store());
        let result = facade.mutate("");
        assert!(result.is_err()); // Empty property name should fail
    }

    #[test]
    fn test_mutate_accepts_valid_property_name() {
        let facade = DegreeCentralityFacade::new(store());
        let result = facade.mutate("degree");
        assert!(result.is_ok()); // Should succeed with valid property name
        let mutation_result = result.unwrap();
        assert_eq!(mutation_result.property_name, "degree");
    }
}
