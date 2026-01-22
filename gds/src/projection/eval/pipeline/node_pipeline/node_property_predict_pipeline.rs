use super::NodeFeatureStep;
use crate::projection::eval::pipeline::PipelineValidationError;
use crate::projection::eval::pipeline::{ExecutableNodePropertyStep, Pipeline};
use crate::types::graph_store::DefaultGraphStore;
use dyn_clone::clone_box;
use serde_json::Value;
use std::collections::HashMap;

/// A prediction pipeline for node property prediction.
///
/// This is an immutable snapshot of a pipeline used for making predictions
/// on new data after training is complete.
pub struct NodePropertyPredictPipeline {
    node_property_steps: Vec<Box<dyn ExecutableNodePropertyStep>>,
    feature_steps: Vec<NodeFeatureStep>,
}

impl NodePropertyPredictPipeline {
    /// Empty pipeline with no steps.
    pub fn empty() -> Self {
        Self {
            node_property_steps: vec![],
            feature_steps: vec![],
        }
    }

    /// Creates a new predict pipeline from lists of steps.
    pub fn new(
        node_property_steps: Vec<Box<dyn ExecutableNodePropertyStep>>,
        feature_steps: Vec<NodeFeatureStep>,
    ) -> Self {
        Self {
            node_property_steps,
            feature_steps,
        }
    }

    /// Creates a predict pipeline from a training pipeline.
    pub fn from_pipeline<P: Pipeline<FeatureStep = NodeFeatureStep>>(pipeline: &P) -> Self {
        let node_property_steps = pipeline
            .node_property_steps()
            .iter()
            .map(|step| clone_box(&**step))
            .collect();

        let feature_steps = pipeline.feature_steps().to_vec();

        Self::new(node_property_steps, feature_steps)
    }

    /// Returns the list of feature properties used by this pipeline.
    pub fn feature_properties(&self) -> Vec<String> {
        use crate::projection::eval::pipeline::FeatureStep;
        self.feature_steps
            .iter()
            .flat_map(|step| step.input_node_properties().iter().cloned())
            .collect()
    }

    pub fn add_feature_step(&mut self, step: NodeFeatureStep) {
        self.feature_steps.push(step);
    }
}

impl Pipeline for NodePropertyPredictPipeline {
    type FeatureStep = NodeFeatureStep;

    fn to_map(&self) -> HashMap<String, Value> {
        use crate::projection::eval::pipeline::FeatureStep;
        let mut map = HashMap::new();

        let node_property_steps: Vec<Value> = self
            .node_property_steps
            .iter()
            .map(|step| {
                let step_map: serde_json::Map<String, Value> = step.to_map().into_iter().collect();
                Value::Object(step_map)
            })
            .collect();
        map.insert(
            "nodePropertySteps".to_string(),
            Value::Array(node_property_steps),
        );

        let feature_steps: Vec<Value> = self
            .feature_steps
            .iter()
            .map(|step| {
                let step_map: serde_json::Map<String, Value> = step.to_map().into_iter().collect();
                Value::Object(step_map)
            })
            .collect();
        map.insert("featureProperties".to_string(), Value::Array(feature_steps));

        map
    }
    fn node_property_steps(&self) -> &[Box<dyn ExecutableNodePropertyStep>] {
        &self.node_property_steps
    }

    fn feature_steps(&self) -> &[Self::FeatureStep] {
        &self.feature_steps
    }

    fn specific_validate_before_execution(
        &self,
        _graph_store: &DefaultGraphStore,
    ) -> Result<(), PipelineValidationError> {
        // No specific validation for predict pipeline
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::pipeline::PipelineValidationError;

    #[test]
    fn test_empty_pipeline() {
        let pipeline = NodePropertyPredictPipeline::empty();
        assert_eq!(pipeline.node_property_steps().len(), 0);
        assert_eq!(pipeline.feature_steps().len(), 0);
    }

    #[test]
    fn test_new_pipeline() {
        let feature_step = NodeFeatureStep::of("age");
        let pipeline = NodePropertyPredictPipeline::new(vec![], vec![feature_step]);

        assert_eq!(pipeline.node_property_steps().len(), 0);
        assert_eq!(pipeline.feature_steps().len(), 1);
    }

    #[test]
    fn test_feature_properties() {
        let feature_step1 = NodeFeatureStep::of("age");
        let feature_step2 = NodeFeatureStep::of("income");
        let pipeline = NodePropertyPredictPipeline::new(vec![], vec![feature_step1, feature_step2]);

        let props = pipeline.feature_properties();
        assert_eq!(props.len(), 2);
        assert!(props.contains(&"age".to_string()));
        assert!(props.contains(&"income".to_string()));
    }

    #[test]
    fn test_to_map() {
        let feature_step = NodeFeatureStep::of("age");
        let pipeline = NodePropertyPredictPipeline::new(vec![], vec![feature_step]);

        let map = pipeline.to_map();
        assert!(map.contains_key("nodePropertySteps"));
        assert!(map.contains_key("featureProperties"));
        assert_eq!(
            map.get("featureProperties")
                .and_then(|v| v.as_array())
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn test_add_feature_step() {
        let mut pipeline = NodePropertyPredictPipeline::empty();
        pipeline.add_feature_step(NodeFeatureStep::of("age"));

        assert_eq!(pipeline.feature_steps().len(), 1);
        assert_eq!(pipeline.feature_steps()[0].node_property(), "age");
    }

    #[test]
    fn test_specific_validate_before_execution() {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::RandomGraphConfig;

        let pipeline = NodePropertyPredictPipeline::empty();
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 100,
            ..RandomGraphConfig::default()
        };
        let graph_store = DefaultGraphStore::random(&config).expect("random graph");

        assert!(pipeline
            .specific_validate_before_execution(&graph_store)
            .is_ok());
    }

    #[test]
    fn test_validate_before_execution_missing_property_errors() {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::RandomGraphConfig;

        // Use a property name that is extremely unlikely to exist in any random graph.
        let missing_prop = "__definitely_missing_feature_property__";
        let pipeline =
            NodePropertyPredictPipeline::new(vec![], vec![NodeFeatureStep::of(missing_prop)]);

        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 50,
            ..RandomGraphConfig::default()
        };
        let graph_store = DefaultGraphStore::random(&config).expect("random graph");

        let err = pipeline
            .validate_before_execution(&graph_store, &[])
            .expect_err("expected missing property validation error");

        match err {
            PipelineValidationError::MissingNodeProperties { properties } => {
                assert!(properties.contains(&missing_prop.to_string()));
            }
            other => panic!("unexpected error: {other}"),
        }
    }
}
