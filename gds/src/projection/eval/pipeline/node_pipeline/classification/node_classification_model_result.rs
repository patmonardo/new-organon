use super::node_classification_pipeline_model_info::NodeClassificationPipelineModelInfo;
use super::node_classification_pipeline_train_config::NodeClassificationPipelineTrainConfig;
use crate::ml::models::Classifier;
use crate::ml::training::statistics::TrainingStatistics;

/// Result of node classification model creation containing the catalog model and training statistics.
///
/// This is a value class that wraps the model catalog entry with training statistics.
pub struct NodeClassificationModelResult {
    classifier: Box<dyn Classifier>,
    train_config: NodeClassificationPipelineTrainConfig,
    model_info: NodeClassificationPipelineModelInfo,
    training_statistics: TrainingStatistics,
}

impl NodeClassificationModelResult {
    pub fn new(
        classifier: Box<dyn Classifier>,
        train_config: NodeClassificationPipelineTrainConfig,
        model_info: NodeClassificationPipelineModelInfo,
        training_statistics: TrainingStatistics,
    ) -> Self {
        Self {
            classifier,
            train_config,
            model_info,
            training_statistics,
        }
    }

    pub fn classifier(&self) -> &dyn Classifier {
        &*self.classifier
    }

    pub fn train_config(&self) -> &NodeClassificationPipelineTrainConfig {
        &self.train_config
    }

    pub fn model_info(&self) -> &NodeClassificationPipelineModelInfo {
        &self.model_info
    }

    pub fn training_statistics(&self) -> &TrainingStatistics {
        &self.training_statistics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::metrics::classification::GlobalAccuracy;
    use crate::ml::models::Features;
    use crate::ml::core::tensor::Matrix;
    use crate::ml::metrics::Metric;
    use crate::ml::models::base::{BaseModelData, ClassifierData};
    use crate::ml::models::training_method::TrainingMethod;
    use crate::projection::eval::pipeline::node_pipeline::node_property_pipeline_base_train_config::NodePropertyPipelineBaseTrainConfig;
    use crate::projection::eval::pipeline::node_pipeline::NodePropertyPredictPipeline;
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
    fn test_new_model_result() {
        let classifier = Box::new(TestClassifier) as Box<dyn Classifier>;
        let metrics: Vec<Box<dyn Metric>> = vec![Box::new(GlobalAccuracy::new())];
        let training_statistics = TrainingStatistics::new(&metrics);
        let config = NodeClassificationPipelineTrainConfig::default();
        let model_info = NodeClassificationPipelineModelInfo::new(
            serde_json::json!({}),
            HashMap::new(),
            NodePropertyPredictPipeline::empty(),
            vec![],
        );

        let result =
            NodeClassificationModelResult::new(classifier, config, model_info, training_statistics);

        // Verify accessors work
        let _model = result.classifier();
        let _stats = result.training_statistics();
    }

    #[test]
    fn test_accessors() {
        let classifier = Box::new(TestClassifier) as Box<dyn Classifier>;
        let metrics: Vec<Box<dyn Metric>> = vec![Box::new(GlobalAccuracy::new())];
        let training_statistics = TrainingStatistics::new(&metrics);
        let config = NodeClassificationPipelineTrainConfig::default();
        let model_info = NodeClassificationPipelineModelInfo::new(
            serde_json::json!({}),
            HashMap::new(),
            NodePropertyPredictPipeline::empty(),
            vec![],
        );

        let result =
            NodeClassificationModelResult::new(classifier, config, model_info, training_statistics);

        assert_eq!(result.train_config().pipeline(), "default_pipeline");
        assert!(result.model_info().classes().is_empty());
    }
}
