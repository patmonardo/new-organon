//! Bellman-Ford algorithm dispatch handler.

use crate::algo::bellman_ford::BellmanFordStats;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::{err, get_bool, get_str, get_u64, timings_json};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn handle_bellman_ford(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "bellman_ford";

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

    let estimate_submode = request.get("submode").and_then(|v| v.as_str());

    let source = match get_u64(request, "source").or_else(|| get_u64(request, "sourceNode")) {
        Some(s) => s,
        None => return err(op, "INVALID_REQUEST", "Missing 'source' parameter"),
    };

    let weight_property = get_str(request, "weightProperty")
        .or_else(|| get_str(request, "weight_property"))
        .or_else(|| get_str(request, "relationshipWeightProperty"))
        .or_else(|| get_str(request, "relationship_weight_property"))
        .unwrap_or("weight")
        .to_string();

    let direction = get_str(request, "direction")
        .or_else(|| get_str(request, "traversalDirection"))
        .unwrap_or("outgoing")
        .to_string();

    let relationship_types: Vec<String> = request
        .get("relationshipTypes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let track_negative_cycles = get_bool(request, "trackNegativeCycles")
        .or_else(|| get_bool(request, "track_negative_cycles"))
        .unwrap_or(true);

    let track_paths = get_bool(request, "trackPaths")
        .or_else(|| get_bool(request, "track_paths"))
        .unwrap_or(true);

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
            let task = Tasks::leaf("bellman_ford::stream".to_string())
                .base()
                .clone();
            let relationship_types = relationship_types.clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let mut builder = gr
                    .facade()
                    .bellman_ford()
                    .source(source)
                    .weight_property(&weight_property)
                    .direction(&direction)
                    .track_negative_cycles(track_negative_cycles)
                    .track_paths(track_paths)
                    .concurrency(concurrency_value);

                if !relationship_types.is_empty() {
                    builder = builder.relationship_types(relationship_types.clone());
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
                    &format!("Bellman-Ford stream failed: {e}"),
                ),
            }
        }
        "stats" => {
            let task = Tasks::leaf("bellman_ford::stats".to_string())
                .base()
                .clone();
            let relationship_types = relationship_types.clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<BellmanFordStats>, String> {
                let mut builder = gr
                    .facade()
                    .bellman_ford()
                    .source(source)
                    .weight_property(&weight_property)
                    .direction(&direction)
                    .track_negative_cycles(track_negative_cycles)
                    .track_paths(track_paths)
                    .concurrency(concurrency_value);

                if !relationship_types.is_empty() {
                    builder = builder.relationship_types(relationship_types.clone());
                }

                let stats = builder.stats().map_err(|e| e.to_string())?;
                Ok(Some(stats))
            };

            let builder = FnStatsResultBuilder(
                |_gr: &GraphResources, stats: Option<BellmanFordStats>, timings| {
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
                Ok(v) => v,
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Bellman-Ford stats failed: {e}"),
                ),
            }
        }
        "estimate" => match estimate_submode {
            Some("memory") | None => {
                let mut builder = graph_resources
                    .facade()
                    .bellman_ford()
                    .source(source)
                    .weight_property(&weight_property)
                    .direction(&direction)
                    .track_negative_cycles(track_negative_cycles)
                    .track_paths(track_paths)
                    .concurrency(concurrency_value);

                if !relationship_types.is_empty() {
                    builder = builder.relationship_types(relationship_types);
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
            Some(other) => err(
                op,
                "INVALID_REQUEST",
                &format!("Invalid estimate submode '{other}'. Use 'memory'"),
            ),
        },
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

            let builder = graph_resources
                .facade()
                .bellman_ford()
                .source(source)
                .weight_property(&weight_property)
                .direction(&direction)
                .track_paths(track_paths)
                .concurrency(concurrency_value);

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
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Bellman-Ford mutate failed: {e:?}"),
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

            let builder = graph_resources
                .facade()
                .bellman_ford()
                .source(source)
                .weight_property(&weight_property)
                .direction(&direction)
                .track_paths(track_paths)
                .concurrency(concurrency_value);

            match builder.write(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Bellman-Ford write failed: {e:?}"),
                ),
            }
        }
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}
