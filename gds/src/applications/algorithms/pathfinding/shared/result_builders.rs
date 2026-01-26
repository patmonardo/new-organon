//! Pathfinding Result Builders (Java parity)
//!
//! These builders transform raw algorithm results (like Vec<NodeId> traversal order)
//! into procedure-friendly results, matching Java GDS ResultBuilder pattern.

use crate::algo::algorithms::pathfinding::PathResult;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTimings, MutateResultBuilder, StatsResultBuilder, StreamResultBuilder,
};
use crate::core::loading::GraphResources;
use serde_json::Value;

/// Raw algorithm result for traversal algorithms (BFS, DFS)
/// This matches Java's HugeLongArray result from traversal algorithms
pub type TraversalResult = Vec<i64>;

/// Stream result builder for pathfinding algorithms
pub struct PathFindingStreamResultBuilder {
    pub path_requested: bool,
}

impl PathFindingStreamResultBuilder {
    pub fn new(path_requested: bool) -> Self {
        Self { path_requested }
    }
}

impl StreamResultBuilder<TraversalResult, PathResult> for PathFindingStreamResultBuilder {
    type Stream = std::vec::IntoIter<PathResult>;

    fn build(
        &self,
        _graph_resources: &GraphResources,
        result: Option<TraversalResult>,
    ) -> Self::Stream {
        let traversal_order = result.unwrap_or_default();

        // For now, create simple PathResult entries
        // In full implementation, this would reconstruct paths if requested
        let path_results: Vec<PathResult> = traversal_order
            .into_iter()
            .enumerate()
            .map(|(index, node_id)| PathResult {
                source: 0, // TODO: get from config
                target: node_id as u64,
                path: if self.path_requested {
                    vec![node_id as u64]
                } else {
                    vec![]
                },
                cost: index as f64,
            })
            .collect();

        path_results.into_iter()
    }
}

/// Stats result builder for pathfinding algorithms
pub struct PathFindingStatsResultBuilder;

impl PathFindingStatsResultBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl StatsResultBuilder<TraversalResult, Value> for PathFindingStatsResultBuilder {
    fn build(
        &self,
        _graph_resources: &GraphResources,
        result: Option<TraversalResult>,
        timings: AlgorithmProcessingTimings,
    ) -> Value {
        let traversal_order = result.unwrap_or_default();

        serde_json::json!({
            "nodes_visited": traversal_order.len(),
            "execution_time_ms": timings.compute_millis,
            "pre_processing_time_ms": timings.pre_processing_millis,
            "post_processing_time_ms": timings.side_effect_millis,
        })
    }
}

/// Mutate result builder for pathfinding algorithms
pub struct PathFindingMutateResultBuilder;

impl PathFindingMutateResultBuilder {
    pub fn new() -> Self {
        Self
    }
}

impl MutateResultBuilder<Value, TraversalResult, Value, Value> for PathFindingMutateResultBuilder {
    fn build(
        &self,
        _graph_resources: &GraphResources,
        _configuration: &Value,
        result: Option<TraversalResult>,
        timings: AlgorithmProcessingTimings,
        metadata: Option<Value>,
    ) -> Value {
        let traversal_order = result.unwrap_or_default();
        let relationships_updated = metadata
            .as_ref()
            .and_then(|m| m.get("relationships_written"))
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        serde_json::json!({
            "nodes_visited": traversal_order.len(),
            "relationships_updated": relationships_updated,
            "execution_time_ms": timings.compute_millis,
            "pre_processing_time_ms": timings.pre_processing_millis,
            "post_processing_time_ms": timings.side_effect_millis,
        })
    }
}
