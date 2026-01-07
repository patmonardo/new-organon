//! CollapsePath dispatch handler.

use crate::procedures::miscellaneous::CollapsePathFacade;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::GraphStore;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn handle_collapse_path(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "collapse_path";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let graph_store = match catalog.get(graph_name) {
        Some(store) => store,
        None => {
            return err(
                op,
                "GRAPH_NOT_FOUND",
                &format!("Graph '{}' not found", graph_name),
            )
        }
    };

    let mode = request.get("mode").and_then(|v| v.as_str()).unwrap_or("mutate");

    let out_name = request
        .get("mutateGraphName")
        .or_else(|| request.get("writeGraphName"))
        .or_else(|| request.get("targetGraphName"))
        .or_else(|| request.get("outputGraphName"))
        .and_then(|v| v.as_str())
        .unwrap_or("collapse_path");

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

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .unwrap_or(4);

    let facade = CollapsePathFacade::new(graph_store)
        .path_templates(path_templates)
        .mutate_relationship_type(mutate_relationship_type)
        .mutate_graph_name(out_name)
        .allow_self_loops(allow_self_loops)
        .concurrency(concurrency);

    match mode {
        "mutate" | "write" => match facade.to_store(out_name) {
            Ok(store) => {
                let node_count = store.node_count() as u64;
                let relationship_count = store.relationship_count() as u64;
                catalog.set(out_name, Arc::new(store));
                json!({
                    "ok": true,
                    "op": op,
                    "data": {
                        "graphName": out_name,
                        "nodeCount": node_count,
                        "relationshipCount": relationship_count,
                    }
                })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("collapsePath failed: {e}")),
        },
        other => err(op, "INVALID_REQUEST", &format!("Invalid mode '{other}'")),
    }
}

fn err(op: &str, code: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message }
    })
}
