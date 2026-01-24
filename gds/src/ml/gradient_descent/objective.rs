use crate::ml::core::batch::Batch;
use crate::ml::core::functions::Constant;
use crate::ml::core::functions::Weights;
use crate::ml::core::tensor::Matrix;
use crate::ml::core::variable::VariableRef;
use crate::ml::models::Features;
use std::sync::Arc;

/// A training objective that computes a loss over a batch of nodes
pub trait Objective {
    type ModelData;

    /// Returns the Weights variables used in the computation graph.
    /// These are the actual Variable objects that can be used with ctx.gradient().
    fn weights(&self) -> Vec<Arc<Weights>>;

    /// Computes the loss for a batch
    fn loss<B: Batch>(&self, batch: &B, train_size: usize) -> VariableRef;

    /// Returns the model data needed for storage/loading
    fn model_data(&self) -> &Self::ModelData;
}

pub fn batch_feature_matrix<B: Batch>(batch: &B, features: &dyn Features) -> Constant {
    let rows = batch.size();
    let cols = features.feature_dimension();
    let mut batch_features = Matrix::zeros(rows, cols);
    let batch_iterator = batch.element_ids();

    for (current_row, element_id) in batch_iterator.enumerate() {
        let feature_vec = features.get(element_id as usize);
        for col in 0..cols {
            batch_features[(current_row, col)] = feature_vec[col];
        }
    }

    Constant::new(Box::new(batch_features))
}
