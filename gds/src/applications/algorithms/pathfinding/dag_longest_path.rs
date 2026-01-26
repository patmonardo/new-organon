//! DAG Longest Path algorithm dispatch handler.

use crate::algo::dag_longest_path::DagLongestPathStats;
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::pathfinding::{err, timings_json, CommonRequest, Mode};
use crate::concurrency::TerminationFlag;
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn handle_dag_longest_path(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "dag_longest_path";

    let common = match CommonRequest::parse(request) {
        Ok(v) => v,
        Err(message) => return err(op, "INVALID_REQUEST", &message),
    };

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
            let task = Tasks::leaf("dag_longest_path::stream".to_string())
                .base()
                .clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let iter = gr
                    .facade()
                    .dag_longest_path()
                    .concurrency(common.concurrency.value())
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
                    &format!("DagLongestPath stream failed: {e}"),
                ),
            }
        }
        Mode::Stats => {
            let task = Tasks::leaf("dag_longest_path::stats".to_string())
                .base()
                .clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<DagLongestPathStats>, String> {
                let stats = gr
                    .facade()
                    .dag_longest_path()
                    .concurrency(common.concurrency.value())
                    .stats()
                    .map_err(|e| e.to_string())?;
                Ok(Some(stats))
            };

            let builder = FnStatsResultBuilder(
                |_gr: &GraphResources, stats: Option<DagLongestPathStats>, timings| {
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
                    &format!("DagLongestPath stats failed: {e}"),
                ),
            }
        }
        Mode::Estimate => match common.estimate_submode.as_deref() {
            Some("memory") | None => {
                let memory = graph_resources
                    .facade()
                    .dag_longest_path()
                    .concurrency(common.concurrency.value())
                    .estimate_memory();

                match memory {
                    Ok(memory) => json!({
                        "ok": true,
                        "op": op,
                        "data": {
                            "minBytes": memory.min(),
                            "maxBytes": memory.max()
                        }
                    }),
                    Err(e) => err(
                        op,
                        "EXECUTION_ERROR",
                        &format!("DagLongestPath estimate failed: {e}"),
                    ),
                }
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

            let builder = graph_resources
                .facade()
                .dag_longest_path()
                .concurrency(common.concurrency.value());

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
                    &format!("DagLongestPath mutate failed: {e:?}"),
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

            let builder = graph_resources
                .facade()
                .dag_longest_path()
                .concurrency(common.concurrency.value());

            match builder.write(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("DagLongestPath write failed: {e:?}"),
                ),
            }
        }
    }
}
