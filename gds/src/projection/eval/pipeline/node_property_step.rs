//! Java GDS: pipeline/src/main/java/org/neo4j/gds/ml/pipeline/NodePropertyStep.java
//!
//! Concrete implementation of a node property step in an ML pipeline.
//!
//! Represents a single algorithm execution step that computes and mutates node properties
//! (e.g., running PageRank or FastRP as part of feature extraction).
//!
//! Node property steps execute algorithms via ProcedureExecutor and then
//! apply the results as node properties on the in-memory graph.

use crate::algo::embeddings::fastrp::{FastRPAlgorithmSpec, FastRPResult};
use crate::algo::pagerank::{PageRankAlgorithmSpec, PageRankResult};
use crate::collections::backends::vec::{VecDouble, VecFloatArray};
use crate::prelude::GraphStore;
use crate::projection::eval::pipeline::{
    ExecutableNodePropertyStep, NodePropertyStepContextConfig,
};
use crate::projection::eval::procedure::{ExecutionContext, ExecutionMode, ProcedureExecutor};
use crate::projection::NodeLabel;
use crate::types::properties::node::{
    DefaultDoubleNodePropertyValues, DefaultFloatArrayNodePropertyValues, NodePropertyValues,
};
use std::collections::{HashMap, HashSet};
use std::error::Error as StdError;
use std::sync::Arc;

/// Configuration key for the mutate property name.
pub const MUTATE_PROPERTY_KEY: &str = "mutateProperty";

/// A minimal built-in algorithm used for smoke tests.
///
/// This is intentionally simple: it writes a constant `f64` node property.
pub const DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE: &str = "gds.debug.writeConstantDouble.mutate";

/// A first real algorithm wiring for ML pipelines.
///
/// Computes PageRank and writes the resulting scores to `mutateProperty`.
pub const PAGERANK_MUTATE: &str = "gds.pagerank.mutate";

/// A common node embedding algorithm used in ML pipelines.
///
/// Note: Execution wiring is deferred; creating a step is still supported.
pub const FASTRP_MUTATE: &str = "gds.fastrp.mutate";

const PIPELINE_GRAPH_NAME: &str = "__pipeline_graph__";

/// Node property step that executes an algorithm to compute node properties.
///
/// # Java Source (NodePropertyStep.java)
/// ```java
/// public final class NodePropertyStep implements ExecutableNodePropertyStep {
///     private final GdsCallableFinder.GdsCallableDefinition callableDefinition;
///     private final Map<String, Object> config;
///     private final List<String> contextNodeLabels;
///     private final List<String> contextRelationshipTypes;
///
///     public void execute(
///         ExecutionContext executionContext,
///         String graphName,
///         Collection<NodeLabel> nodeLabels,
///         Collection<RelationshipType> relTypes,
///         Concurrency trainConcurrency,
///         Stub stub
///     ) {
///         // ... builds config and executes via ProcedureExecutor
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct NodePropertyStep {
    /// Name of the algorithm to execute (e.g., "gds.pagerank.mutate", "gds.fastRP.mutate")
    algorithm_name: String,

    /// Algorithm configuration (user-provided parameters)
    config: HashMap<String, serde_json::Value>,

    /// Context configuration (node labels and relationship types for execution context)
    context_config: NodePropertyStepContextConfig,
}

impl NodePropertyStep {
    /// Create a new node property step.
    ///
    /// # Arguments
    /// * `algorithm_name` - Name of the algorithm (e.g., "pagerank", "fastRP")
    /// * `config` - Algorithm configuration parameters
    pub fn new(algorithm_name: String, config: HashMap<String, serde_json::Value>) -> Self {
        Self {
            algorithm_name,
            config,
            context_config: NodePropertyStepContextConfig::default(),
        }
    }

    /// Create a new node property step with context configuration.
    ///
    /// # Arguments
    /// * `algorithm_name` - Name of the algorithm
    /// * `config` - Algorithm configuration parameters
    /// * `context_node_labels` - Node labels for execution context
    /// * `context_relationship_types` - Relationship types for execution context
    pub fn with_context(
        algorithm_name: String,
        config: HashMap<String, serde_json::Value>,
        context_node_labels: Vec<String>,
        context_relationship_types: Vec<String>,
    ) -> Self {
        Self {
            algorithm_name,
            config,
            context_config: NodePropertyStepContextConfig::new(
                context_node_labels,
                context_relationship_types,
            ),
        }
    }

    /// Get the algorithm name.
    pub fn algorithm_name(&self) -> &str {
        &self.algorithm_name
    }

    /// Get the mutate property name from configuration.
    fn get_mutate_property(&self) -> Result<String, NodePropertyStepError> {
        self.config
            .get(MUTATE_PROPERTY_KEY)
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or_else(|| NodePropertyStepError::MissingMutateProperty {
                algorithm: self.algorithm_name.clone(),
            })
    }
}

fn build_execution_context(
    graph_store: &crate::types::graph_store::DefaultGraphStore,
) -> ExecutionContext {
    let mut context = ExecutionContext::empty();
    context.add_graph(PIPELINE_GRAPH_NAME, Arc::new(graph_store.clone()));
    context
}

impl ExecutableNodePropertyStep for NodePropertyStep {
    fn execute(
        &self,
        graph_store: &mut crate::types::graph_store::DefaultGraphStore,
        node_labels: &[String],
        relationship_types: &[String],
        concurrency: usize,
    ) -> Result<(), Box<dyn StdError>> {
        // Build execution configuration
        let mut exec_config = self.config.clone();
        exec_config.insert(
            "nodeLabels".to_string(),
            serde_json::Value::Array(
                node_labels
                    .iter()
                    .map(|s| serde_json::Value::String(s.clone()))
                    .collect(),
            ),
        );
        exec_config.insert(
            "relationshipTypes".to_string(),
            serde_json::Value::Array(
                relationship_types
                    .iter()
                    .map(|s| serde_json::Value::String(s.clone()))
                    .collect(),
            ),
        );
        exec_config
            .entry("concurrency".to_string())
            .or_insert_with(|| serde_json::Value::Number(concurrency.into()));

        let config_value = serde_json::to_value(exec_config.clone()).map_err(|e| {
            Box::new(NodePropertyStepError::ExecutionFailed {
                algorithm: self.algorithm_name.clone(),
                message: format!("failed to serialize config: {e}"),
            }) as Box<dyn StdError>
        })?;

        match self.algorithm_name.as_str() {
            DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE => {
                let mutate_property = self.get_mutate_property()?;
                let value = exec_config
                    .get("value")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(1.0);

                let node_count = graph_store.node_count();
                let backend = VecDouble::from(vec![value; node_count]);
                let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
                let values: Arc<dyn NodePropertyValues> = Arc::new(values);

                let labels: HashSet<NodeLabel> = node_labels
                    .iter()
                    .map(|label| NodeLabel::of(label.clone()))
                    .collect();

                graph_store
                    .add_node_property(labels, mutate_property, values)
                    .map_err(|e| {
                        Box::new(NodePropertyStepError::ExecutionFailed {
                            algorithm: self.algorithm_name.clone(),
                            message: e.to_string(),
                        }) as Box<dyn StdError>
                    })?;

                Ok(())
            }
            PAGERANK_MUTATE => {
                let mutate_property = self.get_mutate_property()?;

                let context = build_execution_context(graph_store);
                let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);
                let mut spec = PageRankAlgorithmSpec::new(PIPELINE_GRAPH_NAME.to_string());

                let result: PageRankResult =
                    executor.compute(&mut spec, &config_value).map_err(|e| {
                        Box::new(NodePropertyStepError::ExecutionFailed {
                            algorithm: self.algorithm_name.clone(),
                            message: e.to_string(),
                        }) as Box<dyn StdError>
                    })?;

                let node_count = graph_store.node_count();
                if result.scores.len() != node_count {
                    return Err(Box::new(NodePropertyStepError::ExecutionFailed {
                        algorithm: self.algorithm_name.clone(),
                        message: format!(
                            "pagerank returned {} scores for {} nodes",
                            result.scores.len(),
                            node_count
                        ),
                    }));
                }

                let backend = VecDouble::from(result.scores);
                let values = DefaultDoubleNodePropertyValues::from_collection(backend, node_count);
                let values: Arc<dyn NodePropertyValues> = Arc::new(values);

                let labels: HashSet<NodeLabel> = node_labels
                    .iter()
                    .map(|label| NodeLabel::of(label.clone()))
                    .collect();

                graph_store
                    .add_node_property(labels, mutate_property, values)
                    .map_err(|e| {
                        Box::new(NodePropertyStepError::ExecutionFailed {
                            algorithm: self.algorithm_name.clone(),
                            message: e.to_string(),
                        }) as Box<dyn StdError>
                    })?;

                Ok(())
            }
            FASTRP_MUTATE => {
                let mutate_property = self.get_mutate_property()?;

                let context = build_execution_context(graph_store);
                let mut executor = ProcedureExecutor::new(context, ExecutionMode::Stream);
                let mut spec = FastRPAlgorithmSpec::new(PIPELINE_GRAPH_NAME.to_string());

                let result: FastRPResult =
                    executor.compute(&mut spec, &config_value).map_err(|e| {
                        Box::new(NodePropertyStepError::ExecutionFailed {
                            algorithm: self.algorithm_name.clone(),
                            message: e.to_string(),
                        }) as Box<dyn StdError>
                    })?;

                let node_count = graph_store.node_count();
                if result.embeddings.len() != node_count {
                    return Err(Box::new(NodePropertyStepError::ExecutionFailed {
                        algorithm: self.algorithm_name.clone(),
                        message: format!(
                            "fastrp returned {} embeddings for {} nodes",
                            result.embeddings.len(),
                            node_count
                        ),
                    }));
                }

                let dense: Vec<Option<Vec<f32>>> =
                    result.embeddings.into_iter().map(Some).collect();
                let backend = VecFloatArray::from(dense);
                let values = DefaultFloatArrayNodePropertyValues::<VecFloatArray>::from_collection(
                    backend, node_count,
                );
                let values: Arc<dyn NodePropertyValues> = Arc::new(values);

                let labels: HashSet<NodeLabel> = node_labels
                    .iter()
                    .map(|label| NodeLabel::of(label.clone()))
                    .collect();

                graph_store
                    .add_node_property(labels, mutate_property, values)
                    .map_err(|e| {
                        Box::new(NodePropertyStepError::ExecutionFailed {
                            algorithm: self.algorithm_name.clone(),
                            message: e.to_string(),
                        }) as Box<dyn StdError>
                    })?;

                Ok(())
            }
            _ => Err(Box::new(NodePropertyStepError::AlgorithmNotImplemented {
                algorithm: self.algorithm_name.clone(),
            })),
        }
    }

    fn config(&self) -> &HashMap<String, serde_json::Value> {
        &self.config
    }

    fn context_node_labels(&self) -> &[String] {
        self.context_config.context_node_labels()
    }

    fn context_relationship_types(&self) -> &[String] {
        self.context_config.context_relationship_types()
    }

    fn proc_name(&self) -> &str {
        &self.algorithm_name
    }

    fn root_task_name(&self) -> &str {
        &self.algorithm_name
    }

    fn mutate_node_property(&self) -> &str {
        // This is a bit awkward but matches the Java API contract
        // where this method doesn't return Result.
        // We'll panic if the property is missing, which should be caught
        // during validation in the factory.
        self.config
            .get(MUTATE_PROPERTY_KEY)
            .and_then(|v| v.as_str())
            .expect("mutateProperty must be present in config")
    }

    fn to_map(&self) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();

        // Build config with context
        let mut config_with_context = self.config.clone();
        config_with_context.insert(
            NodePropertyStepContextConfig::CONTEXT_NODE_LABELS.to_string(),
            serde_json::Value::Array(
                self.context_config
                    .context_node_labels()
                    .iter()
                    .map(|s| serde_json::Value::String(s.clone()))
                    .collect(),
            ),
        );
        config_with_context.insert(
            NodePropertyStepContextConfig::CONTEXT_RELATIONSHIP_TYPES.to_string(),
            serde_json::Value::Array(
                self.context_config
                    .context_relationship_types()
                    .iter()
                    .map(|s| serde_json::Value::String(s.clone()))
                    .collect(),
            ),
        );

        result.insert(
            "name".to_string(),
            serde_json::Value::String(self.algorithm_name.clone()),
        );
        result.insert(
            "config".to_string(),
            serde_json::to_value(config_with_context).unwrap(),
        );

        result
    }
}

impl PartialEq for NodePropertyStep {
    fn eq(&self, other: &Self) -> bool {
        self.algorithm_name == other.algorithm_name && self.config == other.config
    }
}

impl Eq for NodePropertyStep {}

impl std::hash::Hash for NodePropertyStep {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.algorithm_name.hash(state);
        // Note: HashMap doesn't implement Hash, so we hash the sorted keys
        let mut keys: Vec<_> = self.config.keys().collect();
        keys.sort();
        for key in keys {
            key.hash(state);
            // Hash the JSON value as a string
            if let Ok(json_str) = serde_json::to_string(self.config.get(key).unwrap()) {
                json_str.hash(state);
            }
        }
    }
}

/// Errors that can occur during node property step operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodePropertyStepError {
    /// Missing required mutateProperty configuration
    MissingMutateProperty {
        /// Name of the algorithm
        algorithm: String,
    },

    /// Algorithm execution failed
    ExecutionFailed {
        /// Name of the algorithm
        algorithm: String,
        /// Error message
        message: String,
    },

    /// Algorithm isn't wired into the Rust execution runtime yet.
    AlgorithmNotImplemented {
        /// Name of the algorithm
        algorithm: String,
    },
}

impl std::fmt::Display for NodePropertyStepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodePropertyStepError::MissingMutateProperty { algorithm } => {
                write!(
                    f,
                    "Algorithm '{}' is missing required '{}' configuration",
                    algorithm, MUTATE_PROPERTY_KEY
                )
            }
            NodePropertyStepError::ExecutionFailed { algorithm, message } => {
                write!(f, "Algorithm '{}' execution failed: {}", algorithm, message)
            }
            NodePropertyStepError::AlgorithmNotImplemented { algorithm } => {
                write!(
                    f,
                    "Algorithm '{}' is not implemented in the Rust node-property step runtime",
                    algorithm
                )
            }
        }
    }
}

impl StdError for NodePropertyStepError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_property_step_creation() {
        let mut config = HashMap::new();
        config.insert(
            MUTATE_PROPERTY_KEY.to_string(),
            serde_json::Value::String("pagerank".to_string()),
        );
        config.insert(
            "maxIterations".to_string(),
            serde_json::Value::Number(20.into()),
        );

        let step = NodePropertyStep::new("gds.pagerank.mutate".to_string(), config);

        assert_eq!(step.algorithm_name(), "gds.pagerank.mutate");
        assert_eq!(step.mutate_node_property(), "pagerank");
        assert_eq!(step.context_node_labels(), &[] as &[String]);
    }

    #[test]
    fn test_node_property_step_with_context() {
        let mut config = HashMap::new();
        config.insert(
            MUTATE_PROPERTY_KEY.to_string(),
            serde_json::Value::String("embedding".to_string()),
        );

        let step = NodePropertyStep::with_context(
            "gds.fastRP.mutate".to_string(),
            config,
            vec!["Person".to_string()],
            vec!["KNOWS".to_string()],
        );

        assert_eq!(step.context_node_labels(), &["Person"]);
        assert_eq!(step.context_relationship_types(), &["KNOWS"]);
    }

    #[test]
    fn test_to_map() {
        let mut config = HashMap::new();
        config.insert(
            MUTATE_PROPERTY_KEY.to_string(),
            serde_json::Value::String("score".to_string()),
        );

        let step = NodePropertyStep::with_context(
            "gds.pagerank.mutate".to_string(),
            config,
            vec!["Node".to_string()],
            vec!["REL".to_string()],
        );

        let map = step.to_map();
        assert_eq!(
            map.get("name").and_then(|v| v.as_str()),
            Some("gds.pagerank.mutate")
        );
        assert!(map.contains_key("config"));
    }

    #[test]
    fn test_equality() {
        let mut config1 = HashMap::new();
        config1.insert(
            MUTATE_PROPERTY_KEY.to_string(),
            serde_json::Value::String("prop".to_string()),
        );

        let mut config2 = HashMap::new();
        config2.insert(
            MUTATE_PROPERTY_KEY.to_string(),
            serde_json::Value::String("prop".to_string()),
        );

        let step1 = NodePropertyStep::new("algo".to_string(), config1);
        let step2 = NodePropertyStep::new("algo".to_string(), config2);

        assert_eq!(step1, step2);
    }

    #[test]
    fn test_missing_mutate_property_error() {
        let config = HashMap::new();
        let step = NodePropertyStep::new("gds.pagerank.mutate".to_string(), config);

        let err = step.get_mutate_property().unwrap_err();
        assert!(matches!(
            err,
            NodePropertyStepError::MissingMutateProperty { .. }
        ));
        let msg = err.to_string();
        assert!(msg.contains("missing required 'mutateProperty'"));
    }

    #[test]
    fn test_debug_write_constant_double_mutate_writes_property() {
        use crate::types::graph_store::DefaultGraphStore;
        use crate::types::random::random_graph::RandomGraphConfig;

        let mut config = HashMap::new();
        config.insert(
            MUTATE_PROPERTY_KEY.to_string(),
            serde_json::Value::String("debugProp".to_string()),
        );
        config.insert(
            "value".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(2.5).unwrap()),
        );

        let step = NodePropertyStep::new(DEBUG_WRITE_CONSTANT_DOUBLE_MUTATE.to_string(), config);

        let config = RandomGraphConfig::default().with_seed(123);
        let mut graph_store = DefaultGraphStore::random(&config).unwrap();

        // Add a label so we can attach properties.
        graph_store
            .add_node_label(NodeLabel::of("Node"))
            .expect("add node label");

        step.execute(
            &mut graph_store,
            &["Node".to_string()],
            &["REL".to_string()],
            1,
        )
        .expect("step execute should succeed");

        assert!(graph_store.has_node_property("debugProp"));
    }

    #[test]
    fn test_to_map_includes_context_keys() {
        let mut config = HashMap::new();
        config.insert(
            MUTATE_PROPERTY_KEY.to_string(),
            serde_json::Value::String("score".to_string()),
        );

        let step = NodePropertyStep::with_context(
            "gds.pagerank.mutate".to_string(),
            config,
            vec!["Person".to_string()],
            vec!["KNOWS".to_string()],
        );

        let map = step.to_map();
        let config_map = map
            .get("config")
            .and_then(|v| v.as_object())
            .expect("config map should exist");

        assert!(config_map.contains_key(NodePropertyStepContextConfig::CONTEXT_NODE_LABELS));
        assert!(config_map.contains_key(NodePropertyStepContextConfig::CONTEXT_RELATIONSHIP_TYPES));
    }
}
