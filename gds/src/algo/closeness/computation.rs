use crate::algo::msbfs::{AggregatedNeighborProcessingMsBfs, OMEGA};
use crate::collections::{HugeAtomicDoubleArray, HugeAtomicLongArray};
use crate::concurrency::virtual_threads::{Executor, WorkerContext};
use crate::concurrency::{Concurrency, TerminatedException, TerminationFlag};
use std::sync::Arc;

#[derive(Clone)]
pub struct ClosenessCentralityComputationResult {
    pub centralities: Vec<f64>,
    pub farness: Vec<u64>,
    pub component: Vec<u64>,
}

pub struct ClosenessCentralityComputationRuntime;

impl ClosenessCentralityComputationRuntime {
    /// Parallel closeness centrality over all sources.
    ///
    /// `get_neighbors(node)` must return outgoing neighbors according to the projected graph view.
    pub fn compute_parallel(
        node_count: usize,
        wasserman_faust: bool,
        concurrency: usize,
        termination: &TerminationFlag,
        on_sources_done: Arc<dyn Fn(usize) + Send + Sync>,
        get_neighbors: &(impl Fn(usize) -> Vec<usize> + Send + Sync),
    ) -> Result<ClosenessCentralityComputationResult, TerminatedException> {
        if node_count == 0 {
            return Ok(ClosenessCentralityComputationResult {
                centralities: Vec::new(),
                farness: Vec::new(),
                component: Vec::new(),
            });
        }

        let farness = HugeAtomicLongArray::new(node_count);
        let component = HugeAtomicLongArray::new(node_count);

        let executor = Executor::new(Concurrency::of(concurrency.max(1)));
        let msbfs_state = WorkerContext::new(move || AggregatedNeighborProcessingMsBfs::new(node_count));

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

                        let len = sources_mask.count_ones() as i64;
                        let d = depth as i64;
                        // Saturating-ish: if this ever overflows i64, we're well past realistic sizes.
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

        let scores = HugeAtomicDoubleArray::new(node_count);
        executor.parallel_for(0, node_count, termination, |i| {
            if !termination.running() {
                return;
            }

            let far = farness_out[i];
            if far == 0 {
                scores.set(i, 0.0);
                return;
            }

            let comp = component_out[i] as f64;
            let base = comp / (far as f64);

            let value = if wasserman_faust {
                if node_count <= 1 {
                    0.0
                } else {
                    base * (comp / (node_count as f64 - 1.0))
                }
            } else {
                base
            };

            scores.set(i, value);
        })?;

        let mut out = vec![0.0f64; node_count];
        for i in 0..node_count {
            out[i] = scores.get(i);
        }

        Ok(ClosenessCentralityComputationResult {
            centralities: out,
            farness: farness_out,
            component: component_out,
        })
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
    fn parallel_matches_single_thread() {
        let node_count = 8;
        let adj = undirected_adj(
            &[
                (0, 1),
                (1, 2),
                (2, 3),
                (3, 4),
                (4, 5),
                (5, 6),
                (6, 7),
                (0, 7),
            ],
            node_count,
        );

        let neighbors = |n: usize| adj[n].clone();
        let termination = TerminationFlag::running_true();
        let noop = Arc::new(|_n: usize| {});

        let one = ClosenessCentralityComputationRuntime::compute_parallel(
            node_count,
            false,
            1,
            &termination,
            noop.clone(),
            &neighbors,
        )
        .unwrap();

        let four = ClosenessCentralityComputationRuntime::compute_parallel(
            node_count,
            false,
            4,
            &termination,
            noop,
            &neighbors,
        )
        .unwrap();

        assert_eq!(one.farness, four.farness);
        assert_eq!(one.component, four.component);
        assert_eq!(one.centralities.len(), four.centralities.len());
        for (a, b) in one.centralities.iter().zip(four.centralities.iter()) {
            assert!((a - b).abs() < 1e-12);
        }
    }
}
