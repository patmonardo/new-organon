//! PageRank Pregel runner (Java GDS aligned)
//!
//! This module provides an end-to-end PageRank execution helper using the in-crate
//! Pregel executor. The computation follows Neo4j GDS' delta-based PageRank:
//!
//! - init rank to $\alpha = 1 - d$ (or 0 for non-source nodes)
//! - in each superstep, compute $\delta = d * \sum messages$
//! - update rank as `rank + delta` (not `alpha + delta`)
//! - send `delta / degree` to neighbors when `delta > tolerance`
//!
//! This matches `org.neo4j.gds.pagerank.PageRankComputation`.

use crate::config::PageRankConfig;
use crate::pregel::{
    ComputeContext, ComputeFn, InitContext, InitFn, Messages, PregelBuilder, PregelSchema,
    SyncQueueMessageIterator, SyncQueueMessenger, Visibility,
};
use crate::types::graph::Graph;
use crate::types::ValueType;
use std::collections::HashSet;
use std::sync::Arc;

pub const PAGE_RANK: &str = "pagerank";

#[derive(Debug, Clone)]
pub struct PageRankRunResult {
    pub scores: Vec<f64>,
    pub ran_iterations: usize,
    pub did_converge: bool,
}

/// Run PageRank on a given graph view using the in-crate Pregel runtime.
///
/// - `source_nodes`: if `None` or empty => all nodes are treated as sources.
pub fn run_pagerank(
    graph: Arc<dyn Graph>,
    config: PageRankConfig,
    source_nodes: Option<HashSet<u64>>,
) -> PageRankRunResult {
    let alpha = 1.0 - config.damping_factor;
    let tolerance = config.tolerance;
    let damping_factor = config.damping_factor;

    let source_nodes = match source_nodes {
        None => None,
        Some(set) if set.is_empty() => None,
        Some(set) => Some(Arc::new(set)),
    };

    let schema: PregelSchema = PregelSchema::builder()
        .add(PAGE_RANK, ValueType::Double, Visibility::Public)
        .build();

    let init_fn: InitFn<PageRankConfig> = Arc::new({
        let source_nodes = source_nodes.clone();
        move |context: &mut InitContext<PageRankConfig>| {
            let initial_value = match &source_nodes {
                None => alpha,
                Some(sources) => {
                    if sources.contains(&context.node_id()) {
                        alpha
                    } else {
                        0.0
                    }
                }
            };
            context.set_node_value(PAGE_RANK, initial_value);
        }
    });

    let compute_fn: ComputeFn<PageRankConfig, SyncQueueMessageIterator> = Arc::new(
        move |context: &mut ComputeContext<PageRankConfig, SyncQueueMessageIterator>,
              messages: &mut Messages<SyncQueueMessageIterator>| {
            let rank = context.double_node_value(PAGE_RANK);
            let mut delta = rank;

            if !context.is_initial_superstep() {
                let mut sum = 0.0;
                for message in messages {
                    sum += message;
                }
                delta = damping_factor * sum;
                context.set_node_value(PAGE_RANK, rank + delta);
            }

            if delta > tolerance || context.is_initial_superstep() {
                let degree = context.degree() as f64;
                if degree > 0.0 {
                    context.send_to_neighbors(delta / degree);
                }
            } else {
                context.vote_to_halt();
            }
        },
    );

    let messenger = Arc::new(SyncQueueMessenger::new(graph.node_count()));

    let pregel = PregelBuilder::<PageRankConfig, SyncQueueMessageIterator>::new()
        .graph(Arc::clone(&graph))
        .config(config)
        .schema(schema)
        .init_fn(init_fn)
        .compute_fn(compute_fn)
        .messenger(messenger)
        .build();

    let result = pregel.run();

    let mut scores = Vec::with_capacity(graph.node_count());
    for node_id in 0..graph.node_count() {
        scores.push(result.node_values.double_value(PAGE_RANK, node_id));
    }

    PageRankRunResult {
        scores,
        ran_iterations: result.ran_iterations,
        did_converge: result.did_converge,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    #[test]
    fn test_runner_smoke() {
        let config = RandomGraphConfig {
            seed: Some(7),
            node_count: 3,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        let graph_store = crate::types::prelude::DefaultGraphStore::random(&config).unwrap();
        let graph: Arc<dyn Graph> = graph_store.graph();

        let config = PageRankConfig::builder()
            .max_iterations(20)
            .damping_factor(0.85)
            .tolerance(1e-9)
            .build()
            .unwrap();

        let result = run_pagerank(graph, config, None);
        assert_eq!(result.scores.len(), 3);
        // Symmetric cycle => equal scores
        assert!((result.scores[0] - result.scores[1]).abs() < 1e-8);
        assert!((result.scores[1] - result.scores[2]).abs() < 1e-8);
    }
}
