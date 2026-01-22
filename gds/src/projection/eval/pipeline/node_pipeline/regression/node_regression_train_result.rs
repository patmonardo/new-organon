use super::node_regression_pipeline_model_info::NodeRegressionPipelineModelInfo;
use super::NodeRegressionPipelineTrainConfig;
use crate::ml::models::base::Regressor;
use crate::ml::training::statistics::TrainingStatistics;

/// Result of training a node regression pipeline.
///
/// Contains the trained regressor model and associated training statistics
/// (cross-validation scores, best parameters, etc.).
///
/// Java source: `NodeRegressionTrainResult.java` (Immutables @ValueClass)
#[derive(Debug)]
pub struct NodeRegressionTrainResult {
    regressor: Box<dyn Regressor>,
    training_statistics: TrainingStatistics,
}

impl NodeRegressionTrainResult {
    pub fn new(regressor: Box<dyn Regressor>, training_statistics: TrainingStatistics) -> Self {
        Self {
            regressor,
            training_statistics,
        }
    }

    /// Returns the trained regressor model.
    pub fn regressor(&self) -> &dyn Regressor {
        &*self.regressor
    }

    pub fn into_regressor(self) -> Box<dyn Regressor> {
        self.regressor
    }

    /// Returns training statistics (CV scores, best params, etc.).
    pub fn training_statistics(&self) -> &TrainingStatistics {
        &self.training_statistics
    }
}

/// Result of training a node regression pipeline with catalog integration.
///
/// Extends the basic train result with model catalog metadata.
/// This is what gets stored in ModelCatalog after training.
///
/// Java source: `NodeRegressionTrainPipelineResult` (nested @ValueClass)
///
/// # Generic Parameters
/// This implements the `CatalogModelContainer<DATA, CONFIG, INFO>` pattern:
/// - `DATA`: Box<dyn RegressorData> - serialized model weights/parameters
/// - `CONFIG`: NodeRegressionPipelineTrainConfig - training configuration
/// - `INFO`: NodeRegressionPipelineModelInfo - custom metadata (feature importance, splits)
#[derive(Debug)]
pub struct NodeRegressionTrainPipelineResult {
    regressor: Box<dyn Regressor>,
    train_config: NodeRegressionPipelineTrainConfig,
    model_info: NodeRegressionPipelineModelInfo,

    // Training-specific field
    training_statistics: TrainingStatistics,
}

impl NodeRegressionTrainPipelineResult {
    pub fn new(
        regressor: Box<dyn Regressor>,
        train_config: NodeRegressionPipelineTrainConfig,
        model_info: NodeRegressionPipelineModelInfo,
        training_statistics: TrainingStatistics,
    ) -> Self {
        Self {
            regressor,
            train_config,
            model_info,
            training_statistics,
        }
    }

    /// Returns the trained regressor model.
    pub fn regressor(&self) -> &dyn Regressor {
        &*self.regressor
    }

    /// Returns the training configuration used.
    pub fn train_config(&self) -> &NodeRegressionPipelineTrainConfig {
        &self.train_config
    }

    /// Returns custom model metadata (feature importance, splits, etc.).
    pub fn model_info(&self) -> &NodeRegressionPipelineModelInfo {
        &self.model_info
    }

    /// Returns training statistics.
    pub fn training_statistics(&self) -> &TrainingStatistics {
        &self.training_statistics
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::metrics::Metric;
    use crate::ml::models::base::{BaseModelData, Regressor, RegressorData};
    use crate::ml::models::training_method::TrainingMethod;
    use crate::projection::eval::pipeline::node_pipeline::NodePropertyPipelineBaseTrainConfig;
    use crate::projection::eval::pipeline::node_pipeline::NodePropertyPredictPipeline;
    use serde_json::json;
    use std::any::Any;
    use std::collections::HashMap;

    // Placeholder implementations for tests
    #[derive(Debug)]
    struct TestRegressor;

    impl Regressor for TestRegressor {
        fn data(&self) -> &dyn RegressorData {
            &TestRegressorData
        }

        fn predict(&self, _features: &[f64]) -> f64 {
            0.0
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[derive(Debug)]
    struct TestRegressorData;

    impl BaseModelData for TestRegressorData {
        fn trainer_method(&self) -> TrainingMethod {
            TrainingMethod::LinearRegression
        }

        fn feature_dimension(&self) -> usize {
            1
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl RegressorData for TestRegressorData {}

    #[test]
    fn test_train_result_new() {
        let regressor = Box::new(TestRegressor);
        let metrics: Vec<Box<dyn Metric>> = vec![];
        let stats = TrainingStatistics::new(&metrics);
        let result = NodeRegressionTrainResult::new(regressor, stats);
        assert!(std::ptr::eq(result.regressor().data(), &TestRegressorData));
    }

    #[test]
    fn test_pipeline_result_new() {
        let config = NodeRegressionPipelineTrainConfig::default();
        let model_info = NodeRegressionPipelineModelInfo::new(
            json!({}),
            HashMap::new(),
            NodePropertyPredictPipeline::empty(),
        );
        let metrics: Vec<Box<dyn Metric>> = vec![];
        let training_stats = TrainingStatistics::new(&metrics);

        let result = NodeRegressionTrainPipelineResult::new(
            Box::new(TestRegressor),
            config,
            model_info,
            training_stats,
        );

        assert!(std::ptr::eq(result.regressor().data(), &TestRegressorData));
    }

    #[test]
    fn test_pipeline_result_config_access() {
        let config = NodeRegressionPipelineTrainConfig::default();
        let model_info = NodeRegressionPipelineModelInfo::new(
            json!({}),
            HashMap::new(),
            NodePropertyPredictPipeline::empty(),
        );
        let metrics: Vec<Box<dyn Metric>> = vec![];
        let training_stats = TrainingStatistics::new(&metrics);

        let result = NodeRegressionTrainPipelineResult::new(
            Box::new(TestRegressor),
            config.clone(),
            model_info,
            training_stats,
        );

        assert_eq!(result.train_config().pipeline(), config.pipeline());
        assert_eq!(
            result.train_config().target_property(),
            config.target_property()
        );
    }
}
