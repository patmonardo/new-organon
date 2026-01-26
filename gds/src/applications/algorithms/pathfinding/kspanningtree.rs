//! KSpanningTree algorithm dispatch handler.

use crate::algo::kspanningtree::KSpanningTreeStats;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::{
    err, get_str, get_u64, timings_json, CommonRequest, Mode,
};
use crate::concurrency::TerminationFlag;
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn handle_kspanningtree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "kspanningtree";

    let common = match CommonRequest::parse(request) {
        Ok(v) => v,
        Err(message) => return err(op, "INVALID_REQUEST", &message),
    };

    let source_node = match get_u64(request, "sourceNode")
        .or_else(|| get_u64(request, "source_node"))
        .or_else(|| get_u64(request, "source"))
    {
        Some(v) => v,
        None => return err(op, "INVALID_REQUEST", "Missing 'sourceNode' parameter"),
    };

    let k = get_u64(request, "k").unwrap_or(1);

    let objective = get_str(request, "objective")
        .or_else(|| get_str(request, "obj"))
        .unwrap_or("min")
        .to_string();

    let weight_property = get_str(request, "weightProperty")
        .or_else(|| get_str(request, "weight_property"))
        .map(|s| s.to_string());

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
            let task = Tasks::leaf("kspanningtree::stream".to_string())
                .base()
                .clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let mut builder = gr
                    .facade()
                    .kspanning_tree()
                    .source_node(source_node)
                    .k(k)
                    .objective(&objective);

                if let Some(ref prop) = weight_property {
                    builder = builder.weight_property(prop);
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
                    &format!("KSpanningTree stream failed: {e}"),
                ),
            }
        }
        Mode::Stats => {
            let task = Tasks::leaf("kspanningtree::stats".to_string())
                .base()
                .clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<KSpanningTreeStats>, String> {
                let mut builder = gr
                    .facade()
                    .kspanning_tree()
                    .source_node(source_node)
                    .k(k)
                    .objective(&objective);

                if let Some(ref prop) = weight_property {
                    builder = builder.weight_property(prop);
                }

                let stats = builder.stats().map_err(|e| e.to_string())?;
                Ok(Some(stats))
            };

            let builder = FnStatsResultBuilder(
                |_gr: &GraphResources, stats: Option<KSpanningTreeStats>, timings| {
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
                    &format!("KSpanningTree stats failed: {e}"),
                ),
            }
        }
        Mode::Estimate => match common.estimate_submode.as_deref() {
            Some("memory") | None => {
                let mut builder = graph_resources
                    .facade()
                    .kspanning_tree()
                    .source_node(source_node)
                    .k(k)
                    .objective(&objective);

                if let Some(ref prop) = weight_property {
                    builder = builder.weight_property(prop);
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
                .kspanning_tree()
                .source_node(source_node)
                .k(k)
                .objective(&objective);

            if let Some(ref prop) = weight_property {
                builder = builder.weight_property(prop);
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
                    &format!("KSpanningTree mutate failed: {e:?}"),
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
                .kspanning_tree()
                .source_node(source_node)
                .k(k)
                .objective(&objective);

            if let Some(ref prop) = weight_property {
                builder = builder.weight_property(prop);
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
                    &format!("KSpanningTree write failed: {e:?}"),
                ),
            }
        }
    }
}
