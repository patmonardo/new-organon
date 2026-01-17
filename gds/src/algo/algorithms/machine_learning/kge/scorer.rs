//! Low-level scoring implementation for KGE prediction.

use crate::types::properties::node::NodePropertyValues;
use crate::types::ValueType;
use std::sync::Arc;

use super::ScoreFunction;

/// Per-source scorer that caches the source/relationship composite.
pub(crate) struct LinkScorer {
    embeddings: Arc<dyn NodePropertyValues>,
    relationship_type_embedding: Vec<f64>,
    score_function: ScoreFunction,
    current_candidate_target_f64: Vec<f64>,
    current_candidate_target_f32: Vec<f32>,
    use_f32: bool,
}

impl LinkScorer {
    /// Create a scorer for the given embeddings and score function.
    pub(crate) fn new(
        embeddings: Arc<dyn NodePropertyValues>,
        relationship_type_embedding: Vec<f64>,
        score_function: ScoreFunction,
    ) -> Self {
        let use_f32 = embeddings.value_type() == ValueType::FloatArray;
        let dim = relationship_type_embedding.len();

        Self {
            embeddings,
            relationship_type_embedding,
            score_function,
            current_candidate_target_f64: vec![0.0; dim],
            current_candidate_target_f32: vec![0.0; dim],
            use_f32,
        }
    }

    /// Prepare cached state for a source node.
    pub(crate) fn init(&mut self, source_node: i64) {
        let dim = self.relationship_type_embedding.len();

        if self.use_f32 {
            let current_source = self
                .embeddings
                .float_array_value(source_node as u64)
                .unwrap_or_else(|_| vec![0.0; dim]);

            match self.score_function {
                ScoreFunction::Transe => {
                    for i in 0..dim {
                        self.current_candidate_target_f32[i] =
                            current_source[i] + self.relationship_type_embedding[i] as f32;
                    }
                }
                ScoreFunction::Distmult => {
                    for i in 0..dim {
                        self.current_candidate_target_f32[i] =
                            current_source[i] * self.relationship_type_embedding[i] as f32;
                    }
                }
            }
        } else {
            let current_source = self
                .embeddings
                .double_array_value(source_node as u64)
                .unwrap_or_else(|_| vec![0.0; dim]);

            match self.score_function {
                ScoreFunction::Transe => {
                    for i in 0..dim {
                        self.current_candidate_target_f64[i] =
                            current_source[i] + self.relationship_type_embedding[i];
                    }
                }
                ScoreFunction::Distmult => {
                    for i in 0..dim {
                        self.current_candidate_target_f64[i] =
                            current_source[i] * self.relationship_type_embedding[i];
                    }
                }
            }
        }
    }

    /// Compute the score for a target node.
    pub(crate) fn compute_score(&self, target_node: i64) -> f64 {
        let dim = self.relationship_type_embedding.len();

        if self.use_f32 {
            let target_vec = self
                .embeddings
                .float_array_value(target_node as u64)
                .unwrap_or_else(|_| vec![0.0; dim]);

            match self.score_function {
                ScoreFunction::Distmult => {
                    let mut res = 0.0f64;
                    for i in 0..dim {
                        res += (self.current_candidate_target_f32[i] * target_vec[i]) as f64;
                    }
                    res
                }
                ScoreFunction::Transe => {
                    let mut res = 0.0f64;
                    for i in 0..dim {
                        let elem = (self.current_candidate_target_f32[i] - target_vec[i]) as f64;
                        res += elem * elem;
                    }
                    res.sqrt()
                }
            }
        } else {
            let target_vec = self
                .embeddings
                .double_array_value(target_node as u64)
                .unwrap_or_else(|_| vec![0.0; dim]);

            match self.score_function {
                ScoreFunction::Distmult => {
                    let mut res = 0.0f64;
                    for i in 0..dim {
                        res += self.current_candidate_target_f64[i] * target_vec[i];
                    }
                    res
                }
                ScoreFunction::Transe => {
                    let mut res = 0.0f64;
                    for i in 0..dim {
                        let elem = self.current_candidate_target_f64[i] - target_vec[i];
                        res += elem * elem;
                    }
                    res.sqrt()
                }
            }
        }
    }
}
