//! All Shortest Paths algorithm dispatch handler.

use crate::algo::all_shortest_paths::AllShortestPathsStats;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::{
    err, get_str, timings_json, CommonRequest, Mode,
};
use crate::concurrency::TerminationFlag;
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn handle_all_shortest_paths(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "all_shortest_paths";

    let common = match CommonRequest::parse(request) {
        Ok(v) => v,
        Err(message) => return err(op, "INVALID_REQUEST", &message),
    };

    let weighted = request
        .get("weighted")
        .or_else(|| request.get("useWeights"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let direction = get_str(request, "direction")
        .or_else(|| get_str(request, "traversalDirection"))
        .unwrap_or("outgoing")
        .to_string();

    let weight_property = get_str(request, "weightProperty")
        .or_else(|| get_str(request, "weight_property"))
        .or_else(|| get_str(request, "relationshipWeightProperty"))
        .or_else(|| get_str(request, "relationship_weight_property"))
        .unwrap_or("weight")
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

    let max_results = request
        .get("maxResults")
        .or_else(|| request.get("max_results"))
        .and_then(|v| v.as_u64())
        .map(|v| v as usize);

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
            let task = Tasks::leaf("all_shortest_paths::stream".to_string())
                .base()
                .clone();
            let relationship_types = relationship_types.clone();
            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let mut builder = gr
                    .facade()
                    .all_shortest_paths()
                    .weighted(weighted)
                    .direction(&direction)
                    .weight_property(&weight_property)
                    .concurrency(common.concurrency.value());

                if !relationship_types.is_empty() {
                    builder = builder.relationship_types(relationship_types.clone());
                }
                if let Some(max) = max_results {
                    builder = builder.max_results(max);
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
                    &format!("AllShortestPaths stream failed: {e}"),
                ),
            }
        }
        Mode::Stats => {
            let task = Tasks::leaf("all_shortest_paths::stats".to_string())
                .base()
                .clone();
            let relationship_types = relationship_types.clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<AllShortestPathsStats>, String> {
                let mut builder = gr
                    .facade()
                    .all_shortest_paths()
                    .weighted(weighted)
                    .direction(&direction)
                    .weight_property(&weight_property)
                    .concurrency(common.concurrency.value());

                if !relationship_types.is_empty() {
                    builder = builder.relationship_types(relationship_types.clone());
                }
                if let Some(max) = max_results {
                    builder = builder.max_results(max);
                }

                let stats = builder.stats().map_err(|e| e.to_string())?;
                Ok(Some(stats))
            };

            let builder = FnStatsResultBuilder(
                |_gr: &GraphResources, stats: Option<AllShortestPathsStats>, timings| {
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
                    &format!("AllShortestPaths stats failed: {e}"),
                ),
            }
        }
        Mode::Estimate => match common.estimate_submode.as_deref() {
            Some("memory") | None => {
                let mut builder = graph_resources
                    .facade()
                    .all_shortest_paths()
                    .weighted(weighted)
                    .direction(&direction)
                    .weight_property(&weight_property)
                    .concurrency(common.concurrency.value());

                if !relationship_types.is_empty() {
                    builder = builder.relationship_types(relationship_types);
                }
                if let Some(max) = max_results {
                    builder = builder.max_results(max);
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
                .all_shortest_paths()
                .weighted(weighted)
                .direction(&direction)
                .weight_property(&weight_property)
                .concurrency(common.concurrency.value());

            if !relationship_types.is_empty() {
                builder = builder.relationship_types(relationship_types);
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
                    &format!("AllShortestPaths mutate failed: {e:?}"),
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
                .all_shortest_paths()
                .weighted(weighted)
                .direction(&direction)
                .weight_property(&weight_property)
                .concurrency(common.concurrency.value());

            if !relationship_types.is_empty() {
                builder = builder.relationship_types(relationship_types);
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
                    &format!("AllShortestPaths write failed: {e:?}"),
                ),
            }
        }
    }
}
