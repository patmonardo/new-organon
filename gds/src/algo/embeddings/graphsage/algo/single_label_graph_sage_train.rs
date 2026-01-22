//! Java: `SingleLabelGraphSageTrain`.

use crate::algo::embeddings::graphsage::algo::graph_sage::MODEL_TYPE;
use crate::algo::embeddings::graphsage::algo::graph_sage_model_data::{
    FeatureFunctionData, GraphSageModelData, LayerData, MatrixData, VectorData,
};
use crate::algo::embeddings::graphsage::feature_function::FeatureFunction;
use crate::algo::embeddings::graphsage::graphsage_helper;
use crate::algo::embeddings::graphsage::graphsage_model_trainer::{
    GraphSageModelTrainer, GraphSageTrainMetrics,
};
use crate::algo::embeddings::graphsage::layer::Layer;
use crate::algo::embeddings::graphsage::single_label_feature_function::SingleLabelFeatureFunction;
use crate::algo::embeddings::graphsage::train_config_transformer::TrainConfigTransformer;
use crate::algo::embeddings::graphsage::types::{
    AggregatorType, GraphSageTrainConfig, GraphSageTrainParameters,
};
use crate::concurrency::TerminationFlag;
use crate::core::model::Model;
use crate::ml::core::tensor::{Matrix, Vector};
use crate::types::graph::Graph;
use std::sync::Arc;

use super::graph_sage_train::GraphSageTrain;

pub struct SingleLabelGraphSageTrain {
    graph: Arc<dyn Graph>,
    config: GraphSageTrainConfig,
    termination_flag: TerminationFlag,
    gds_version: String,
}

impl SingleLabelGraphSageTrain {
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

impl GraphSageTrain for SingleLabelGraphSageTrain {
    fn compute(&self) -> Model<GraphSageModelData, GraphSageTrainConfig, GraphSageTrainMetrics> {
        self.termination_flag.assert_running();
        let params: GraphSageTrainParameters = TrainConfigTransformer::to_parameters(&self.config);

        let features = graphsage_helper::initialize_single_label_features(
            self.graph.as_ref(),
            &params.feature_properties,
        );

        // Feature dimension = extracted vector length
        let feature_dim = features.get(0).len();

        let feature_function: Arc<dyn FeatureFunction> = Arc::new(SingleLabelFeatureFunction);
        let trainer = GraphSageModelTrainer::new(
            params,
            Arc::clone(&feature_function),
            Vec::new(),
            self.termination_flag.clone(),
        );

        let train_result = trainer.train(Arc::clone(&self.graph), Arc::new(features), feature_dim);

        // Serialize layers from runtime weights (minimal: read out weight tensors).
        let model_data = GraphSageModelData {
            layers: serialize_layers(train_result.model_data.layers()),
            feature_function: FeatureFunctionData::SingleLabel,
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

pub(super) fn serialize_layers(layers: &[Arc<dyn Layer>]) -> Vec<LayerData> {
    use crate::ml::core::tensor::Tensor;
    let mut out = Vec::with_capacity(layers.len());
    for layer in layers {
        let agg = layer.aggregator();
        match agg.typ() {
            AggregatorType::Mean => {
                let w = agg.weights()[0].snapshot();
                let m = w
                    .as_any()
                    .downcast_ref::<Matrix>()
                    .expect("mean weights must be Matrix");
                out.push(LayerData::Mean {
                    sample_size: layer.sample_size(),
                    activation: agg.activation_function_type(),
                    weights: MatrixData {
                        rows: m.rows(),
                        cols: m.cols(),
                        data: m.data().to_vec(),
                    },
                });
            }
            AggregatorType::Pool => {
                let ws = agg.weights();
                let pw = ws[0].snapshot();
                let sw = ws[1].snapshot();
                let nw = ws[2].snapshot();
                let b = ws[3].snapshot();
                let pw = pw.as_any().downcast_ref::<Matrix>().unwrap();
                let sw = sw.as_any().downcast_ref::<Matrix>().unwrap();
                let nw = nw.as_any().downcast_ref::<Matrix>().unwrap();
                let b = b.as_any().downcast_ref::<Vector>().unwrap();
                out.push(LayerData::Pool {
                    sample_size: layer.sample_size(),
                    activation: agg.activation_function_type(),
                    pool_weights: MatrixData {
                        rows: pw.rows(),
                        cols: pw.cols(),
                        data: pw.data().to_vec(),
                    },
                    self_weights: MatrixData {
                        rows: sw.rows(),
                        cols: sw.cols(),
                        data: sw.data().to_vec(),
                    },
                    neighbors_weights: MatrixData {
                        rows: nw.rows(),
                        cols: nw.cols(),
                        data: nw.data().to_vec(),
                    },
                    bias: VectorData {
                        len: b.len(),
                        data: b.data().to_vec(),
                    },
                });
            }
        }
    }
    out
}
