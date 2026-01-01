//! Triangle Count algorithm dispatch handler.
//!
//! Handles JSON requests for Triangle Count operations,
//! delegating to the facade layer for execution.

use crate::procedures::community::triangle_count::{
    TriangleCountFacade, TriangleCountRow,
};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Triangle Count requests
pub fn handle_triangle_count(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "triangleCount";

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

    let max_degree = request
        .get("maxDegree")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize);

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
    let mut facade = TriangleCountFacade::new(graph_store).concurrency(concurrency);

    if let Some(max_deg) = max_degree {
        facade = facade.max_degree(max_deg as u64);
    }

    // Execute based on mode
    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<TriangleCountRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Triangle Count execution failed: {:?}", e),
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
                &format!("Triangle Count stats failed: {:?}", e),
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
                &format!("Triangle Count mutate failed: {:?}", e),
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
                &format!("Triangle Count write failed: {:?}", e),
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
                &format!("Triangle Count memory estimation failed: {:?}", e),
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
