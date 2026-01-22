// Translated from Neo4j Graph Data Science:
// https://github.com/neo4j/graph-data-science
// pipeline/src/main/java/org/neo4j/gds/ml/pipeline/PredictPipelineExecutor.java

use std::error::Error as StdError;
use std::sync::Arc;

use crate::projection::eval::pipeline::{NodePropertyStepExecutor, Pipeline, PipelineGraphFilter};
use crate::types::graph_store::{DefaultGraphStore, GraphStore};

/// Abstract pipeline executor for prediction (no training/test splits).
///
/// This is a simplified executor for running trained models on new data.
/// Unlike PipelineExecutor, this doesn't split datasets or train models -
/// it just runs node property steps and executes prediction.
///
/// # Direct Integration Approach
///
/// Takes Arc<DefaultGraphStore> directly instead of wrapping in ExecutionContext.
///
/// # Type Parameters
///
/// * `PIPELINE` - The pipeline type (e.g., NodeClassificationPipeline)
/// * `RESULT` - The prediction result type
///
/// # Java Source (PredictPipelineExecutor.java)
/// ```java
/// public abstract class PredictPipelineExecutor<
///     PIPELINE_CONFIG extends AlgoBaseConfig & GraphNameConfig,
///     PIPELINE extends Pipeline<?>,
///     RESULT
/// > extends Algorithm<RESULT> {
///     protected final PIPELINE pipeline;
///     protected final PIPELINE_CONFIG config;
///     protected final ExecutionContext executionContext;
///     protected final GraphStore graphStore;
///
///     protected abstract RESULT execute();
///     protected abstract PipelineGraphFilter nodePropertyStepFilter();
///
///     @Override
///     public RESULT compute() { /* template method */ }
/// }
/// ```
pub trait PredictPipelineExecutor<PIPELINE: Pipeline, RESULT> {
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

    /// Access the node labels for this execution.
    fn node_labels(&self) -> &[String];

    /// Access the relationship types for this execution.
    fn relationship_types(&self) -> &[String];

    /// Access the concurrency setting.
    fn concurrency(&self) -> usize;

    /// Execute the prediction algorithm.
    ///
    /// Called after node property steps are executed and features are validated.
    ///
    /// Java: `abstract RESULT execute()`
    fn execute(&mut self) -> Result<RESULT, PredictPipelineExecutorError>;

    /// Get the graph filter for node property steps.
    ///
    /// This defines which nodes/relationships to use for feature extraction.
    /// In prediction, this typically includes all nodes to predict on plus
    /// any context nodes needed for algorithms.
    ///
    /// Java: `abstract PipelineGraphFilter nodePropertyStepFilter()`
    fn node_property_step_filter(&self) -> PipelineGraphFilter;

    /// Execute the complete prediction pipeline (template method).
    ///
    /// This implements the standard prediction flow:
    /// 1. Get node property step filter
    /// 2. Validate pipeline against graph
    /// 3. Create node property step executor
    /// 4. Execute node property steps
    /// 5. Validate feature properties
    /// 6. Execute prediction
    /// 7. Cleanup intermediate properties
    ///
    /// Java: `@Override public RESULT compute()`
    fn compute(&mut self) -> Result<RESULT, PredictPipelineExecutorError> {
        // 1. Get node property step filter
        let node_property_step_filter = self.node_property_step_filter();

        // 2. Validate pipeline before execution
        // featureInput nodeLabels contain nodes to predict on plus contextNodeLabels
        self.pipeline()
            .validate_before_execution(self.graph_store(), &node_property_step_filter.node_labels)
            .map_err(|e| PredictPipelineExecutorError::PipelineValidationFailed(Box::new(e)))?;

        // 3. Create node property step executor
        // For prediction, all relationship types in the graph are available
        let all_relationship_types: std::collections::HashSet<String> = self
            .graph_store()
            .relationship_types()
            .iter()
            .map(|rt| rt.name().to_string())
            .collect();

        let mut node_property_step_executor = NodePropertyStepExecutor::new(
            node_property_step_filter.node_labels.clone(),
            node_property_step_filter.relationship_types.clone(),
            all_relationship_types,
            self.concurrency(),
        );

        // 4. Execute node property steps (scoped to avoid overlapping borrows on `self`)
        {
            let (pipeline, graph_store) = self.pipeline_and_graph_store_mut();
            node_property_step_executor
                .execute_node_property_steps(graph_store, pipeline.node_property_steps())
                .map_err(|e| PredictPipelineExecutorError::StepExecutionFailed(Box::new(e)))?;
        }

        // 5. Validate feature properties
        self.pipeline()
            .validate_feature_properties(self.graph_store(), &node_property_step_filter.node_labels)
            .map_err(|e| PredictPipelineExecutorError::FeatureValidationFailed(Box::new(e)))?;

        // 6. Execute prediction
        let result = self.execute();

        // 7. Cleanup (always runs, even if error occurred)
        let cleanup_error = {
            let (pipeline, graph_store) = self.pipeline_and_graph_store_mut();
            node_property_step_executor
                .cleanup_intermediate_properties(graph_store, pipeline.node_property_steps())
                .err()
        };

        // Return result, or cleanup error if result was Ok
        match (result, cleanup_error) {
            (Ok(value), None) => Ok(value),
            (Ok(_), Some(e)) => Err(PredictPipelineExecutorError::CleanupFailed(Box::new(e))),
            (Err(e), _) => Err(e),
        }
    }
}

/// Errors that can occur during prediction pipeline execution.
#[derive(Debug)]
pub enum PredictPipelineExecutorError {
    /// Pipeline validation failed before execution.
    PipelineValidationFailed(Box<dyn StdError>),

    /// Step execution failed.
    StepExecutionFailed(Box<dyn StdError>),

    /// Feature validation failed after steps.
    FeatureValidationFailed(Box<dyn StdError>),

    /// Prediction execution failed.
    ExecutionFailed(String),

    /// Cleanup failed.
    CleanupFailed(Box<dyn StdError>),
}

impl std::fmt::Display for PredictPipelineExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PipelineValidationFailed(e) => {
                write!(f, "Pipeline validation failed: {}", e)
            }
            Self::StepExecutionFailed(e) => {
                write!(f, "Failed to execute node property steps: {}", e)
            }
            Self::FeatureValidationFailed(e) => {
                write!(f, "Feature property validation failed: {}", e)
            }
            Self::ExecutionFailed(msg) => {
                write!(f, "Prediction execution failed: {}", msg)
            }
            Self::CleanupFailed(e) => {
                write!(f, "Cleanup of intermediate properties failed: {}", e)
            }
        }
    }
}

impl StdError for PredictPipelineExecutorError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::PipelineValidationFailed(e)
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

    use std::collections::HashMap;

    use crate::config::GraphStoreConfig;
    use crate::projection::NodeLabel;
    use crate::projection::RelationshipType;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph::SimpleIdMap;
    use crate::types::schema::Direction;
    use crate::types::schema::MutableGraphSchema;

    use crate::projection::eval::pipeline::node_pipeline::NodeFeatureStep;
    use crate::projection::eval::pipeline::node_property_step::DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE;
    use crate::projection::eval::pipeline::ExecutableNodePropertyStep;
    use crate::projection::eval::pipeline::NodePropertyStep;
    use crate::projection::eval::pipeline::PipelineValidationError;
    use crate::projection::eval::pipeline::MUTATE_PROPERTY_KEY;

    use crate::types::graph_store::GraphName;
    use crate::types::graph_store::{Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation};

    #[test]
    fn test_error_display() {
        let error = PredictPipelineExecutorError::ExecutionFailed("prediction error".to_string());
        let display = format!("{}", error);
        assert!(display.contains("prediction error"));
        assert!(display.contains("Prediction execution failed"));

        let error = PredictPipelineExecutorError::CleanupFailed(Box::new(std::fmt::Error));
        let display = format!("{}", error);
        assert!(display.contains("Cleanup of intermediate properties failed"));

        let error = PredictPipelineExecutorError::StepExecutionFailed(Box::new(std::fmt::Error));
        let display = format!("{}", error);
        assert!(display.contains("Failed to execute node property steps"));

        let error =
            PredictPipelineExecutorError::FeatureValidationFailed(Box::new(std::fmt::Error));
        let display = format!("{}", error);
        assert!(display.contains("Feature property validation failed"));

        let error =
            PredictPipelineExecutorError::PipelineValidationFailed(Box::new(std::fmt::Error));
        let display = format!("{}", error);
        assert!(display.contains("Pipeline validation failed"));
    }

    #[test]
    fn test_predict_pipeline_executor_compute_smoke_runs_node_property_step() {
        fn store_from_outgoing(outgoing: Vec<Vec<i64>>) -> DefaultGraphStore {
            let node_count = outgoing.len();

            let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];
            for (source, targets) in outgoing.iter().enumerate() {
                for &target in targets {
                    if target >= 0 {
                        let t = target as usize;
                        if t < node_count {
                            incoming[t].push(source as i64);
                        }
                    }
                }
            }

            let rel_type = RelationshipType::of("REL");

            let mut schema_builder = MutableGraphSchema::empty();
            schema_builder
                .relationship_schema_mut()
                .add_relationship_type(rel_type.clone(), Direction::Directed);
            let schema = schema_builder.build();

            let mut relationship_topologies = HashMap::new();
            relationship_topologies.insert(
                rel_type,
                RelationshipTopology::new(outgoing, Some(incoming)),
            );

            let original_ids: Vec<i64> = (0..node_count as i64).collect();
            let id_map = SimpleIdMap::from_original_ids(original_ids);

            DefaultGraphStore::new(
                GraphStoreConfig::default(),
                GraphName::new("g"),
                DatabaseInfo::new(
                    DatabaseId::new("db"),
                    DatabaseLocation::remote("localhost", 7687, None, None),
                ),
                schema,
                Capabilities::default(),
                id_map,
                relationship_topologies,
            )
        }

        struct SmokePipeline {
            node_property_steps: Vec<Box<dyn ExecutableNodePropertyStep>>,
            feature_steps: Vec<NodeFeatureStep>,
        }

        impl Pipeline for SmokePipeline {
            type FeatureStep = NodeFeatureStep;

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
                Ok(())
            }

            fn to_map(&self) -> HashMap<String, serde_json::Value> {
                HashMap::new()
            }
        }

        struct SmokeExecutor {
            pipeline: SmokePipeline,
            graph_store: Arc<DefaultGraphStore>,
            node_labels: Vec<String>,
            relationship_types: Vec<String>,
            expected_property: String,
        }

        impl PredictPipelineExecutor<SmokePipeline, ()> for SmokeExecutor {
            fn pipeline(&self) -> &SmokePipeline {
                &self.pipeline
            }

            fn pipeline_and_graph_store_mut(
                &mut self,
            ) -> (&SmokePipeline, &mut Arc<DefaultGraphStore>) {
                (&self.pipeline, &mut self.graph_store)
            }

            fn graph_store_mut(&mut self) -> &mut Arc<DefaultGraphStore> {
                &mut self.graph_store
            }

            fn graph_store(&self) -> &Arc<DefaultGraphStore> {
                &self.graph_store
            }

            fn node_labels(&self) -> &[String] {
                &self.node_labels
            }

            fn relationship_types(&self) -> &[String] {
                &self.relationship_types
            }

            fn concurrency(&self) -> usize {
                1
            }

            fn execute(&mut self) -> Result<(), PredictPipelineExecutorError> {
                // This runs before cleanup; prove the step actually mutated the store.
                if !self
                    .graph_store()
                    .has_node_property(&self.expected_property)
                {
                    return Err(PredictPipelineExecutorError::ExecutionFailed(format!(
                        "expected node property '{}' to exist before cleanup",
                        self.expected_property
                    )));
                }
                Ok(())
            }

            fn node_property_step_filter(&self) -> PipelineGraphFilter {
                PipelineGraphFilter::new(
                    self.node_labels.clone(),
                    Some(self.relationship_types.clone()),
                )
            }
        }

        // Build a tiny graph.
        let mut store = store_from_outgoing(vec![vec![1], vec![0]]);
        store
            .add_node_label(NodeLabel::of("N"))
            .expect("add node label");

        let graph_store = Arc::new(store);

        // Node-property step will create `feat` and feature step will require it.
        let mut config = HashMap::new();
        config.insert(
            MUTATE_PROPERTY_KEY.to_string(),
            serde_json::Value::String("feat".to_string()),
        );
        config.insert(
            "value".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(3.0).unwrap()),
        );

        let step = NodePropertyStep::new(DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE.to_string(), config);
        let pipeline = SmokePipeline {
            node_property_steps: vec![Box::new(step)],
            feature_steps: vec![NodeFeatureStep::of("feat")],
        };

        let mut executor = SmokeExecutor {
            pipeline,
            graph_store,
            node_labels: vec!["N".to_string()],
            relationship_types: vec!["REL".to_string()],
            expected_property: "feat".to_string(),
        };

        executor.compute().expect("pipeline compute should succeed");

        // Cleanup should have removed the intermediate property.
        assert!(!executor.graph_store().has_node_property("feat"));
    }

    #[test]
    fn test_predict_executor_propagates_step_execution_error() {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;

        #[derive(Clone)]
        struct FailingStep {
            config: HashMap<String, serde_json::Value>,
        }

        impl ExecutableNodePropertyStep for FailingStep {
            fn execute(
                &self,
                _graph_store: &mut DefaultGraphStore,
                _node_labels: &[String],
                _relationship_types: &[String],
                _concurrency: usize,
            ) -> Result<(), Box<dyn StdError + Send + Sync>> {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "boom",
                )))
            }

            fn config(&self) -> &HashMap<String, serde_json::Value> {
                &self.config
            }

            fn proc_name(&self) -> &str {
                "gds.pagerank.mutate"
            }

            fn mutate_node_property(&self) -> &str {
                "pagerank"
            }
        }

        struct TestPipeline {
            steps: Vec<Box<dyn ExecutableNodePropertyStep>>,
        }

        impl Pipeline for TestPipeline {
            type FeatureStep = NodeFeatureStep;

            fn node_property_steps(&self) -> &[Box<dyn ExecutableNodePropertyStep>] {
                &self.steps
            }

            fn feature_steps(&self) -> &[Self::FeatureStep] {
                &[]
            }

            fn specific_validate_before_execution(
                &self,
                _graph_store: &DefaultGraphStore,
            ) -> Result<(), PipelineValidationError> {
                Ok(())
            }

            fn to_map(&self) -> HashMap<String, serde_json::Value> {
                HashMap::new()
            }
        }

        struct TestPredictExecutor {
            pipeline: TestPipeline,
            graph_store: Arc<DefaultGraphStore>,
            node_labels: Vec<String>,
            relationship_types: Vec<String>,
        }

        impl PredictPipelineExecutor<TestPipeline, ()> for TestPredictExecutor {
            fn pipeline(&self) -> &TestPipeline {
                &self.pipeline
            }

            fn pipeline_and_graph_store_mut(
                &mut self,
            ) -> (&TestPipeline, &mut Arc<DefaultGraphStore>) {
                (&self.pipeline, &mut self.graph_store)
            }

            fn graph_store_mut(&mut self) -> &mut Arc<DefaultGraphStore> {
                &mut self.graph_store
            }

            fn graph_store(&self) -> &Arc<DefaultGraphStore> {
                &self.graph_store
            }

            fn node_labels(&self) -> &[String] {
                &self.node_labels
            }

            fn relationship_types(&self) -> &[String] {
                &self.relationship_types
            }

            fn concurrency(&self) -> usize {
                1
            }

            fn execute(&mut self) -> Result<(), PredictPipelineExecutorError> {
                Ok(())
            }

            fn node_property_step_filter(&self) -> PipelineGraphFilter {
                PipelineGraphFilter::new(
                    self.node_labels.clone(),
                    Some(self.relationship_types.clone()),
                )
            }
        }

        let config = RandomGraphConfig::default().with_seed(9);
        let graph_store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let mut step_config = HashMap::new();
        step_config.insert(
            MUTATE_PROPERTY_KEY.to_string(),
            serde_json::json!("pagerank"),
        );

        let steps: Vec<Box<dyn ExecutableNodePropertyStep>> = vec![Box::new(FailingStep {
            config: step_config,
        })];

        let pipeline = TestPipeline { steps };

        let mut executor = TestPredictExecutor {
            pipeline,
            graph_store,
            node_labels: vec!["Node".to_string()],
            relationship_types: vec!["REL".to_string()],
        };

        let result = executor.compute();
        assert!(matches!(
            result,
            Err(PredictPipelineExecutorError::StepExecutionFailed(_))
        ));
    }

    #[test]
    fn test_node_property_step_filter_labels_used_for_validation() {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;

        #[derive(Debug)]
        struct FilterPipeline {
            expected_labels: Vec<String>,
            seen_labels: Arc<std::sync::Mutex<Vec<String>>>,
        }

        impl Pipeline for FilterPipeline {
            type FeatureStep = NodeFeatureStep;

            fn node_property_steps(&self) -> &[Box<dyn ExecutableNodePropertyStep>] {
                &[]
            }

            fn feature_steps(&self) -> &[Self::FeatureStep] {
                &[]
            }

            fn validate_before_execution(
                &self,
                _graph_store: &DefaultGraphStore,
                node_labels: &[String],
            ) -> Result<(), PipelineValidationError> {
                let mut guard = self.seen_labels.lock().unwrap();
                *guard = node_labels.to_vec();
                if node_labels != self.expected_labels.as_slice() {
                    return Err(PipelineValidationError::Other {
                        message: "unexpected labels".to_string(),
                    });
                }
                Ok(())
            }

            fn specific_validate_before_execution(
                &self,
                _graph_store: &DefaultGraphStore,
            ) -> Result<(), PipelineValidationError> {
                Ok(())
            }

            fn to_map(&self) -> HashMap<String, serde_json::Value> {
                HashMap::new()
            }
        }

        struct FilterExecutor {
            pipeline: FilterPipeline,
            graph_store: Arc<DefaultGraphStore>,
        }

        impl PredictPipelineExecutor<FilterPipeline, ()> for FilterExecutor {
            fn pipeline(&self) -> &FilterPipeline {
                &self.pipeline
            }

            fn pipeline_and_graph_store_mut(
                &mut self,
            ) -> (&FilterPipeline, &mut Arc<DefaultGraphStore>) {
                (&self.pipeline, &mut self.graph_store)
            }

            fn graph_store_mut(&mut self) -> &mut Arc<DefaultGraphStore> {
                &mut self.graph_store
            }

            fn graph_store(&self) -> &Arc<DefaultGraphStore> {
                &self.graph_store
            }

            fn node_labels(&self) -> &[String] {
                &[]
            }

            fn relationship_types(&self) -> &[String] {
                &[]
            }

            fn concurrency(&self) -> usize {
                1
            }

            fn execute(&mut self) -> Result<(), PredictPipelineExecutorError> {
                Ok(())
            }

            fn node_property_step_filter(&self) -> PipelineGraphFilter {
                PipelineGraphFilter::new(
                    self.pipeline.expected_labels.clone(),
                    Some(vec!["REL".to_string()]),
                )
            }
        }

        let config = RandomGraphConfig::default().with_seed(11);
        let graph_store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let expected_labels = vec!["Node".to_string()];
        let seen_labels = Arc::new(std::sync::Mutex::new(Vec::new()));

        let pipeline = FilterPipeline {
            expected_labels: expected_labels.clone(),
            seen_labels: Arc::clone(&seen_labels),
        };

        let mut executor = FilterExecutor {
            pipeline,
            graph_store,
        };

        executor.compute().expect("predict compute should succeed");

        let guard = seen_labels.lock().unwrap();
        assert_eq!(&*guard, &expected_labels);
    }
}
