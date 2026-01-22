//! Java: `GraphSage` (inference / embeddings generation).

use crate::algo::embeddings::graphsage::algo::graph_sage_result::GraphSageResult;
use crate::algo::embeddings::graphsage::graphsage_embeddings_generator::GraphSageEmbeddingsGenerator;
use crate::algo::embeddings::graphsage::graphsage_helper;
use crate::algo::embeddings::graphsage::graphsage_model_trainer::GraphSageTrainMetrics;
use crate::algo::embeddings::graphsage::types::GraphSageTrainConfig;
use crate::collections::HugeObjectArray;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::model::Model;
use crate::core::utils::progress::TaskProgressTracker;
use crate::types::graph::Graph;
use std::sync::Arc;

/// Java constant: `GraphSage.MODEL_TYPE`
pub const MODEL_TYPE: &str = "graphSage";

pub struct GraphSage {
    graph: Arc<dyn Graph>,
    model: Arc<
        Model<
            super::graph_sage_model_data::GraphSageModelData,
            GraphSageTrainConfig,
            GraphSageTrainMetrics,
        >,
    >,
    concurrency: Concurrency,
    batch_size: usize,
    progress_tracker: TaskProgressTracker,
    termination_flag: TerminationFlag,
}

impl GraphSage {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: Arc<dyn Graph>,
        model: Arc<
            Model<
                super::graph_sage_model_data::GraphSageModelData,
                GraphSageTrainConfig,
                GraphSageTrainMetrics,
            >,
        >,
        concurrency: Concurrency,
        batch_size: usize,
        progress_tracker: TaskProgressTracker,
        termination_flag: TerminationFlag,
    ) -> Self {
        Self {
            graph,
            model,
            concurrency,
            batch_size,
            progress_tracker,
            termination_flag,
        }
    }

    pub fn compute(&self) -> GraphSageResult {
        let train_config = self.model.train_config();
        let data = self
            .model
            .data()
            .expect("GraphSage model must be loaded (has data)");
        let runtime = data.to_runtime();

        let layers = runtime.layers().to_vec();
        let feature_function = runtime.feature_function();

        let random_seed = train_config.random_seed.unwrap_or(42);
        let embeddings_generator = GraphSageEmbeddingsGenerator::new(
            layers,
            self.batch_size,
            self.concurrency,
            feature_function,
            Some(random_seed),
            self.progress_tracker.clone(),
            self.termination_flag.clone(),
        );

        let features: HugeObjectArray<Vec<f64>> = if train_config.is_multi_label {
            graphsage_helper::initialize_multi_label_features(
                self.graph.as_ref(),
                &train_config.feature_properties,
            )
        } else {
            graphsage_helper::initialize_single_label_features(
                self.graph.as_ref(),
                &train_config.feature_properties,
            )
        };

        let embeddings =
            embeddings_generator.make_embeddings(Arc::clone(&self.graph), Arc::new(features));
        GraphSageResult { embeddings }
    }
}
