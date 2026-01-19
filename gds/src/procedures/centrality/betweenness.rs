//! Betweenness Centrality Facade
//!
//! **What is it?**: Fraction of shortest paths that pass through each node
//! **Why care?**: Identifies "bridge" nodes that connect different network regions
//! **Complexity**: O(V*(V+E)) using Brandes' algorithm - more expensive!
//! **Best for**: Finding bottlenecks and critical connectors in networks
//!
//! ## What Betweenness Means
//!
//! For each node N:
//! - For every pair of other nodes (S, T), find shortest path from S to T
//! - Count how many of those shortest paths pass through N
//! - Betweenness = (# paths through N) / (# shortest paths total)
//!
//! High betweenness = critical for network flow/communication
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! let results = graph
//!     .betweenness()
//!     .stream()?
//!     .collect::<Vec<_>>();
//!
//! let stats = graph.betweenness().stats()?;
//! println!("Max betweenness: {} (bottleneck identified)", stats.max);
//! ```

use crate::algo::betweenness::storage::BetweennessCentralityStorageRuntime;
use crate::algo::betweenness::BetweennessCentralityComputationRuntime;
use crate::collections::backends::vec::VecDouble;
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::ProgressTracker;
use crate::core::utils::progress::{EmptyTaskRegistryFactory, TaskRegistryFactory, Tasks};
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::{CentralityScore, Result};
use crate::projection::orientation::Orientation;
use crate::projection::NodeLabel;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use crate::types::properties::node::impls::default_node_property_values::DefaultDoubleNodePropertyValues;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about betweenness centrality in the graph
#[derive(Debug, Clone, serde::Serialize)]
pub struct BetweennessStats {
    /// Minimum betweenness score
    pub min: f64,
    /// Maximum betweenness score
    pub max: f64,
    /// Average betweenness
    pub mean: f64,
    /// Standard deviation
    pub stddev: f64,
    /// Median (50th percentile)
    pub p50: f64,
    /// 90th percentile
    pub p90: f64,
    /// 99th percentile
    pub p99: f64,
    /// Number of "bridge" nodes (high betweenness > mean + stddev)
    pub bridge_nodes: u64,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Mutation result for betweenness centrality, including the updated graph store.
#[derive(Debug, Clone)]
pub struct BetweennessCentralityMutateResult {
    pub summary: MutationResult,
    pub updated_store: Arc<DefaultGraphStore>,
}

/// Betweenness centrality facade/builder bound to a live graph store.
#[derive(Clone)]
pub struct BetweennessCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    direction: String,
    concurrency: usize,
    relationship_weight_property: Option<String>,
    sampling_strategy: String,
    sampling_size: Option<usize>,
    random_seed: u64,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl BetweennessCentralityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            direction: "both".to_string(),
            concurrency: 4,
            relationship_weight_property: None,
            sampling_strategy: "all".to_string(),
            sampling_size: None,
            random_seed: 42,
            task_registry: Arc::new(EmptyTaskRegistryFactory),
        }
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

    /// Optional relationship weight property.
    ///
    /// When set, betweenness runs the weighted Brandes variant (Dijkstra forward phase).
    pub fn relationship_weight_property(mut self, property: Option<String>) -> Self {
        self.relationship_weight_property = property.filter(|p| !p.trim().is_empty());
        self
    }

    /// Sampling strategy for selecting source nodes.
    ///
    /// Supported values:
    /// - "all" (default): use all nodes as sources
    /// - "random_degree": sample sources weighted by node degree
    pub fn sampling_strategy(mut self, strategy: &str) -> Self {
        self.sampling_strategy = strategy.to_string();
        self
    }

    /// Optional sampling size (number of source nodes to process).
    ///
    /// If not set, all nodes are used.
    pub fn sampling_size(mut self, size: Option<usize>) -> Self {
        self.sampling_size = size;
        self
    }

    /// Seed for sampling RNG.
    pub fn random_seed(mut self, seed: u64) -> Self {
        self.random_seed = seed;
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
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "concurrency must be positive".to_string(),
                ),
            );
        }
        if let Some(size) = self.sampling_size {
            if size == 0 {
                return Err(
                    crate::projection::eval::procedure::AlgorithmError::Execution(
                        "sampling_size must be positive".to_string(),
                    ),
                );
            }
        }
        Ok(())
    }

    fn compute_scores(&self) -> Result<(Vec<f64>, std::time::Duration)> {
        let start = Instant::now();

        // For weighted traversal, we must select the relationship property across all rel types.
        let rel_types: HashSet<RelationshipType> = self.graph_store.relationship_types();
        let graph_view = if let Some(weight_prop) = &self.relationship_weight_property {
            let selectors: HashMap<RelationshipType, String> = rel_types
                .iter()
                .map(|t| (t.clone(), weight_prop.clone()))
                .collect();
            self.graph_store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    self.orientation(),
                )
                .map_err(|e| {
                    crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
                })?
        } else {
            self.graph_store
                .get_graph_with_types_and_orientation(&rel_types, self.orientation())
                .map_err(|e| {
                    crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
                })?
        };

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), start.elapsed()));
        }

        // Create storage and computation runtimes
        let storage = BetweennessCentralityStorageRuntime::new(
            &*self.graph_store,
            self.orientation(),
            self.relationship_weight_property.as_deref(),
        )?;
        let mut computation = BetweennessCentralityComputationRuntime::new(node_count);

        let sources: Vec<usize> = {
            let requested = self.sampling_size.unwrap_or(node_count).min(node_count);
            storage.select_sources(&self.sampling_strategy, Some(requested), self.random_seed)
        };

        let mut progress_tracker = crate::core::utils::progress::TaskProgressTracker::with_registry(
            Tasks::leaf_with_volume("betweenness".to_string(), sources.len())
                .base()
                .clone(),
            crate::concurrency::Concurrency::of(self.concurrency.max(1)),
            crate::core::utils::progress::JobId::new(),
            self.task_registry.as_ref(),
        );
        progress_tracker.begin_subtask_with_volume(sources.len());

        let divisor = if self.orientation() == Orientation::Undirected {
            2.0
        } else {
            1.0
        };

        let progress_handle = progress_tracker.clone();
        let on_source_done = Arc::new(move || {
            let mut tracker = progress_handle.clone();
            tracker.log_progress(1);
        });

        let termination = TerminationFlag::running_true();

        // Call storage.compute_betweenness - Applications talk only to procedures
        let result = storage
            .compute_betweenness(
                &mut computation,
                &sources,
                divisor,
                self.concurrency,
                &termination,
                on_source_done,
            )
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "Betweenness terminated: {e}"
                ))
            })?;

        progress_tracker.end_subtask();

        Ok((result.centralities, start.elapsed()))
    }

    /// Stream mode: Get betweenness score for each node
    ///
    /// Returns an iterator over (node_id, score) tuples.
    ///
    /// **Warning**: This algorithm is O(V*E), so streaming on large graphs
    /// may take a while. Consider computing stats instead for overview.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::BetweenessBuilder;
    /// let builder = BetweenessBuilder::new();
    /// for score in builder.stream()? {
    ///     if score.score > 0.1 {
    ///         println!("Bridge node: {} (betweenness: {:.4})", score.node_id, score.score);
    ///     }
    /// }
    /// ```
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

    /// Stats mode: Get aggregated statistics
    ///
    /// This is the recommended way to analyze betweenness on large graphs.
    /// Returns min, max, mean, stddev, percentiles, and identifies "bridge" nodes.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::BetweenessBuilder;
    /// let builder = BetweenessBuilder::new();
    /// let stats = builder.stats()?;
    /// println!("Found {} bridge nodes", stats.bridge_nodes);
    /// println!("Execution took {}ms", stats.execution_time_ms);
    /// ```
    pub fn stats(&self) -> Result<BetweennessStats> {
        self.validate()?;
        let (scores, elapsed) = self.compute_scores()?;
        if scores.is_empty() {
            return Ok(BetweennessStats {
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                stddev: 0.0,
                p50: 0.0,
                p90: 0.0,
                p99: 0.0,
                bridge_nodes: 0,
                execution_time_ms: elapsed.as_millis() as u64,
            });
        }

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

        let threshold = mean + stddev;
        let bridge_nodes = scores.iter().filter(|v| **v > threshold).count() as u64;

        Ok(BetweennessStats {
            min,
            max,
            mean,
            stddev,
            p50: percentile(50.0),
            p90: percentile(90.0),
            p99: percentile(99.0),
            bridge_nodes,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores betweenness scores as a node property.
    /// Useful for follow-up analysis like identifying connectors.
    ///
    /// ## Example
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::BetweenessBuilder;
    /// let builder = BetweenessBuilder::new();
    /// let result = builder.mutate("betweenness")?;
    /// println!("Computed and stored for {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<BetweennessCentralityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;
        let start_time = Instant::now();
        let (scores, _elapsed) = self.compute_scores()?;

        let nodes_updated = scores.len() as u64;

        // Build property values
        let node_count = scores.len();
        let backend = VecDouble::from(scores);
        let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
        let values: Arc<dyn crate::types::properties::node::NodePropertyValues> = Arc::new(values);

        // Clone store, add property, and return updated store
        let mut new_store = self.graph_store.as_ref().clone();
        let labels: HashSet<NodeLabel> = new_store.node_labels();
        new_store
            .add_node_property(labels, property_name.to_string(), values)
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "Betweenness mutate failed to add property: {e}"
                ))
            })?;

        let execution_time = start_time.elapsed();
        let summary = MutationResult::new(nodes_updated, property_name.to_string(), execution_time);

        Ok(BetweennessCentralityMutateResult {
            summary,
            updated_store: Arc::new(new_store),
        })
    }

    /// Write mode is not implemented yet for betweenness.
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "BetweennessCentrality mutate/write is not implemented yet".to_string(),
            ),
        )
    }

    /// Estimate memory requirements for betweenness centrality computation.
    ///
    /// # Returns
    /// Memory range estimate (min/max bytes)
    ///
    /// # Example
    /// ```ignore
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::BetweennessCentralityFacade;
    /// let facade = BetweennessCentralityFacade::new(graph);
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min(), memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Global scores (atomic f64 per node).
        let scores_memory = node_count * std::mem::size_of::<f64>();

        // Per-worker state (approx):
        // - sigma: u64 per node
        // - delta: f64 per node
        // - distance: i32 per node (unweighted) or f64 per node (weighted)
        // - stack/queue/visited: usize per node
        // - predecessors: worst-case proportional to relationships (very graph dependent)
        let per_node_base = std::mem::size_of::<u64>()
            + std::mem::size_of::<f64>()
            + if self.relationship_weight_property.is_some() {
                std::mem::size_of::<f64>()
            } else {
                std::mem::size_of::<i32>()
            };
        let per_node_worklists = 3 * std::mem::size_of::<usize>();

        let per_worker = node_count * (per_node_base + per_node_worklists);
        let workers = self.concurrency.max(1);

        // Predecessors are the dominant cost in dense graphs; approximate it using relationship count.
        let preds_estimate = self.graph_store.relationship_count() * std::mem::size_of::<usize>();

        // Add fixed overhead.
        let overhead = 1024 * 1024;

        let min = scores_memory + workers * per_worker + overhead;
        let max = min + preds_estimate + (min / 5);

        MemoryRange::of_range(min, max)
    }
}

// Sampling logic no longer needs a standalone item wrapper.

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    fn store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            seed: Some(19),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_stream_returns_node_count_rows() {
        let facade = BetweennessCentralityFacade::new(store());
        let rows: Vec<_> = facade.stream().unwrap().collect();
        assert_eq!(rows.len(), 8);
    }

    #[test]
    fn test_stats_shape() {
        let facade = BetweennessCentralityFacade::new(store());
        let stats = facade.stats().unwrap();
        assert!(stats.max >= stats.min);
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let facade = BetweennessCentralityFacade::new(store());
        assert!(facade.clone().mutate("").is_err());
        let result = facade.mutate("betweenness");
        assert!(result.is_ok());
        let mutation_result = result.unwrap();
        assert_eq!(mutation_result.summary.property_name, "betweenness");
        assert!(mutation_result
            .updated_store
            .has_node_property("betweenness"));
    }
}
