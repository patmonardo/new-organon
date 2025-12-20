use super::computation::{NodeSimilarityComputationResult, NodeSimilarityComputationRuntime};
use super::storage::NodeSimilarityStorageRuntime;
use crate::define_algorithm_spec;
use super::similarity_metric::NodeSimilarityMetric;
use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::orientation::Orientation;
use crate::projection::RelationshipType;
use serde::{Deserialize, Serialize};

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
        let mut computation = NodeSimilarityComputationRuntime::new();

        // For NodeSimilarity, we usually process all relationships to build vectors,
        // or specific types if configured.
        // Assuming all types for now as per previous simple logic, or need to parse `relationshipTypes` from config?
        // Standard GDS config usually has `relationshipTypes`.
        // Let's assume default view (all types, natural orientation).

        let rel_types: std::collections::HashSet<RelationshipType> = std::collections::HashSet::new();
        let graph_view = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::InvalidGraph(format!("Failed to obtain graph view: {}", e)))?;

        let results = storage.compute(&mut computation, graph_view.as_ref(), &parsed_config);

        // Convert to result type
        let mapped_results: Vec<NodeSimilarityResult> = results.into_iter().map(NodeSimilarityResult::from).collect();

        Ok(NodeSimilarityAlgorithmResult::new(mapped_results)) // TODO: This doesn't match single object return if execute expects aggregation?
        // Wait, `define_algorithm_spec` `execute` usually returns `Result<T, ...>`.
        // If `output_type` is `NodeSimilarityResult`, does it expect a single result (like stats) or a stream?
        // In WCC, it was `WccResult` which contained `components` vec.
        // Here we probably want to support Stream, so we might need an iterator or a wrapper struct.
        // But `Stream` mode in GDS usually implies the algorithm runtime yields items.
        // The `execute` block in `define_algorithm_spec` (as seen in WCC) returned `WccResult`.

        // Let's look at WCC spec again. output_type: WccResult.
        // And WccResult had `components: Vec<u64>`.
        // So for Node Similarity, passing back a huge Vec might be okay for in-memory, but real Stream mode is lazy.
        // However, sticking to the pattern:
        // We can define a result struct that wraps the vec.
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSimilarityAlgorithmResult {
    pub similarities: Vec<NodeSimilarityResult>,
}

// Re-defining for correct macro usage if needed, or adjusting the macro invocation.
// WCC used output_type: WccResult.
// If I use `NodeSimilarityResult` as the row type, `execute` usually needs to return that?
// Actually, `define_algorithm_spec` generates the boilerplate.
// If `execute` returns `Vec<NodeSimilarityResult>`, then `output_type` should be `Vec<NodeSimilarityResult>`?
// Or maybe `NodeSimilarityResult` implies the row?
// Ref WCC: `output_type: WccResult`, execute returns `WccResult`.
// WccResult is ONE object containing the whole result.
// So I should define `NodeSimilarityAlgorithmResult` holding the vector.

impl NodeSimilarityAlgorithmResult {
    pub fn new(similarities: Vec<NodeSimilarityResult>) -> Self {
        Self { similarities }
    }
}

// The `define_algorithm_spec!` macro generates `NODE_SIMILARITYAlgorithmSpec`.
// Provide a stable alias that matches the naming used across the codebase.
pub type NodeSimilarityAlgorithmSpec = NODE_SIMILARITYAlgorithmSpec;

