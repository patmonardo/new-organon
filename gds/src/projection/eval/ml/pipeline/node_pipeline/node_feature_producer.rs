use super::{NodePropertyPipelineBaseTrainConfig, NodePropertyTrainingPipeline};
use crate::projection::eval::ml::pipeline::{
    ExecutableNodePropertyStep, NodePropertyStepExecutor, NodePropertyStepExecutorError,
};
use crate::types::graph_store::DefaultGraphStore;
use std::sync::Arc;

// TODO: These types will be implemented when ml-models package is translated
// use crate::ml::models::Features;
// use crate::ml::models::FeaturesFactory;

/// Placeholder for Features type until ml-models is translated
pub type Features = ();

/// Producer for extracting node features in ML pipelines.
///
/// This struct orchestrates:
/// 1. Executing node property steps to compute intermediate properties
/// 2. Validating that required feature properties exist
/// 3. Extracting features (eager or lazy) from the graph
/// 4. Cleaning up intermediate properties
///
/// **Java equivalent**: `NodeFeatureProducer<PIPELINE_CONFIG extends NodePropertyPipelineBaseTrainConfig>`
pub struct NodeFeatureProducer<C: NodePropertyPipelineBaseTrainConfig> {
    step_executor: NodePropertyStepExecutor,
    graph_store: Arc<DefaultGraphStore>,
    train_config: C,
}

impl<C: NodePropertyPipelineBaseTrainConfig> NodeFeatureProducer<C> {
    /// Creates a new NodeFeatureProducer.
    ///
    /// **Java**: Private constructor
    fn new(
        step_executor: NodePropertyStepExecutor,
        graph_store: Arc<DefaultGraphStore>,
        train_config: C,
    ) -> Self {
        Self {
            step_executor,
            graph_store,
            train_config,
        }
    }

    /// Factory method to create a NodeFeatureProducer.
    ///
    /// **Java**:
    /// ```java
    /// public static <PIPELINE_CONFIG extends NodePropertyPipelineBaseTrainConfig>
    /// NodeFeatureProducer<PIPELINE_CONFIG> create(
    ///     GraphStore graphStore,
    ///     PIPELINE_CONFIG config,
    ///     ExecutionContext executionContext,
    ///     ProgressTracker progressTracker
    /// )
    /// ```
    ///
    /// **Note**: This simplified version removes ExecutionContext and ProgressTracker
    /// following the Direct Integration pattern.
    pub fn create(graph_store: Arc<DefaultGraphStore>, config: C) -> Self {
        use std::collections::HashSet;

        // TODO: Get these from config when the methods are implemented
        let node_labels = config.node_labels();
        let relationship_types = vec![]; // TODO: config.internalRelationshipTypes()
        let available_relationship_types = HashSet::new(); // TODO: graphStore.relationshipTypes()
        let concurrency = 4; // TODO: Get from config

        let step_executor = NodePropertyStepExecutor::new(
            node_labels,
            relationship_types,
            available_relationship_types,
            concurrency,
        );

        Self::new(step_executor, graph_store, config)
    }

    /// Executes the pipeline and extracts features.
    ///
    /// This method:
    /// 1. Executes node property steps to compute intermediate properties
    /// 2. Validates that feature properties exist
    /// 3. Creates a filtered graph with target node labels
    /// 4. Extracts features (eager or lazy depending on pipeline requirements)
    /// 5. Cleans up intermediate properties (in finally block)
    ///
    /// **Java**:
    /// ```java
    /// public Features procedureFeatures(NodePropertyTrainingPipeline pipeline) {
    ///     try {
    ///         stepExecutor.executeNodePropertySteps(pipeline.nodePropertySteps());
    ///         Collection<NodeLabel> targetNodeLabels = trainConfig.nodeLabelIdentifiers(graphStore);
    ///         pipeline.validateFeatureProperties(graphStore, targetNodeLabels);
    ///
    ///         var targetNodeLabelGraph = graphStore.getGraph(targetNodeLabels);
    ///         if (pipeline.requireEagerFeatures()) {
    ///             return FeaturesFactory.extractEagerFeatures(targetNodeLabelGraph, pipeline.featureProperties());
    ///         } else {
    ///             return FeaturesFactory.extractLazyFeatures(targetNodeLabelGraph, pipeline.featureProperties());
    ///         }
    ///     } finally {
    ///         stepExecutor.cleanupIntermediateProperties(pipeline.nodePropertySteps());
    ///     }
    /// }
    /// ```
    pub fn procedure_features<P: NodePropertyTrainingPipeline>(
        &mut self,
        pipeline: &P,
    ) -> Result<Features, NodeFeatureProducerError> {
        // Execute node property steps to compute intermediate properties
        self.step_executor
            .execute_node_property_steps(&mut self.graph_store, pipeline.node_property_steps())
            .map_err(NodeFeatureProducerError::StepExecutionFailed)?;

        // Get target node labels
        let _target_node_labels = self
            .train_config
            .target_node_label_identifiers(&self.graph_store);

        // Validate that feature properties exist
        // TODO: Implement pipeline.validate_feature_properties() when Pipeline trait is extended
        // pipeline.validate_feature_properties(&self.graph_store, &target_node_labels)
        //     .map_err(NodeFeatureProducerError::FeatureValidationFailed)?;

        // Create a filtered graph with only target node labels
        // TODO: Implement graph_store.get_graph(labels) when GraphStore API is extended
        // let target_node_label_graph = self.graph_store.get_graph(&target_node_labels)
        //     .map_err(NodeFeatureProducerError::GraphFilterFailed)?;

        // Extract features (eager or lazy)
        // TODO: Implement FeaturesFactory when ml-models is translated
        let _eager = pipeline.require_eager_features();
        let _props = pipeline.feature_properties();
        let features = (); // Placeholder

        // Cleanup intermediate properties (always executed, like finally block)
        self.step_executor
            .cleanup_intermediate_properties(&mut self.graph_store, pipeline.node_property_steps())
            .map_err(NodeFeatureProducerError::CleanupFailed)?;

        Ok(features)
    }

    /// Validates node property step context configurations.
    ///
    /// **Java**:
    /// ```java
    /// public void validateNodePropertyStepsContextConfigs(List<ExecutableNodePropertyStep> steps) {
    ///     stepExecutor.validNodePropertyStepsContextConfigs(steps);
    /// }
    /// ```
    pub fn validate_node_property_steps_context_configs(
        &self,
        steps: &[Box<dyn ExecutableNodePropertyStep>],
    ) -> Result<(), NodePropertyStepExecutorError> {
        self.step_executor
            .validate_node_property_steps_context_configs(&self.graph_store, steps)
    }
}

/// Errors that can occur during feature production.
#[derive(Debug)]
pub enum NodeFeatureProducerError {
    /// Error executing node property steps
    StepExecutionFailed(NodePropertyStepExecutorError),
    /// Error validating feature properties
    FeatureValidationFailed(String),
    /// Error filtering graph by node labels
    GraphFilterFailed(String),
    /// Error extracting features
    FeatureExtractionFailed(String),
    /// Error cleaning up intermediate properties
    CleanupFailed(NodePropertyStepExecutorError),
}

impl std::fmt::Display for NodeFeatureProducerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StepExecutionFailed(e) => {
                write!(f, "Failed to execute node property steps: {}", e)
            }
            Self::FeatureValidationFailed(msg) => write!(f, "Feature validation failed: {}", msg),
            Self::GraphFilterFailed(msg) => write!(f, "Failed to filter graph: {}", msg),
            Self::FeatureExtractionFailed(msg) => write!(f, "Failed to extract features: {}", msg),
            Self::CleanupFailed(e) => write!(f, "Failed to cleanup intermediate properties: {}", e),
        }
    }
}

impl std::error::Error for NodeFeatureProducerError {}

// Placeholder config for testing
#[derive(Clone)]
pub struct PlaceholderNodePropertyConfig;

impl NodePropertyPipelineBaseTrainConfig for PlaceholderNodePropertyConfig {
    fn pipeline(&self) -> &str {
        "placeholder"
    }
    fn target_node_labels(&self) -> Vec<String> {
        vec![]
    }
    fn target_property(&self) -> &str {
        "target"
    }
    fn random_seed(&self) -> Option<u64> {
        Some(42)
    }
}

impl NodeFeatureProducer<PlaceholderNodePropertyConfig> {
    /// Create a placeholder NodeFeatureProducer for testing.
    pub fn placeholder() -> Self {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::RandomGraphConfig;
        use std::collections::HashSet;

        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(42),
            ..RandomGraphConfig::default()
        };
        let graph_store =
            Arc::new(DefaultGraphStore::random(&config).expect("Failed to generate random graph"));
        let placeholder_config = PlaceholderNodePropertyConfig;
        let node_labels = placeholder_config.node_labels();
        let step_executor = NodePropertyStepExecutor::new(node_labels, vec![], HashSet::new(), 1);

        Self::new(step_executor, graph_store, placeholder_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::RandomGraphConfig;
    use std::sync::Arc;

    // Mock config for testing
    struct MockTrainConfig {
        pipeline_name: String,
        target_labels: Vec<String>,
        target_prop: String,
    }

    impl NodePropertyPipelineBaseTrainConfig for MockTrainConfig {
        fn pipeline(&self) -> &str {
            &self.pipeline_name
        }

        fn target_node_labels(&self) -> Vec<String> {
            self.target_labels.clone()
        }

        fn target_property(&self) -> &str {
            &self.target_prop
        }

        fn random_seed(&self) -> Option<u64> {
            Some(42)
        }
    }

    #[test]
    fn test_create_node_feature_producer() {
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 100,
            ..RandomGraphConfig::default()
        };
        let graph_store = Arc::new(DefaultGraphStore::random(&config).expect("random graph"));

        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["Person".to_string()],
            target_prop: "label".to_string(),
        };

        let _producer = NodeFeatureProducer::create(graph_store, config);
        // Success if it doesn't panic
    }

    #[test]
    fn test_create_with_wildcard_labels() {
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 100,
            ..RandomGraphConfig::default()
        };
        let graph_store = Arc::new(DefaultGraphStore::random(&config).expect("random graph"));

        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["*".to_string()],
            target_prop: "label".to_string(),
        };

        let _producer = NodeFeatureProducer::create(graph_store, config);
        // Success if it doesn't panic
    }

    #[test]
    fn test_validate_steps_context_configs() {
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 100,
            ..RandomGraphConfig::default()
        };
        let graph_store = Arc::new(DefaultGraphStore::random(&config).expect("random graph"));

        let config = MockTrainConfig {
            pipeline_name: "test-pipeline".to_string(),
            target_labels: vec!["Person".to_string()],
            target_prop: "label".to_string(),
        };

        let producer = NodeFeatureProducer::create(graph_store, config);

        // Empty steps should validate successfully
        let steps: Vec<Box<dyn ExecutableNodePropertyStep>> = vec![];
        assert!(producer
            .validate_node_property_steps_context_configs(&steps)
            .is_ok());
    }
}
