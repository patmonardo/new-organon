//! Leiden algorithm dispatch handler.
//!
//! Handles JSON requests for Leiden community detection operations,
//! delegating to the facade layer for execution.

use crate::applications::algorithms::community::shared::{err, timings_json};
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator,
    RequestScopedDependencies,
};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::CatalogLoader;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::community::leiden::LeidenFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Leiden requests
pub fn handle_leiden(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "leiden";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let gamma = request.get("gamma").and_then(|v| v.as_f64()).unwrap_or(1.0);

    let theta = request
        .get("theta")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.01);

    let tolerance = request
        .get("tolerance")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0001);

    let max_iterations = request
        .get("maxIterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as usize;

    let random_seed = request
        .get("randomSeed")
        .and_then(|v| v.as_u64())
        .unwrap_or(42);

    let concurrency_value = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

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
            let task = Tasks::leaf("leiden::stream".to_string()).base().clone();

            let compute = move |gr: &crate::core::loading::GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let iter = gr
                    .facade()
                    .leiden()
                    .gamma(gamma)
                    .theta(theta)
                    .tolerance(tolerance)
                    .max_iterations(max_iterations)
                    .random_seed(random_seed)
                    .stream()
                    .map_err(|e| e.to_string())?;
                let rows = iter
                    .map(|row| serde_json::to_value(row).map_err(|e| e.to_string()))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Some(rows))
            };

            let result_builder = FnStreamResultBuilder::new(
                |_gr: &crate::core::loading::GraphResources, rows: Option<Vec<Value>>| {
                    rows.unwrap_or_default().into_iter()
                },
            );

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
                    &format!("Leiden stream failed: {e}"),
                ),
            }
        }
        "stats" => {
            let task = Tasks::leaf("leiden::stats".to_string()).base().clone();

            let compute = move |gr: &crate::core::loading::GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Value>, String> {
                let stats = gr
                    .facade()
                    .leiden()
                    .gamma(gamma)
                    .theta(theta)
                    .tolerance(tolerance)
                    .max_iterations(max_iterations)
                    .random_seed(random_seed)
                    .stats()
                    .map_err(|e| e.to_string())?;
                let stats_value = serde_json::to_value(stats).map_err(|e| e.to_string())?;
                Ok(Some(stats_value))
            };

            let builder = FnStatsResultBuilder(
                |_gr: &crate::core::loading::GraphResources, stats: Option<Value>, timings| {
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "stats",
                        "data": stats,
                        "timings": timings_json(timings)
                    })
                },
            );

            match convenience.process_stats(&graph_resources, concurrency, task, compute, builder) {
                Ok(response) => response,
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Leiden stats failed: {e}"),
                ),
            }
        }
        "mutate" => {
            let facade = LeidenFacade::new(Arc::clone(graph_resources.store()));
            match facade.mutate() {
                Ok(result) => json!({"ok": true, "op": op, "data": result}),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Leiden mutate failed: {:?}", e),
                ),
            }
        }
        "write" => {
            let facade = LeidenFacade::new(Arc::clone(graph_resources.store()));
            match facade.write() {
                Ok(result) => json!({"ok": true, "op": op, "data": result}),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Leiden write failed: {:?}", e),
                ),
            }
        }
        "estimate" => {
            let facade = LeidenFacade::new(Arc::clone(graph_resources.store()));
            match facade.estimate_memory() {
                Ok(range) => json!({"ok": true, "op": op, "data": range}),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Leiden memory estimation failed: {:?}", e),
                ),
            }
        }
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}
