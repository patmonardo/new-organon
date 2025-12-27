//! KCore algorithm dispatch handler.
//!
//! Handles JSON requests for KCore decomposition operations,
//! delegating to the facade layer for execution.

use crate::procedures::facades::community::kcore::{KCoreBuilder, KCoreRow};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle KCore requests
pub fn handle_kcore(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "kcore";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    // Get graph store
    let graph_store = match catalog.get(graph_name) {
        Some(store) => store,
        None => return err(op, "GRAPH_NOT_FOUND", &format!("Graph '{}' not found", graph_name)),
    };

    // Create builder
    let builder = KCoreBuilder::new(graph_store).concurrency(concurrency);

    // Execute based on mode
    match mode {
        "stream" => match builder.stream() {
            Ok(rows_iter) => {
                let rows: Vec<KCoreRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("KCore execution failed: {:?}", e)),
        },
        "stats" => match builder.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("KCore stats failed: {:?}", e)),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
