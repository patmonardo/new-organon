//! Top-K prediction driver for KGE.

use crate::concurrency::TerminationFlag;
use crate::core::utils::queue::BoundedLongLongPriorityQueue;
use crate::types::ValueType;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

use super::error::KgeError;
use super::parameters::{KgeGraph, KgePredictParameters};
use super::result::{KgePredictResult, KgePrediction};
use super::scorer::LinkScorer;

/// Compute top-K link predictions for the given graph and parameters.
///
/// Skips self-links and existing edges. Uses per-source bounded queues and
/// parallelizes across sources.
pub fn compute_kge_predict(
    graph: &dyn KgeGraph,
    parameters: &KgePredictParameters,
    termination_flag: &TerminationFlag,
) -> Result<KgePredictResult, KgeError> {
    if parameters.top_k == 0 {
        return Err(KgeError::InvalidTopK);
    }

    let embeddings = graph
        .node_properties(&parameters.node_embedding_property)
        .ok_or_else(|| KgeError::MissingNodeProperty(parameters.node_embedding_property.clone()))?;

    let dim = parameters.relationship_type_embedding.len();
    if dim == 0 {
        return Err(KgeError::InvalidRelationshipTypeEmbedding);
    }

    let value_type = embeddings.value_type();
    if value_type != ValueType::FloatArray && value_type != ValueType::DoubleArray {
        return Err(KgeError::UnsupportedEmbeddingValueType(
            value_type.name().to_string(),
        ));
    }

    if let Some(embedding_dim) = embeddings.dimension() {
        if embedding_dim != dim {
            return Err(KgeError::EmbeddingDimensionMismatch {
                expected: embedding_dim,
                actual: dim,
            });
        }
    }

    let node_count = graph.node_count() as i64;
    let sources: Vec<i64> = parameters
        .source_nodes
        .clone()
        .unwrap_or_else(|| (0..node_count).collect());
    let targets: Vec<i64> = parameters
        .target_nodes
        .clone()
        .unwrap_or_else(|| (0..node_count).collect());

    let higher_is_better = parameters.scoring_function.higher_is_better();

    // Use per-source bounded queues; parallelize across sources.
    let embeddings = Arc::clone(&embeddings);
    let relationship_type_embedding = parameters.relationship_type_embedding.clone();

    let results: Vec<(i64, Vec<KgePrediction>, u64)> = sources
        .par_iter()
        .map(|&source| {
            termination_flag.assert_running();

            let mut scorer = LinkScorer::new(
                Arc::clone(&embeddings),
                relationship_type_embedding.clone(),
                parameters.scoring_function,
            );

            scorer.init(source);

            let mut queue = if higher_is_better {
                BoundedLongLongPriorityQueue::max(parameters.top_k)
            } else {
                BoundedLongLongPriorityQueue::min(parameters.top_k)
            };

            let mut considered: u64 = 0;

            for &target in &targets {
                if source == target {
                    continue;
                }
                if graph.exists(source, target) {
                    continue;
                }

                considered += 1;

                let score = scorer.compute_score(target);
                if score.is_nan() {
                    continue;
                }

                let _ = queue.offer(source, target, score);
            }

            let mut preds = Vec::with_capacity(queue.size());
            queue.for_each(|s, t, p| {
                preds.push(KgePrediction {
                    source_node_id: s,
                    target_node_id: t,
                    score: p,
                })
            });

            (source, preds, considered)
        })
        .collect();

    let mut by_source = HashMap::new();
    let mut links_considered = 0u64;

    for (source, preds, considered) in results {
        links_considered += considered;
        if !preds.is_empty() {
            by_source.insert(source, preds);
        }
    }

    Ok(KgePredictResult::new(by_source, links_considered))
}
