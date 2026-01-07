use crate::algo::msbfs::AggregatedNeighborProcessingMsBfs;
use crate::collections::HugeAtomicDoubleArray;
use crate::concurrency::TerminationFlag;

/// Pure state runtime for harmonic centrality.
///
/// Storage orchestrates graph access, batching, and termination; this runtime only manages
/// accumulation and normalization of inverse farness.
pub struct HarmonicComputationRuntime {
    inverse_farness: HugeAtomicDoubleArray,
    node_count: usize,
}

impl HarmonicComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            inverse_farness: HugeAtomicDoubleArray::new(node_count),
            node_count,
        }
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }

    /// Runs a single MSBFS batch (up to `OMEGA` sources) using the provided neighbor callback.
    pub fn run_batch(
        &self,
        msbfs: &mut AggregatedNeighborProcessingMsBfs,
        source_offset: usize,
        source_len: usize,
        termination: &TerminationFlag,
        get_neighbors: &(impl Fn(usize) -> Vec<usize> + Send + Sync),
    ) {
        if !termination.running() {
            return;
        }

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
                self.inverse_farness.get_and_add(node_id, delta);
            },
        );
    }

    /// Runs a single MSBFS batch (up to `OMEGA` sources) with termination checks.
    pub fn run_batch_with_termination(
        &self,
        msbfs: &mut AggregatedNeighborProcessingMsBfs,
        source_offset: usize,
        source_len: usize,
        termination: &TerminationFlag,
        get_neighbors: &(impl Fn(usize) -> Vec<usize> + Send + Sync),
    ) {
        if !termination.running() {
            return;
        }

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

                let len = sources_mask.count_ones() as f64;
                let delta = len * (1.0 / depth as f64);
                self.inverse_farness.get_and_add(node_id, delta);
            },
        );
    }

    /// Finalizes centralities by normalizing with `(node_count - 1)` when applicable.
    pub fn finalize(&self) -> Vec<f64> {
        let mut out = vec![0.0f64; self.node_count];
        if self.node_count > 1 {
            let norm = (self.node_count - 1) as f64;
            for i in 0..self.node_count {
                out[i] = self.inverse_farness.get(i) / norm;
            }
        }

        out
    }
}

impl Default for HarmonicComputationRuntime {
    fn default() -> Self {
        Self::new(0)
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
    fn run_batch_produces_expected_scores() {
        // 0-1-2 line
        let node_count = 3;
        let adj = undirected_adj(&[(0, 1), (1, 2)], node_count);
        let neighbors = |n: usize| adj[n].clone();
        let termination = TerminationFlag::running_true();

        let computation = HarmonicComputationRuntime::new(node_count);
        let mut msbfs = AggregatedNeighborProcessingMsBfs::new(node_count);

        computation.run_batch(&mut msbfs, 0, node_count, &termination, &neighbors);
        let scores = computation.finalize();

        assert_eq!(scores.len(), node_count);
        assert!((scores[0] - 0.75).abs() < 1e-12);
        assert!((scores[1] - 1.0).abs() < 1e-12);
        assert!((scores[2] - 0.75).abs() < 1e-12);
    }
}
