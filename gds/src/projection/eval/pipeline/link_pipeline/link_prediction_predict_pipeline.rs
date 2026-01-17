// Phase 4.2: LinkPredictionPredictPipeline - Prediction pipeline for link prediction

use super::LinkFeatureStep;
use crate::projection::eval::pipeline::ExecutableNodePropertyStep;
use std::collections::HashMap;
use std::marker::PhantomData;

/// Prediction pipeline for link prediction.
///
/// Immutable snapshot of a training pipeline, containing the frozen node
/// property steps and link feature steps used for prediction.
pub struct LinkPredictionPredictPipeline {
    /// Node property steps (preprocessing) - frozen from training
    /// Note: This will become a concrete list of steps once link training execution is wired.
    node_property_steps: PhantomData<Vec<Box<dyn ExecutableNodePropertyStep>>>,

    /// Link feature steps (feature extraction) - frozen from training
    feature_steps: Vec<Box<dyn LinkFeatureStep>>,
}

impl LinkPredictionPredictPipeline {
    /// Creates an empty predict pipeline.
    pub fn empty() -> Self {
        Self {
            node_property_steps: PhantomData,
            feature_steps: Vec::new(),
        }
    }

    /// Creates a new LinkPredictionPredictPipeline from steps.
    ///
    /// # Arguments
    ///
    /// * `node_property_steps` - Preprocessing steps (placeholder)
    /// * `feature_steps` - Feature extraction steps
    pub fn from_steps(
        _node_property_steps: PhantomData<Vec<Box<dyn ExecutableNodePropertyStep>>>,
        feature_steps: Vec<Box<dyn LinkFeatureStep>>,
    ) -> Self {
        Self {
            node_property_steps: PhantomData,
            feature_steps,
        }
    }

    /// Creates a LinkPredictionPredictPipeline from a training pipeline.
    ///
    /// Copies feature steps and drops training-only configuration.
    pub fn from_training_pipeline(
        training_pipeline: &super::LinkPredictionTrainingPipeline,
    ) -> Self {
        // Copy feature steps from the training pipeline
        let feature_steps = training_pipeline.link_feature_steps();

        Self {
            node_property_steps: PhantomData,
            feature_steps,
        }
    }

    /// Creates a LinkPredictionPredictPipeline from iterators.
    pub fn from_iterators(
        _node_property_steps: impl Iterator<Item = PhantomData<Box<dyn ExecutableNodePropertyStep>>>,
        feature_steps: impl Iterator<Item = Box<dyn LinkFeatureStep>>,
    ) -> Self {
        Self {
            node_property_steps: PhantomData,
            feature_steps: feature_steps.collect(),
        }
    }

    /// Returns the node property steps (placeholder).
    pub fn node_property_steps(&self) -> &PhantomData<Vec<Box<dyn ExecutableNodePropertyStep>>> {
        &self.node_property_steps
    }

    /// Returns the feature steps.
    pub fn feature_steps(&self) -> &[Box<dyn LinkFeatureStep>] {
        &self.feature_steps
    }

    /// Converts the pipeline to a map (for serialization).
    pub fn to_map(&self) -> HashMap<String, Vec<HashMap<String, serde_json::Value>>> {
        let mut map = HashMap::new();

        // Node property steps (placeholder)
        // Note: Convert actual node property steps once they are stored concretely.
        let node_steps: Vec<HashMap<String, serde_json::Value>> = Vec::new();
        map.insert("nodePropertySteps".to_string(), node_steps);

        // Feature steps
        let feature_steps_maps: Vec<HashMap<String, serde_json::Value>> = self
            .feature_steps
            .iter()
            .map(|step| step.to_map())
            .collect();
        map.insert("featureSteps".to_string(), feature_steps_maps);

        map
    }

    /// Validates the pipeline before execution.
    ///
    /// For predict pipelines, validation is minimal (no training-specific checks).
    pub fn validate_before_execution(&self, _graph_store: PhantomData<()>) -> Result<(), String> {
        // Predict pipelines don't have training-specific validation
        // The steps were already validated during training
        Ok(())
    }
}

impl Default for LinkPredictionPredictPipeline {
    fn default() -> Self {
        Self::empty()
    }
}

// Note: Implement the shared Pipeline trait once associated types are finalized.

impl Clone for LinkPredictionPredictPipeline {
    fn clone(&self) -> Self {
        Self {
            node_property_steps: PhantomData,
            feature_steps: self.feature_steps.clone(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::LinkPredictionPredictPipeline;
    use crate::projection::eval::pipeline::link_pipeline::link_functions::{
        CosineFeatureStep, HadamardFeatureStep,
    };
    use crate::projection::eval::pipeline::link_pipeline::LinkFeatureStep;
    use std::marker::PhantomData;

    #[test]
    fn test_empty_pipeline() {
        let pipeline = LinkPredictionPredictPipeline::empty();
        assert_eq!(pipeline.feature_steps().len(), 0);
    }

    #[test]
    fn test_default_pipeline() {
        let pipeline = LinkPredictionPredictPipeline::default();
        assert_eq!(pipeline.feature_steps().len(), 0);
    }

    #[test]
    fn test_from_steps() {
        let feature_steps = vec![
            Box::new(HadamardFeatureStep::new(vec!["embedding".to_string()]))
                as Box<dyn LinkFeatureStep>,
        ];

        let pipeline = LinkPredictionPredictPipeline::from_steps(PhantomData, feature_steps);

        assert_eq!(pipeline.feature_steps().len(), 1);
        assert_eq!(pipeline.feature_steps()[0].name(), "HADAMARD");
    }

    #[test]
    fn test_from_iterators() {
        let feature_steps = vec![
            Box::new(HadamardFeatureStep::new(vec!["embedding".to_string()]))
                as Box<dyn LinkFeatureStep>,
            Box::new(CosineFeatureStep::new(vec!["features".to_string()]))
                as Box<dyn LinkFeatureStep>,
        ];

        let pipeline = LinkPredictionPredictPipeline::from_iterators(
            std::iter::empty(),
            feature_steps.into_iter(),
        );

        assert_eq!(pipeline.feature_steps().len(), 2);
    }

    #[test]
    fn test_to_map() {
        let feature_steps = vec![
            Box::new(HadamardFeatureStep::new(vec!["embedding".to_string()]))
                as Box<dyn LinkFeatureStep>,
        ];

        let pipeline = LinkPredictionPredictPipeline::from_steps(PhantomData, feature_steps);

        let map = pipeline.to_map();
        assert!(map.contains_key("nodePropertySteps"));
        assert!(map.contains_key("featureSteps"));
        assert_eq!(map.get("featureSteps").unwrap().len(), 1);
    }

    #[test]
    fn test_validate_before_execution() {
        let pipeline = LinkPredictionPredictPipeline::empty();
        let result = pipeline.validate_before_execution(PhantomData);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pipeline_trait_feature_steps() {
        let feature_steps = vec![Box::new(HadamardFeatureStep::new(vec!["prop".to_string()]))
            as Box<dyn LinkFeatureStep>];

        let pipeline = LinkPredictionPredictPipeline::from_steps(PhantomData, feature_steps);

        // LinkPredictionPredictPipeline does not implement the node Pipeline trait.
        assert_eq!(pipeline.feature_steps().len(), 1);
    }

    #[test]
    fn test_pipeline_is_immutable_by_api() {
        let feature_steps = vec![
            Box::new(HadamardFeatureStep::new(vec!["embedding".to_string()]))
                as Box<dyn LinkFeatureStep>,
        ];

        let pipeline = LinkPredictionPredictPipeline::from_steps(PhantomData, feature_steps);

        assert_eq!(pipeline.feature_steps().len(), 1);
    }

    #[test]
    fn test_pipeline_from_steps_preserves_feature_steps() {
        let feature_steps = vec![
            Box::new(HadamardFeatureStep::new(vec!["embedding".to_string()]))
                as Box<dyn LinkFeatureStep>,
            Box::new(CosineFeatureStep::new(vec!["features".to_string()]))
                as Box<dyn LinkFeatureStep>,
        ];

        let predict_pipeline =
            LinkPredictionPredictPipeline::from_steps(PhantomData, feature_steps);

        assert_eq!(predict_pipeline.feature_steps().len(), 2);
    }
}
