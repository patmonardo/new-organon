use super::similarity_metric::NodeSimilarityMetric;
use super::NodeSimilarityStorageRuntime;
use super::{NodeSimilarityComputationResult, NodeSimilarityComputationRuntime};
use crate::algo::algorithms::similarity::similarity_stats;
use crate::algo::algorithms::pathfinding::PathResult;
use crate::config::validation::ConfigError;
use crate::define_algorithm_spec;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::RelationshipType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSimilarityConfig {
    #[serde(default = "default_metric")]
    pub similarity_metric: NodeSimilarityMetric,
    #[serde(default = "default_cutoff")]
    pub similarity_cutoff: f64,
    #[serde(default = "default_top_k")]
    pub top_k: usize,
    #[serde(default = "default_top_n")]
    pub top_n: usize,
    #[serde(default = "default_concurrency")]
    pub concurrency: usize,
    pub weight_property: Option<String>,
}

fn default_metric() -> NodeSimilarityMetric {
    NodeSimilarityMetric::Jaccard
}
fn default_cutoff() -> f64 {
    0.1
}
fn default_top_k() -> usize {
    10
}
fn default_top_n() -> usize {
    0
}
fn default_concurrency() -> usize {
    4
}

impl Default for NodeSimilarityConfig {
    fn default() -> Self {
        Self {
            similarity_metric: default_metric(),
            similarity_cutoff: default_cutoff(),
            top_k: default_top_k(),
            top_n: default_top_n(),
            concurrency: default_concurrency(),
            weight_property: None,
        }
    }
}

impl crate::config::ValidatedConfig for NodeSimilarityConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        crate::config::validate_positive(self.concurrency as f64, "concurrency")?;
        crate::config::validate_range(self.similarity_cutoff, 0.0, 1.0, "similarityCutoff")?;
        crate::config::validate_range(self.top_k as f64, 1.0, 1_000_000.0, "topK")?;
        crate::config::validate_range(self.top_n as f64, 0.0, 1_000_000.0, "topN")?;
        if let Some(prop) = &self.weight_property {
            if prop.trim().is_empty() {
                return Err(ConfigError::InvalidParameter {
                    parameter: "weightProperty".to_string(),
                    reason: "weightProperty cannot be empty".to_string(),
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSimilarityResult {
    pub source: u64,
    pub target: u64,
    pub similarity: f64,
}

impl From<NodeSimilarityComputationResult> for NodeSimilarityResult {
    fn from(r: NodeSimilarityComputationResult) -> Self {
        Self {
            source: r.source,
            target: r.target,
            similarity: r.similarity,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSimilarityStats {
    #[serde(rename = "nodesCompared")]
    pub nodes_compared: u64,
    #[serde(rename = "similarityPairs")]
    pub similarity_pairs: u64,
    #[serde(rename = "similarityDistribution")]
    pub similarity_distribution: HashMap<String, f64>,
    #[serde(rename = "computeMillis")]
    pub compute_millis: u64,
    pub success: bool,
}

/// Summary of a mutate operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSimilarityMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSimilarityWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Node Similarity: summary + stats + updated store
#[derive(Debug, Clone)]
pub struct NodeSimilarityMutateResult {
    pub summary: NodeSimilarityMutationSummary,
    pub stats: NodeSimilarityStats,
    pub updated_store: Arc<crate::types::prelude::DefaultGraphStore>,
}

fn result_paths(results: &[NodeSimilarityResult]) -> Vec<PathResult> {
    results
        .iter()
        .map(|r| PathResult {
            source: r.source,
            target: r.target,
            path: vec![r.source, r.target],
            cost: r.similarity,
        })
        .collect()
}

/// Node similarity result builder (facade adapter).
pub struct NodeSimilarityResultBuilder<'a> {
    results: &'a [NodeSimilarityResult],
}

impl<'a> NodeSimilarityResultBuilder<'a> {
    pub fn new(results: &'a [NodeSimilarityResult]) -> Self {
        Self { results }
    }

    pub fn stats(&self) -> NodeSimilarityStats {
        let mut sources = HashSet::new();
        let tuples: Vec<(u64, u64, f64)> = self
            .results
            .iter()
            .map(|r| {
                sources.insert(r.source);
                (r.source, r.target, r.similarity)
            })
            .collect();

        let stats = similarity_stats(|| tuples.into_iter(), true);

        NodeSimilarityStats {
            nodes_compared: sources.len() as u64,
            similarity_pairs: self.results.len() as u64,
            similarity_distribution: stats.summary(),
            compute_millis: stats.compute_millis,
            success: stats.success,
        }
    }

    pub fn paths(&self) -> Vec<PathResult> {
        result_paths(self.results)
    }

    pub fn mutation_summary(
        &self,
        property_name: &str,
        nodes_updated: u64,
        execution_time: Duration,
    ) -> NodeSimilarityMutationSummary {
        NodeSimilarityMutationSummary {
            nodes_updated,
            property_name: property_name.to_string(),
            execution_time_ms: execution_time.as_millis() as u64,
        }
    }

    pub fn write_summary(
        &self,
        property_name: &str,
        nodes_written: u64,
        execution_time: Duration,
    ) -> NodeSimilarityWriteSummary {
        NodeSimilarityWriteSummary {
            nodes_written,
            property_name: property_name.to_string(),
            execution_time_ms: execution_time.as_millis() as u64,
        }
    }
}

define_algorithm_spec! {
    name: "node_similarity",
    output_type: NodeSimilarityAlgorithmResult,
    projection_hint: Dense, // Review hint
    modes: [Stream, Stats],
    execute: |_self, graph_store, config_input, _context| {
        let parsed_config: NodeSimilarityConfig = serde_json::from_value(config_input.clone())
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to parse config: {}", e)))?;

        // Create runtimes
        let storage = NodeSimilarityStorageRuntime::new(parsed_config.concurrency);
        let computation = NodeSimilarityComputationRuntime::new();

        // For NodeSimilarity, we usually process all relationships to build vectors,
        // or specific types if configured.
        // Assuming all types for now as per previous simple logic, or need to parse `relationshipTypes` from config?
        // Standard GDS config usually has `relationshipTypes`.
        // Let's assume default view (all types, natural orientation).

        // Note: for selector-based graph views, passing an empty relationship set can mean
        // different things across GraphStore implementations. Expand to all types explicitly.
        let rel_types: HashSet<RelationshipType> = graph_store.relationship_types();

        let graph_view = if let Some(prop) = parsed_config.weight_property.as_ref() {
            let selectors: HashMap<RelationshipType, String> = rel_types
                .iter()
                .cloned()
                .map(|t| (t, prop.clone()))
                .collect();
            graph_store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    Orientation::Natural,
                )
                .map_err(|e| {
                    AlgorithmError::InvalidGraph(format!("Failed to obtain graph view: {}", e))
                })?
        } else {
            graph_store
                .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
                .map_err(|e| {
                    AlgorithmError::InvalidGraph(format!("Failed to obtain graph view: {}", e))
                })?
        };

        let results = storage.compute(&computation, graph_view.as_ref(), &parsed_config);

        // Convert to result type
        let mapped_results: Vec<NodeSimilarityResult> = results.into_iter().map(NodeSimilarityResult::from).collect();

        Ok(NodeSimilarityAlgorithmResult::new(mapped_results))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSimilarityAlgorithmResult {
    pub similarities: Vec<NodeSimilarityResult>,
}

impl NodeSimilarityAlgorithmResult {
    pub fn new(similarities: Vec<NodeSimilarityResult>) -> Self {
        Self { similarities }
    }
}

// The `define_algorithm_spec!` macro generates `NODE_SIMILARITYAlgorithmSpec`.
// Provide a stable alias that matches the naming used across the codebase.
pub type NodeSimilarityAlgorithmSpec = NODE_SIMILARITYAlgorithmSpec;
