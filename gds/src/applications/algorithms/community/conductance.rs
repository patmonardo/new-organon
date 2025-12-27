//! Conductance algorithm dispatch handler.
//!
//! Handles JSON requests for Conductance community quality operations,
//! delegating to the facade layer for execution.

use crate::procedures::facades::community::conductance::{ConductanceBuilder, ConductanceRow};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Conductance requests
pub fn handle_conductance(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "conductance";

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
    let builder = ConductanceBuilder::new(graph_store, community_property.to_string());

    // Execute based on mode
    match mode {
        "stream" => match builder.stream() {
            Ok(rows_iter) => {
                let rows: Vec<ConductanceRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Conductance execution failed: {:?}", e)),
        },
        "stats" => match builder.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Conductance stats failed: {:?}", e)),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
