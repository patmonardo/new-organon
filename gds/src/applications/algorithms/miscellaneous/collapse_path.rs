//! CollapsePath dispatch handler.

use crate::applications::algorithms::miscellaneous::err;
use crate::concurrency::Concurrency;
use crate::core::loading::CatalogLoader;
use crate::procedures::miscellaneous::CollapsePathFacade;
use crate::types::catalog::GraphCatalog;
use crate::types::prelude::GraphStore;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Instant;

pub fn handle_collapse_path(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "collapse_path";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("mutate");

    let out_name = request
        .get("mutateGraphName")
        .or_else(|| request.get("writeGraphName"))
        .or_else(|| request.get("targetGraphName"))
        .or_else(|| request.get("outputGraphName"))
        .and_then(|v| v.as_str())
        .unwrap_or("collapse_path")
        .to_string();

    let path_templates: Vec<Vec<String>> = match request.get("pathTemplates") {
        Some(Value::Array(paths)) => paths
            .iter()
            .filter_map(|p| match p {
                Value::Array(inner) => Some(
                    inner
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>(),
                ),
                _ => None,
            })
            .collect(),
        _ => Vec::new(),
    };

    let mutate_relationship_type = request
        .get("mutateRelationshipType")
        .and_then(|v| v.as_str())
        .unwrap_or("collapsed")
        .to_string();

    let allow_self_loops = request
        .get("allowSelfLoops")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

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

    let graph_resources = match CatalogLoader::load_or_err(catalog.as_ref(), graph_name) {
        Ok(resources) => resources,
        Err(e) => return err(op, "GRAPH_NOT_FOUND", &e.to_string()),
    };

    match mode {
        "stats" => {
            let facade = CollapsePathFacade::new(Arc::clone(graph_resources.store()))
                .path_templates(path_templates)
                .mutate_relationship_type(mutate_relationship_type)
                .mutate_graph_name(out_name.clone())
                .allow_self_loops(allow_self_loops)
                .concurrency(concurrency_value);

            match facade.stats(&out_name) {
                Ok(stats) => json!({
                    "ok": true,
                    "op": op,
                    "mode": "stats",
                    "data": stats,
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("collapsePath stats failed: {e}"),
                ),
            }
        }
        "mutate" => {
            let facade = CollapsePathFacade::new(Arc::clone(graph_resources.store()))
                .path_templates(path_templates)
                .mutate_relationship_type(mutate_relationship_type)
                .mutate_graph_name(out_name.clone())
                .allow_self_loops(allow_self_loops)
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
                Err(e) => err(op, "EXECUTION_ERROR", &format!("collapsePath failed: {e}")),
            }
        }
        "write" => {
            let start = Instant::now();
            let facade = CollapsePathFacade::new(Arc::clone(graph_resources.store()))
                .path_templates(path_templates)
                .mutate_relationship_type(mutate_relationship_type)
                .mutate_graph_name(out_name.clone())
                .allow_self_loops(allow_self_loops)
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
                Err(e) => err(op, "EXECUTION_ERROR", &format!("collapsePath failed: {e}")),
            }
        }
        other => err(op, "INVALID_REQUEST", &format!("Invalid mode '{other}'")),
    }
}
