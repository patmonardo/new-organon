use super::SimilarityComputer;
use std::sync::Arc;

pub struct CombinedSimilarityComputer {
    similarity_computers: Vec<Arc<dyn SimilarityComputer>>,
    is_symmetric: bool,
}

impl CombinedSimilarityComputer {
    pub fn new(similarity_computers: Vec<Arc<dyn SimilarityComputer>>) -> Self {
        let is_symmetric = similarity_computers.iter().all(|c| c.is_symmetric());
        Self {
            similarity_computers,
            is_symmetric,
        }
    }
}

impl SimilarityComputer for CombinedSimilarityComputer {
    fn similarity(&self, first_node_id: u64, second_node_id: u64) -> f64 {
        if self.similarity_computers.is_empty() {
            return 0.0;
        }

        let mut sum = 0.0;
        for computer in &self.similarity_computers {
            sum += computer.safe_similarity(first_node_id, second_node_id);
        }
        sum / (self.similarity_computers.len() as f64)
    }

    fn is_symmetric(&self) -> bool {
        self.is_symmetric
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ConstantSimilarityComputer {
        value: f64,
        symmetric: bool,
    }

    impl SimilarityComputer for ConstantSimilarityComputer {
        fn similarity(&self, _first_node_id: u64, _second_node_id: u64) -> f64 {
            self.value
        }

        fn is_symmetric(&self) -> bool {
            self.symmetric
        }
    }

    #[test]
    fn averages_similarities() {
        let combined = CombinedSimilarityComputer::new(vec![
            Arc::new(ConstantSimilarityComputer {
                value: 0.25,
                symmetric: true,
            }),
            Arc::new(ConstantSimilarityComputer {
                value: 0.75,
                symmetric: true,
            }),
        ]);

        let similarity = combined.similarity(1, 2);
        assert!((similarity - 0.5).abs() < 1e-12);
    }

    #[test]
    fn safe_similarity_treats_nan_as_zero() {
        let combined = CombinedSimilarityComputer::new(vec![
            Arc::new(ConstantSimilarityComputer {
                value: f64::NAN,
                symmetric: true,
            }),
            Arc::new(ConstantSimilarityComputer {
                value: 1.0,
                symmetric: true,
            }),
        ]);

        // NaN should become 0.0 via safe_similarity.
        let similarity = combined.similarity(1, 2);
        assert!((similarity - 0.5).abs() < 1e-12);
    }

    #[test]
    fn is_symmetric_is_true_only_if_all_are_symmetric() {
        let combined = CombinedSimilarityComputer::new(vec![
            Arc::new(ConstantSimilarityComputer {
                value: 1.0,
                symmetric: true,
            }),
            Arc::new(ConstantSimilarityComputer {
                value: 1.0,
                symmetric: false,
            }),
        ]);

        assert!(!combined.is_symmetric());
    }
}
