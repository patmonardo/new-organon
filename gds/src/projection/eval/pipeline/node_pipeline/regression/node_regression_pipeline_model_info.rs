use crate::projection::eval::pipeline::TrainingMethod;
use crate::projection::eval::pipeline::node_pipeline::NodePropertyPredictPipeline;
use crate::projection::eval::pipeline::Pipeline;
use std::fmt;
use std::collections::HashMap;

// Placeholder types until ml-metrics, ml-training, and pipeline packages are complete
pub type Metric = ();
pub type ModelCandidateStats = ();

/// Custom metadata for trained node regression models.
///
/// This is the `CUSTOM_INFO` generic parameter in `Model<DATA, CONFIG, INFO>`.
/// Stores model-specific information beyond the raw trained weights:
/// - Test/train metric scores
/// - Best hyperparameters selected during model selection
/// - Pipeline configuration (features, node property steps)
///
/// Java source: `NodeRegressionPipelineModelInfo.java` (Immutables @ValueClass)
///
/// # Model.CustomInfo Pattern
/// Each pipeline type (Classification, Regression, LinkPrediction) has its own
/// ModelInfo implementation with pipeline-specific metadata. This enables:
/// - Feature importance tracking
/// - Hyperparameter history
/// - Pipeline reproducibility
pub struct NodeRegressionPipelineModelInfo {
    /// Metrics evaluated on the held-out test set.
    test_metrics: HashMap<Metric, f64>,

    /// Metrics evaluated on the outer training set (train portion of train/test split).
    outer_train_metrics: HashMap<Metric, f64>,

    /// Best model candidate selected during cross-validation.
    /// Contains winning hyperparameters and CV scores.
    best_candidate: ModelCandidateStats,

    /// The prediction pipeline (features + node property steps).
    /// Used for reproducibility and serving.
    pipeline: NodePropertyPredictPipeline,
}

impl fmt::Debug for NodeRegressionPipelineModelInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeRegressionPipelineModelInfo")
            .field("test_metrics_len", &self.test_metrics.len())
            .field("outer_train_metrics_len", &self.outer_train_metrics.len())
            .field(
                "node_property_steps_len",
                &self.pipeline.node_property_steps().len(),
            )
            .field(
                "feature_properties_len",
                &self.pipeline.feature_properties().len(),
            )
            .finish()
    }
}

impl NodeRegressionPipelineModelInfo {
    pub fn new(
        test_metrics: HashMap<Metric, f64>,
        outer_train_metrics: HashMap<Metric, f64>,
        best_candidate: ModelCandidateStats,
        pipeline: NodePropertyPredictPipeline,
    ) -> Self {
        Self {
            test_metrics,
            outer_train_metrics,
            best_candidate,
            pipeline,
        }
    }

    /// Returns metrics evaluated on test set.
    pub fn test_metrics(&self) -> &HashMap<Metric, f64> {
        &self.test_metrics
    }

    /// Returns metrics evaluated on outer training set.
    pub fn outer_train_metrics(&self) -> &HashMap<Metric, f64> {
        &self.outer_train_metrics
    }

    /// Returns best model candidate stats (hyperparameters + CV scores).
    pub fn best_candidate(&self) -> &ModelCandidateStats {
        &self.best_candidate
    }

    /// Returns the prediction pipeline configuration.
    pub fn pipeline(&self) -> &NodePropertyPredictPipeline {
        &self.pipeline
    }

    /// Returns the training method of the best model.
    ///
    /// Java: `Optional<TrainingMethod> optionalTrainerMethod()`
    pub fn optional_trainer_method(&self) -> Option<TrainingMethod> {
        // This will be populated once ModelCandidateStats and TrainerConfig are available.
        None
    }

    /// Convert model info to map for serialization.
    ///
    /// Java source: `toMap()` method (Immutables @Value.Derived)
    pub fn to_map(&self) -> HashMap<String, serde_json::Value> {
        use serde_json::json;

        let node_property_steps: Vec<serde_json::Value> = self
            .pipeline
            .node_property_steps()
            .iter()
            .map(|step| {
                let step_map: serde_json::Map<String, serde_json::Value> =
                    step.to_map().into_iter().collect();
                serde_json::Value::Object(step_map)
            })
            .collect();

        let feature_properties: Vec<serde_json::Value> = self
            .pipeline
            .feature_properties()
            .into_iter()
            .map(serde_json::Value::String)
            .collect();

        HashMap::from([
            ("bestParameters".to_string(), json!({})),
            ("metrics".to_string(), json!({})),
            ("pipeline".to_string(), json!(self.pipeline.to_map())),
            (
                "nodePropertySteps".to_string(),
                serde_json::Value::Array(node_property_steps),
            ),
            (
                "featureProperties".to_string(),
                serde_json::Value::Array(feature_properties),
            ),
        ])
    }
}

/// Builder for NodeRegressionPipelineModelInfo.
///
/// Java: `ImmutableNodeRegressionPipelineModelInfo.builder()`
#[derive(Default)]
pub struct NodeRegressionPipelineModelInfoBuilder {
    test_metrics: Option<HashMap<Metric, f64>>,
    outer_train_metrics: Option<HashMap<Metric, f64>>,
    best_candidate: Option<ModelCandidateStats>,
    pipeline: Option<NodePropertyPredictPipeline>,
}

impl NodeRegressionPipelineModelInfoBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn test_metrics(mut self, metrics: HashMap<Metric, f64>) -> Self {
        self.test_metrics = Some(metrics);
        self
    }

    pub fn outer_train_metrics(mut self, metrics: HashMap<Metric, f64>) -> Self {
        self.outer_train_metrics = Some(metrics);
        self
    }

    pub fn best_candidate(mut self, candidate: ModelCandidateStats) -> Self {
        self.best_candidate = Some(candidate);
        self
    }

    pub fn pipeline(mut self, pipeline: NodePropertyPredictPipeline) -> Self {
        self.pipeline = Some(pipeline);
        self
    }

    pub fn build(self) -> Result<NodeRegressionPipelineModelInfo, String> {
        Ok(NodeRegressionPipelineModelInfo {
            test_metrics: self.test_metrics.ok_or("test_metrics is required")?,
            outer_train_metrics: self
                .outer_train_metrics
                .ok_or("outer_train_metrics is required")?,
            best_candidate: self.best_candidate.ok_or("best_candidate is required")?,
            pipeline: self.pipeline.ok_or("pipeline is required")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_info_new() {
        let info = NodeRegressionPipelineModelInfo::new(
            HashMap::new(),
            HashMap::new(),
            (),
            NodePropertyPredictPipeline::empty(),
        );

        assert!(info.test_metrics().is_empty());
        assert!(info.outer_train_metrics().is_empty());
    }

    #[test]
    fn test_model_info_builder() {
        let result = NodeRegressionPipelineModelInfoBuilder::new()
            .test_metrics(HashMap::new())
            .outer_train_metrics(HashMap::new())
            .best_candidate(())
            .pipeline(NodePropertyPredictPipeline::empty())
            .build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_builder_requires_all_fields() {
        let result = NodeRegressionPipelineModelInfoBuilder::new()
            .test_metrics(HashMap::new())
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("outer_train_metrics"));
    }

    #[test]
    fn test_to_map_structure() {
        let info = NodeRegressionPipelineModelInfo::new(
            HashMap::new(),
            HashMap::new(),
            (),
            NodePropertyPredictPipeline::empty(),
        );

        let map = info.to_map();
        assert!(map.contains_key("bestParameters"));
        assert!(map.contains_key("metrics"));
        assert!(map.contains_key("pipeline"));
        assert!(map.contains_key("nodePropertySteps"));
        assert!(map.contains_key("featureProperties"));
    }
}
