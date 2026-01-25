//! Louvain algorithm dispatch handler.
//!
//! Handles JSON requests for Louvain community detection operations,
//! delegating to the machinery stack for stream/stats and keeping mutate/write/estimate on the facade.

use crate::applications::algorithms::community::{err, timings_json};
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::CatalogLoader;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::community::louvain::LouvainFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Louvain requests
pub fn handle_louvain(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "louvain";

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

    let mutate_property = request
        .get("mutateProperty")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let write_property = request
        .get("writeProperty")
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
            let task = Tasks::leaf("louvain::stream".to_string()).base().clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let iter = gr
                    .facade()
                    .louvain()
                    .concurrency(concurrency_value)
                    .stream()
                    .map_err(|e| e.to_string())?;
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
                    &format!("Louvain stream failed: {e}"),
                ),
            }
        }
        "stats" => {
            let task = Tasks::leaf("louvain::stats".to_string()).base().clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Value>, String> {
                let stats = gr
                    .facade()
                    .louvain()
                    .concurrency(concurrency_value)
                    .stats()
                    .map_err(|e| e.to_string())?;
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
                Err(e) => err(op, "EXECUTION_ERROR", &format!("Louvain stats failed: {e}")),
            }
        }
        "mutate" => {
            let property_name = match mutate_property {
                Some(value) => value,
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'mutateProperty' parameter for mutate mode",
                    )
                }
            };

            let facade = LouvainFacade::new(Arc::clone(graph_resources.store()))
                .concurrency(concurrency_value);
            match facade.mutate(&property_name) {
                Ok(result) => {
                    catalog.set(graph_name, result.updated_store);
                    json!({
                        "ok": true,
                        "op": op,
                        "data": {
                            "nodes_updated": result.summary.nodes_updated,
                            "property_name": result.summary.property_name,
                            "execution_time_ms": result.summary.execution_time_ms
                        }
                    })
                }
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Louvain mutate failed: {:?}", e),
                ),
            }
        }
        "write" => {
            let property_name = match write_property {
                Some(value) => value,
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'writeProperty' parameter for write mode",
                    )
                }
            };

            let facade = LouvainFacade::new(Arc::clone(graph_resources.store()))
                .concurrency(concurrency_value);
            match facade.write(&property_name) {
                Ok(result) => json!({"ok": true, "op": op, "data": result}),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Louvain write failed: {:?}", e),
                ),
            }
        }
        "estimate" => {
            let facade = LouvainFacade::new(Arc::clone(graph_resources.store()))
                .concurrency(concurrency_value);
            let memory = facade.estimate_memory();
            json!({
                "ok": true,
                "op": op,
                "data": {"min_bytes": memory.min(), "max_bytes": memory.max()}
            })
        }
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}
