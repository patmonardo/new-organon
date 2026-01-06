//! PageRank computation runtime
//!
//! Implements a standard PageRank power-iteration with:
//! - damping factor $d$
//! - convergence by max absolute delta < tolerance
//! - dangling mass redistribution according to the teleport distribution
//!   (uniform for vanilla PR, uniform-over-sources for personalized PR).

use crate::collections::HugeAtomicDoubleArray;
use crate::concurrency::{install_with_concurrency, Concurrency};
use crate::core::graph_dimensions::GraphDimensions;
use crate::mem::{Estimate, MemoryEstimation, MemoryRange, MemoryTree};
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, serde::Serialize)]
pub struct PageRankRunResult {
    pub scores: Vec<f64>,
    pub ran_iterations: usize,
    pub did_converge: bool,
}

/// Pure PageRank state/runtime.
///
/// Holds parameters and executes the power-iteration given graph callbacks.
#[derive(Clone, Debug)]
pub struct PageRankComputationRuntime {
    max_iterations: usize,
    damping_factor: f64,
    tolerance: f64,
    source_nodes: Option<HashSet<u64>>,
}

impl PageRankComputationRuntime {
    pub fn new(
        max_iterations: usize,
        damping_factor: f64,
        tolerance: f64,
        source_nodes: Option<HashSet<u64>>,
    ) -> Self {
        Self {
            max_iterations,
            damping_factor,
            tolerance,
            source_nodes,
        }
    }

    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn teleport_probability(&self, node_id: usize, node_count: usize) -> f64 {
        match &self.source_nodes {
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

    fn initial_rank(&self, node_count: usize) -> Vec<f64> {
        (0..node_count)
            .map(|i| self.teleport_probability(i, node_count))
            .collect()
    }

    pub fn run(
        &self,
        node_count: usize,
        out_degree: &[usize],
        concurrency: usize,
        stream_neighbors: &(impl Fn(usize, &mut dyn FnMut(usize)) + Sync),
    ) -> PageRankRunResult {
        if node_count == 0 {
            return PageRankRunResult {
                scores: Vec::new(),
                ran_iterations: 0,
                did_converge: true,
            };
        }

        let damping = self.damping_factor;
        let tolerance = self.tolerance;
        let max_iterations = self.max_iterations;

        let mut rank: Vec<f64> = self.initial_rank(node_count);

        let concurrency = Concurrency::from_usize(concurrency.max(1));

        let mut did_converge = false;
        let mut ran_iterations = 0;

        for iter in 0..max_iterations {
            ran_iterations = iter + 1;

            let dangling_mass: f64 = rank
                .iter()
                .enumerate()
                .filter_map(|(i, r)| (out_degree[i] == 0).then_some(*r))
                .sum();

            let next = HugeAtomicDoubleArray::new(node_count);
            for i in 0..node_count {
                let p_i = self.teleport_probability(i, node_count);
                let base = (1.0 - damping) * p_i + damping * dangling_mass * p_i;
                next.set(i, base);
            }

            install_with_concurrency(concurrency, || {
                (0..node_count).into_par_iter().for_each(|source| {
                    let deg = out_degree[source];
                    if deg == 0 {
                        return;
                    }

                    let contrib = damping * rank[source] / deg as f64;
                    let mut push = |target: usize| {
                        next.get_and_add(target, contrib);
                    };
                    stream_neighbors(source, &mut push);
                });
            });

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
