//! Node property step execution registry.
//!
//! Extracted from `node_property_step.rs` to keep `NodePropertyStep` thin
//! and aligned with the Java implementation (config assembly + delegation).

use crate::algo::degree_centrality::DEGREE_CENTRALITYAlgorithmSpec as DegreeCentralityAlgorithmSpec;
use crate::algo::embeddings::fastrp::FastRPAlgorithmSpec;
use crate::algo::harmonic::HARMONICAlgorithmSpec as HarmonicAlgorithmSpec;
use crate::algo::hits::HITSAlgorithmSpec;
use crate::algo::pagerank::PAGERANKAlgorithmSpec as PageRankAlgorithmSpec;
use crate::collections::backends::vec::VecDouble;
use crate::projection::eval::pipeline::node_property_step::{
    NodePropertyStepError, MUTATE_PROPERTY_KEY,
};
use crate::projection::eval::pipeline::procedure_registry::{ProcedureKind, ProcedureRegistry};
use crate::projection::eval::algorithm::{ExecutionContext, ExecutionMode, ProcedureExecutor};
use crate::projection::NodeLabel;
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphStore;
use crate::types::properties::node::{DefaultDoubleNodePropertyValues, NodePropertyValues};
use std::collections::{HashMap, HashSet};
use std::error::Error as StdError;
use std::sync::Arc;

const PIPELINE_GRAPH_NAME: &str = "__pipeline_graph__";

pub(crate) fn execute_node_property_step(
    algorithm_name: &str,
    graph_store: &mut DefaultGraphStore,
    exec_config: &HashMap<String, serde_json::Value>,
    config_value: &serde_json::Value,
    node_labels: &[String],
) -> Result<(), Box<dyn StdError + Send + Sync>> {
    let procedure = ProcedureRegistry::resolve(algorithm_name).ok_or_else(|| {
        Box::new(NodePropertyStepError::AlgorithmNotImplemented {
            algorithm: algorithm_name.to_string(),
        }) as Box<dyn StdError + Send + Sync>
    })?;

    match procedure {
        ProcedureKind::DebugWriteConstantDouble => {
            let mutate_property = mutate_property_from_config(algorithm_name, exec_config)?;
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
                        algorithm: algorithm_name.to_string(),
                        message: e.to_string(),
                    }) as Box<dyn StdError + Send + Sync>
                })?;

            Ok(())
        }
        ProcedureKind::PageRankMutate => {
            let context = build_execution_context(graph_store);
            let mut executor = ProcedureExecutor::new(context, ExecutionMode::MutateNodeProperty);
            let mut spec = PageRankAlgorithmSpec::new(PIPELINE_GRAPH_NAME.to_string());

            executor.compute(&mut spec, config_value).map_err(|e| {
                Box::new(NodePropertyStepError::ExecutionFailed {
                    algorithm: algorithm_name.to_string(),
                    message: e.to_string(),
                }) as Box<dyn StdError + Send + Sync>
            })?;

            Ok(())
        }
        ProcedureKind::FastRPMutate => {
            let context = build_execution_context(graph_store);
            let mut executor = ProcedureExecutor::new(context, ExecutionMode::MutateNodeProperty);
            let mut spec = FastRPAlgorithmSpec::new(PIPELINE_GRAPH_NAME.to_string());

            executor.compute(&mut spec, config_value).map_err(|e| {
                Box::new(NodePropertyStepError::ExecutionFailed {
                    algorithm: algorithm_name.to_string(),
                    message: e.to_string(),
                }) as Box<dyn StdError + Send + Sync>
            })?;

            Ok(())
        }
        ProcedureKind::DegreeCentralityMutate => {
            let context = build_execution_context(graph_store);
            let mut executor = ProcedureExecutor::new(context, ExecutionMode::MutateNodeProperty);
            let mut spec = DegreeCentralityAlgorithmSpec::new(PIPELINE_GRAPH_NAME.to_string());

            executor.compute(&mut spec, config_value).map_err(|e| {
                Box::new(NodePropertyStepError::ExecutionFailed {
                    algorithm: algorithm_name.to_string(),
                    message: e.to_string(),
                }) as Box<dyn StdError + Send + Sync>
            })?;

            Ok(())
        }
        ProcedureKind::HitsMutate => {
            let context = build_execution_context(graph_store);
            let mut executor = ProcedureExecutor::new(context, ExecutionMode::MutateNodeProperty);
            let mut spec = HITSAlgorithmSpec::new(PIPELINE_GRAPH_NAME.to_string());

            executor.compute(&mut spec, config_value).map_err(|e| {
                Box::new(NodePropertyStepError::ExecutionFailed {
                    algorithm: algorithm_name.to_string(),
                    message: e.to_string(),
                }) as Box<dyn StdError + Send + Sync>
            })?;

            Ok(())
        }
        ProcedureKind::HarmonicMutate => {
            let context = build_execution_context(graph_store);
            let mut executor = ProcedureExecutor::new(context, ExecutionMode::MutateNodeProperty);
            let mut spec = HarmonicAlgorithmSpec::new(PIPELINE_GRAPH_NAME.to_string());

            executor.compute(&mut spec, config_value).map_err(|e| {
                Box::new(NodePropertyStepError::ExecutionFailed {
                    algorithm: algorithm_name.to_string(),
                    message: e.to_string(),
                }) as Box<dyn StdError + Send + Sync>
            })?;

            Ok(())
        }
    }
}

fn mutate_property_from_config(
    algorithm_name: &str,
    exec_config: &HashMap<String, serde_json::Value>,
) -> Result<String, NodePropertyStepError> {
    exec_config
        .get(MUTATE_PROPERTY_KEY)
        .and_then(|v| v.as_str())
        .map(String::from)
        .ok_or_else(|| NodePropertyStepError::MissingMutateProperty {
            algorithm: algorithm_name.to_string(),
        })
}

fn build_execution_context(graph_store: &DefaultGraphStore) -> ExecutionContext {
    let mut context = ExecutionContext::empty();
    context.add_graph(PIPELINE_GRAPH_NAME, Arc::new(graph_store.clone()));
    context
}
