//! Java: `GraphSageTrain` (abstract Algorithm).

use crate::algo::embeddings::graphsage::algo::graph_sage_model_data::GraphSageModelData;
use crate::algo::embeddings::graphsage::graphsage_model_trainer::GraphSageTrainMetrics;
use crate::algo::embeddings::graphsage::types::GraphSageTrainConfig;
use crate::core::model::Model;

pub trait GraphSageTrain {
    fn compute(&self) -> Model<GraphSageModelData, GraphSageTrainConfig, GraphSageTrainMetrics>;
}
