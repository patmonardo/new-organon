use crate::ml::metrics::ModelCandidateStats;
use crate::projection::eval::pipeline::node_pipeline::NodePropertyPredictPipeline;
use crate::projection::eval::pipeline::Pipeline;
use crate::projection::eval::pipeline::TrainingMethod;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

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
    best_parameters: Value,
    metrics: HashMap<String, Value>,
    pipeline: NodePropertyPredictPipeline,
}

impl fmt::Debug for NodeRegressionPipelineModelInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeRegressionPipelineModelInfo")
            .field("metrics_len", &self.metrics.len())
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
        best_parameters: Value,
        metrics: HashMap<String, Value>,
        pipeline: NodePropertyPredictPipeline,
    ) -> Self {
        Self {
            best_parameters,
            metrics,
            pipeline,
        }
    }

    pub fn best_parameters(&self) -> &Value {
        &self.best_parameters
    }

    pub fn metrics(&self) -> &HashMap<String, Value> {
        &self.metrics
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

        let mut map = HashMap::new();
        map.insert("bestParameters".to_string(), self.best_parameters.clone());
        map.insert(
            "metrics".to_string(),
            Value::Object(
                self.metrics
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            ),
        );
        map.insert("pipeline".to_string(), json!(self.pipeline.to_map()));
        map.insert(
            "nodePropertySteps".to_string(),
            serde_json::Value::Array(node_property_steps),
        );
        map.insert(
            "featureProperties".to_string(),
            serde_json::Value::Array(feature_properties),
        );

        map
    }
}

/// Builder for NodeRegressionPipelineModelInfo.
///
/// Java: `ImmutableNodeRegressionPipelineModelInfo.builder()`
#[derive(Default)]
pub struct NodeRegressionPipelineModelInfoBuilder {
    test_metrics: Option<HashMap<String, f64>>,
    outer_train_metrics: Option<HashMap<String, f64>>,
    best_candidate: Option<ModelCandidateStats>,
    pipeline: Option<NodePropertyPredictPipeline>,
}

impl NodeRegressionPipelineModelInfoBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn test_metrics(mut self, metrics: HashMap<String, f64>) -> Self {
        self.test_metrics = Some(metrics);
        self
    }

    pub fn outer_train_metrics(mut self, metrics: HashMap<String, f64>) -> Self {
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
        let test_metrics = self.test_metrics.ok_or("test_metrics is required")?;
        let outer_train_metrics = self
            .outer_train_metrics
            .ok_or("outer_train_metrics is required")?;
        let best_candidate = self.best_candidate.ok_or("best_candidate is required")?;
        let pipeline = self.pipeline.ok_or("pipeline is required")?;

        Ok(NodeRegressionPipelineModelInfo::of(
            &test_metrics,
            &outer_train_metrics,
            &best_candidate,
            pipeline,
        ))
    }
}

impl NodeRegressionPipelineModelInfo {
    pub fn of(
        test_metrics: &HashMap<String, f64>,
        outer_train_metrics: &HashMap<String, f64>,
        best_candidate: &ModelCandidateStats,
        pipeline: NodePropertyPredictPipeline,
    ) -> Self {
        let metrics = render_metrics(best_candidate, test_metrics, outer_train_metrics);
        Self::new(best_candidate.trainer_config.clone(), metrics, pipeline)
    }
}

fn render_metrics(
    best_candidate: &ModelCandidateStats,
    test_metrics: &HashMap<String, f64>,
    outer_train_metrics: &HashMap<String, f64>,
) -> HashMap<String, Value> {
    let mut metrics: HashMap<String, Value> = HashMap::new();

    for (name, scores) in &best_candidate.training_stats {
        let mut entry = serde_json::Map::new();
        entry.insert("train".to_string(), scores.to_map());
        if let Some(validation) = best_candidate.validation_stats.get(name) {
            entry.insert("validation".to_string(), validation.to_map());
        }
        metrics.insert(name.clone(), Value::Object(entry));
    }

    append_additional_metrics(&mut metrics, test_metrics, "test");
    append_additional_metrics(&mut metrics, outer_train_metrics, "outerTrain");

    metrics
}

fn append_additional_metrics(
    metrics: &mut HashMap<String, Value>,
    additional: &HashMap<String, f64>,
    key: &str,
) {
    for (metric, score) in additional {
        let entry = metrics
            .entry(metric.clone())
            .or_insert_with(|| Value::Object(serde_json::Map::new()));

        if let Value::Object(obj) = entry {
            obj.insert(
                key.to_string(),
                Value::Number(serde_json::Number::from_f64(*score).unwrap()),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_info_new() {
        let info = NodeRegressionPipelineModelInfo::new(
            serde_json::json!({}),
            HashMap::new(),
            NodePropertyPredictPipeline::empty(),
        );

        assert!(info.metrics().is_empty());
    }

    #[test]
    fn test_model_info_builder() {
        let result = NodeRegressionPipelineModelInfoBuilder::new()
            .test_metrics(HashMap::new())
            .outer_train_metrics(HashMap::new())
            .best_candidate(ModelCandidateStats::new(
                serde_json::json!({}),
                HashMap::new(),
                HashMap::new(),
            ))
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
            serde_json::json!({}),
            HashMap::new(),
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
