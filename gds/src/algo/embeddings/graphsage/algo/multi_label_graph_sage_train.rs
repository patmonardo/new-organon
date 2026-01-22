//! Java: `MultiLabelGraphSageTrain`.

use crate::algo::embeddings::graphsage::algo::graph_sage::MODEL_TYPE;
use crate::algo::embeddings::graphsage::algo::graph_sage_model_data::{
    FeatureFunctionData, GraphSageModelData, MatrixData,
};
use crate::algo::embeddings::graphsage::feature_function::FeatureFunction;
use crate::algo::embeddings::graphsage::graphsage_helper;
use crate::algo::embeddings::graphsage::graphsage_model_trainer::{
    GraphSageModelTrainer, GraphSageTrainMetrics,
};
use crate::algo::embeddings::graphsage::layer_factory::generate_weights;
use crate::algo::embeddings::graphsage::multi_label_feature_function::MultiLabelFeatureFunction;
use crate::algo::embeddings::graphsage::train_config_transformer::TrainConfigTransformer;
use crate::algo::embeddings::graphsage::types::{GraphSageTrainConfig, GraphSageTrainParameters};
use crate::concurrency::TerminationFlag;
use crate::core::model::Model;
use crate::ml::core::functions::Weights;
use crate::ml::core::tensor::{Matrix, Tensor};
use crate::types::graph::Graph;
use crate::types::schema::NodeLabel;
use std::collections::HashMap;
use std::sync::Arc;

use super::graph_sage_train::GraphSageTrain;

pub struct MultiLabelGraphSageTrain {
    graph: Arc<dyn Graph>,
    config: GraphSageTrainConfig,
    termination_flag: TerminationFlag,
    gds_version: String,
}

impl MultiLabelGraphSageTrain {
    pub const WEIGHT_BOUND: f64 = 1.0;

    pub fn new(
        graph: Arc<dyn Graph>,
        config: GraphSageTrainConfig,
        termination_flag: TerminationFlag,
        gds_version: String,
    ) -> Self {
        Self {
            graph,
            config,
            termination_flag,
            gds_version,
        }
    }
}

impl GraphSageTrain for MultiLabelGraphSageTrain {
    fn compute(&self) -> Model<GraphSageModelData, GraphSageTrainConfig, GraphSageTrainMetrics> {
        self.termination_flag.assert_running();
        let params: GraphSageTrainParameters = TrainConfigTransformer::to_parameters(&self.config);

        let projected_dim = self
            .config
            .projected_feature_dimension
            .expect("multi-label training requires projected_feature_dimension");

        let multi = graphsage_helper::multi_label_feature_extractors(
            self.graph.as_ref(),
            &params.feature_properties,
        );

        // Java: weightsByLabel uses featureCountPerLabel (includes bias).
        let mut weights_by_label: HashMap<NodeLabel, Arc<Weights>> = HashMap::new();
        for (label, feature_count) in &multi.feature_count_per_label {
            let seed = self.config.random_seed.unwrap_or(42) ^ (label.name().len() as u64);
            let w = Arc::new(generate_weights(
                projected_dim,
                *feature_count,
                Self::WEIGHT_BOUND,
                seed,
            ));
            weights_by_label.insert(label.clone(), w);
        }

        let feature_fn: Arc<dyn FeatureFunction> = Arc::new(MultiLabelFeatureFunction::new(
            weights_by_label.clone(),
            projected_dim,
        ));

        let label_projection_weights: Vec<Arc<Weights>> =
            weights_by_label.values().cloned().collect();

        let features = graphsage_helper::initialize_multi_label_features(
            self.graph.as_ref(),
            &params.feature_properties,
        );

        let trainer = GraphSageModelTrainer::new(
            params,
            Arc::clone(&feature_fn),
            label_projection_weights,
            self.termination_flag.clone(),
        );

        let train_result =
            trainer.train(Arc::clone(&self.graph), Arc::new(features), projected_dim);

        let mut weights_ser: Vec<(String, MatrixData)> = Vec::new();
        for (label, w) in weights_by_label {
            let t = w.snapshot();
            let m = t
                .as_any()
                .downcast_ref::<Matrix>()
                .expect("label projection weights must be Matrix");
            weights_ser.push((
                label.name().to_string(),
                MatrixData {
                    rows: m.rows(),
                    cols: m.cols(),
                    data: m.data().to_vec(),
                },
            ));
        }

        let model_data = GraphSageModelData {
            layers: super::single_label_graph_sage_train::serialize_layers(
                train_result.model_data.layers(),
            ),
            feature_function: FeatureFunctionData::MultiLabel {
                projected_feature_dimension: projected_dim,
                weights_by_label: weights_ser,
            },
        };

        Model::new(
            self.config.model_user.clone(),
            self.config.model_name.clone(),
            MODEL_TYPE.to_string(),
            self.graph.schema().clone(),
            Some(model_data),
            self.config.clone(),
            self.gds_version.clone(),
            train_result.metrics,
        )
    }
}
