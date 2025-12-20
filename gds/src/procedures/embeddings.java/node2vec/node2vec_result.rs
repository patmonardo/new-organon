use crate::collections::HugeObjectArray;

/// Result of Node2Vec training.
///
/// Java: `record Node2VecResult(HugeObjectArray<FloatVector> embeddings, List<Double> lossPerIteration)`
pub struct Node2VecResult {
    pub embeddings: HugeObjectArray<Vec<f32>>,
    pub loss_per_iteration: Vec<f64>,
}


