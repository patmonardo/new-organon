use super::{
    NodeRegressionPipelineModelInfo, NodeRegressionPipelineTrainConfig,
    NodeRegressionTrainPipelineResult, NodeRegressionTrainResult, NodeRegressionTrainingPipeline,
};
use crate::projection::eval::pipeline::node_pipeline::NodePropertyPredictPipeline;
use crate::projection::eval::pipeline::ResultToModelConverter;
use crate::types::schema::GraphSchema;

/// Converts node regression training results to catalog models.
///
/// This is the bridge between pipeline training and the model catalog.
/// Takes a `NodeRegressionTrainResult` (trained regressor + statistics) and
/// produces a `NodeRegressionTrainPipelineResult` (catalog-ready model).
///
/// Java source: `NodeRegressionToModelConverter.java`
///
/// # Model Creation Pattern
/// ```text
/// TrainResult → ModelConverter → CatalogModel
/// ```
///
/// The converter calls `Model.of(...)` with:
/// - GDS version
/// - Model type ("NodeRegression")
/// - Graph schema (node labels, property types)
/// - Regressor data (trained weights/parameters)
/// - Train config (hyperparameters, splits, metrics)
/// - Model info (custom metadata, feature importance)
#[derive(Debug, Clone)]
pub struct NodeRegressionToModelConverter {
    _pipeline: NodeRegressionTrainingPipeline,
    config: NodeRegressionPipelineTrainConfig,
}

impl NodeRegressionToModelConverter {
    pub fn new(
        pipeline: NodeRegressionTrainingPipeline,
        config: NodeRegressionPipelineTrainConfig,
    ) -> Self {
        Self {
            _pipeline: pipeline,
            config,
        }
    }

    /// Convert training result to catalog model.
    ///
    /// Java source: `toModel(NodeRegressionTrainResult, GraphSchema)`
    ///
    /// This is the key method that creates a `Model<RegressorData, TrainConfig, ModelInfo>`
    /// from training artifacts. The Model structure is:
    ///
    /// ```text
    /// Model<DATA, CONFIG, INFO> {
    ///     gds_version: String,           // e.g., "2.5.0"
    ///     model_type: String,            // "NodeRegression"
    ///     graph_schema: GraphSchema,     // Node labels, property types
    ///     data: RegressorData,           // Trained weights/trees
    ///     train_config: CONFIG,          // Training configuration
    ///     custom_info: INFO,             // ModelInfo with metrics/features
    /// }
    /// ```
    pub fn to_model(
        &self,
        train_result: NodeRegressionTrainResult,
        _original_schema: &GraphSchema,
    ) -> NodeRegressionTrainPipelineResult {
        let training_statistics = train_result.training_statistics().clone();
        let model_info = NodeRegressionPipelineModelInfo::of(
            training_statistics.winning_model_test_metrics(),
            training_statistics.winning_model_outer_train_metrics(),
            training_statistics.best_candidate(),
            NodePropertyPredictPipeline::from_pipeline(&self._pipeline),
        );

        NodeRegressionTrainPipelineResult::new(
            train_result.into_regressor(),
            self.config.clone(),
            model_info,
            training_statistics,
        )
    }
}
impl ResultToModelConverter<NodeRegressionTrainPipelineResult, NodeRegressionTrainResult>
    for NodeRegressionToModelConverter
{
    fn to_model(
        &self,
        result: NodeRegressionTrainResult,
        original_schema: &GraphSchema,
    ) -> NodeRegressionTrainPipelineResult {
        self.to_model(result, original_schema)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::metrics::regression::RegressionMetric;
    use crate::ml::metrics::{EvaluationScores, Metric, ModelCandidateStats};
    use crate::ml::models::base::{BaseModelData, Regressor, RegressorData};
    use crate::ml::models::training_method::TrainingMethod;
    use crate::ml::training::statistics::TrainingStatistics;
    use crate::types::schema::GraphSchema;
    use std::any::Any;

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
    fn test_converter_new() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = NodeRegressionPipelineTrainConfig::default();
        let _converter = NodeRegressionToModelConverter::new(pipeline, config);
    }

    #[test]
    fn test_to_model_structure() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = NodeRegressionPipelineTrainConfig::default();
        let converter = NodeRegressionToModelConverter::new(pipeline, config);

        let regressor = Box::new(TestRegressor);
        let metrics: Vec<Box<dyn Metric>> = vec![Box::new(RegressionMetric::MeanSquaredError)];
        let mut stats = TrainingStatistics::new(&metrics);
        let mut training_stats = std::collections::HashMap::new();
        let mut validation_stats = std::collections::HashMap::new();
        let scores = EvaluationScores::new(0.0, 0.0, 0.0);
        let metric_key = RegressionMetric::MeanSquaredError.name().to_string();
        training_stats.insert(metric_key.clone(), scores.clone());
        validation_stats.insert(metric_key, scores);
        stats.add_candidate_stats(ModelCandidateStats::new(
            serde_json::json!({}),
            training_stats,
            validation_stats,
        ));
        let train_result = NodeRegressionTrainResult::new(regressor, stats);
        let schema = GraphSchema::empty();
        let _model = converter.to_model(train_result, &schema);

        // Verify converter produces model (when Model system is complete, add assertions)
    }

    #[test]
    fn test_converter_trait_impl() {
        let pipeline = NodeRegressionTrainingPipeline::new();
        let config = NodeRegressionPipelineTrainConfig::default();
        let converter = NodeRegressionToModelConverter::new(pipeline, config);

        let regressor = Box::new(TestRegressor);
        let metrics: Vec<Box<dyn Metric>> = vec![Box::new(RegressionMetric::MeanSquaredError)];
        let mut stats = TrainingStatistics::new(&metrics);
        let mut training_stats = std::collections::HashMap::new();
        let mut validation_stats = std::collections::HashMap::new();
        let scores = EvaluationScores::new(0.0, 0.0, 0.0);
        let metric_key = RegressionMetric::MeanSquaredError.name().to_string();
        training_stats.insert(metric_key.clone(), scores.clone());
        validation_stats.insert(metric_key, scores);
        stats.add_candidate_stats(ModelCandidateStats::new(
            serde_json::json!({}),
            training_stats,
            validation_stats,
        ));
        let train_result = NodeRegressionTrainResult::new(regressor, stats);
        let schema = GraphSchema::empty();

        // Use trait method
        let _model = ResultToModelConverter::to_model(&converter, train_result, &schema);
    }
}
