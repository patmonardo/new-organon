use super::metrics::SimilarityComputer;
use rayon::prelude::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct KnnComputationResult {
    pub source: u64,
    pub target: u64,
    pub similarity: f64,
}

#[derive(Default)]
pub struct KnnComputationRuntime;

impl KnnComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn compute(
        &self,
        node_count: usize,
        k: usize,
        similarity_cutoff: f64,
        similarity: Arc<dyn SimilarityComputer>,
    ) -> Vec<KnnComputationResult> {
        if node_count == 0 || k == 0 {
            return Vec::new();
        }

        let cutoff = similarity_cutoff.max(0.0);

        (0..node_count)
            .into_par_iter()
            .flat_map_iter(|i| {
                let source = i as u64;
                let mut rows: Vec<KnnComputationResult> = Vec::with_capacity(k.min(node_count.saturating_sub(1)));

                for j in 0..node_count {
                    if i == j {
                        continue;
                    }
                    let target = j as u64;
                    let s = similarity.safe_similarity(source, target);
                    if s >= cutoff {
                        rows.push(KnnComputationResult {
                            source,
                            target,
                            similarity: s,
                        });
                    }
                }

                rows.sort_by(|a, b| b.similarity.partial_cmp(&a.similarity).unwrap_or(std::cmp::Ordering::Equal));
                rows.truncate(k);
                rows
            })
            .collect()
    }
}
