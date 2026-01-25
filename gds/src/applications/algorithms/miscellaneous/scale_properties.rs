//! ScaleProperties dispatch handler.

use crate::algo::scale_properties::ScalePropertiesScaler;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::miscellaneous::{err, timings_json};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn handle_scale_properties(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "scale_properties";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let mut node_properties: Vec<String> = request
        .get("nodeProperties")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    if node_properties.is_empty() {
        if let Some(prop) = request
            .get("sourceProperty")
            .or_else(|| request.get("property"))
            .and_then(|v| v.as_str())
        {
            node_properties.push(prop.to_string());
        }
    }

    let scaler = match request
        .get("scaler")
        .or_else(|| request.get("variant"))
        .and_then(|v| v.as_str())
    {
        Some("stdScore") => ScalePropertiesScaler::StdScore,
        Some("mean") => ScalePropertiesScaler::Mean,
        Some("max") => ScalePropertiesScaler::Max,
        Some("center") => ScalePropertiesScaler::Center,
        Some("log") => ScalePropertiesScaler::Log,
        Some("none") => ScalePropertiesScaler::None,
        _ => ScalePropertiesScaler::MinMax,
    };

    let concurrency_value = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .filter(|v| *v > 0)
        .unwrap_or(1);

    if Concurrency::new(concurrency_value).is_none() {
        return err(
            op,
            "INVALID_REQUEST",
            "concurrency must be greater than zero",
        );
    }

    let concurrency = Concurrency::new(concurrency_value).unwrap();

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
            let task = Tasks::leaf("scale_properties::stream".to_string())
                .base()
                .clone();
            let node_properties = node_properties.clone();
            let scaler = scaler.clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let iter = gr
                    .facade()
                    .scale_properties()
                    .node_properties(node_properties.clone())
                    .scaler(scaler.clone())
                    .concurrency(concurrency_value)
                    .stream()
                    .map_err(|e| e.to_string())?;

                let rows = iter
                    .map(|row| json!({ "nodeId": row.node_id, "values": row.values }))
                    .collect::<Vec<_>>();
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
                            "sideEffectMillis": 0,
                        }),
                    })
                }
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("scaleProperties stream failed: {e}"),
                ),
            }
        }
        "stats" => {
            let task = Tasks::leaf("scale_properties::stats".to_string())
                .base()
                .clone();
            let node_properties = node_properties.clone();
            let scaler = scaler.clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Value>, String> {
                let stats = gr
                    .facade()
                    .scale_properties()
                    .node_properties(node_properties.clone())
                    .scaler(scaler.clone())
                    .concurrency(concurrency_value)
                    .stats()
                    .map_err(|e| e.to_string())?;
                serde_json::to_value(stats)
                    .map(Some)
                    .map_err(|e| e.to_string())
            };

            let builder =
                FnStatsResultBuilder(|_gr: &GraphResources, stats: Option<Value>, timings| {
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "stats",
                        "data": stats,
                        "timings": timings_json(timings),
                    })
                });

            match convenience.process_stats(&graph_resources, concurrency, task, compute, builder) {
                Ok(response) => response,
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("scaleProperties stats failed: {e}"),
                ),
            }
        }
        "estimate" => {
            let facade = graph_resources
                .facade()
                .scale_properties()
                .node_properties(node_properties.clone())
                .scaler(scaler.clone())
                .concurrency(concurrency_value);

            let memory = facade.estimate_memory();
            json!({
                "ok": true,
                "op": op,
                "data": {
                    "min_bytes": memory.min(),
                    "max_bytes": memory.max(),
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

            let facade = graph_resources
                .facade()
                .scale_properties()
                .node_properties(node_properties.clone())
                .scaler(scaler.clone())
                .concurrency(concurrency_value);

            match facade.mutate(property_name) {
                Ok(result) => {
                    catalog.set(graph_name, result.updated_store);
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "mutate",
                        "data": {
                            "nodesUpdated": result.summary.nodes_updated,
                            "propertyName": result.summary.property_name,
                            "executionTimeMs": result.summary.execution_time_ms,
                        }
                    })
                }
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("scaleProperties mutate failed: {e}"),
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

            let facade = graph_resources
                .facade()
                .scale_properties()
                .node_properties(node_properties.clone())
                .scaler(scaler.clone())
                .concurrency(concurrency_value);

            match facade.write(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "mode": "write",
                    "data": result,
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("scaleProperties write failed: {e}"),
                ),
            }
        }
        other => err(op, "INVALID_REQUEST", &format!("Invalid mode '{other}'")),
    }
}
