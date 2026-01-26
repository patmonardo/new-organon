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

use crate::algo::algorithms::{ConfigValidator, WriteResult};
pub use crate::algo::degree_centrality::storage::Orientation;
use crate::algo::degree_centrality::{
    DegreeCentralityComputationRuntime, DegreeCentralityConfig, DegreeCentralityMutateResult,
    DegreeCentralityMutationSummary, DegreeCentralityResult, DegreeCentralityResultBuilder,
    DegreeCentralityStats, DegreeCentralityStorageRuntime,
};
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, JobId, ProgressTracker, TaskProgressTracker, TaskRegistryFactory,
    Tasks,
};
use crate::mem::MemoryRange;
use crate::algo::algorithms::{CentralityScore, Result};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::NodeLabel;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::DefaultDoubleNodePropertyValues;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Instant;

// ============================================================================
// Facade Type
// ============================================================================

/// DegreeCentrality algorithm facade/builder bound to a live graph store.
pub struct DegreeCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    config: DegreeCentralityConfig,
    /// Progress tracking components
    task_registry_factory: Option<Box<dyn TaskRegistryFactory>>,
    user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>, // Placeholder for now
}

impl DegreeCentralityFacade {
    /// Create a new DegreeCentrality facade bound to a graph store.
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            config: DegreeCentralityConfig::default(),
            task_registry_factory: None,
            user_log_registry_factory: None,
        }
    }

    /// Create a facade using the spec.rs config model.
    pub fn from_spec_config(
        graph_store: Arc<DefaultGraphStore>,
        config: DegreeCentralityConfig,
    ) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;

        Ok(Self {
            graph_store,
            config,
            task_registry_factory: None,
            user_log_registry_factory: None,
        })
    }

    /// Parse JSON into spec.rs config and return a configured facade.
    pub fn from_spec_json(
        graph_store: Arc<DefaultGraphStore>,
        raw_config: &serde_json::Value,
    ) -> Result<Self> {
        let parsed: DegreeCentralityConfig = serde_json::from_value(raw_config.clone())
            .map_err(|e| AlgorithmError::Execution(format!("Config parsing failed: {e}")))?;
        Self::from_spec_config(graph_store, parsed)
    }

    /// Apply a spec.rs config onto an existing facade.
    pub fn with_spec_config(mut self, config: DegreeCentralityConfig) -> Result<Self> {
        config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))?;
        self.config = config;
        Ok(self)
    }

    /// Normalize scores by max degree.
    pub fn normalize(mut self, normalize: bool) -> Self {
        self.config.normalize = normalize;
        self
    }

    /// Set orientation for degree computation.
    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.config.orientation = match orientation {
            Orientation::Natural => "natural".to_string(),
            Orientation::Reverse => "reverse".to_string(),
            Orientation::Undirected => "undirected".to_string(),
        };
        self
    }

    /// Use relationship weights when computing degree (sum of weights).
    pub fn weighted(mut self, weighted: bool) -> Self {
        self.config.weighted = weighted;
        self
    }

    /// Set concurrency level
    ///
    /// Number of parallel threads to use.
    /// Degree centrality benefits from parallelism in large graphs.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.config.concurrency = concurrency;
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
        self.config
            .validate()
            .map_err(|e| AlgorithmError::Execution(format!("Invalid config: {e}")))
    }

    fn compute(&self) -> Result<DegreeCentralityResult> {
        self.validate()?;

        let start = Instant::now();

        let orientation = match self.config.orientation.to_lowercase().as_str() {
            "natural" | "outgoing" => Orientation::Natural,
            "reverse" | "incoming" => Orientation::Reverse,
            "undirected" | "both" => Orientation::Undirected,
            other => {
                return Err(AlgorithmError::Execution(format!(
                    "Invalid orientation '{other}'. Use 'natural', 'reverse', or 'undirected'"
                )))
            }
        };

        let storage = DegreeCentralityStorageRuntime::with_settings(
            self.graph_store.as_ref(),
            orientation,
            self.config.weighted,
        )?;

        let node_count = storage.node_count();

        let empty_factory = EmptyTaskRegistryFactory;
        let registry_factory: &dyn TaskRegistryFactory = match self.task_registry_factory.as_deref()
        {
            Some(factory) => factory,
            None => &empty_factory,
        };

        let mut progress_tracker = TaskProgressTracker::with_registry(
            Tasks::leaf_with_volume("degree_centrality".to_string(), node_count)
                .base()
                .clone(),
            Concurrency::of(self.config.concurrency.max(1)),
            JobId::new(),
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

        let termination = TerminationFlag::default();
        let computation = DegreeCentralityComputationRuntime::new();

        let mut scores = match storage.compute_parallel(
            &computation,
            self.config.concurrency,
            &termination,
            on_nodes_done,
        ) {
            Ok(scores) => scores,
            Err(e) => {
                tracker.lock().unwrap().end_subtask_with_failure();
                return Err(AlgorithmError::Execution(format!(
                    "Degree centrality terminated: {e}"
                )));
            }
        };

        if self.config.normalize {
            computation.normalize_scores(&mut scores);
        }

        tracker.lock().unwrap().end_subtask();

        Ok(DegreeCentralityResult {
            centralities: scores,
            node_count,
            execution_time: start.elapsed(),
        })
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
        let result = self.compute()?;
        Ok(DegreeCentralityResultBuilder::new(result).stats())
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
    pub fn mutate(self, property_name: &str) -> Result<DegreeCentralityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let result = self.compute()?;
        let scores = result.centralities.clone();
        let nodes_updated = scores.len() as u64;
        let builder = DegreeCentralityResultBuilder::new(result);

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
                AlgorithmError::Execution(format!(
                    "Degree centrality mutate failed to add property: {e}"
                ))
            })?;

        let summary = DegreeCentralityMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: builder.execution_time_ms(),
        };

        Ok(DegreeCentralityMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
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
        let result = self.compute()?;

        // For now, use placeholder write - just count the nodes that would be written
        // TODO: Implement actual persistence to external storage
        let nodes_written = result.centralities.len() as u64;

        let execution_time = result.execution_time;
        Ok(WriteResult::new(
            nodes_written,
            property_name.to_string(),
            execution_time,
        ))
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
        assert_eq!(mutation_result.summary.property_name, "degree");
        assert!(mutation_result.updated_store.has_node_property("degree"));
    }
}
