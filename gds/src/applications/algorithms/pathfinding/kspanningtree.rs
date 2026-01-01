//! K-Spanning Tree pathfinding algorithm dispatch handler.
//!
//! Handles JSON requests for K-Spanning Tree operations,
//! delegating to the facade layer for execution.

use crate::procedures::pathfinding::kspanningtree::KSpanningTreeBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle K-Spanning Tree requests
pub fn handle_kspanningtree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "kspanningtree";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let source_node = match request.get("sourceNode").and_then(|v| v.as_u64()) {
        Some(s) => s,
        None => {
            return err(
                op,
                "INVALID_REQUEST",
                "Missing or invalid 'sourceNode' parameter",
            )
        }
    };

    let k = request.get("k").and_then(|v| v.as_u64()).unwrap_or(1);

    let objective = request
        .get("objective")
        .and_then(|v| v.as_str())
        .unwrap_or("min");

    let weight_property = request.get("weightProperty").and_then(|v| v.as_str());

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
    let mut builder = KSpanningTreeBuilder::new(graph_store)
        .source_node(source_node)
        .k(k)
        .objective(objective);

    if let Some(prop) = weight_property {
        builder = builder.weight_property(prop);
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
                &format!("K-Spanning Tree execution failed: {:?}", e),
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
                &format!("K-Spanning Tree stats failed: {:?}", e),
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
