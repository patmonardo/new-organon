//! Harmonic centrality algorithm dispatch handler.
//!
//! Handles JSON requests for harmonic centrality operations,
//! delegating to the facade layer for execution.

use crate::procedures::facades::centrality::harmonic::HarmonicCentralityFacade;
use crate::procedures::facades::traits::CentralityScore;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle harmonic centrality requests
pub fn handle_harmonic(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "harmonic";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let direction = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("both");

    // Get graph store
    let graph_store = match catalog.get(graph_name) {
        Some(store) => store,
        None => return err(op, "GRAPH_NOT_FOUND", &format!("Graph '{}' not found", graph_name)),
    };

    // Create facade
    let facade = HarmonicCentralityFacade::new(graph_store).direction(direction);

    // Execute based on mode
    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<CentralityScore> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Harmonic execution failed: {:?}", e)),
        },
        "stats" => match facade.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Harmonic stats failed: {:?}", e)),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
