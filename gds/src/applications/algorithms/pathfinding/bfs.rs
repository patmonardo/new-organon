//! BFS algorithm dispatch handler.
//!
//! Handles JSON requests for BFS traversal operations,
//! delegating to the facade layer for execution.

use crate::algo::bfs::BfsStats;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::{
    err, get_bool, get_u64, get_usize, timings_json,
};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle BFS requests.
pub fn handle_bfs(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "bfs";

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

    let source = match get_u64(request, "source").or_else(|| get_u64(request, "sourceNode")) {
        Some(s) => s,
        None => return err(op, "INVALID_REQUEST", "Missing 'source' parameter"),
    };

    let targets: Vec<u64> =
        if let Some(t) = get_u64(request, "target").or_else(|| get_u64(request, "targetNode")) {
            vec![t]
        } else if let Some(arr) = request.get("targets").and_then(|v| v.as_array()) {
            arr.iter().filter_map(|v| v.as_u64()).collect()
        } else {
            Vec::new()
        };

    let max_depth = request
        .get("maxDepth")
        .or_else(|| request.get("max_depth"))
        .and_then(|v| v.as_u64())
        .map(|n| n as u32);

    let track_paths = get_bool(request, "trackPaths")
        .or_else(|| get_bool(request, "track_paths"))
        .unwrap_or(false);

    let delta = get_usize(request, "delta");

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
            let task = Tasks::leaf("bfs::stream".to_string()).base().clone();

            let targets = targets.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let mut builder = gr
                    .facade()
                    .bfs()
                    .source(source)
                    .track_paths(track_paths)
                    .concurrency(concurrency_value);

                if !targets.is_empty() {
                    builder = builder.targets(targets.clone());
                }
                if let Some(max_depth) = max_depth {
                    builder = builder.max_depth(max_depth);
                }
                if let Some(delta) = delta {
                    builder = builder.delta(delta);
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
                Err(e) => err(op, "EXECUTION_ERROR", &format!("BFS stream failed: {e}")),
            }
        }
        "stats" => {
            let task = Tasks::leaf("bfs::stats".to_string()).base().clone();

            let targets = targets.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<BfsStats>, String> {
                let mut builder = gr
                    .facade()
                    .bfs()
                    .source(source)
                    .track_paths(track_paths)
                    .concurrency(concurrency_value);

                if !targets.is_empty() {
                    builder = builder.targets(targets.clone());
                }
                if let Some(max_depth) = max_depth {
                    builder = builder.max_depth(max_depth);
                }
                if let Some(delta) = delta {
                    builder = builder.delta(delta);
                }

                let stats = builder.stats().map_err(|e| e.to_string())?;
                Ok(Some(stats))
            };

            let builder =
                FnStatsResultBuilder(|_gr: &GraphResources, stats: Option<BfsStats>, timings| {
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
                Err(e) => err(op, "EXECUTION_ERROR", &format!("BFS stats failed: {e}")),
            }
        }
        "estimate" => {
            let mut builder = graph_resources.facade().bfs().source(source);
            if !targets.is_empty() {
                builder = builder.targets(targets);
            }
            if let Some(max_depth) = max_depth {
                builder = builder.max_depth(max_depth);
            }
            builder = builder.track_paths(track_paths);
            builder = builder.concurrency(concurrency_value);
            if let Some(delta) = delta {
                builder = builder.delta(delta);
            }

            let memory = builder.estimate_memory();
            json!({
                "ok": true,
                "op": op,
                "data": {
                    "minBytes": memory.min(),
                    "maxBytes": memory.max()
                }
            })
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

            let mut builder = graph_resources
                .facade()
                .bfs()
                .source(source)
                .track_paths(track_paths)
                .concurrency(concurrency_value);

            if !targets.is_empty() {
                builder = builder.targets(targets);
            }
            if let Some(max_depth) = max_depth {
                builder = builder.max_depth(max_depth);
            }
            if let Some(delta) = delta {
                builder = builder.delta(delta);
            }

            match builder.mutate(property_name) {
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
                Err(e) => err(op, "EXECUTION_ERROR", &format!("BFS mutate failed: {e:?}")),
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

            let mut builder = graph_resources
                .facade()
                .bfs()
                .source(source)
                .track_paths(track_paths)
                .concurrency(concurrency_value);

            if !targets.is_empty() {
                builder = builder.targets(targets);
            }
            if let Some(max_depth) = max_depth {
                builder = builder.max_depth(max_depth);
            }
            if let Some(delta) = delta {
                builder = builder.delta(delta);
            }

            match builder.write(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(op, "EXECUTION_ERROR", &format!("BFS write failed: {e:?}")),
            }
        }
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}
