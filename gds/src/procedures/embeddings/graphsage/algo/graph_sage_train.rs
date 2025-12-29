//! Java: `GraphSageTrain` (abstract Algorithm).

use crate::core::model::Model;
use crate::procedures::embeddings::graphsage::algo::graph_sage_model_data::GraphSageModelData;
use crate::procedures::embeddings::graphsage::graphsage_model_trainer::GraphSageTrainMetrics;
use crate::procedures::embeddings::graphsage::types::GraphSageTrainConfig;

pub trait GraphSageTrain {
    fn compute(&self) -> Model<GraphSageModelData, GraphSageTrainConfig, GraphSageTrainMetrics>;
}
