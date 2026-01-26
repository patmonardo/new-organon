use crate::algo::algorithms::similarity::similarity_stats;
use crate::algo::algorithms::pathfinding::PathResult;
use crate::algo::similarity::node_similarity::NodeSimilarityResult;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteredNodeSimilarityStats {
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
pub struct FilteredNodeSimilarityMutationSummary {
    pub nodes_updated: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Summary of a write operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteredNodeSimilarityWriteSummary {
    pub nodes_written: u64,
    pub property_name: String,
    pub execution_time_ms: u64,
}

/// Mutate result for Filtered Node Similarity: summary + stats + updated store
#[derive(Debug, Clone)]
pub struct FilteredNodeSimilarityMutateResult {
    pub summary: FilteredNodeSimilarityMutationSummary,
    pub stats: FilteredNodeSimilarityStats,
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

/// Filtered node similarity result builder (facade adapter).
pub struct FilteredNodeSimilarityResultBuilder<'a> {
    results: &'a [NodeSimilarityResult],
}

impl<'a> FilteredNodeSimilarityResultBuilder<'a> {
    pub fn new(results: &'a [NodeSimilarityResult]) -> Self {
        Self { results }
    }

    pub fn stats(&self) -> FilteredNodeSimilarityStats {
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

        FilteredNodeSimilarityStats {
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
    ) -> FilteredNodeSimilarityMutationSummary {
        FilteredNodeSimilarityMutationSummary {
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
    ) -> FilteredNodeSimilarityWriteSummary {
        FilteredNodeSimilarityWriteSummary {
            nodes_written,
            property_name: property_name.to_string(),
            execution_time_ms: execution_time.as_millis() as u64,
        }
    }
}
