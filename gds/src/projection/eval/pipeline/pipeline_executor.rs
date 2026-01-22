// Translated from Neo4j Graph Data Science:
// https://github.com/neo4j/graph-data-science
// pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PipelineExecutor.java

use std::collections::{HashMap, HashSet};
use std::error::Error as StdError;
use std::sync::Arc;

use crate::projection::eval::pipeline::{NodePropertyStepExecutor, Pipeline, PipelineGraphFilter};
use crate::types::graph_store::{DefaultGraphStore, GraphStore};
use crate::types::schema::GraphSchema;

/// Dataset split types for training and evaluation pipelines.
///
/// Java GDS defines this as a nested enum inside PipelineExecutor:
/// ```java
/// public enum DatasetSplits {
///     TRAIN,
///     TEST,
///     TEST_COMPLEMENT,
///     FEATURE_INPUT
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DatasetSplits {
    /// Training dataset split.
    Train,
    /// Test dataset split (for evaluation).
    Test,
    /// Test complement (all nodes not in test set).
    TestComplement,
    /// Feature input (nodes used for feature extraction, includes train+test+context).
    FeatureInput,
}

/// Abstract pipeline executor for training and evaluation.
///
/// This trait implements the template method pattern, providing a standard
/// execution flow (compute()) while allowing subclasses to customize specific
/// steps via abstract methods.
///
/// # Direct Integration Approach
///
/// Unlike Java's ExecutionContext wrapper, this takes Arc<DefaultGraphStore>
/// directly, making GraphStore access explicit and clear.
///
/// # Type Parameters
///
/// * `PIPELINE` - The pipeline type (e.g., NodeClassificationPipeline)
/// * `RESULT` - The result type (e.g., classification predictions)
///
/// # Java Source (PipelineExecutor.java)
/// ```java
/// public abstract class PipelineExecutor<
///     PIPELINE_CONFIG extends AlgoBaseConfig & GraphNameConfig,
///     PIPELINE extends Pipeline<?>,
///     RESULT
/// > extends Algorithm<RESULT> {
///     protected final PIPELINE pipeline;
///     protected final PIPELINE_CONFIG config;
///     protected final ExecutionContext executionContext;
///     protected final GraphStore graphStore;
///     protected final GraphSchema schemaBeforeSteps;
///
///     public abstract Map<DatasetSplits, PipelineGraphFilter> generateDatasetSplitGraphFilters();
///     public abstract void splitDatasets();
///     protected abstract RESULT execute(Map<DatasetSplits, PipelineGraphFilter> dataSplits);
///     protected abstract Set<RelationshipType> getAvailableRelTypesForNodePropertySteps();
///
///     @Override
///     public RESULT compute() { /* template method */ }
/// }
/// ```
pub trait PipelineExecutor<PIPELINE: Pipeline, RESULT> {
    /// Access the pipeline being executed.
    fn pipeline(&self) -> &PIPELINE;

    /// Borrow pipeline + graph store together.
    ///
    /// This exists to avoid borrow-checker conflicts in the default `compute()`
    /// implementation (disjoint field borrows are expressed by the implementor).
    fn pipeline_and_graph_store_mut(&mut self) -> (&PIPELINE, &mut Arc<DefaultGraphStore>);

    /// Access the graph store (mutable for property steps).
    fn graph_store_mut(&mut self) -> &mut Arc<DefaultGraphStore>;

    /// Access the graph store (immutable).
    fn graph_store(&self) -> &Arc<DefaultGraphStore>;

    /// Access the schema before node property steps were executed.
    ///
    /// This captures the original graph schema, useful for validation
    /// and determining which properties were added by the pipeline.
    fn schema_before_steps(&self) -> &GraphSchema;

    /// Access the node labels for this execution.
    fn node_labels(&self) -> &[String];

    /// Access the relationship types for this execution.
    fn relationship_types(&self) -> &[String];

    /// Access the concurrency setting.
    fn concurrency(&self) -> usize;

    /// Generate dataset split graph filters.
    ///
    /// Creates filters for:
    /// - TRAIN: Training data nodes/relationships
    /// - TEST: Test data nodes/relationships
    /// - TEST_COMPLEMENT: All nodes not in test set
    /// - FEATURE_INPUT: All nodes used for feature extraction (train+test+context)
    ///
    /// Java: `abstract Map<DatasetSplits, PipelineGraphFilter> generateDatasetSplitGraphFilters()`
    fn generate_dataset_split_graph_filters(&self) -> HashMap<DatasetSplits, PipelineGraphFilter>;

    /// Split the dataset into train/test partitions.
    ///
    /// This may create properties or modify graph state to mark splits.
    /// Called after filters are generated but before step execution.
    ///
    /// Java: `abstract void splitDatasets()`
    fn split_datasets(&mut self) -> Result<(), PipelineExecutorError>;

    /// Execute the pipeline algorithm on the split datasets.
    ///
    /// Called after:
    /// - Dataset splits are created
    /// - Node property steps are executed
    /// - Feature properties are validated
    ///
    /// Java: `abstract RESULT execute(Map<DatasetSplits, PipelineGraphFilter> dataSplits)`
    fn execute(
        &mut self,
        data_splits: &HashMap<DatasetSplits, PipelineGraphFilter>,
    ) -> Result<RESULT, PipelineExecutorError>;

    /// Get relationship types available for node property steps.
    ///
    /// This determines which relationship types can be used by algorithms
    /// during feature extraction.
    ///
    /// Java: `abstract Set<RelationshipType> getAvailableRelTypesForNodePropertySteps()`
    fn get_available_rel_types_for_node_property_steps(&self) -> HashSet<String>;

    /// Additional graph store cleanup after execution.
    ///
    /// Override to clean up temporary properties or state after pipeline runs.
    /// Default implementation does nothing.
    ///
    /// Java: `protected void additionalGraphStoreCleanup(Map<DatasetSplits, PipelineGraphFilter> datasets)`
    fn additional_graph_store_cleanup(
        &mut self,
        _datasets: &HashMap<DatasetSplits, PipelineGraphFilter>,
    ) -> Result<(), PipelineExecutorError> {
        Ok(())
    }

    /// Execute the complete pipeline (template method).
    ///
    /// This implements the standard pipeline execution flow:
    /// 1. Generate dataset split filters
    /// 2. Validate pipeline against graph
    /// 3. Create node property step executor
    /// 4. Validate step context configs
    /// 5. Split datasets
    /// 6. Execute node property steps
    /// 7. Validate feature properties
    /// 8. Execute algorithm (train/test/predict)
    /// 9. Cleanup intermediate properties
    ///
    /// Java: `@Override public RESULT compute()`
    fn compute(&mut self) -> Result<RESULT, PipelineExecutorError> {
        // 1. Generate dataset split filters
        let data_split_graph_filters = self.generate_dataset_split_graph_filters();
        let feature_input_graph_filter = data_split_graph_filters
            .get(&DatasetSplits::FeatureInput)
            .ok_or_else(|| {
                PipelineExecutorError::MissingDatasetSplit("FEATURE_INPUT".to_string())
            })?;

        // 2. Validate pipeline before execution
        // featureInput nodeLabels contain source&target nodeLabel used in training/testing plus contextNodeLabels
        self.pipeline()
            .validate_before_execution(self.graph_store(), &feature_input_graph_filter.node_labels)
            .map_err(|e| PipelineExecutorError::PipelineValidationFailed(Box::new(e)))?;

        // 3. Create node property step executor
        let mut node_property_step_executor = NodePropertyStepExecutor::new(
            feature_input_graph_filter.node_labels.clone(),
            feature_input_graph_filter.relationship_types.clone(),
            self.get_available_rel_types_for_node_property_steps(),
            self.concurrency(),
        );

        // 4. Validate node property steps context configs
        node_property_step_executor
            .validate_node_property_steps_context_configs(
                self.graph_store(),
                self.pipeline().node_property_steps(),
            )
            .map_err(|e| PipelineExecutorError::StepValidationFailed(Box::new(e)))?;

        // 5. Split datasets
        self.split_datasets()?;

        // 6. Execute node property steps (scoped to avoid overlapping borrows on `self`)
        {
            let (pipeline, graph_store) = self.pipeline_and_graph_store_mut();
            node_property_step_executor
                .execute_node_property_steps(graph_store, pipeline.node_property_steps())
                .map_err(|e| PipelineExecutorError::StepExecutionFailed(Box::new(e)))?;
        }

        // 7. Validate feature properties
        self.pipeline()
            .validate_feature_properties(self.graph_store(), self.node_labels())
            .map_err(|e| PipelineExecutorError::FeatureValidationFailed(Box::new(e)))?;

        // 8. Execute algorithm
        let result: Result<RESULT, PipelineExecutorError> = self.execute(&data_split_graph_filters);

        // 9. Cleanup (always runs, even if error occurred)
        let cleanup_result = (|| -> Result<(), PipelineExecutorError> {
            {
                let (pipeline, graph_store) = self.pipeline_and_graph_store_mut();
                node_property_step_executor
                    .cleanup_intermediate_properties(graph_store, pipeline.node_property_steps())
                    .map_err(|e| PipelineExecutorError::CleanupFailed(Box::new(e)))?;
            }

            self.additional_graph_store_cleanup(&data_split_graph_filters)?;

            Ok(())
        })();

        // Return result, or cleanup error if result was Ok
        match (result, cleanup_result) {
            (Ok(value), Ok(())) => Ok(value),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), _) => Err(e),
        }
    }
}

/// Concrete implementation of PipelineExecutor for training pipelines.
///
/// This provides a basic implementation that can be used for training ML models.
/// It implements the required methods with sensible defaults for training scenarios.
pub struct TrainingPipelineExecutor<PIPELINE: Pipeline> {
    pipeline: PIPELINE,
    graph_store: Arc<DefaultGraphStore>,
    schema_before_steps: GraphSchema,
    node_labels: Vec<String>,
    relationship_types: Vec<String>,
    concurrency: usize,
}

impl<PIPELINE: Pipeline> TrainingPipelineExecutor<PIPELINE> {
    /// Create a new training pipeline executor.
    pub fn new(
        pipeline: PIPELINE,
        graph_store: Arc<DefaultGraphStore>,
        node_labels: Vec<String>,
        relationship_types: Vec<String>,
        concurrency: usize,
    ) -> Self {
        let schema_before_steps = graph_store.schema().clone();
        Self {
            pipeline,
            graph_store,
            schema_before_steps,
            node_labels,
            relationship_types,
            concurrency,
        }
    }
}

impl<PIPELINE: Pipeline> PipelineExecutor<PIPELINE, ()> for TrainingPipelineExecutor<PIPELINE> {
    fn pipeline(&self) -> &PIPELINE {
        &self.pipeline
    }

    fn pipeline_and_graph_store_mut(&mut self) -> (&PIPELINE, &mut Arc<DefaultGraphStore>) {
        (&self.pipeline, &mut self.graph_store)
    }

    fn graph_store_mut(&mut self) -> &mut Arc<DefaultGraphStore> {
        &mut self.graph_store
    }

    fn graph_store(&self) -> &Arc<DefaultGraphStore> {
        &self.graph_store
    }

    fn schema_before_steps(&self) -> &GraphSchema {
        &self.schema_before_steps
    }

    fn node_labels(&self) -> &[String] {
        &self.node_labels
    }

    fn relationship_types(&self) -> &[String] {
        &self.relationship_types
    }

    fn concurrency(&self) -> usize {
        self.concurrency
    }

    fn generate_dataset_split_graph_filters(&self) -> HashMap<DatasetSplits, PipelineGraphFilter> {
        // Basic implementation: create filters for all nodes
        // In a real implementation, this would create train/test/validation splits
        let all_nodes_filter = PipelineGraphFilter {
            node_labels: self.node_labels.clone(),
            relationship_types: self.relationship_types.clone(),
        };

        let mut filters = HashMap::new();
        filters.insert(DatasetSplits::Train, all_nodes_filter.clone());
        filters.insert(DatasetSplits::Test, all_nodes_filter.clone());
        filters.insert(DatasetSplits::TestComplement, all_nodes_filter.clone());
        filters.insert(DatasetSplits::FeatureInput, all_nodes_filter);
        filters
    }

    fn split_datasets(&mut self) -> Result<(), PipelineExecutorError> {
        // Basic implementation: no actual splitting needed for this simple case
        // In a real implementation, this would create actual train/test splits
        Ok(())
    }

    fn execute(
        &mut self,
        _data_splits: &HashMap<DatasetSplits, PipelineGraphFilter>,
    ) -> Result<(), PipelineExecutorError> {
        // Basic implementation: just validate that we can execute
        // In a real implementation, this would run the actual training algorithm
        Ok(())
    }

    fn get_available_rel_types_for_node_property_steps(&self) -> HashSet<String> {
        self.relationship_types.iter().cloned().collect()
    }
}

/// Errors that can occur during pipeline execution.
#[derive(Debug)]
pub enum PipelineExecutorError {
    /// Dataset split is missing from the filter map.
    MissingDatasetSplit(String),

    /// Pipeline validation failed before execution.
    PipelineValidationFailed(Box<dyn StdError>),

    /// Step validation failed (context configs invalid).
    StepValidationFailed(Box<dyn StdError>),

    /// Dataset splitting failed.
    DatasetSplitFailed(String),

    /// Step execution failed.
    StepExecutionFailed(Box<dyn StdError>),

    /// Feature validation failed after steps.
    FeatureValidationFailed(Box<dyn StdError>),

    /// Algorithm execution failed.
    ExecutionFailed(String),

    /// Cleanup failed.
    CleanupFailed(Box<dyn StdError>),
}

impl std::fmt::Display for PipelineExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingDatasetSplit(split) => {
                write!(f, "Missing dataset split `{}`", split)
            }
            Self::PipelineValidationFailed(e) => {
                write!(f, "Pipeline validation failed: {}", e)
            }
            Self::StepValidationFailed(e) => {
                write!(f, "Node property step context validation failed: {}", e)
            }
            Self::DatasetSplitFailed(msg) => {
                write!(f, "Failed to split datasets: {}", msg)
            }
            Self::StepExecutionFailed(e) => {
                write!(f, "Failed to execute node property steps: {}", e)
            }
            Self::FeatureValidationFailed(e) => {
                write!(f, "Feature property validation failed: {}", e)
            }
            Self::ExecutionFailed(msg) => {
                write!(f, "Pipeline execution failed: {}", msg)
            }
            Self::CleanupFailed(e) => {
                write!(f, "Cleanup of intermediate properties failed: {}", e)
            }
        }
    }
}

impl StdError for PipelineExecutorError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::PipelineValidationFailed(e)
            | Self::StepValidationFailed(e)
            | Self::StepExecutionFailed(e)
            | Self::FeatureValidationFailed(e)
            | Self::CleanupFailed(e) => Some(&**e),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::pipeline::{FeatureStep, ExecutableNodePropertyStep, PipelineValidationError};

    #[test]
    fn test_dataset_splits_enum() {
        // Test enum variants exist
        let train = DatasetSplits::Train;
        let test = DatasetSplits::Test;
        let _test_complement = DatasetSplits::TestComplement;
        let _feature_input = DatasetSplits::FeatureInput;

        // Test equality
        assert_eq!(train, DatasetSplits::Train);
        assert_ne!(train, test);

        // Test hash (can use in HashMap)
        let mut map = HashMap::new();
        map.insert(train, "train data");
        map.insert(test, "test data");
        assert_eq!(map.get(&DatasetSplits::Train), Some(&"train data"));
    }

    #[test]
    fn test_error_display() {
        let error = PipelineExecutorError::MissingDatasetSplit("TRAIN".to_string());
        let display = format!("{}", error);
        assert!(display.contains("TRAIN"));
        assert!(display.contains("Missing dataset split"));

        let error = PipelineExecutorError::ExecutionFailed("algorithm error".to_string());
        let display = format!("{}", error);
        assert!(display.contains("algorithm error"));
        assert!(display.contains("Pipeline execution failed"));
    }

    #[test]
    fn test_training_pipeline_executor_creation() {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;
        use std::collections::HashMap;

        // Mock FeatureStep for testing
        #[derive(Clone)]
        struct MockFeatureStep;
        impl FeatureStep for MockFeatureStep {
            fn name(&self) -> &str {
                "mock"
            }
            fn input_node_properties(&self) -> &[String] {
                &[]
            }
            fn configuration(&self) -> &std::collections::HashMap<String, serde_json::Value> {
                use std::sync::OnceLock;
                static CONFIG: OnceLock<std::collections::HashMap<String, serde_json::Value>> =
                    OnceLock::new();
                CONFIG.get_or_init(std::collections::HashMap::new)
            }
            fn to_map(&self) -> std::collections::HashMap<String, serde_json::Value> {
                std::collections::HashMap::new()
            }
        }

        // Create a simple test pipeline
        #[derive(Debug)]
        struct TestPipeline;
        impl Pipeline for TestPipeline {
            type FeatureStep = MockFeatureStep;
            fn node_property_steps(
                &self,
            ) -> &[Box<dyn ExecutableNodePropertyStep>]
            {
                &[]
            }
            fn feature_steps(&self) -> &[Self::FeatureStep] {
                &[]
            }
            fn specific_validate_before_execution(
                &self,
                _graph_store: &DefaultGraphStore,
            ) -> Result<(), PipelineValidationError>
            {
                Ok(())
            }
            fn to_map(&self) -> HashMap<String, serde_json::Value> {
                HashMap::new()
            }
        }

        // Create a test graph store
        let config = RandomGraphConfig::default().with_seed(42);
        let graph_store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        // Create executor
        let pipeline = TestPipeline;
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let concurrency = 4;

        let executor = TrainingPipelineExecutor::new(
            pipeline,
            graph_store,
            node_labels,
            relationship_types,
            concurrency,
        );

        // Test basic properties
        assert_eq!(executor.concurrency(), 4);
        assert_eq!(executor.node_labels(), &["Node"]);
        assert_eq!(executor.relationship_types(), &["REL"]);
    }

    #[test]
    fn test_generate_dataset_split_filters() {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;
        use std::collections::HashMap;

        // Mock FeatureStep for testing
        #[derive(Clone)]
        struct MockFeatureStep;
        impl FeatureStep for MockFeatureStep {
            fn name(&self) -> &str {
                "mock"
            }
            fn input_node_properties(&self) -> &[String] {
                &[]
            }
            fn configuration(&self) -> &std::collections::HashMap<String, serde_json::Value> {
                use std::sync::OnceLock;
                static CONFIG: OnceLock<std::collections::HashMap<String, serde_json::Value>> =
                    OnceLock::new();
                CONFIG.get_or_init(std::collections::HashMap::new)
            }
            fn to_map(&self) -> std::collections::HashMap<String, serde_json::Value> {
                std::collections::HashMap::new()
            }
        }

        // Create a simple test pipeline
        #[derive(Debug)]
        struct TestPipeline;
        impl Pipeline for TestPipeline {
            type FeatureStep = MockFeatureStep;
            fn node_property_steps(
                &self,
            ) -> &[Box<dyn ExecutableNodePropertyStep>]
            {
                &[]
            }
            fn feature_steps(&self) -> &[Self::FeatureStep] {
                &[]
            }
            fn specific_validate_before_execution(
                &self,
                _graph_store: &DefaultGraphStore,
            ) -> Result<(), PipelineValidationError>
            {
                Ok(())
            }
            fn to_map(&self) -> HashMap<String, serde_json::Value> {
                HashMap::new()
            }
        }

        // Create a test graph store
        let config = RandomGraphConfig::default().with_seed(42);
        let graph_store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        // Create executor
        let pipeline = TestPipeline;
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let concurrency = 4;

        let executor = TrainingPipelineExecutor::new(
            pipeline,
            graph_store,
            node_labels,
            relationship_types,
            concurrency,
        );

        // Generate filters
        let filters = executor.generate_dataset_split_graph_filters();

        // Should have all four dataset splits
        assert!(filters.contains_key(&DatasetSplits::Train));
        assert!(filters.contains_key(&DatasetSplits::Test));
        assert!(filters.contains_key(&DatasetSplits::TestComplement));
        assert!(filters.contains_key(&DatasetSplits::FeatureInput));

        // All filters should have the same node labels and relationship types
        for filter in filters.values() {
            assert_eq!(filter.node_labels, vec!["Node".to_string()]);
            assert_eq!(filter.relationship_types, vec!["REL".to_string()]);
        }
    }

    #[test]
    fn test_feature_input_filter_is_present_and_correct() {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;
        use std::collections::HashMap;

        #[derive(Clone)]
        struct MockFeatureStep;
        impl FeatureStep for MockFeatureStep {
            fn name(&self) -> &str {
                "mock"
            }
            fn input_node_properties(&self) -> &[String] {
                &[]
            }
            fn configuration(&self) -> &std::collections::HashMap<String, serde_json::Value> {
                use std::sync::OnceLock;
                static CONFIG: OnceLock<std::collections::HashMap<String, serde_json::Value>> =
                    OnceLock::new();
                CONFIG.get_or_init(std::collections::HashMap::new)
            }
            fn to_map(&self) -> std::collections::HashMap<String, serde_json::Value> {
                std::collections::HashMap::new()
            }
        }

        #[derive(Debug)]
        struct TestPipeline;
        impl Pipeline for TestPipeline {
            type FeatureStep = MockFeatureStep;
            fn node_property_steps(
                &self,
            ) -> &[Box<dyn ExecutableNodePropertyStep>]
            {
                &[]
            }
            fn feature_steps(&self) -> &[Self::FeatureStep] {
                &[]
            }
            fn specific_validate_before_execution(
                &self,
                _graph_store: &DefaultGraphStore,
            ) -> Result<(), PipelineValidationError>
            {
                Ok(())
            }
            fn to_map(&self) -> HashMap<String, serde_json::Value> {
                HashMap::new()
            }
        }

        let config = RandomGraphConfig::default().with_seed(7);
        let graph_store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let pipeline = TestPipeline;
        let node_labels = vec!["Person".to_string()];
        let relationship_types = vec!["KNOWS".to_string()];

        let executor = TrainingPipelineExecutor::new(
            pipeline,
            graph_store,
            node_labels.clone(),
            relationship_types.clone(),
            1,
        );

        let filters = executor.generate_dataset_split_graph_filters();

        let feature_input = filters
            .get(&DatasetSplits::FeatureInput)
            .expect("FEATURE_INPUT split should exist");

        assert_eq!(feature_input.node_labels, node_labels);
        assert_eq!(feature_input.relationship_types, relationship_types);

        // Verify TRAIN/TEST keys are present and distinct
        assert!(filters.contains_key(&DatasetSplits::Train));
        assert!(filters.contains_key(&DatasetSplits::Test));
        assert_ne!(DatasetSplits::Train, DatasetSplits::Test);
    }
}
