//! Java: `GraphSageModel` (record).

use crate::algo::embeddings::graphsage::types::GraphSageTrainConfig;

#[derive(Debug, Clone)]
pub struct GraphSageModel {
    pub layers: Vec<crate::algo::embeddings::graphsage::types::LayerConfig>,
    pub config: GraphSageTrainConfig,
}
