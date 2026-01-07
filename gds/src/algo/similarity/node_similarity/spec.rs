use super::computation::{NodeSimilarityComputationResult, NodeSimilarityComputationRuntime};
use super::similarity_metric::NodeSimilarityMetric;
use super::storage::NodeSimilarityStorageRuntime;
use crate::define_algorithm_spec;
use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

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
