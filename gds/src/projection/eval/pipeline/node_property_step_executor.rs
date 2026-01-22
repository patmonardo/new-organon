// Translated from Neo4j Graph Data Science:
// https://github.com/neo4j/graph-data-science
// pipeline/src/main/java/org/neo4j/gds/ml/pipeline/NodePropertyStepExecutor.java

use std::collections::HashSet;
use std::error::Error as StdError;
use std::sync::Arc;

use crate::projection::eval::pipeline::ExecutableNodePropertyStep;
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore;

/// Executor for running sequences of node property steps in ML pipelines.
///
/// This executor:
/// - Validates that context configurations are compatible with the graph
/// - Executes steps in sequence, updating the graph store
/// - Tracks progress and handles errors
/// - Cleans up intermediate properties after execution
///
/// Each step handles execution and mutation internally (ProcedureExecutor-backed).
#[derive(Debug)]
pub struct NodePropertyStepExecutor {
    node_labels: Vec<String>,
    relationship_types: Vec<String>,
    available_relationship_types_for_node_properties: HashSet<String>,
    concurrency: usize,
}

impl NodePropertyStepExecutor {
    /// Create a new executor for node property steps.
    ///
    /// # Arguments
    ///
    /// * `node_labels` - Node labels to use for pipeline execution
    /// * `relationship_types` - Relationship types to use for pipeline execution
    /// * `available_relationship_types_for_node_properties` - Relationship types available for feature input
    /// * `concurrency` - Number of threads to use for parallel execution
    pub fn new(
        node_labels: Vec<String>,
        relationship_types: Vec<String>,
        available_relationship_types_for_node_properties: HashSet<String>,
        concurrency: usize,
    ) -> Self {
        Self {
            node_labels,
            relationship_types,
            available_relationship_types_for_node_properties,
            concurrency,
        }
    }

    /// Validate that all step context configurations are compatible with the graph store.
    ///
    /// This checks that:
    /// - Context node labels exist in the graph
    /// - Context relationship types exist in the graph
    ///
    /// # Errors
    ///
    /// Returns an error if any context configuration references non-existent labels or types.
    pub fn validate_node_property_steps_context_configs(
        &self,
        graph_store: &DefaultGraphStore,
        steps: &[Box<dyn ExecutableNodePropertyStep>],
    ) -> Result<(), NodePropertyStepExecutorError> {
        for step in steps {
            // Validate context node labels
            let context_node_labels = step.context_node_labels();
            self.validate_node_labels(graph_store, context_node_labels, step.proc_name())?;

            // Validate context relationship types
            let context_rel_types = step.context_relationship_types();
            self.validate_relationship_types(graph_store, context_rel_types, step.proc_name())?;
        }

        Ok(())
    }

    /// Execute all node property steps in sequence.
    ///
    /// Each step:
    /// 1. Executes the algorithm via the step's execute method
    /// 2. Mutates the graph store with computed properties
    ///
    /// Note: In Java, steps determine their own feature input labels/types via
    /// featureInputNodeLabels() and featureInputRelationshipTypes() methods.
    /// In our direct integration, the step's execute() method handles this internally.
    ///
    /// # Errors
    ///
    /// Returns an error if any step execution fails.
    pub fn execute_node_property_steps(
        &mut self,
        graph_store: &mut Arc<DefaultGraphStore>,
        steps: &[Box<dyn ExecutableNodePropertyStep>],
    ) -> Result<(), NodePropertyStepExecutorError> {
        let graph_store = Arc::get_mut(graph_store)
            .ok_or_else(|| NodePropertyStepExecutorError::GraphStoreLocked)?;

        for (i, step) in steps.iter().enumerate() {
            // Resolve feature input labels/types using step context and pipeline defaults.
            let feature_input_node_labels = self.feature_input_node_labels(step.as_ref());
            let feature_input_relationship_types =
                self.feature_input_relationship_types(step.as_ref());

            step.execute(
                graph_store,
                &feature_input_node_labels,
                &feature_input_relationship_types,
                self.concurrency,
            )
            .map_err(|e| NodePropertyStepExecutorError::StepExecutionFailed {
                step_index: i,
                step_name: step.proc_name().to_string(),
                source: e,
            })?;
        }

        Ok(())
    }

    /// Clean up intermediate properties created during pipeline execution.
    ///
    /// This removes all mutated properties from the graph store, keeping only
    /// the final pipeline outputs.
    pub fn cleanup_intermediate_properties(
        &mut self,
        graph_store: &mut Arc<DefaultGraphStore>,
        steps: &[Box<dyn ExecutableNodePropertyStep>],
    ) -> Result<(), NodePropertyStepExecutorError> {
        let graph_store = Arc::get_mut(graph_store)
            .ok_or_else(|| NodePropertyStepExecutorError::GraphStoreLocked)?;

        for step in steps {
            let property_name = step.mutate_node_property();
            // DefaultGraphStore removes the property across label-indexes.
            let _ = graph_store.remove_node_property(property_name);
        }

        Ok(())
    }

    // Private validation helpers

    fn validate_node_labels(
        &self,
        graph_store: &DefaultGraphStore,
        labels: &[String],
        step_name: &str,
    ) -> Result<(), NodePropertyStepExecutorError> {
        for label in labels {
            let node_label = NodeLabel::of(label.clone());
            if !graph_store.has_node_label(&node_label) {
                return Err(NodePropertyStepExecutorError::InvalidNodeLabel {
                    label: label.clone(),
                    step_name: step_name.to_string(),
                });
            }
        }
        Ok(())
    }

    fn validate_relationship_types(
        &self,
        graph_store: &DefaultGraphStore,
        types: &[String],
        step_name: &str,
    ) -> Result<(), NodePropertyStepExecutorError> {
        for rel_type in types {
            let relationship_type = RelationshipType::of(rel_type.clone());
            if !graph_store.has_relationship_type(&relationship_type) {
                return Err(NodePropertyStepExecutorError::InvalidRelationshipType {
                    rel_type: rel_type.clone(),
                    step_name: step_name.to_string(),
                });
            }
        }
        Ok(())
    }

    fn feature_input_node_labels(&self, step: &dyn ExecutableNodePropertyStep) -> Vec<String> {
        if step.context_node_labels().is_empty() {
            return self.node_labels.clone();
        }

        let mut labels = self.node_labels.clone();
        for label in step.context_node_labels() {
            if !labels.iter().any(|existing| existing == label) {
                labels.push(label.clone());
            }
        }

        labels
    }

    fn feature_input_relationship_types(
        &self,
        step: &dyn ExecutableNodePropertyStep,
    ) -> Vec<String> {
        let base_types: Vec<String> = if step.context_relationship_types().is_empty() {
            self.relationship_types.clone()
        } else {
            step.context_relationship_types().to_vec()
        };

        if self
            .available_relationship_types_for_node_properties
            .is_empty()
        {
            return base_types;
        }

        base_types
            .into_iter()
            .filter(|rel_type| {
                self.available_relationship_types_for_node_properties
                    .contains(rel_type)
            })
            .collect()
    }
}

// Note: Memory estimation and task creation are omitted in direct integration.
// Java uses these for cost-based optimization and progress tracking, but they
// require the full ProcedureFacade/ModelCatalog infrastructure.
//
// If needed later, these can be added:
// - estimate_node_property_steps() - aggregate memory estimates from all steps
// - tasks() - create progress tracking task tree

/// Errors that can occur during node property step execution.
#[derive(Debug)]
pub enum NodePropertyStepExecutorError {
    /// A node label referenced in a step's context config doesn't exist in the graph.
    InvalidNodeLabel { label: String, step_name: String },

    /// A relationship type referenced in a step's context config doesn't exist in the graph.
    InvalidRelationshipType { rel_type: String, step_name: String },

    /// Failed to execute a specific step in the pipeline.
    StepExecutionFailed {
        step_index: usize,
        step_name: String,
        source: Box<dyn StdError + Send + Sync>,
    },

    /// Graph store is locked and cannot be mutated.
    GraphStoreLocked,
}

impl std::fmt::Display for NodePropertyStepExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidNodeLabel { label, step_name } => {
                write!(
                    f,
                    "Invalid node label '{}' for contextNodeLabels for step `{}`",
                    label, step_name
                )
            }
            Self::InvalidRelationshipType {
                rel_type,
                step_name,
            } => {
                write!(
                    f,
                    "Invalid relationship type '{}' for contextRelationshipTypes for step `{}`",
                    rel_type, step_name
                )
            }
            Self::StepExecutionFailed {
                step_index,
                step_name,
                source,
            } => {
                write!(
                    f,
                    "Failed to execute step {} ('{}') in pipeline: {}",
                    step_index, step_name, source
                )
            }
            Self::GraphStoreLocked => {
                write!(
                    f,
                    "Graph store is locked and cannot be mutated during step execution"
                )
            }
        }
    }
}

impl StdError for NodePropertyStepExecutorError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::StepExecutionFailed { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::eval::pipeline::NodePropertyStep;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;
    use std::collections::HashMap;
    use std::sync::Mutex;

    fn create_test_graph_store() -> Arc<DefaultGraphStore> {
        let config = RandomGraphConfig {
            node_count: 10,
            ..RandomGraphConfig::default()
        };
        Arc::new(DefaultGraphStore::random(&config).expect("Failed to create random graph"))
    }

    fn create_test_step(algorithm_name: &str) -> Box<dyn ExecutableNodePropertyStep> {
        let mut config = HashMap::new();
        config.insert("maxIterations".to_string(), serde_json::json!(20));
        config.insert(
            "mutateProperty".to_string(),
            serde_json::json!("testProperty"),
        );

        Box::new(NodePropertyStep::new(algorithm_name.to_string(), config))
    }

    #[test]
    fn test_executor_creation() {
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let available_rel_types = HashSet::new();

        let executor =
            NodePropertyStepExecutor::new(node_labels, relationship_types, available_rel_types, 4);

        assert_eq!(executor.concurrency, 4);
    }

    #[test]
    fn test_validate_context_configs_valid() {
        let graph_store = create_test_graph_store();
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let available_rel_types = HashSet::new();

        let executor = NodePropertyStepExecutor::new(
            node_labels.clone(),
            relationship_types,
            available_rel_types,
            4,
        );

        // Create step with valid configuration
        let mut config = HashMap::new();
        config.insert("maxIterations".to_string(), serde_json::json!(20));

        let step = Box::new(NodePropertyStep::new(
            "gds.pagerank.mutate".to_string(),
            config,
        ));

        let steps: Vec<Box<dyn ExecutableNodePropertyStep>> = vec![step];
        let result = executor.validate_node_property_steps_context_configs(&*graph_store, &steps);

        assert!(
            result.is_ok(),
            "Validation should succeed with valid labels"
        );
    }

    #[test]
    fn test_validate_context_configs_invalid_label() {
        let graph_store = create_test_graph_store();
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let available_rel_types = HashSet::new();

        let executor =
            NodePropertyStepExecutor::new(node_labels, relationship_types, available_rel_types, 4);

        // Create step with a context label that does not exist in the graph
        let mut config = HashMap::new();
        config.insert("maxIterations".to_string(), serde_json::json!(20));

        let step = Box::new(NodePropertyStep::with_context(
            "gds.pagerank.mutate".to_string(),
            config,
            vec!["__NO_SUCH_LABEL__".to_string()],
            vec![],
        ));

        let steps: Vec<Box<dyn ExecutableNodePropertyStep>> = vec![step];
        let result = executor.validate_node_property_steps_context_configs(&*graph_store, &steps);

        assert!(
            matches!(
                result,
                Err(NodePropertyStepExecutorError::InvalidNodeLabel { .. })
            ),
            "Expected InvalidNodeLabel error, got: {result:?}"
        );
    }

    #[test]
    fn test_validate_context_configs_invalid_relationship_type() {
        let graph_store = create_test_graph_store();
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let available_rel_types = HashSet::new();

        let executor = NodePropertyStepExecutor::new(
            node_labels.clone(),
            relationship_types,
            available_rel_types,
            4,
        );

        // Create step with a context relationship type that does not exist in the graph
        let mut config = HashMap::new();
        config.insert("maxIterations".to_string(), serde_json::json!(20));

        let step = Box::new(NodePropertyStep::with_context(
            "gds.pagerank.mutate".to_string(),
            config,
            vec![],
            vec!["__NO_SUCH_REL_TYPE__".to_string()],
        ));

        let steps: Vec<Box<dyn ExecutableNodePropertyStep>> = vec![step];
        let result = executor.validate_node_property_steps_context_configs(&*graph_store, &steps);

        assert!(
            matches!(
                result,
                Err(NodePropertyStepExecutorError::InvalidRelationshipType { .. })
            ),
            "Expected InvalidRelationshipType error, got: {result:?}"
        );
    }

    #[test]
    fn test_cleanup_intermediate_properties() {
        let mut graph_store = create_test_graph_store();
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string()];
        let available_rel_types = HashSet::new();

        let mut executor =
            NodePropertyStepExecutor::new(node_labels, relationship_types, available_rel_types, 4);

        let steps: Vec<Box<dyn ExecutableNodePropertyStep>> = vec![
            create_test_step("gds.pagerank.mutate"),
            create_test_step("gds.louvain.mutate"),
        ];

        let result = executor.cleanup_intermediate_properties(&mut graph_store, &steps);

        // Cleanup is best-effort; it should never fail the pipeline for missing properties.
        assert!(result.is_ok(), "Cleanup should succeed");
    }

    #[test]
    fn test_feature_input_resolution_uses_context_and_available_rel_types() {
        #[derive(Clone)]
        struct CaptureStep {
            context_labels: Vec<String>,
            context_rel_types: Vec<String>,
            config: HashMap<String, serde_json::Value>,
            captured: Arc<Mutex<Option<(Vec<String>, Vec<String>)>>>,
        }

        impl ExecutableNodePropertyStep for CaptureStep {
            fn execute(
                &self,
                _graph_store: &mut DefaultGraphStore,
                node_labels: &[String],
                relationship_types: &[String],
                _concurrency: usize,
            ) -> Result<(), Box<dyn StdError + Send + Sync>> {
                let mut guard = self.captured.lock().expect("capture lock");
                *guard = Some((node_labels.to_vec(), relationship_types.to_vec()));
                Ok(())
            }

            fn config(&self) -> &HashMap<String, serde_json::Value> {
                &self.config
            }

            fn context_node_labels(&self) -> &[String] {
                &self.context_labels
            }

            fn context_relationship_types(&self) -> &[String] {
                &self.context_rel_types
            }

            fn proc_name(&self) -> &str {
                "capture.step"
            }

            fn mutate_node_property(&self) -> &str {
                "captured"
            }
        }

        let mut graph_store = create_test_graph_store();
        let node_labels = vec!["Node".to_string()];
        let relationship_types = vec!["REL".to_string(), "REL2".to_string()];
        let available_rel_types: HashSet<String> = vec!["REL2".to_string()].into_iter().collect();

        let mut executor =
            NodePropertyStepExecutor::new(node_labels, relationship_types, available_rel_types, 2);

        let captured = Arc::new(Mutex::new(None));
        let step = CaptureStep {
            context_labels: vec!["Context".to_string()],
            context_rel_types: vec![],
            config: HashMap::new(),
            captured: Arc::clone(&captured),
        };

        executor
            .execute_node_property_steps(&mut graph_store, &[Box::new(step)])
            .expect("execute should succeed");

        let captured = captured.lock().expect("capture lock");
        let (labels, rel_types) = captured.clone().expect("captured inputs");
        assert_eq!(labels, vec!["Node".to_string(), "Context".to_string()]);
        assert_eq!(rel_types, vec!["REL2".to_string()]);
    }

    #[test]
    fn test_error_display() {
        let error = NodePropertyStepExecutorError::InvalidNodeLabel {
            label: "TestLabel".to_string(),
            step_name: "test_step".to_string(),
        };

        let display = format!("{}", error);
        assert!(display.contains("TestLabel"));
        assert!(display.contains("test_step"));
        assert!(display.contains("contextNodeLabels"));
        assert!(display.contains("for step `test_step`"));

        let error = NodePropertyStepExecutorError::InvalidRelationshipType {
            rel_type: "TEST_REL".to_string(),
            step_name: "rel_step".to_string(),
        };
        let display = format!("{}", error);
        assert!(display.contains("TEST_REL"));
        assert!(display.contains("contextRelationshipTypes"));
        assert!(display.contains("for step `rel_step`"));

        let error = NodePropertyStepExecutorError::GraphStoreLocked;
        let display = format!("{}", error);
        assert!(display.contains("locked"));
    }
}
