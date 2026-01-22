//! Java: `GraphSageModel` (record).

use crate::algo::embeddings::graphsage::types::{GraphSageTrainConfig, LayerConfig};

#[derive(Debug, Clone)]
pub struct GraphSageModel {
    pub layers: Vec<LayerConfig>,
    pub config: GraphSageTrainConfig,
}
