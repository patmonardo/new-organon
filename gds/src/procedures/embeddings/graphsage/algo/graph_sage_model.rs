//! Java: `GraphSageModel` (record).

use crate::procedures::embeddings::graphsage::types::GraphSageTrainConfig;

#[derive(Debug, Clone)]
pub struct GraphSageModel {
    pub layers: Vec<crate::procedures::embeddings::graphsage::types::LayerConfig>,
    pub config: GraphSageTrainConfig,
}
