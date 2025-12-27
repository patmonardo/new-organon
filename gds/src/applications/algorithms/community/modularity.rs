//! Modularity algorithm dispatch handler.
//!
//! Handles JSON requests for modularity computation operations,
//! delegating to the facade layer for execution.

use crate::procedures::facades::community::modularity::{ModularityBuilder, ModularityRow};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle modularity requests
pub fn handle_modularity(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "modularity";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let community_property = match request.get("communityProperty").and_then(|v| v.as_str()) {
        Some(prop) => prop,
        None => return err(op, "INVALID_REQUEST", "Missing 'communityProperty' parameter"),
    };

    // Get graph store
    let graph_store = match catalog.get(graph_name) {
        Some(store) => store,
        None => return err(op, "GRAPH_NOT_FOUND", &format!("Graph '{}' not found", graph_name)),
    };

    // Create builder
    let builder = ModularityBuilder::new(graph_store, community_property.to_string());

    // Execute based on mode
    match mode {
        "stream" => match builder.stream() {
            Ok(rows_iter) => {
                let rows: Vec<ModularityRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Modularity execution failed: {:?}", e)),
        },
        "stats" => match builder.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Modularity stats failed: {:?}", e)),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
