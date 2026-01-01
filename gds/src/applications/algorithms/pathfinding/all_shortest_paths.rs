//! All Shortest Paths pathfinding algorithm dispatch handler.
//!
//! Handles JSON requests for all shortest paths operations,
//! delegating to the facade layer for execution.

use crate::procedures::pathfinding::all_shortest_paths::AllShortestPathsBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle All Shortest Paths requests
pub fn handle_all_shortest_paths(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "allShortestPaths";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let weighted = request
        .get("weighted")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let relationship_types =
        if let Some(types) = request.get("relationshipTypes").and_then(|v| v.as_array()) {
            types
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

    let direction = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing");

    let weight_property = request
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("weight");

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    let max_results = request
        .get("maxResults")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

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
    let mut builder = AllShortestPathsBuilder::new(graph_store)
        .weighted(weighted)
        .relationship_types(relationship_types)
        .direction(direction)
        .weight_property(weight_property)
        .concurrency(concurrency);

    if let Some(max) = max_results {
        builder = builder.max_results(max);
    }

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
                &format!("All Shortest Paths execution failed: {:?}", e),
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
                &format!("All Shortest Paths stats failed: {:?}", e),
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
