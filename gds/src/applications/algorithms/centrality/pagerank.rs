//! PageRank centrality algorithm dispatch handler.
//!
//! Handles JSON requests for PageRank operations,
//! delegating to the facade layer for execution.

use crate::applications::algorithms::centrality::{err, timings_json};
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::CatalogLoader;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::centrality::pagerank::PageRankFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle PageRank requests
pub fn handle_pagerank(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "pagerank";

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

    let direction = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing")
        .to_string();

    let source_nodes = request
        .get("sourceNodes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_u64()).collect::<Vec<_>>())
        .unwrap_or_default();

    let iterations = request
        .get("iterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(20) as u32;

    let damping_factor = request
        .get("dampingFactor")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.85);

    let tolerance = request
        .get("tolerance")
        .and_then(|v| v.as_f64())
        .unwrap_or(1e-4);

    let estimate_submode = request.get("submode").and_then(|v| v.as_str());

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
            let stream_direction = direction.clone();
            let stream_source_nodes = source_nodes.clone();

            let task = Tasks::leaf("pagerank::stream".to_string()).base().clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let mut builder = gr
                    .facade()
                    .pagerank()
                    .direction(&stream_direction)
                    .iterations(iterations)
                    .damping_factor(damping_factor)
                    .tolerance(tolerance)
                    .concurrency(concurrency_value);

                if !stream_source_nodes.is_empty() {
                    builder = builder.source_nodes(stream_source_nodes.clone());
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
                    &format!("PageRank execution failed: {e}"),
                ),
            }
        }
        "stats" => {
            let stats_direction = direction.clone();
            let stats_source_nodes = source_nodes.clone();

            let task = Tasks::leaf("pagerank::stats".to_string()).base().clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Value>, String> {
                let mut builder = gr
                    .facade()
                    .pagerank()
                    .direction(&stats_direction)
                    .iterations(iterations)
                    .damping_factor(damping_factor)
                    .tolerance(tolerance)
                    .concurrency(concurrency_value);

                if !stats_source_nodes.is_empty() {
                    builder = builder.source_nodes(stats_source_nodes.clone());
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
                Ok(v) => v,
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("PageRank stats failed: {e}"),
                ),
            }
        }
        "mutate" => {
            let property_name = match request.get("mutateProperty").and_then(|v| v.as_str()) {
                Some(name) => name,
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'mutateProperty' parameter for mutate mode",
                    )
                }
            };
            let mut facade = PageRankFacade::new(Arc::clone(graph_resources.store()))
                .direction(&direction)
                .iterations(iterations)
                .damping_factor(damping_factor)
                .tolerance(tolerance)
                .concurrency(concurrency_value);
            if !source_nodes.is_empty() {
                facade = facade.source_nodes(source_nodes.clone());
            }
            match facade.mutate(property_name) {
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
                    &format!("PageRank mutate failed: {:?}", e),
                ),
            }
        }
        "write" => {
            let property_name = match request.get("writeProperty").and_then(|v| v.as_str()) {
                Some(name) => name,
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'writeProperty' parameter for write mode",
                    )
                }
            };
            let mut facade = PageRankFacade::new(Arc::clone(graph_resources.store()))
                .direction(&direction)
                .iterations(iterations)
                .damping_factor(damping_factor)
                .tolerance(tolerance)
                .concurrency(concurrency_value);
            if !source_nodes.is_empty() {
                facade = facade.source_nodes(source_nodes.clone());
            }
            match facade.write(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("PageRank write failed: {:?}", e),
                ),
            }
        }
        "estimate" => match estimate_submode {
            Some("memory") => {
                let mut facade = PageRankFacade::new(Arc::clone(graph_resources.store()))
                    .direction(&direction)
                    .iterations(iterations)
                    .damping_factor(damping_factor)
                    .tolerance(tolerance)
                    .concurrency(concurrency_value);
                if !source_nodes.is_empty() {
                    facade = facade.source_nodes(source_nodes.clone());
                }
                let memory = facade.estimate_memory();
                json!({
                    "ok": true,
                    "op": op,
                    "data": {
                        "min_bytes": memory.min(),
                        "max_bytes": memory.max()
                    }
                })
            }
            Some(other) => err(
                op,
                "INVALID_REQUEST",
                &format!("Invalid estimate submode '{other}'. Use 'memory'"),
            ),
            None => err(
                op,
                "INVALID_REQUEST",
                "Missing 'submode' parameter for estimate mode",
            ),
        },
        _ => err(
            op,
            "INVALID_REQUEST",
            "Invalid mode. Use 'stream', 'stats', 'mutate', 'write', or 'estimate'",
        ),
    }
}
