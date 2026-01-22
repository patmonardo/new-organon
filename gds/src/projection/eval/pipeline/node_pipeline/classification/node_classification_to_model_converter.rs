use super::node_classification_model_result::NodeClassificationModelResult;
use super::node_classification_pipeline_model_info::NodeClassificationPipelineModelInfo;
use super::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
use super::node_classification_train_result::NodeClassificationTrainResult;
use super::node_classification_training_pipeline::NodeClassificationTrainingPipeline;
use crate::projection::eval::pipeline::node_pipeline::NodePropertyPredictPipeline;
use crate::projection::eval::pipeline::ResultToModelConverter;
use crate::types::schema::GraphSchema;

/// Converter from training result to catalog model.
///
/// Transforms a NodeClassificationTrainResult into a NodeClassificationModelResult
/// suitable for storage in the model catalog.
pub struct NodeClassificationToModelConverter {
    pipeline: NodeClassificationTrainingPipeline,
    config: NodeClassificationPipelineTrainConfig,
}

impl NodeClassificationToModelConverter {
    pub fn new(
        pipeline: NodeClassificationTrainingPipeline,
        config: NodeClassificationPipelineTrainConfig,
    ) -> Self {
        Self { pipeline, config }
    }

    pub fn pipeline(&self) -> &NodeClassificationTrainingPipeline {
        &self.pipeline
    }

    pub fn config(&self) -> &NodeClassificationPipelineTrainConfig {
        &self.config
    }

    /// Convert training result to model result.
    pub fn to_model(
        &self,
        result: NodeClassificationTrainResult,
        _original_schema: &GraphSchema,
    ) -> NodeClassificationModelResult {
        let training_statistics = result.training_statistics().clone();
        let model_info = NodeClassificationPipelineModelInfo::of(
            training_statistics.winning_model_test_metrics(),
            training_statistics.winning_model_outer_train_metrics(),
            training_statistics.best_candidate(),
            NodePropertyPredictPipeline::from_pipeline(&self.pipeline),
            result
                .class_id_map()
                .original_ids_list()
                .iter()
                .map(|id| *id as i64)
                .collect(),
        );

        NodeClassificationModelResult::new(
            result.into_classifier(),
            self.config.clone(),
            model_info,
            training_statistics,
        )
    }
}

impl ResultToModelConverter<NodeClassificationModelResult, NodeClassificationTrainResult>
    for NodeClassificationToModelConverter
{
    fn to_model(
        &self,
        result: NodeClassificationTrainResult,
        original_schema: &GraphSchema,
    ) -> NodeClassificationModelResult {
        self.to_model(result, original_schema)
    }
}

// Note: Implement ResultToModelConverter trait once it is translated.
// impl ResultToModelConverter<NodeClassificationModelResult, NodeClassificationTrainResult> for NodeClassificationToModelConverter {
//     fn to_model(&self, result: &NodeClassificationTrainResult, original_schema: &GraphSchema) -> NodeClassificationModelResult {
//         self.to_model(result, original_schema)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::long_multiset::LongMultiSet;
    use crate::ml::core::subgraph::LocalIdMap;
    use crate::ml::core::tensor::Matrix;
    use crate::ml::metrics::classification::GlobalAccuracy;
    use crate::ml::metrics::{EvaluationScores, Metric, ModelCandidateStats};
    use crate::ml::models::base::{BaseModelData, ClassifierData};
    use crate::ml::models::training_method::TrainingMethod;
    use crate::ml::models::Classifier;
    use crate::ml::models::Features;
    use crate::ml::training::statistics::TrainingStatistics;
    use crate::types::schema::GraphSchema;
    use serde_json::json;
    use std::any::Any;
    use std::collections::HashMap;

    #[derive(Debug)]
    struct TestClassifierData;

    impl BaseModelData for TestClassifierData {
        fn trainer_method(&self) -> TrainingMethod {
            TrainingMethod::LogisticRegression
        }

        fn feature_dimension(&self) -> usize {
            1
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl ClassifierData for TestClassifierData {
        fn number_of_classes(&self) -> usize {
            2
        }
    }

    #[derive(Debug)]
    struct TestClassifier;

    impl Classifier for TestClassifier {
        fn data(&self) -> &dyn ClassifierData {
            &TestClassifierData
        }

        fn predict_probabilities(&self, _features: &[f64]) -> Vec<f64> {
            vec![0.5, 0.5]
        }

        fn predict_probabilities_batch(&self, batch: &[usize], _features: &dyn Features) -> Matrix {
            Matrix::new(vec![0.5; batch.len() * 2], batch.len(), 2)
        }
    }

    #[test]
    fn test_new_converter() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let config = NodeClassificationPipelineTrainConfig::default();

        let converter = NodeClassificationToModelConverter::new(pipeline, config);

        // Verify accessors work
        let _pipeline = converter.pipeline();
        let _config = converter.config();
    }

    #[test]
    fn test_to_model() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let config = NodeClassificationPipelineTrainConfig::default();
        let converter = NodeClassificationToModelConverter::new(pipeline, config);

        let metrics: Vec<Box<dyn Metric>> = vec![Box::new(GlobalAccuracy::new())];
        let mut training_statistics = TrainingStatistics::new(&metrics);

        let mut training_stats = HashMap::new();
        training_stats.insert(
            GlobalAccuracy::NAME.to_string(),
            EvaluationScores::new(0.8, 0.8, 0.8),
        );

        let mut validation_stats = HashMap::new();
        validation_stats.insert(
            GlobalAccuracy::NAME.to_string(),
            EvaluationScores::new(0.75, 0.75, 0.75),
        );

        training_statistics.add_candidate_stats(ModelCandidateStats::new(
            json!({ "method": "logisticRegression" }),
            training_stats,
            validation_stats,
        ));

        let train_result = NodeClassificationTrainResult::new(
            Box::new(TestClassifier),
            training_statistics,
            LocalIdMap::of(&[0, 1, 2]),
            LongMultiSet::new(),
        );
        let schema = GraphSchema::empty();

        let model_result = converter.to_model(train_result, &schema);

        // Verify result was created
        let _classifier = model_result.classifier();
        let _stats = model_result.training_statistics();
    }

    #[test]
    fn test_converter_references() {
        let pipeline = NodeClassificationTrainingPipeline::new();
        let config = NodeClassificationPipelineTrainConfig::default();

        let converter = NodeClassificationToModelConverter::new(pipeline, config);

        // Verify pipeline and config are accessible
        assert_eq!(
            converter.pipeline().pipeline_type(),
            NodeClassificationTrainingPipeline::PIPELINE_TYPE
        );
    }
}
