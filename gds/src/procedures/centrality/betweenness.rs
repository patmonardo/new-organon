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

use crate::core::utils::progress::{EmptyTaskRegistryFactory, TaskRegistryFactory};
use crate::mem::MemoryRange;
use crate::algo::betweenness::BetweennessCentralityComputationRuntime;
use crate::procedures::builder_base::{ConfigValidator, WriteResult};
use crate::procedures::traits::{CentralityScore, Result};
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
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

/// Betweenness centrality facade/builder bound to a live graph store.
#[derive(Clone)]
pub struct BetweennessCentralityFacade {
    graph_store: Arc<DefaultGraphStore>,
    direction: String,
    concurrency: usize,
    task_registry: Arc<dyn TaskRegistryFactory>,
}

impl BetweennessCentralityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            direction: "both".to_string(),
            concurrency: 4,
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
        Ok(())
    }

    fn checked_node_id(value: usize) -> Result<NodeId> {
        NodeId::try_from(value as i64).map_err(|_| {
            crate::projection::eval::procedure::AlgorithmError::Execution(format!(
                "node_id must fit into i64 (got {})",
                value
            ))
        })
    }

    fn compute_scores(&self) -> Result<(Vec<f64>, std::time::Duration)> {
        let start = Instant::now();

        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph_view = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, self.orientation())
            .map_err(|e| {
                crate::projection::eval::procedure::AlgorithmError::Graph(e.to_string())
            })?;

        let node_count = graph_view.node_count();
        if node_count == 0 {
            return Ok((Vec::new(), start.elapsed()));
        }

        let fallback = graph_view.default_property_value();
        let get_neighbors = |node_idx: usize| -> Vec<usize> {
            let node_id = match Self::checked_node_id(node_idx) {
                Ok(value) => value,
                Err(_) => return Vec::new(),
            };

            graph_view
                .stream_relationships(node_id, fallback)
                .map(|cursor| cursor.target_id())
                .filter(|target| *target >= 0)
                .map(|target| target as usize)
                .collect()
        };

        let divisor = if self.orientation() == Orientation::Undirected {
            2.0
        } else {
            1.0
        };

        let mut runtime = BetweennessCentralityComputationRuntime::new(node_count);
        let result = runtime.compute(node_count, divisor, &get_neighbors);

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
    pub fn mutate(
        self,
        property_name: &str,
    ) -> Result<crate::procedures::builder_base::MutationResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "BetweennessCentrality mutate/write is not implemented yet".to_string(),
            ),
        )
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

        // Memory for betweenness scores (one f64 per node)
        let scores_memory = node_count * std::mem::size_of::<f64>();

        // Memory for Brandes algorithm structures (stacks, queues, distances)
        // Brandes algorithm uses: predecessor lists, distance arrays, dependency arrays
        let brandes_memory = node_count * 8 * 3; // Rough estimate for algorithm structures

        // Memory for BFS computations per source node
        let bfs_memory = node_count * 4; // Queues and visited arrays

        // Additional overhead for computation (temporary vectors, etc.)
        let computation_overhead = 1024 * 1024; // 1MB for temporary structures

        let total_memory = scores_memory + brandes_memory + bfs_memory + computation_overhead;
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
        assert!(facade.mutate("betweenness").is_err());
    }
}
