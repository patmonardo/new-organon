//! Label Propagation algorithm dispatch handler.
//!
//! Handles JSON requests for Label Propagation community detection operations,
//! delegating to the facade layer for execution.

use crate::applications::algorithms::community::{err, timings_json};
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::CatalogLoader;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::community::label_propagation::LabelPropagationFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Label Propagation requests
pub fn handle_label_propagation(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "labelPropagation";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let concurrency_value = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    let max_iterations = request
        .get("maxIterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(10);

    let node_weight_property = request
        .get("nodeWeightProperty")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let seed_property = request
        .get("seedProperty")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let concurrency = match Concurrency::new(concurrency_value) {
        Some(value) => value,
        None => {
            return err(
                op,
                "INVALID_REQUEST",
                "concurrency must be greater than zero",
            )
        }
    };

    let graph_resources = match CatalogLoader::load_or_err(catalog.as_ref(), graph_name) {
        Ok(resources) => resources,
        Err(e) => return err(op, "GRAPH_NOT_FOUND", &e.to_string()),
    };

    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    match mode {
        "stream" => {
            let task = Tasks::leaf("label_propagation::stream".to_string())
                .base()
                .clone();
            let seed_property = seed_property.clone();
            let node_weight_property = node_weight_property.clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let mut builder = gr
                    .facade()
                    .label_propagation()
                    .concurrency(concurrency_value)
                    .max_iterations(max_iterations);

                if let Some(ref node_weight) = node_weight_property {
                    builder = builder.node_weight_property(node_weight);
                }

                if let Some(ref seed) = seed_property {
                    builder = builder.seed_property(seed);
                }

                let iter = builder.stream().map_err(|e| e.to_string())?;
                let rows = iter
                    .map(|row| serde_json::to_value(row).map_err(|e| e.to_string()))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Some(rows))
            };

            let result_builder =
                FnStreamResultBuilder::new(|_gr: &GraphResources, rows: Option<Vec<Value>>| {
                    rows.unwrap_or_default().into_iter()
                });

            match convenience.process_stream(
                &graph_resources,
                concurrency,
                task,
                compute,
                result_builder,
            ) {
                Ok(stream) => {
                    let rows: Vec<Value> = stream.collect();
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "stream",
                        "data": rows,
                        "timings": json!({
                            "preProcessingMillis": 0,
                            "computeMillis": 0,
                            "sideEffectMillis": 0
                        })
                    })
                }
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Label Propagation stream failed: {e}"),
                ),
            }
        }
        "stats" => {
            let task = Tasks::leaf("label_propagation::stats".to_string())
                .base()
                .clone();
            let seed_property = seed_property.clone();
            let node_weight_property = node_weight_property.clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Value>, String> {
                let mut builder = gr
                    .facade()
                    .label_propagation()
                    .concurrency(concurrency_value)
                    .max_iterations(max_iterations);

                if let Some(ref node_weight) = node_weight_property {
                    builder = builder.node_weight_property(node_weight);
                }

                if let Some(ref seed) = seed_property {
                    builder = builder.seed_property(seed);
                }

                let stats = builder.stats().map_err(|e| e.to_string())?;
                let stats_value = serde_json::to_value(stats).map_err(|e| e.to_string())?;
                Ok(Some(stats_value))
            };

            let builder =
                FnStatsResultBuilder(|_gr: &GraphResources, stats: Option<Value>, timings| {
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "stats",
                        "data": stats,
                        "timings": timings_json(timings)
                    })
                });

            match convenience.process_stats(&graph_resources, concurrency, task, compute, builder) {
                Ok(response) => response,
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Label Propagation stats failed: {e}"),
                ),
            }
        }
        "mutate" => {
            // Accept either explicit mutateProperty, expectedPropertyName, or fall back to a default
            let property_name = request
                .get("mutateProperty")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or_else(|| {
                    request
                        .get("expectedPropertyName")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .unwrap_or_else(|| "label".to_string());

            let mut facade = LabelPropagationFacade::new(Arc::clone(graph_resources.store()))
                .concurrency(concurrency_value)
                .max_iterations(max_iterations);

            if let Some(ref node_weight) = node_weight_property {
                facade = facade.node_weight_property(node_weight);
            }

            if let Some(ref seed) = seed_property {
                facade = facade.seed_property(seed);
            }

            match facade.mutate(&property_name) {
                Ok(result) => {
                    catalog.set(graph_name, result.updated_store);
                    json!({"ok": true, "op": op, "data": {
                        "nodes_updated": result.summary.nodes_updated,
                        "property_name": result.summary.property_name,
                        "execution_time_ms": result.summary.execution_time_ms
                    }})
                }
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Label Propagation mutate failed: {:?}", e),
                ),
            }
        }
        "write" => {
            let mut facade = LabelPropagationFacade::new(Arc::clone(graph_resources.store()))
                .concurrency(concurrency_value)
                .max_iterations(max_iterations);

            if let Some(ref node_weight) = node_weight_property {
                facade = facade.node_weight_property(node_weight);
            }

            if let Some(ref seed) = seed_property {
                facade = facade.seed_property(seed);
            }

            let property_name = match request.get("writeProperty").and_then(|v| v.as_str()) {
                Some(s) => s.to_string(),
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'writeProperty' parameter for write mode",
                    )
                }
            };

            match facade.write(&property_name) {
                Ok(result) => json!({"ok": true, "op": op, "data": result}),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Label Propagation write failed: {:?}", e),
                ),
            }
        }
        "estimate" => {
            let mut facade = LabelPropagationFacade::new(Arc::clone(graph_resources.store()))
                .concurrency(concurrency_value)
                .max_iterations(max_iterations);

            if let Some(ref node_weight) = node_weight_property {
                facade = facade.node_weight_property(node_weight);
            }

            if let Some(ref seed) = seed_property {
                facade = facade.seed_property(seed);
            }

            match facade.estimate_memory() {
                Ok(range) => json!({"ok": true, "op": op, "data": range}),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Label Propagation memory estimation failed: {:?}", e),
                ),
            }
        }
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}
