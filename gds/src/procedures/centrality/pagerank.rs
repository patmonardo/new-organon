//! PageRank Facade
//!
//! **What is it?**: Iterative algorithm computing node importance via link structure
//! **Why care?**: Models "random surfer" - nodes that many nodes link to are important
//! **Complexity**: O(k*(V + E)) where k is iterations
//! **Best for**: Finding important/authoritative nodes in networks
//!
//! ## The Random Surfer Model
//!
//! PageRank imagines a random person surfing the web:
//! - With probability `damping_factor`, they follow a random outgoing link
//! - With probability `1 - damping_factor`, they jump to a random page
//!
//! Nodes with more incoming links (and links from important nodes) get higher scores.
//!
//! ## Example
//!
//! ```rust,no_run
//! # use gds::Graph;
//! # let graph = Graph::default();
//! let results = graph
//!     .pagerank()
//!     .iterations(20)
//!     .damping_factor(0.85)
//!     .tolerance(1e-4)
//!     .stream()?
//!     .collect::<Vec<_>>();
//! ```

use crate::config::base_types::AlgoBaseConfig;
use crate::config::PageRankConfig;
use crate::mem::MemoryRange;
use crate::procedures::builder_base::{ConfigValidator, MutationResult, WriteResult};
use crate::procedures::traits::{CentralityScore, Result};
use crate::algo::pagerank::run_pagerank;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

// Import upgraded systems
use crate::core::utils::progress::{
    EmptyTaskRegistryFactory, ProgressTracker, TaskRegistryFactory, Tasks,
};

// ============================================================================
// Statistics Type
// ============================================================================

/// Statistics about PageRank computation
#[derive(Debug, Clone, serde::Serialize)]
pub struct PageRankStats {
    /// Minimum PageRank score
    pub min: f64,
    /// Maximum PageRank score
    pub max: f64,
    /// Average PageRank score
    pub mean: f64,
    /// Standard deviation of scores
    pub stddev: f64,
    /// Median score (50th percentile)
    pub p50: f64,
    /// 90th percentile score
    pub p90: f64,
    /// 99th percentile score
    pub p99: f64,
    /// How many iterations actually ran
    pub iterations_ran: u32,
    /// Did algorithm converge to tolerance?
    pub converged: bool,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

// ============================================================================
// Builder Type
// ============================================================================

/// PageRank algorithm facade - fluent configuration
///
/// Use this to configure and run PageRank with custom parameters.
/// Supports multiple execution modes via method chaining.
///
/// ## Default Configuration
/// - iterations: 20
/// - damping_factor: 0.85 (traditional value from Google)
/// - tolerance: 1e-4
///
/// ## Example
/// ```rust,no_run
/// # use gds::Graph;
/// # let graph = Graph::default();
/// # use gds::procedures::centrality::PageRankFacade;
/// let facade = PageRankFacade::new()
///     .iterations(30)
///     .damping_factor(0.85)
///     .tolerance(1e-5);
/// ```
#[derive(Clone)]
pub struct PageRankFacade {
    graph_store: Arc<DefaultGraphStore>,
    direction: String,
    /// Pregel concurrency (Rayon worker threads)
    concurrency: usize,
    /// Task registry for progress tracking
    task_registry: Arc<dyn TaskRegistryFactory>,
    /// Maximum iterations to run
    iterations: u32,
    /// Probability of following a relationship (damping)
    damping_factor: f64,
    /// Convergence threshold on delta
    tolerance: f64,
    /// Optional source nodes for personalized PageRank
    source_nodes: Option<Vec<u64>>,
}

impl PageRankFacade {
    /// Create a new PageRank facade bound to a live graph store.
    ///
    /// Defaults:
    /// - iterations: 20
    /// - damping_factor: 0.85
    /// - tolerance: 1e-4
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            direction: "outgoing".to_string(),
            concurrency: num_cpus::get().max(1),
            task_registry: Arc::new(EmptyTaskRegistryFactory),
            iterations: 20,
            damping_factor: 0.85,
            tolerance: 1e-4,
            source_nodes: None,
        }
    }

    /// Set Pregel concurrency (Rayon worker threads).
    ///
    /// Use `1` for deterministic single-threaded debugging.
    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set task registry factory for progress tracking
    pub fn task_registry(mut self, task_registry: Arc<dyn TaskRegistryFactory>) -> Self {
        self.task_registry = task_registry;
        self
    }

    /// Direction of traversal: "outgoing", "incoming", or "both".
    ///
    /// PageRank typically uses outgoing (natural) relationships.
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = direction.to_string();
        self
    }

    /// Personalize PageRank by only seeding `source_nodes` with $\alpha$.
    /// When set, all non-source nodes start at 0.
    pub fn source_nodes(mut self, source_nodes: Vec<u64>) -> Self {
        self.source_nodes = Some(source_nodes);
        self
    }

    /// Set maximum iterations
    ///
    /// The algorithm will stop after this many iterations or when converged,
    /// whichever comes first.
    ///
    /// Higher values = more accurate but slower.
    /// Typical: 10-50 iterations
    pub fn iterations(mut self, n: u32) -> Self {
        self.iterations = n;
        self
    }

    /// Set damping factor (probability of following a link)
    ///
    /// Range: (0.0, 1.0)
    ///
    /// - 0.85 (default): Traditional Google PageRank value
    /// - Higher (0.95): Edges matter more, random nodes less
    /// - Lower (0.5): Random teleportation matters more
    pub fn damping_factor(mut self, d: f64) -> Self {
        self.damping_factor = d;
        self
    }

    /// Set convergence tolerance
    ///
    /// The algorithm converges when max delta between iterations < tolerance.
    ///
    /// - 1e-4 (default): Good balance
    /// - 1e-6: Very tight, slower
    /// - 1e-3: Loose, faster
    pub fn tolerance(mut self, t: f64) -> Self {
        self.tolerance = t;
        self
    }

    /// Validate configuration before execution
    pub fn validate(&self) -> Result<()> {
        if self.concurrency == 0 {
            return Err(
                crate::projection::eval::procedure::AlgorithmError::Execution(
                    "concurrency must be greater than 0".to_string(),
                ),
            );
        }
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        ConfigValidator::iterations(self.iterations, "iterations")?;
        ConfigValidator::in_range(self.damping_factor, 0.01, 0.99, "damping_factor")?;
        ConfigValidator::positive(self.tolerance, "tolerance")?;
        Ok(())
    }

    fn orientation(&self) -> Orientation {
        match self.direction.as_str() {
            "incoming" => Orientation::Reverse,
            "outgoing" => Orientation::Natural,
            _ => Orientation::Undirected,
        }
    }

    fn compute_scores(self) -> Result<(Vec<f64>, u32, bool, std::time::Duration)> {
        self.validate()?;
        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, self.orientation())
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let pr_config = PageRankConfig::builder()
            .base(AlgoBaseConfig {
                concurrency: self.concurrency,
                ..AlgoBaseConfig::default()
            })
            .max_iterations(self.iterations as usize)
            .damping_factor(self.damping_factor)
            .tolerance(self.tolerance)
            .build()
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                    "PageRankConfig invalid: {e}"
                ))
            })?;

        let source_set = self
            .source_nodes
            .clone()
            .map(|v| v.into_iter().collect::<std::collections::HashSet<u64>>());

        let mut progress_tracker = ProgressTracker::with_concurrency(
            Tasks::leaf("pagerank", self.iterations as usize),
            self.concurrency,
        );
        progress_tracker.begin_subtask(self.iterations as usize);

        let run = run_pagerank(graph_view, pr_config, source_set);

        progress_tracker.log_progress(self.iterations as usize);
        progress_tracker.end_subtask();

        Ok((
            run.scores,
            run.ran_iterations as u32,
            run.did_converge,
            start.elapsed(),
        ))
    }

    /// Stream mode: Get PageRank score for each node
    ///
    /// Returns an iterator over (node_id, score) tuples.
    ///
    /// Use this when you want individual results, e.g.:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::PageRankFacade;
    /// let builder = PageRankFacade::new();
    /// for score in builder.stream()? {
    ///     println!("Node {} has score {}", score.node_id, score.score);
    /// }
    /// ```
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
        let (scores, _iters, _converged, _elapsed) = self.compute_scores()?;
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
    /// Returns min, max, mean, stddev, percentiles, and convergence info.
    ///
    /// Use this when you want overview statistics:
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::PageRankBuilder;
    /// let builder = PageRankFacade::new();
    /// let stats = builder.stats()?;
    /// println!("Converged: {}, Iterations: {}", stats.converged, stats.iterations_ran);
    /// ```
    pub fn stats(self) -> Result<PageRankStats> {
        let (scores, iterations_ran, converged, elapsed) = self.compute_scores()?;
        if scores.is_empty() {
            return Ok(PageRankStats {
                min: 0.0,
                max: 0.0,
                mean: 0.0,
                stddev: 0.0,
                p50: 0.0,
                p90: 0.0,
                p99: 0.0,
                iterations_ran,
                converged,
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

        Ok(PageRankStats {
            min,
            max,
            mean,
            stddev,
            p50: percentile(50.0),
            p90: percentile(90.0),
            p99: percentile(99.0),
            iterations_ran,
            converged,
            execution_time_ms: elapsed.as_millis() as u64,
        })
    }

    /// Mutate mode: Compute and store as node property
    ///
    /// Stores PageRank scores as a node property for use by other algorithms.
    ///
    /// ```rust,no_run
    /// # use gds::Graph;
    /// # let graph = Graph::default();
    /// # use gds::procedures::centrality::PageRankFacade;
    /// let facade = PageRankFacade::new().damping_factor(0.85);
    /// let result = facade.mutate("pagerank")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(self, property_name: &str) -> Result<MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let start_time = Instant::now();
        let (scores, _iterations_ran, _converged, _elapsed) = self.compute_scores()?;

        // For now, use placeholder mutation - just count the nodes that would be updated
        // TODO: Implement actual graph mutation using the mutation machinery
        let nodes_updated = scores.len() as u64;

        let execution_time = start_time.elapsed();
        Ok(MutationResult::new(nodes_updated, property_name.to_string(), execution_time))
    }

    /// Write mode: Compute and write results to external storage
    ///
    /// Writes the PageRank scores to an external data store.
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
    /// # use gds::procedures::centrality::PageRankFacade;
    /// let facade = PageRankFacade::new();
    /// let result = facade.write("pagerank")?;
    /// println!("Wrote {} records", result.records_written);
    /// ```
    pub fn write(self, property_name: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        let start_time = Instant::now();
        let (scores, _iterations_ran, _converged, _elapsed) = self.compute_scores()?;

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
    /// # use gds::procedures::centrality::PageRankFacade;
    /// let facade = PageRankFacade::new();
    /// let memory = facade.estimate_memory();
    /// println!("Will use between {} and {} bytes", memory.min(), memory.max());
    /// ```
    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Memory for PageRank scores (one f64 per node)
        let scores_memory = node_count * std::mem::size_of::<f64>();

        // Memory for previous iteration scores (double buffering)
        let prev_scores_memory = scores_memory;

        // Memory for convergence tracking
        let convergence_memory = scores_memory;

        // Additional overhead for computation (temporary vectors, etc.)
        let computation_overhead = 1024 * 1024; // 1MB for temporary structures

        let total_memory =
            scores_memory + prev_scores_memory + convergence_memory + computation_overhead;
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
            seed: Some(23),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).unwrap())
    }

    #[test]
    fn test_builder_defaults() {
        let facade = PageRankFacade::new(store());
        assert_eq!(facade.iterations, 20);
        assert_eq!(facade.damping_factor, 0.85);
        assert_eq!(facade.tolerance, 1e-4);
    }

    #[test]
    fn test_builder_fluent_chain() {
        let facade = PageRankFacade::new(store())
            .iterations(30)
            .damping_factor(0.90)
            .tolerance(1e-5);

        assert_eq!(facade.iterations, 30);
        assert_eq!(facade.damping_factor, 0.90);
        assert_eq!(facade.tolerance, 1e-5);
    }

    #[test]
    fn test_validate_iterations() {
        let facade = PageRankFacade::new(store()).iterations(0);
        assert!(facade.validate().is_err()); // 0 is invalid

        let facade = PageRankFacade::new(store()).iterations(2_000_000);
        assert!(facade.validate().is_err()); // Too large is invalid

        let facade = PageRankFacade::new(store()).iterations(50);
        assert!(facade.validate().is_ok()); // 50 is valid
    }

    #[test]
    fn test_validate_damping_factor() {
        let facade = PageRankFacade::new(store()).damping_factor(0.0);
        assert!(facade.validate().is_err()); // 0.0 is invalid

        let facade = PageRankFacade::new(store()).damping_factor(1.0);
        assert!(facade.validate().is_err()); // 1.0 is invalid

        let facade = PageRankFacade::new(store()).damping_factor(0.85);
        assert!(facade.validate().is_ok()); // 0.85 is valid
    }

    #[test]
    fn test_validate_tolerance() {
        let facade = PageRankFacade::new(store()).tolerance(0.0);
        assert!(facade.validate().is_err()); // 0.0 is invalid (not positive)

        let facade = PageRankFacade::new(store()).tolerance(1e-4);
        assert!(facade.validate().is_ok()); // positive is valid
    }

    #[test]
    fn test_stream_requires_validation() {
        let facade = PageRankFacade::new(store()).iterations(0); // Invalid
        assert!(facade.stream().is_err());
    }

    #[test]
    fn test_stats_requires_validation() {
        let facade = PageRankFacade::new(store()).damping_factor(0.0); // Invalid
        assert!(facade.stats().is_err());
    }

    #[test]
    fn test_mutate_requires_validation() {
        let facade = PageRankFacade::new(store()).tolerance(0.0); // Invalid
        assert!(facade.mutate("pr").is_err());
    }

    #[test]
    fn test_mutate_validates_property_name() {
        let facade = PageRankFacade::new(store()); // Valid config
        assert!(facade.mutate("").is_err()); // But empty property name
    }

    #[test]
    fn test_mutate_accepts_valid_property() {
        let facade = PageRankFacade::new(store());
        let result = facade.mutate("pagerank");
        assert!(result.is_ok()); // Should succeed with valid property
        let mutation_result = result.unwrap();
        assert_eq!(mutation_result.property_name, "pagerank");
    }

    #[test]
    fn test_stream_returns_node_count_rows() {
        let rows: Vec<_> = PageRankFacade::new(store()).stream().unwrap().collect();
        assert_eq!(rows.len(), 8);
    }

    #[test]
    fn test_stats_shape() {
        let stats = PageRankFacade::new(store()).stats().unwrap();
        assert!(stats.max >= stats.min);
    }

    #[test]
    fn test_cycle_three_nodes_equal_scores() {
        let config = RandomGraphConfig {
            seed: Some(7),
            node_count: 3,
            // 1.0 density gives a symmetric complete digraph in the generator, which
            // should yield equal PageRank scores.
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let scores: Vec<_> = PageRankFacade::new(store)
            .iterations(50)
            .tolerance(1e-9)
            .stream()
            .unwrap()
            .map(|r| r.score)
            .collect();

        assert_eq!(scores.len(), 3);
        assert!((scores[0] - scores[1]).abs() < 1e-8);
        assert!((scores[1] - scores[2]).abs() < 1e-8);
    }
}
