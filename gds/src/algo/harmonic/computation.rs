use crate::algo::msbfs::{AggregatedNeighborProcessingMsBfs, OMEGA};
use crate::collections::HugeAtomicDoubleArray;
use crate::concurrency::virtual_threads::{Executor, WorkerContext};
use crate::concurrency::{Concurrency, TerminatedException, TerminationFlag};
use std::sync::Arc;

pub struct HarmonicComputationRuntime;

impl HarmonicComputationRuntime {
    /// Parallel harmonic centrality over all sources.
    ///
    /// Java parity:
    /// - Uses ANP MSBFS batching (up to `OMEGA` sources per batch)
    /// - For each reached node at BFS depth `d>0`, adds `sources_at_node / d`
    /// - Normalizes by `(node_count - 1)` at the end
    pub fn compute_parallel(
        node_count: usize,
        concurrency: usize,
        termination: &TerminationFlag,
        on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
        get_neighbors: &(impl Fn(usize) -> Vec<usize> + Send + Sync),
    ) -> Result<Vec<f64>, TerminatedException> {
        if node_count == 0 {
            return Ok(Vec::new());
        }

        let inverse_farness = HugeAtomicDoubleArray::new(node_count);

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
                msbfs.run(
                    source_offset,
                    source_len,
                    false,
                    |n| (get_neighbors)(n),
                    |node_id, depth, sources_mask| {
                        if depth == 0 {
                            return;
                        }

                        let len = sources_mask.count_ones() as f64;
                        let delta = len * (1.0 / depth as f64);
                        inverse_farness.get_and_add(node_id, delta);
                    },
                );
            });

            (on_sources_done.as_ref())(source_len);
        })?;

        let mut out = vec![0.0f64; node_count];
        if node_count > 1 {
            let norm = (node_count - 1) as f64;
            for i in 0..node_count {
                out[i] = inverse_farness.get(i) / norm;
            }
        }

        Ok(out)
    }
}

impl Default for HarmonicComputationRuntime {
    fn default() -> Self {
        Self
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
    fn compute_parallel_is_deterministic() {
        // 0-1-2 line
        let node_count = 3;
        let adj = undirected_adj(&[(0, 1), (1, 2)], node_count);
        let neighbors = |n: usize| adj[n].clone();
        let termination = TerminationFlag::running_true();
        let noop = Arc::new(|_n: usize| {});

        let one = HarmonicComputationRuntime::compute_parallel(
            node_count,
            1,
            &termination,
            noop.clone(),
            &neighbors,
        )
        .unwrap();

        let four = HarmonicComputationRuntime::compute_parallel(
            node_count,
            4,
            &termination,
            noop,
            &neighbors,
        )
        .unwrap();

        assert_eq!(one.len(), node_count);
        assert_eq!(four.len(), node_count);
        for (a, b) in one.iter().zip(four.iter()) {
            assert!((a - b).abs() < 1e-12);
        }
    }
}
