//! Java: `SingleLabelFeatureFunction`.

use crate::collections::HugeObjectArray;
use crate::ml::core::functions::LazyConstant;
use crate::ml::core::tensor::Matrix;
use crate::ml::core::variable::VariableRef;
use crate::types::graph::Graph;
use std::sync::Arc;

use super::feature_function::FeatureFunction;

pub struct SingleLabelFeatureFunction;

impl FeatureFunction for SingleLabelFeatureFunction {
    fn apply(
        &self,
        _graph: Arc<dyn Graph>,
        node_ids: &[u64],
        features: Arc<HugeObjectArray<Vec<f64>>>,
    ) -> VariableRef {
        let feature_dimension = features.get(0).len();
        let dims = vec![node_ids.len(), feature_dimension];

        let node_ids = node_ids.to_vec();
        let features = Arc::clone(&features);

        Arc::new(LazyConstant::new(
            move || {
                let mut batch_features = Matrix::create(0.0, node_ids.len(), feature_dimension);
                for (batch_idx, &node_id) in node_ids.iter().enumerate() {
                    let row = features.get(node_id as usize);
                    batch_features.set_row(batch_idx, row);
                }
                Box::new(batch_features)
            },
            dims,
        ))
    }
}
