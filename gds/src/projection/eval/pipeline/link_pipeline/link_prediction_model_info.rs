// Phase 1.6: LinkPredictionModelInfo - Model metadata for link prediction

use std::collections::HashMap;

use super::LinkPredictionPredictPipeline;
use crate::ml::metrics::ModelCandidateStats;
use crate::projection::eval::pipeline::TrainingMethod;
use serde_json::Value;

/// Custom metadata for Link Prediction models.
///
/// Implements `CustomInfo` trait for `Model<ClassifierData, LinkPredictionTrainConfig, LinkPredictionModelInfo>`.
///
/// Contains:
/// - **Best parameters**: Winning hyperparameters from model selection
/// - **Metrics**: Test and validation metrics (AUCPR, ROC_AUC, etc.)
/// - **Pipeline**: The predict pipeline (node property steps + link feature steps)
///
/// # Model Selection Process
///
/// ```text
/// 1. RandomSearch over TrainerConfig candidates
/// 2. Cross-validation for each candidate → ModelCandidateStats
/// 3. Select best by metric (e.g., AUCPR) → bestParameters
/// 4. Train final model on full train set
/// 5. Evaluate on test set → testMetrics
/// 6. Bundle → LinkPredictionModelInfo
/// ```
///
/// # Usage in Model
///
/// ```text
/// Model::of(
///     gds_version: "2.5.0",
///     model_type: "LinkPrediction",
///     graph_schema: training_graph_schema,
///     data: classifier_data,              // Trained Classifier
///     train_config: link_pred_config,     // LinkPredictionTrainConfig
///     custom_info: model_info,            // LinkPredictionModelInfo (THIS)
/// )
/// ```
#[derive(Clone)]
pub struct LinkPredictionModelInfo {
    /// Best hyperparameters selected during training
    best_parameters: Value,

    /// Rendered metrics (test + outer train) as string map
    metrics: HashMap<String, serde_json::Value>,

    /// Predict pipeline (subset of training pipeline)
    pipeline: LinkPredictionPredictPipeline,
}

impl LinkPredictionModelInfo {
    /// Creates a new LinkPredictionModelInfo.
    ///
    /// Typically called via `of()` static factory which renders metrics from raw values.
    pub fn new(
        best_parameters: Value,
        metrics: HashMap<String, serde_json::Value>,
        pipeline: LinkPredictionPredictPipeline,
    ) -> Self {
        Self {
            best_parameters,
            metrics,
            pipeline,
        }
    }

    /// Creates LinkPredictionModelInfo from test/train metrics and best candidate.
    ///
    /// # Arguments
    ///
    /// * `test_metrics` - Metrics on held-out test set
    /// * `outer_train_metrics` - Metrics on outer training set
    /// * `best_candidate` - Best model candidate from hyperparameter search
    /// * `pipeline` - Predict pipeline for this model
    ///
    /// # Returns
    ///
    /// LinkPredictionModelInfo with rendered metrics
    pub fn of(
        test_metrics: HashMap<String, f64>,
        outer_train_metrics: HashMap<String, f64>,
        best_candidate: ModelCandidateStats,
        pipeline: LinkPredictionPredictPipeline,
    ) -> Self {
        let metrics = render_metrics(&best_candidate, &test_metrics, &outer_train_metrics);
        let best_parameters = best_candidate.trainer_config.clone();

        Self::new(best_parameters, metrics, pipeline)
    }

    /// Returns the best hyperparameters.
    pub fn best_parameters(&self) -> &Value {
        &self.best_parameters
    }

    /// Returns the metrics map.
    pub fn metrics(&self) -> &HashMap<String, serde_json::Value> {
        &self.metrics
    }

    /// Returns the predict pipeline.
    pub fn pipeline(&self) -> &LinkPredictionPredictPipeline {
        &self.pipeline
    }

    /// Converts to map representation for serialization.
    ///
    /// Includes:
    /// - `bestParameters`: Trainer config with method
    /// - `metrics`: Rendered test/train metrics
    /// - `pipeline`: Full pipeline map
    /// - `nodePropertySteps`: Node property steps (legacy)
    /// - `featureSteps`: Link feature steps (legacy)
    pub fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();

        map.insert("bestParameters".to_string(), self.best_parameters.clone());
        map.insert(
            "metrics".to_string(),
            serde_json::Value::Object(
                self.metrics
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect(),
            ),
        );

        let pipeline_map = self.pipeline.to_map();
        map.insert("pipeline".to_string(), serde_json::json!(pipeline_map));

        // Back-compat fields (mirrors Java shape).
        map.insert(
            "nodePropertySteps".to_string(),
            serde_json::json!(self
                .pipeline
                .to_map()
                .get("nodePropertySteps")
                .cloned()
                .unwrap_or_default()),
        );
        map.insert(
            "featureSteps".to_string(),
            serde_json::json!(self
                .pipeline
                .to_map()
                .get("featureSteps")
                .cloned()
                .unwrap_or_default()),
        );

        map
    }

    /// Returns the optional training method.
    ///
    /// Extracted from `best_parameters().method()`.
    pub fn optional_trainer_method(&self) -> Option<TrainingMethod> {
        // Note: Return Some(best_parameters.method()) once TrainerConfig exists.
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

impl std::fmt::Debug for LinkPredictionModelInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkPredictionModelInfo")
            .field("metrics_keys", &self.metrics.keys().collect::<Vec<_>>())
            .field("pipeline", &"<LinkPredictionPredictPipeline>")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_info_creation() {
        let params = serde_json::json!({});
        let metrics = HashMap::new();
        let pipeline = LinkPredictionPredictPipeline::empty();

        let info = LinkPredictionModelInfo::new(params, metrics, pipeline);

        assert!(info.metrics().is_empty());
    }

    #[test]
    fn test_of_factory() {
        let test_metrics = HashMap::new();
        let train_metrics = HashMap::new();
        let best_candidate =
            ModelCandidateStats::new(serde_json::json!({}), HashMap::new(), HashMap::new());
        let pipeline = LinkPredictionPredictPipeline::empty();

        let info =
            LinkPredictionModelInfo::of(test_metrics, train_metrics, best_candidate, pipeline);

        assert!(info.metrics().is_empty()); // Placeholder implementation
    }

    #[test]
    fn test_accessors() {
        let info = LinkPredictionModelInfo::new(
            serde_json::json!({}),
            HashMap::new(),
            LinkPredictionPredictPipeline::empty(),
        );

        let _params = info.best_parameters();
        let _metrics = info.metrics();
        let _pipeline = info.pipeline();
    }

    #[test]
    fn test_to_map() {
        let info = LinkPredictionModelInfo::new(
            serde_json::json!({}),
            HashMap::new(),
            LinkPredictionPredictPipeline::empty(),
        );

        let map = info.to_map();

        assert!(map.contains_key("bestParameters"));
        assert!(map.contains_key("metrics"));
        assert!(map.contains_key("pipeline"));
        assert!(map.contains_key("nodePropertySteps"));
        assert!(map.contains_key("featureSteps"));
    }

    #[test]
    fn test_optional_trainer_method() {
        let info = LinkPredictionModelInfo::new(
            serde_json::json!({}),
            HashMap::new(),
            LinkPredictionPredictPipeline::empty(),
        );

        assert!(info.optional_trainer_method().is_none()); // Placeholder
    }

    #[test]
    fn test_clone() {
        let info1 = LinkPredictionModelInfo::new(
            serde_json::json!({}),
            HashMap::new(),
            LinkPredictionPredictPipeline::empty(),
        );
        let info2 = info1.clone();

        assert_eq!(info1.metrics().len(), info2.metrics().len());
    }
}
