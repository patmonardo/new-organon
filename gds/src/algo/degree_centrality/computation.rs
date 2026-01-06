use crate::collections::HugeAtomicDoubleArray;
use crate::concurrency::TerminationFlag;

/// Pure kernel helpers for degree centrality.
///
/// Storage owns orchestration (partitioning, concurrency, termination, progress).
pub struct DegreeCentralityComputationRuntime;

impl DegreeCentralityComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute_range(
        &self,
        start: usize,
        end: usize,
        termination: &TerminationFlag,
        out: &HugeAtomicDoubleArray,
        degree_fn: &(impl Fn(usize) -> f64 + Send + Sync),
    ) {
        for node in start..end {
            if !termination.running() {
                return;
            }
            out.set(node, (degree_fn)(node));
        }
    }

    /// Normalizes scores by dividing by the maximum score.
    ///
    /// If all scores are zero, this is a no-op.
    pub fn normalize_scores(&self, scores: &mut [f64]) {
        let mut max = 0.0f64;
        for &v in scores.iter() {
            if v > max {
                max = v;
            }
        }

        if max > 0.0 {
            for v in scores.iter_mut() {
                *v /= max;
            }
        }
    }
}

impl Default for DegreeCentralityComputationRuntime {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_is_noop_for_all_zero() {
        let mut scores = vec![0.0, 0.0, 0.0];
        DegreeCentralityComputationRuntime::new().normalize_scores(&mut scores);
        assert_eq!(scores, vec![0.0, 0.0, 0.0]);
    }

    #[test]
    fn normalize_divides_by_max() {
        let mut scores = vec![1.0, 2.0, 4.0];
        DegreeCentralityComputationRuntime::new().normalize_scores(&mut scores);
        assert!((scores[0] - 0.25).abs() < 1e-12);
        assert!((scores[1] - 0.5).abs() < 1e-12);
        assert!((scores[2] - 1.0).abs() < 1e-12);
    }
}
