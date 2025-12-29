//! Yen's K-Shortest Paths pathfinding algorithm dispatch handler.
//!
//! Handles JSON requests for Yen's operations,
//! delegating to the facade layer for execution.

use crate::procedures::facades::pathfinding::yens::YensBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Yen's requests
pub fn handle_yens(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "yens";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let source = match request.get("source").and_then(|v| v.as_u64()) {
        Some(s) => s,
        None => return err(op, "INVALID_REQUEST", "Missing 'source' parameter"),
    };

    let target = match request.get("target").and_then(|v| v.as_u64()) {
        Some(t) => t,
        None => return err(op, "INVALID_REQUEST", "Missing 'target' parameter"),
    };

    let k = request.get("k").and_then(|v| v.as_u64()).unwrap_or(3) as usize;

    let weight_property = request
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("weight");

    let relationship_types = request
        .get("relationshipTypes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or(vec![]);

    let direction = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing");

    let track_relationships = request
        .get("trackRelationships")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    // Get graph store
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

    // Create builder
    let builder = YensBuilder::new(graph_store)
        .source(source)
        .target(target)
        .k(k)
        .weight_property(weight_property)
        .relationship_types(relationship_types)
        .direction(direction)
        .track_relationships(track_relationships)
        .concurrency(concurrency);

    // Execute based on mode
    match mode {
        "stream" => match builder.stream() {
            Ok(rows_iter) => {
                let rows: Vec<_> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Yen's execution failed: {:?}", e),
            ),
        },
        "stats" => match builder.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Yen's stats failed: {:?}", e),
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
