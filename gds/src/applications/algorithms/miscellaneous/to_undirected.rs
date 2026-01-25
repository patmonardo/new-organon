//! ToUndirected dispatch handler.

use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::applications::algorithms::miscellaneous::{err, timings_json};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::{CatalogLoader, GraphResources};
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::miscellaneous::ToUndirectedFacade;
use crate::types::catalog::GraphCatalog;
use crate::types::prelude::GraphStore;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Instant;

pub fn handle_to_undirected(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "to_undirected";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("mutate");

    let relationship_type = match request.get("relationshipType").and_then(|v| v.as_str()) {
        Some(rel) => rel.to_string(),
        None => {
            return err(
                op,
                "INVALID_REQUEST",
                "Missing 'relationshipType' parameter",
            )
        }
    };

    let out_name = request
        .get("mutateGraphName")
        .or_else(|| request.get("writeGraphName"))
        .or_else(|| request.get("targetGraphName"))
        .or_else(|| request.get("outputGraphName"))
        .and_then(|v| v.as_str())
        .unwrap_or("to_undirected")
        .to_string();

    let mutate_relationship_type = request
        .get("mutateRelationshipType")
        .and_then(|v| v.as_str())
        .unwrap_or("undirected")
        .to_string();

    let concurrency_value = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .filter(|v| *v > 0)
        .unwrap_or(4);

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
        "stats" => {
            let task = Tasks::leaf("to_undirected::stats".to_string())
                .base()
                .clone();
            let relationship_type = relationship_type.clone();
            let mutate_relationship_type = mutate_relationship_type.clone();
            let target = out_name.clone();

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Value>, String> {
                let stats = gr
                    .facade()
                    .to_undirected()
                    .relationship_type(relationship_type.clone())
                    .mutate_relationship_type(mutate_relationship_type.clone())
                    .mutate_graph_name(target.clone())
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
                    &format!("toUndirected stats failed: {e}"),
                ),
            }
        }
        "mutate" => {
            let facade = ToUndirectedFacade::new(Arc::clone(graph_resources.store()))
                .relationship_type(relationship_type.clone())
                .mutate_relationship_type(mutate_relationship_type.clone())
                .mutate_graph_name(out_name.clone())
                .concurrency(concurrency_value);

            match facade.to_store(&out_name) {
                Ok(store) => {
                    let node_count = GraphStore::node_count(&store) as u64;
                    let relationship_count = GraphStore::relationship_count(&store) as u64;
                    catalog.set(&out_name, Arc::new(store));
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "mutate",
                        "data": {
                            "graphName": out_name,
                            "nodeCount": node_count,
                            "relationshipCount": relationship_count,
                        }
                    })
                }
                Err(e) => err(op, "EXECUTION_ERROR", &format!("toUndirected failed: {e}")),
            }
        }
        "write" => {
            let start = Instant::now();
            let facade = ToUndirectedFacade::new(Arc::clone(graph_resources.store()))
                .relationship_type(relationship_type.clone())
                .mutate_relationship_type(mutate_relationship_type.clone())
                .mutate_graph_name(out_name.clone())
                .concurrency(concurrency_value);

            match facade.to_store(&out_name) {
                Ok(store) => {
                    let node_count = GraphStore::node_count(&store) as u64;
                    let relationship_count = GraphStore::relationship_count(&store) as u64;
                    let execution_time_ms = start.elapsed().as_millis() as u64;
                    catalog.set(&out_name, Arc::new(store));
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "write",
                        "data": {
                            "graphName": out_name,
                            "nodeCount": node_count,
                            "relationshipCount": relationship_count,
                            "writeResult": {
                                "nodesWritten": node_count,
                                "propertyName": out_name,
                                "executionTimeMs": execution_time_ms,
                            }
                        }
                    })
                }
                Err(e) => err(op, "EXECUTION_ERROR", &format!("toUndirected failed: {e}")),
            }
        }
        other => err(op, "INVALID_REQUEST", &format!("Invalid mode '{other}'")),
    }
}
