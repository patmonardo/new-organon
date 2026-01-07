//! ToUndirected dispatch handler.

use crate::procedures::miscellaneous::ToUndirectedFacade;
use crate::types::catalog::GraphCatalog;
use crate::types::prelude::GraphStore;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn handle_to_undirected(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "to_undirected";

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

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("mutate");

    let relationship_type = match request.get("relationshipType").and_then(|v| v.as_str()) {
        Some(rel) => rel,
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
        .unwrap_or("to_undirected");

    let mutate_relationship_type = request
        .get("mutateRelationshipType")
        .and_then(|v| v.as_str())
        .unwrap_or("undirected");

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .filter(|v| *v > 0)
        .unwrap_or(4);

    let facade = ToUndirectedFacade::new(graph_store)
        .relationship_type(relationship_type)
        .mutate_relationship_type(mutate_relationship_type)
        .mutate_graph_name(out_name)
        .concurrency(concurrency);

    match mode {
        "stats" => match facade.stats() {
            Ok(stats) => json!({ "ok": true, "op": op, "data": stats }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("toUndirected failed: {e}")),
        },
        "mutate" | "write" => match facade.to_store(out_name) {
            Ok(store) => {
                let node_count = GraphStore::node_count(&store) as u64;
                let relationship_count = GraphStore::relationship_count(&store) as u64;
                catalog.set(out_name, Arc::new(store));
                json!({
                    "ok": true,
                    "op": op,
                    "data": {
                        "graphName": out_name,
                        "nodeCount": node_count,
                        "relationshipCount": relationship_count
                    }
                })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("toUndirected failed: {e}")),
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
