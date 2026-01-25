use crate::collections::long_multiset::LongMultiSet;
use crate::config::validation::ConfigError;
use crate::ml::core::subgraph::LocalIdMap;
use crate::ml::metrics::{ClassificationMetric, ClassificationMetricSpecification, Metric};
use crate::projection::eval::pipeline::node_pipeline::NodePropertyPipelineBaseTrainConfig;

/// Training configuration for node classification pipelines.
///
/// Extends the base training configuration with classification-specific metrics.
#[derive(Debug, Clone)]
pub struct NodeClassificationPipelineTrainConfig {
    pipeline_name: String,
    target_labels: Vec<String>,
    target_property: String,
    random_seed: Option<u64>,
    metrics: Vec<ClassificationMetricSpecification>,
}

impl NodeClassificationPipelineTrainConfig {
    pub fn new(
        pipeline_name: String,
        target_labels: Vec<String>,
        target_property: String,
        random_seed: Option<u64>,
        metrics: Vec<ClassificationMetricSpecification>,
    ) -> Self {
        Self {
            pipeline_name,
            target_labels,
            target_property,
            random_seed,
            metrics,
        }
    }

    pub fn metrics_specs(&self) -> &[ClassificationMetricSpecification] {
        &self.metrics
    }

    /// Create concrete metrics from specifications given class ID map and class counts.
    pub fn metrics(
        &self,
        class_id_map: &LocalIdMap,
        class_counts: &LongMultiSet,
    ) -> Vec<Box<dyn Metric>> {
        self.metrics
            .iter()
            .flat_map(|spec| spec.create_metrics(class_id_map, class_counts))
            .collect()
    }

    /// Filter classification metrics (non-model-specific).
    pub fn classification_metrics(metrics: &[Box<dyn Metric>]) -> Vec<&dyn ClassificationMetric> {
        metrics
            .iter()
            .filter(|metric| !metric.is_model_specific())
            .filter_map(|metric| metric.as_classification_metric())
            .collect()
    }
}

impl Default for NodeClassificationPipelineTrainConfig {
    fn default() -> Self {
        Self::new(
            "default_pipeline".to_string(),
            vec!["*".to_string()],
            "target".to_string(),
            Some(42),
            vec![],
        )
    }
}

impl NodePropertyPipelineBaseTrainConfig for NodeClassificationPipelineTrainConfig {
    fn pipeline(&self) -> &str {
        &self.pipeline_name
    }

    fn target_node_labels(&self) -> Vec<String> {
        self.target_labels.clone()
    }

    fn target_property(&self) -> &str {
        &self.target_property
    }

    fn random_seed(&self) -> Option<u64> {
        self.random_seed
    }
}

impl crate::config::ValidatedConfig for NodeClassificationPipelineTrainConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        crate::config::validate_non_empty_string(self.pipeline(), "pipeline")?;
        crate::config::validate_non_empty_string(self.target_property(), "targetProperty")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::metrics::classification::GlobalAccuracy;
    use crate::ml::metrics::OutOfBagError;

    #[test]
    fn test_new_config() {
        let config = NodeClassificationPipelineTrainConfig::new(
            "test_pipeline".to_string(),
            vec!["Label1".to_string()],
            "target_property".to_string(),
            Some(42),
            vec![],
        );
        assert_eq!(config.metrics_specs().len(), 0);
        assert_eq!(config.pipeline(), "test_pipeline");
        assert_eq!(config.target_property(), "target_property");
        assert_eq!(config.random_seed(), Some(42));
    }

    #[test]
    fn test_default_config() {
        let config = NodeClassificationPipelineTrainConfig::default();
        assert_eq!(config.metrics_specs().len(), 0);
        assert_eq!(config.pipeline(), "default_pipeline");
        assert_eq!(config.target_node_labels(), vec!["*"]);
        assert_eq!(config.target_property(), "target");
        assert_eq!(config.random_seed(), Some(42));
    }

    #[test]
    fn test_metrics_placeholder() {
        let config = NodeClassificationPipelineTrainConfig::default();
        let class_id_map = LocalIdMap::of(&[0, 1, 2]);
        let class_counts = LongMultiSet::new();

        let metrics = config.metrics(&class_id_map, &class_counts);
        assert_eq!(metrics.len(), 0);
    }

    #[test]
    fn test_classification_metrics_filter() {
        let metrics: Vec<Box<dyn Metric>> = vec![
            Box::new(OutOfBagError::new()),
            Box::new(GlobalAccuracy::new()),
        ];
        let filtered = NodeClassificationPipelineTrainConfig::classification_metrics(&metrics);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_metrics_from_specifications() {
        let spec = ClassificationMetricSpecification::parse("F1(class=*)").unwrap();
        let config = NodeClassificationPipelineTrainConfig::new(
            "test_pipeline".to_string(),
            vec!["Label1".to_string()],
            "target_property".to_string(),
            Some(42),
            vec![spec],
        );

        let class_id_map = LocalIdMap::of(&[0, 1, 2]);
        let class_counts = LongMultiSet::new();
        let metrics = config.metrics(&class_id_map, &class_counts);

        let mut names: Vec<String> = metrics.iter().map(|m| m.name().to_string()).collect();
        names.sort();
        assert_eq!(names.len(), 3);
        assert!(names.iter().all(|name| name.starts_with("F1(class=")));
    }

    #[test]
    fn test_trait_implementation() {
        let config = NodeClassificationPipelineTrainConfig::new(
            "my_pipeline".to_string(),
            vec!["Person".to_string(), "Company".to_string()],
            "class_label".to_string(),
            Some(1337),
            vec![],
        );

        // Test NodePropertyPipelineBaseTrainConfig trait methods
        assert_eq!(config.pipeline(), "my_pipeline");
        assert_eq!(config.target_node_labels(), vec!["Person", "Company"]);
        assert_eq!(config.target_property(), "class_label");
        assert_eq!(config.random_seed(), Some(1337));
    }
}
