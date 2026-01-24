use crate::types::graph::graph::Graph;
use crate::types::graph::MappedNodeId;
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::Arc;

use super::{NodeSimilarityComputationResult, NodeSimilarityComputationRuntime};

pub struct NodeSimilarityStorageRuntime {
    concurrency: usize,
}

impl NodeSimilarityStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self { concurrency }
    }

    pub fn compute(
        &self,
        computation: &NodeSimilarityComputationRuntime,
        graph: &dyn Graph,
        config: &super::spec::NodeSimilarityConfig,
    ) -> Vec<NodeSimilarityComputationResult> {
        let node_count = graph.node_count();
        let sources: Vec<u64> = (0..node_count as u64).collect();
        self.compute_with_filters(computation, graph, config, sources, None, true)
    }

    pub fn compute_with_filters(
        &self,
        computation: &NodeSimilarityComputationRuntime,
        graph: &dyn Graph,
        config: &super::spec::NodeSimilarityConfig,
        sources: Vec<u64>,
        target_mask: Option<Vec<bool>>,
        enforce_ordering: bool,
    ) -> Vec<NodeSimilarityComputationResult> {
        let thread_count = self.concurrency.max(1);
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(thread_count)
            .build();

        let run = || {
            self.compute_inner(
                computation,
                graph,
                config,
                sources,
                target_mask.as_deref(),
                enforce_ordering,
            )
        };

        match pool {
            Ok(pool) => pool.install(run),
            Err(_) => run(),
        }
    }

    fn compute_inner(
        &self,
        computation: &NodeSimilarityComputationRuntime,
        graph: &dyn Graph,
        config: &super::spec::NodeSimilarityConfig,
        sources: Vec<u64>,
        target_mask: Option<&[bool]>,
        enforce_ordering: bool,
    ) -> Vec<NodeSimilarityComputationResult> {
        let node_count = graph.node_count();

        let metric = Arc::from(config.similarity_metric.create(config.similarity_cutoff));

        let use_weights = config.weight_property.is_some();
        let (vectors, weights) = build_vectors(graph, node_count, use_weights);
        let weights_ref = weights.as_deref();

        let results: Vec<NodeSimilarityComputationResult> = sources
            .par_iter()
            .flat_map(|&source| {
                let source_vec = &vectors[source as usize];
                if source_vec.is_empty() {
                    return Vec::new();
                }

                let candidates =
                    enumerate_candidates(graph, source, source_vec, target_mask, enforce_ordering);

                computation.score_source_from_candidates(
                    source,
                    candidates,
                    &vectors,
                    weights_ref,
                    &metric,
                    config.top_k,
                )
            })
            .collect();

        computation.select_top_n(results, config.top_n)
    }
}

fn build_vectors(
    graph: &dyn Graph,
    node_count: usize,
    with_weights: bool,
) -> (Vec<Vec<u64>>, Option<Vec<Vec<f64>>>) {
    // Use 1.0 as the fallback weight so missing/unselected properties behave as unweighted.
    let fallback_weight = 1.0;

    if with_weights {
        let pairs: Vec<(Vec<u64>, Vec<f64>)> = (0..node_count)
            .into_par_iter()
            .map(|node| {
                let node_id = node as MappedNodeId;
                let mut pairs: Vec<(u64, f64)> = graph
                    .stream_relationships(node_id, fallback_weight)
                    .map(|cursor| (cursor.target_id() as u64, cursor.property()))
                    .collect();

                pairs.sort_unstable_by_key(|(t, _)| *t);

                let mut neighbors = Vec::with_capacity(pairs.len());
                let mut weights = Vec::with_capacity(pairs.len());
                for (t, w) in pairs {
                    neighbors.push(t);
                    weights.push(w);
                }
                (neighbors, weights)
            })
            .collect();

        let mut vectors = Vec::with_capacity(node_count);
        let mut weights = Vec::with_capacity(node_count);
        for (v, w) in pairs {
            vectors.push(v);
            weights.push(w);
        }

        (vectors, Some(weights))
    } else {
        let vectors: Vec<Vec<u64>> = (0..node_count)
            .into_par_iter()
            .map(|node| {
                let node_id = node as MappedNodeId;
                let mut neighbors: Vec<u64> = graph
                    .stream_relationships(node_id, fallback_weight)
                    .map(|cursor| cursor.target_id() as u64)
                    .collect();
                neighbors.sort_unstable();
                neighbors
            })
            .collect();

        (vectors, None)
    }
}

fn enumerate_candidates(
    graph: &dyn Graph,
    source: u64,
    source_vector: &[u64],
    target_mask: Option<&[bool]>,
    enforce_ordering: bool,
) -> HashSet<u64> {
    // Use a HashSet to dedupe candidates across multiple shared neighbors.
    // Use 1.0 fallback for property access; candidate enumeration is topology-only.
    let fallback_weight = 1.0;
    let mut candidates: HashSet<u64> = HashSet::new();

    // Inverse traversal for each neighbor to find nodes that share that neighbor.
    // (This matches the classic “two-hop” candidate enumeration used by Node Similarity.)
    for &neighbor in source_vector {
        let inverse = graph.stream_inverse_relationships(neighbor as MappedNodeId, fallback_weight);
        for inv_rel in inverse {
            let potential_target = inv_rel.source_id() as u64;
            if potential_target == source {
                continue;
            }

            if enforce_ordering && potential_target <= source {
                continue;
            }

            if let Some(mask) = target_mask {
                let idx = potential_target as usize;
                if idx >= mask.len() || !mask[idx] {
                    continue;
                }
            }

            candidates.insert(potential_target);
        }
    }

    candidates
}
