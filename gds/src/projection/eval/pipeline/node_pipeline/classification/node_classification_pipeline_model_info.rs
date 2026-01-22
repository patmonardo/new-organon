use crate::ml::metrics::ModelCandidateStats;
use crate::projection::eval::pipeline::node_pipeline::NodePropertyPredictPipeline;
use crate::projection::eval::pipeline::pipeline_trait::Pipeline;
use crate::projection::eval::pipeline::TrainingMethod;
use serde_json::Value;
use std::collections::HashMap;

/// Model information for node classification pipelines.
///
/// Contains the best training parameters, metrics, pipeline definition, and predicted classes.
///
/// Note: Cannot derive Clone or Debug because NodePropertyPredictPipeline contains Box<dyn Trait>.
pub struct NodeClassificationPipelineModelInfo {
    best_parameters: Value,
    metrics: HashMap<String, Value>,
    pipeline: NodePropertyPredictPipeline,
    classes: Vec<i64>,
}

impl NodeClassificationPipelineModelInfo {
    pub fn new(
        best_parameters: Value,
        metrics: HashMap<String, Value>,
        pipeline: NodePropertyPredictPipeline,
        classes: Vec<i64>,
    ) -> Self {
        Self {
            best_parameters,
            metrics,
            pipeline,
            classes,
        }
    }

    /// Create model info from training results.
    pub fn of(
        test_metrics: &HashMap<String, f64>,
        outer_train_metrics: &HashMap<String, f64>,
        best_candidate: &ModelCandidateStats,
        pipeline: NodePropertyPredictPipeline,
        classes: Vec<i64>,
    ) -> Self {
        let metrics = render_metrics(best_candidate, test_metrics, outer_train_metrics);
        Self::new(
            best_candidate.trainer_config.clone(),
            metrics,
            pipeline,
            classes,
        )
    }

    pub fn best_parameters(&self) -> &Value {
        &self.best_parameters
    }

    pub fn metrics(&self) -> &HashMap<String, Value> {
        &self.metrics
    }

    pub fn pipeline(&self) -> &NodePropertyPredictPipeline {
        &self.pipeline
    }

    pub fn classes(&self) -> &[i64] {
        &self.classes
    }

    /// Convert to map representation for serialization.
    pub fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();

        map.insert("bestParameters".to_string(), self.best_parameters.clone());
        map.insert(
            "classes".to_string(),
            Value::Array(
                self.classes
                    .iter()
                    .map(|c| Value::Number((*c).into()))
                    .collect(),
            ),
        );
        map.insert(
            "metrics".to_string(),
            Value::Object(
                self.metrics
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            ),
        );
        map.insert(
            "pipeline".to_string(),
            Value::Object(self.pipeline.to_map().into_iter().collect()),
        );
        map.insert(
            "nodePropertySteps".to_string(),
            Value::Array(
                self.pipeline
                    .node_property_steps()
                    .iter()
                    .map(|step| Value::Object(step.to_map().into_iter().collect()))
                    .collect(),
            ),
        );
        map.insert(
            "featureProperties".to_string(),
            Value::Array(
                self.pipeline
                    .feature_properties()
                    .iter()
                    .map(|s| Value::String(s.clone()))
                    .collect(),
            ),
        );

        map
    }

    /// Get optional training method.
    pub fn optional_trainer_method(&self) -> Option<TrainingMethod> {
        // TrainerConfig decoding is not yet wired; return None for now.
        None
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

    // Append test and outer-train metrics (if present)
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
    use serde_json::json;

    #[test]
    fn test_new_model_info() {
        let best_parameters = json!({});
        let metrics = HashMap::new();
        let pipeline = NodePropertyPredictPipeline::empty();
        let classes = vec![0, 1, 2];

        let info =
            NodeClassificationPipelineModelInfo::new(best_parameters, metrics, pipeline, classes);

        assert_eq!(info.classes(), &[0, 1, 2]);
    }

    #[test]
    fn test_of_constructor() {
        let test_metrics: HashMap<String, f64> = HashMap::new();
        let outer_train_metrics: HashMap<String, f64> = HashMap::new();
        let best_candidate = ModelCandidateStats::new(json!({}), HashMap::new(), HashMap::new());
        let pipeline = NodePropertyPredictPipeline::empty();
        let classes = vec![10, 20, 30];

        let info = NodeClassificationPipelineModelInfo::of(
            &test_metrics,
            &outer_train_metrics,
            &best_candidate,
            pipeline,
            classes,
        );

        assert_eq!(info.classes(), &[10, 20, 30]);
    }

    #[test]
    fn test_to_map() {
        let best_parameters = json!({});
        let metrics = HashMap::new();
        let pipeline = NodePropertyPredictPipeline::empty();
        let classes = vec![0, 1];

        let info =
            NodeClassificationPipelineModelInfo::new(best_parameters, metrics, pipeline, classes);

        let map = info.to_map();

        // Verify map contains expected keys
        assert!(map.contains_key("bestParameters"));
        assert!(map.contains_key("classes"));
        assert!(map.contains_key("metrics"));
        assert!(map.contains_key("pipeline"));
        assert!(map.contains_key("featureProperties"));
    }

    #[test]
    fn test_optional_trainer_method() {
        let info = NodeClassificationPipelineModelInfo::new(
            json!({}),
            HashMap::new(),
            NodePropertyPredictPipeline::empty(),
            vec![0, 1],
        );

        // Should return None until TrainerConfig is implemented
        assert!(info.optional_trainer_method().is_none());
    }
}
