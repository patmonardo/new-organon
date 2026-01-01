//! K1Coloring algorithm dispatch handler.
//!
//! Handles JSON requests for K1Coloring operations,
//! delegating to the facade layer for execution.

use crate::procedures::facades::community::k1coloring::{K1ColoringFacade, K1ColoringRow};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle K1Coloring requests
pub fn handle_k1coloring(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "k1coloring";

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
        None => {
            return err(
                op,
                "GRAPH_NOT_FOUND",
                &format!("Graph '{}' not found", graph_name),
            )
        }
    };

    // Create facade
    let facade = K1ColoringFacade::new(graph_store).concurrency(concurrency);

    // Execute based on mode
    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<K1ColoringRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("K1Coloring execution failed: {:?}", e),
            ),
        },
        "stats" => match facade.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("K1Coloring stats failed: {:?}", e),
            ),
        },
        "mutate" => match facade.mutate() {
            Ok(result) => json!({
                "ok": true,
                "op": op,
                "data": result
            }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("K1Coloring mutate failed: {:?}", e),
            ),
        },
        "write" => match facade.write() {
            Ok(result) => json!({
                "ok": true,
                "op": op,
                "data": result
            }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("K1Coloring write failed: {:?}", e),
            ),
        },
        "estimate_memory" => match facade.estimate_memory() {
            Ok(range) => json!({
                "ok": true,
                "op": op,
                "data": range
            }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("K1Coloring memory estimation failed: {:?}", e),
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
