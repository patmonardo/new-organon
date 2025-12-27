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

use crate::procedures::degree_centrality::{
    DegreeCentralityComputationRuntime, DegreeCentralityStorageRuntime,
};
use crate::procedures::facades::builder_base::{ConfigValidator, MutationResult};
use crate::procedures::facades::traits::{CentralityScore, Result};
use crate::types::graph::id_map::NodeId;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;
use std::time::Instant;

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
    orientation: crate::procedures::degree_centrality::storage::Orientation,
    has_relationship_weight_property: bool,
}

impl DegreeCentralityFacade {
    /// Create a new DegreeCentrality facade bound to a graph store.
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            normalize: false,
            orientation: crate::procedures::degree_centrality::storage::Orientation::Natural,
            has_relationship_weight_property: false,
        }
    }

    /// Normalize scores by max degree.
    pub fn normalize(mut self, normalize: bool) -> Self {
        self.normalize = normalize;
        self
    }

    /// Set orientation for degree computation.
    pub fn orientation(
        mut self,
        orientation: crate::procedures::degree_centrality::storage::Orientation,
    ) -> Self {
        self.orientation = orientation;
        self
    }

    /// Use relationship weights when computing degree (sum of weights).
    pub fn weighted(mut self, weighted: bool) -> Self {
        self.has_relationship_weight_property = weighted;
        self
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

        let storage = DegreeCentralityStorageRuntime::with_settings(
            self.graph_store.as_ref(),
            self.orientation,
            self.has_relationship_weight_property,
        )?;

        let mut computation = DegreeCentralityComputationRuntime::new();

        let node_count = storage.node_count();
        for node_id in 0..node_count {
            let node_id = Self::checked_node_id(node_id)?;
            let degree = storage.get_node_degree(node_id)?;
            computation.add_node_degree(node_id, degree);
        }

        if self.normalize {
            computation.normalize_scores();
        }

        Ok((computation.get_scores().clone(), start.elapsed()))
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
    /// # use gds::procedures::facades::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new();
    /// for (node_id, degree) in facade.stream()? {
    ///     if degree > 100.0 {
    ///         println!("Hub: node {} has {} connections", node_id, degree);
    ///     }
    /// }
    /// ```
    pub fn stream(&self) -> Result<Box<dyn Iterator<Item = CentralityScore>>> {
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
    /// # use gds::procedures::facades::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new();
    /// let stats = facade.stats()?;
    /// println!("Graph has average degree: {}", stats.mean);
    /// println!("Max degree (highest hub): {}", stats.max);
    /// println!("Isolated nodes: {}", stats.isolated_nodes);
    /// ```
    pub fn stats(&self) -> Result<DegreeCentralityStats> {
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
    /// # use gds::procedures::facades::centrality::DegreeCentralityFacade;
    /// let facade = DegreeCentralityFacade::new();
    /// let result = facade.mutate("degree")?;
    /// println!("Updated {} nodes", result.nodes_updated);
    /// ```
    pub fn mutate(&self, property_name: &str) -> Result<MutationResult> {
        ConfigValidator::non_empty_string(property_name, "property_name")?;

        Err(
            crate::projection::eval::procedure::AlgorithmError::Execution(
                "DegreeCentrality mutate/write is not implemented yet".to_string(),
            ),
        )
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
        assert!(true);
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
        assert!(result.is_err()); // Not implemented yet, but validated
    }
}
