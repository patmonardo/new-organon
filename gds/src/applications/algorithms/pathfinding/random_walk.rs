//! Random Walk algorithm dispatch handler.

use crate::algo::random_walk::RandomWalkStats;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::{
    err, get_u64, get_usize, timings_json, CommonRequest, Mode,
};
use crate::concurrency::TerminationFlag;
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

fn get_f64(request: &Value, key: &str) -> Option<f64> {
    request.get(key).and_then(|v| v.as_f64())
}

pub fn handle_random_walk(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "random_walk";

    let common = match CommonRequest::parse(request) {
        Ok(v) => v,
        Err(message) => return err(op, "INVALID_REQUEST", &message),
    };

    let walks_per_node = get_usize(request, "walksPerNode")
        .or_else(|| get_usize(request, "walks_per_node"))
        .unwrap_or(10);

    let walk_length = get_usize(request, "walkLength")
        .or_else(|| get_usize(request, "walk_length"))
        .unwrap_or(80);

    let return_factor = get_f64(request, "returnFactor")
        .or_else(|| get_f64(request, "return_factor"))
        .unwrap_or(1.0);

    let in_out_factor = get_f64(request, "inOutFactor")
        .or_else(|| get_f64(request, "in_out_factor"))
        .unwrap_or(1.0);

    let source_nodes: Vec<u64> = request
        .get("sourceNodes")
        .or_else(|| request.get("source_nodes"))
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_u64()).collect())
        .unwrap_or_default();

    let random_seed = get_u64(request, "randomSeed").or_else(|| get_u64(request, "random_seed"));

    let graph_resources = match CatalogLoader::load_or_err(catalog.as_ref(), &common.graph_name) {
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

    match common.mode {
        Mode::Stream => {
            let task = Tasks::leaf("random_walk::stream".to_string())
                .base()
                .clone();

            let source_nodes = source_nodes.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let builder = gr
                    .facade()
                    .random_walk()
                    .walks_per_node(walks_per_node)
                    .walk_length(walk_length)
                    .return_factor(return_factor)
                    .in_out_factor(in_out_factor)
                    .source_nodes(source_nodes.clone())
                    .concurrency(common.concurrency.value());

                let builder = match random_seed {
                    Some(seed) => builder.random_seed(seed),
                    None => builder,
                };

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
                common.concurrency,
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
                    &format!("RandomWalk stream failed: {e}"),
                ),
            }
        }
        Mode::Stats => {
            let task = Tasks::leaf("random_walk::stats".to_string()).base().clone();

            let source_nodes = source_nodes.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<RandomWalkStats>, String> {
                let builder = gr
                    .facade()
                    .random_walk()
                    .walks_per_node(walks_per_node)
                    .walk_length(walk_length)
                    .return_factor(return_factor)
                    .in_out_factor(in_out_factor)
                    .source_nodes(source_nodes.clone())
                    .concurrency(common.concurrency.value());

                let builder = match random_seed {
                    Some(seed) => builder.random_seed(seed),
                    None => builder,
                };

                let stats = builder.stats().map_err(|e| e.to_string())?;
                Ok(Some(stats))
            };

            let builder = FnStatsResultBuilder(
                |_gr: &GraphResources, stats: Option<RandomWalkStats>, timings| {
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "stats",
                        "data": stats,
                        "timings": timings_json(timings)
                    })
                },
            );

            match convenience.process_stats(
                &graph_resources,
                common.concurrency,
                task,
                compute,
                builder,
            ) {
                Ok(v) => v,
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("RandomWalk stats failed: {e}"),
                ),
            }
        }
        Mode::Estimate => match common.estimate_submode.as_deref() {
            Some("memory") | None => {
                let builder = graph_resources
                    .facade()
                    .random_walk()
                    .walks_per_node(walks_per_node)
                    .walk_length(walk_length)
                    .return_factor(return_factor)
                    .in_out_factor(in_out_factor)
                    .source_nodes(source_nodes)
                    .concurrency(common.concurrency.value());

                let builder = match random_seed {
                    Some(seed) => builder.random_seed(seed),
                    None => builder,
                };

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
        Mode::Mutate => {
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
                .random_walk()
                .walks_per_node(walks_per_node)
                .walk_length(walk_length)
                .return_factor(return_factor)
                .in_out_factor(in_out_factor)
                .concurrency(common.concurrency.value());

            if !source_nodes.is_empty() {
                builder = builder.source_nodes(source_nodes);
            }
            if let Some(seed) = random_seed {
                builder = builder.random_seed(seed);
            }

            match builder.mutate(property_name) {
                Ok(result) => {
                    catalog.set(&common.graph_name, result.updated_store);
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
                    &format!("RandomWalk mutate failed: {e:?}"),
                ),
            }
        }
        Mode::Write => {
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
                .random_walk()
                .walks_per_node(walks_per_node)
                .walk_length(walk_length)
                .return_factor(return_factor)
                .in_out_factor(in_out_factor)
                .concurrency(common.concurrency.value());

            if !source_nodes.is_empty() {
                builder = builder.source_nodes(source_nodes);
            }
            if let Some(seed) = random_seed {
                builder = builder.random_seed(seed);
            }

            match builder.write(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("RandomWalk write failed: {e:?}"),
                ),
            }
        }
    }
}
