//! Closeness Centrality compute kernels
//!
//! Java parity reference:
//! - `org.neo4j.gds.closeness.ClosenessCentrality`
//!
//! This module intentionally contains no GraphStore/projection logic.
//! The storage runtime owns orchestration and provides neighbor access.

use crate::algo::msbfs::{AggregatedNeighborProcessingMsBfs, OMEGA};
use crate::collections::{HugeAtomicDoubleArray, HugeAtomicLongArray};
use crate::concurrency::virtual_threads::{Executor, WorkerContext};
use crate::concurrency::{Concurrency, TerminatedException, TerminationFlag};
use std::sync::Arc;

/// Computation runtime for closeness centrality (state-only, no graph access)
pub struct ClosenessCentralityComputationRuntime;

impl ClosenessCentralityComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Phase 1: compute per-node farness and component size using ANP MSBFS batching.
    pub fn compute_farness_parallel(
        &self,
        node_count: usize,
        concurrency: usize,
        termination: &TerminationFlag,
        on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
        get_neighbors: &(impl Fn(usize) -> Vec<usize> + Send + Sync),
    ) -> Result<(Vec<u64>, Vec<u64>), TerminatedException> {
        if node_count == 0 {
            return Ok((Vec::new(), Vec::new()));
        }

        let farness = HugeAtomicLongArray::new(node_count);
        let component = HugeAtomicLongArray::new(node_count);

        let executor = Executor::new(Concurrency::of(concurrency.max(1)));
        let msbfs_state =
            WorkerContext::new(move || AggregatedNeighborProcessingMsBfs::new(node_count));

        let batch_count = (node_count + OMEGA - 1) / OMEGA;
        executor.parallel_for(0, batch_count, termination, |batch_idx| {
            if !termination.running() {
                return;
            }

            let source_offset = batch_idx * OMEGA;
            let source_len = (source_offset + OMEGA).min(node_count) - source_offset;

            msbfs_state.with(|msbfs| {
                msbfs.run_with_termination(
                    source_offset,
                    source_len,
                    false,
                    Some(termination),
                    |n| (get_neighbors)(n),
                    |node_id, depth, sources_mask| {
                        if depth == 0 {
                            return;
                        }

                        let len = sources_mask.count_ones() as i64;
                        let d = depth as i64;
                        let far_delta = len.saturating_mul(d);

                        farness.get_and_add(node_id, far_delta);
                        component.get_and_add(node_id, len);
                    },
                );
            });

            (on_sources_done.as_ref())(source_len);
        })?;

        let mut farness_out = vec![0u64; node_count];
        let mut component_out = vec![0u64; node_count];
        for i in 0..node_count {
            let far = farness.get(i);
            let comp = component.get(i);
            farness_out[i] = far.max(0) as u64;
            component_out[i] = comp.max(0) as u64;
        }

        Ok((farness_out, component_out))
    }

    /// Phase 2: compute closeness scores from farness + component.
    pub fn compute_closeness_parallel(
        &self,
        node_count: usize,
        wasserman_faust: bool,
        concurrency: usize,
        termination: &TerminationFlag,
        farness: &[u64],
        component: &[u64],
        on_nodes_done: Arc<dyn Fn(usize) + Send + Sync>,
    ) -> Result<Vec<f64>, TerminatedException> {
        if node_count == 0 {
            return Ok(Vec::new());
        }

        let executor = Executor::new(Concurrency::of(concurrency.max(1)));
        let scores = HugeAtomicDoubleArray::new(node_count);

        executor.parallel_for(0, node_count, termination, |i| {
            if !termination.running() {
                return;
            }

            let far = farness[i];
            if far == 0 {
                scores.set(i, 0.0);
                return;
            }

            let comp = component[i] as f64;
            let base = comp / (far as f64);

            let value = if wasserman_faust {
                if node_count <= 1 {
                    0.0
                } else {
                    // Java: (comp/far) * (comp/(nodeCount-1))
                    base * (comp / (node_count as f64 - 1.0))
                }
            } else {
                base
            };

            scores.set(i, value);
        })?;

        // Report progress as one chunk for the whole phase.
        (on_nodes_done.as_ref())(node_count);

        let mut out = vec![0.0f64; node_count];
        for i in 0..node_count {
            out[i] = scores.get(i);
        }

        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn undirected_adj(edges: &[(usize, usize)], node_count: usize) -> Vec<Vec<usize>> {
        let mut adj = vec![Vec::<usize>::new(); node_count];
        for &(a, b) in edges {
            adj[a].push(b);
            if a != b {
                adj[b].push(a);
            }
        }
        for v in adj.iter_mut() {
            v.sort_unstable();
            v.dedup();
        }
        adj
    }

    #[test]
    fn farness_and_scores_are_deterministic() {
        let node_count = 6;
        let adj = undirected_adj(&[(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)], node_count);
        let neighbors = |n: usize| adj[n].clone();

        let termination = TerminationFlag::running_true();
        let noop_sources = Arc::new(|_n: usize| {});

        let runtime = ClosenessCentralityComputationRuntime::new();

        let (f1, c1) = runtime
            .compute_farness_parallel(
                node_count,
                2,
                &termination,
                noop_sources.clone(),
                &neighbors,
            )
            .unwrap();

        let (f2, c2) = runtime
            .compute_farness_parallel(node_count, 4, &termination, noop_sources, &neighbors)
            .unwrap();

        assert_eq!(f1, f2);
        assert_eq!(c1, c2);

        let noop_nodes = Arc::new(|_n: usize| {});
        let s1 = runtime
            .compute_closeness_parallel(
                node_count,
                false,
                2,
                &termination,
                &f1,
                &c1,
                noop_nodes.clone(),
            )
            .unwrap();

        let s2 = runtime
            .compute_closeness_parallel(node_count, true, 2, &termination, &f1, &c1, noop_nodes)
            .unwrap();

        // Wasserman-Faust should not exceed base when comp <= node_count.
        for (base, wf) in s1.iter().zip(s2.iter()) {
            assert!(*wf <= *base + 1e-12 || node_count <= 1);
        }
    }
}
