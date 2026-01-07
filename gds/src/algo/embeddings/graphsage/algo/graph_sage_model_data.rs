//! Serializable model data for GraphSAGE (Java parity: `ModelData`).
//!
//! Java's `ModelData` stores `Layer[]` and a `FeatureFunction` instance. In Rust we keep the
//! runtime equivalents in `graphsage::model_data::ModelData` (trait objects), but the core
//! model catalog expects `serde::Serialize`d payloads.
//!
//! This type is the "catalog payload" and can be converted into runtime `ModelData`.

use crate::algo::embeddings::graphsage::activation_function_factory::ActivationFunctionFactory;
use crate::algo::embeddings::graphsage::activation_function_wrapper::ActivationFunctionWrapper;
use crate::algo::embeddings::graphsage::layer::Layer;
use crate::algo::embeddings::graphsage::max_pool_aggregating_layer::MaxPoolAggregatingLayer;
use crate::algo::embeddings::graphsage::mean_aggregating_layer::MeanAggregatingLayer;
use crate::algo::embeddings::graphsage::model_data::ModelData;
use crate::algo::embeddings::graphsage::multi_label_feature_function::MultiLabelFeatureFunction;
use crate::algo::embeddings::graphsage::single_label_feature_function::SingleLabelFeatureFunction;
use crate::algo::embeddings::graphsage::types::ActivationFunctionType;
use crate::core::model::ModelData as CatalogModelData;
use crate::ml::core::functions::Weights;
use crate::ml::core::tensor::{Matrix, Vector};
use crate::types::schema::NodeLabel;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixData {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorData {
    pub len: usize,
    pub data: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayerData {
    Mean {
        sample_size: usize,
        activation: ActivationFunctionType,
        weights: MatrixData,
    },
    Pool {
        sample_size: usize,
        activation: ActivationFunctionType,
        pool_weights: MatrixData,
        self_weights: MatrixData,
        neighbors_weights: MatrixData,
        bias: VectorData,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureFunctionData {
    SingleLabel,
    MultiLabel {
        projected_feature_dimension: usize,
        weights_by_label: Vec<(String, MatrixData)>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSageModelData {
    pub layers: Vec<LayerData>,
    pub feature_function: FeatureFunctionData,
}

impl GraphSageModelData {
    pub fn to_runtime(&self) -> ModelData {
        let layers: Vec<Arc<dyn Layer>> = self
            .layers
            .iter()
            .map(|ld| match ld {
                LayerData::Mean {
                    sample_size,
                    activation,
                    weights,
                } => {
                    let activation_wrapper =
                        ActivationFunctionFactory::activation_function_wrapper(*activation);
                    let activation_wrapper: Arc<dyn ActivationFunctionWrapper> =
                        Arc::from(activation_wrapper);
                    let w = Arc::new(Weights::new(Box::new(Matrix::new(
                        weights.data.clone(),
                        weights.rows,
                        weights.cols,
                    ))));
                    Arc::new(MeanAggregatingLayer::new(
                        w,
                        *sample_size,
                        activation_wrapper,
                    )) as Arc<dyn Layer>
                }
                LayerData::Pool {
                    sample_size,
                    activation,
                    pool_weights,
                    self_weights,
                    neighbors_weights,
                    bias,
                } => {
                    let activation_wrapper =
                        ActivationFunctionFactory::activation_function_wrapper(*activation);
                    let activation_wrapper: Arc<dyn ActivationFunctionWrapper> =
                        Arc::from(activation_wrapper);
                    let pw = Arc::new(Weights::new(Box::new(Matrix::new(
                        pool_weights.data.clone(),
                        pool_weights.rows,
                        pool_weights.cols,
                    ))));
                    let sw = Arc::new(Weights::new(Box::new(Matrix::new(
                        self_weights.data.clone(),
                        self_weights.rows,
                        self_weights.cols,
                    ))));
                    let nw = Arc::new(Weights::new(Box::new(Matrix::new(
                        neighbors_weights.data.clone(),
                        neighbors_weights.rows,
                        neighbors_weights.cols,
                    ))));
                    let b = Arc::new(Weights::new(Box::new(Vector::new(bias.data.clone()))));

                    Arc::new(MaxPoolAggregatingLayer::new(
                        *sample_size,
                        pw,
                        sw,
                        nw,
                        b,
                        activation_wrapper,
                    )) as Arc<dyn Layer>
                }
            })
            .collect();

        let feature_function = match &self.feature_function {
            FeatureFunctionData::SingleLabel => Arc::new(SingleLabelFeatureFunction) as Arc<_>,
            FeatureFunctionData::MultiLabel {
                projected_feature_dimension,
                weights_by_label,
            } => {
                let mut map: HashMap<NodeLabel, Arc<Weights>> = HashMap::new();
                for (label, m) in weights_by_label {
                    map.insert(
                        NodeLabel::of(label.as_str()),
                        Arc::new(Weights::new(Box::new(Matrix::new(
                            m.data.clone(),
                            m.rows,
                            m.cols,
                        )))),
                    );
                }
                Arc::new(MultiLabelFeatureFunction::new(
                    map,
                    *projected_feature_dimension,
                )) as Arc<_>
            }
        };

        ModelData::of(layers, feature_function)
    }
}

impl CatalogModelData for GraphSageModelData {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
