//! Leiden algorithm dispatch handler.
//!
//! Handles JSON requests for Leiden community detection operations,
//! delegating to the facade layer for execution.

use crate::procedures::facades::community::leiden::{LeidenBuilder, LeidenRow};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Leiden requests
pub fn handle_leiden(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "leiden";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let gamma = request
        .get("gamma")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let theta = request
        .get("theta")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.01);

    let tolerance = request
        .get("tolerance")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0001);

    let max_iterations = request
        .get("maxIterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as usize;

    let random_seed = request
        .get("randomSeed")
        .and_then(|v| v.as_u64())
        .unwrap_or(42);

    // Get graph store
    let graph_store = match catalog.get(graph_name) {
        Some(store) => store,
        None => return err(op, "GRAPH_NOT_FOUND", &format!("Graph '{}' not found", graph_name)),
    };

    // Create builder
    let builder = LeidenBuilder::new(graph_store)
        .gamma(gamma)
        .theta(theta)
        .tolerance(tolerance)
        .max_iterations(max_iterations)
        .random_seed(random_seed);

    // Execute based on mode
    match mode {
        "stream" => match builder.stream() {
            Ok(rows_iter) => {
                let rows: Vec<LeidenRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Leiden execution failed: {:?}", e)),
        },
        "stats" => match builder.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Leiden stats failed: {:?}", e)),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
