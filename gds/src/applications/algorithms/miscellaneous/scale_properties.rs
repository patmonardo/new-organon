//! ScaleProperties dispatch handler.

use crate::procedures::miscellaneous::ScalePropertiesFacade;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::GraphName;
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use serde_json::{json, Value};
use std::sync::Arc;

pub fn handle_scale_properties(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "scale_properties";

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

    let mode = request.get("mode").and_then(|v| v.as_str()).unwrap_or("stream");

    let source_property = request
        .get("sourceProperty")
        .or_else(|| request.get("property"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    let facade = ScalePropertiesFacade::new(graph_store)
        .source_property(source_property.clone())
        .concurrency(concurrency);

    if mode == "estimate" {
        if source_property.is_empty() {
            return err(
                op,
                "INVALID_REQUEST",
                "Missing 'sourceProperty' (or 'property') parameter",
            );
        }
        let memory = facade.estimate_memory();
        return json!({
            "ok": true,
            "op": op,
            "data": {
                "min_bytes": memory.min(),
                "max_bytes": memory.max()
            }
        });
    }

    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<Value> = rows_iter
                    .map(|row| json!({ "nodeId": row.node_id, "value": row.value }))
                    .collect();
                json!({ "ok": true, "op": op, "data": rows })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("scaleProperties failed: {e}")),
        },
        "stats" => match facade.stats() {
            Ok(stats) => json!({ "ok": true, "op": op, "data": stats }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("scaleProperties failed: {e}")),
        },
        "mutate" | "write" => {
            let target_property = request
                .get("mutateProperty")
                .or_else(|| request.get("targetProperty"))
                .or_else(|| request.get("writeProperty"))
                .and_then(|v| v.as_str());

            let Some(target_property) = target_property else {
                return err(op, "INVALID_REQUEST", "Missing mutateProperty/targetProperty");
            };

            let out_name = request
                .get("mutateGraphName")
                .or_else(|| request.get("writeGraphName"))
                .or_else(|| request.get("targetGraphName"))
                .or_else(|| request.get("outputGraphName"))
                .and_then(|v| v.as_str())
                .unwrap_or("scale_properties");

            // Use store-level utility so we keep the stable applications+procedures boundary.
            let store: DefaultGraphStore = match catalog.get(graph_name) {
                Some(store) => store.as_ref().clone(),
                None => {
                    return err(
                        op,
                        "GRAPH_NOT_FOUND",
                        &format!("Graph '{}' not found", graph_name),
                    )
                }
            };

            match store.with_scaled_node_property_minmax(
                GraphName::new(out_name),
                &source_property,
                target_property,
                concurrency,
            ) {
                Ok(new_store) => {
                    let node_count = GraphStore::node_count(&new_store) as u64;
                    let relationship_count = GraphStore::relationship_count(&new_store) as u64;
                    catalog.set(out_name, Arc::new(new_store));
                    json!({
                        "ok": true,
                        "op": op,
                        "data": {
                            "graphName": out_name,
                            "nodeCount": node_count,
                            "relationshipCount": relationship_count,
                            "property": target_property
                        }
                    })
                }
                Err(e) => err(op, "EXECUTION_ERROR", &format!("scaleProperties failed: {e}")),
            }
        }
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
