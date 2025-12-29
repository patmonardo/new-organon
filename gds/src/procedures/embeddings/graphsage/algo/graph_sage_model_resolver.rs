//! Java: `GraphSageModelResolver`.

use crate::core::model::{Model, ModelCatalog};
use crate::procedures::embeddings::graphsage::algo::graph_sage_model_data::GraphSageModelData;
use crate::procedures::embeddings::graphsage::graphsage_model_trainer::GraphSageTrainMetrics;
use crate::procedures::embeddings::graphsage::types::GraphSageTrainConfig;
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
