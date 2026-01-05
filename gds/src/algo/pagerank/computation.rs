//! PageRank computation runtime
//!
//! Implements a standard PageRank power-iteration with:
//! - damping factor $d$
//! - convergence by max absolute delta < tolerance
//! - dangling mass redistribution according to the teleport distribution
//!   (uniform for vanilla PR, uniform-over-sources for personalized PR).

use crate::collections::HugeAtomicDoubleArray;
use crate::concurrency::{install_with_concurrency, Concurrency};
use crate::config::PageRankConfig;
use crate::core::graph_dimensions::GraphDimensions;
use crate::mem::{Estimate, MemoryEstimation, MemoryRange, MemoryTree};
use crate::types::graph::Graph;
use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, serde::Serialize)]
pub struct PageRankComputationResult {
    pub scores: Vec<f64>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PageRankRunResult {
    pub scores: Vec<f64>,
    pub ran_iterations: usize,
    pub did_converge: bool,
}

fn teleport_probability(node_id: usize, node_count: usize, sources: &Option<HashSet<u64>>) -> f64 {
    match sources {
        None => 1.0 / node_count as f64,
        Some(s) if s.is_empty() => 1.0 / node_count as f64,
        Some(s) => {
            if s.contains(&(node_id as u64)) {
                1.0 / s.len() as f64
            } else {
                0.0
            }
        }
    }
}

fn initial_rank(node_id: usize, node_count: usize, sources: &Option<HashSet<u64>>) -> f64 {
    // Match the teleport distribution for the initial vector.
    teleport_probability(node_id, node_count, sources)
}

/// Run PageRank on the provided graph view.
///
/// `source_nodes` enables personalized PageRank (uniform over the given set).
pub fn run_pagerank(
    graph: Arc<dyn Graph>,
    config: PageRankConfig,
    source_nodes: Option<HashSet<u64>>,
) -> PageRankRunResult {
    let node_count_u64 = graph.node_count();
    let node_count = node_count_u64 as usize;

    if node_count == 0 {
        return PageRankRunResult {
            scores: Vec::new(),
            ran_iterations: 0,
            did_converge: true,
        };
    }

    let damping = config.damping_factor;
    let tolerance = config.tolerance;
    let max_iterations = config.max_iterations;

    // Precompute out-degrees.
    let mut out_degree: Vec<usize> = vec![0; node_count];
    for i in 0..node_count {
        out_degree[i] = graph.degree(i as i64);
    }

    // Initialize rank vector.
    let mut rank: Vec<f64> = (0..node_count)
        .map(|i| initial_rank(i, node_count, &source_nodes))
        .collect();

    let concurrency = Concurrency::from_usize(config.base.concurrency);

    let mut did_converge = false;
    let mut ran_iterations = 0;

    for iter in 0..max_iterations {
        ran_iterations = iter + 1;

        // Dangling mass = sum of rank over nodes with 0 out-degree.
        let dangling_mass: f64 = rank
            .iter()
            .enumerate()
            .filter_map(|(i, r)| (out_degree[i] == 0).then_some(*r))
            .sum();

        // next starts with teleport + dangling redistribution.
        // We distribute dangling mass according to teleport distribution, which
        // aligns with typical personalized PageRank handling.
        let next = HugeAtomicDoubleArray::new(node_count);
        for i in 0..node_count {
            let p_i = teleport_probability(i, node_count, &source_nodes);
            let base = (1.0 - damping) * p_i + damping * dangling_mass * p_i;
            next.set(i, base);
        }

        // Accumulate contributions along outgoing edges.
        let fallback = graph.default_property_value();
        install_with_concurrency(concurrency, || {
            (0..node_count).into_par_iter().for_each(|source| {
                let deg = out_degree[source];
                if deg == 0 {
                    return;
                }

                let contrib = damping * rank[source] / deg as f64;
                for cursor in graph.stream_relationships(source as i64, fallback) {
                    let target = cursor.target_id() as usize;
                    next.get_and_add(target, contrib);
                }
            });
        });

        // Materialize + compute max delta.
        let mut next_rank: Vec<f64> = vec![0.0; node_count];
        for i in 0..node_count {
            next_rank[i] = next.get(i);
        }

        let max_delta = next_rank
            .iter()
            .zip(rank.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);

        rank = next_rank;

        if max_delta < tolerance {
            did_converge = true;
            break;
        }
    }

    PageRankRunResult {
        scores: rank,
        ran_iterations,
        did_converge,
    }
}

// -------------------------------------------------------------------------------------------------
// Memory estimation
// -------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PageRankMemoryEstimation;

impl MemoryEstimation for PageRankMemoryEstimation {
    fn description(&self) -> String {
        "PageRank".to_string()
    }

    fn estimate(&self, dimensions: &dyn GraphDimensions, _concurrency: usize) -> MemoryTree {
        let node_count = dimensions.node_count();

        let rank_vec = Estimate::size_of_double_array(node_count);
        let next_vec = Estimate::size_of_double_array(node_count);
        // Approximate out-degree as an int array (usize varies, but this keeps estimates stable).
        let out_degree = Estimate::size_of_int_array(node_count);

        let components = vec![
            MemoryTree::leaf(
                "this.instance".into(),
                MemoryRange::of(Estimate::BYTES_OBJECT_HEADER),
            ),
            MemoryTree::leaf("rank".into(), MemoryRange::of(rank_vec)),
            MemoryTree::leaf("next".into(), MemoryRange::of(next_vec)),
            MemoryTree::leaf("outDegree".into(), MemoryRange::of(out_degree)),
        ];

        let total = components
            .iter()
            .fold(MemoryRange::empty(), |acc, t| acc.add(t.memory_usage()));

        MemoryTree::new("PageRank".into(), total, components)
    }
}

pub fn estimate_pagerank_memory(
    dimensions: &dyn GraphDimensions,
    concurrency: usize,
) -> MemoryTree {
    PageRankMemoryEstimation.estimate(dimensions, concurrency)
}
