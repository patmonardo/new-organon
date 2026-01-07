//! Java: `GraphSageModelResolver`.

use crate::algo::embeddings::graphsage::algo::graph_sage_model_data::GraphSageModelData;
use crate::algo::embeddings::graphsage::graphsage_model_trainer::GraphSageTrainMetrics;
use crate::algo::embeddings::graphsage::types::GraphSageTrainConfig;
use crate::core::model::{Model, ModelCatalog};
use anyhow::Result;
use std::sync::Arc;

pub struct GraphSageModelResolver;

impl GraphSageModelResolver {
    pub fn resolve_model(
        model_catalog: &impl ModelCatalog,
        username: &str,
        model_name: &str,
    ) -> Result<Arc<Model<GraphSageModelData, GraphSageTrainConfig, GraphSageTrainMetrics>>> {
        model_catalog.get::<GraphSageModelData, GraphSageTrainConfig, GraphSageTrainMetrics>(
            username, model_name,
        )
    }
}
