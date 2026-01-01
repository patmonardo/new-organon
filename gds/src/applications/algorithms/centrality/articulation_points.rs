//! Articulation Points centrality algorithm dispatch handler.
//!
//! Handles JSON requests for Articulation Points operations,
//! delegating to the facade layer for execution.

use crate::procedures::centrality::articulation_points::{
    ArticulationPointRow, ArticulationPointsFacade,
};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Articulation Points requests
pub fn handle_articulation_points(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "articulation_points";

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
        .unwrap_or(4) as usize;

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

    // Create facade with configuration
    let facade = ArticulationPointsFacade::new(graph_store).concurrency(concurrency);

    // Execute based on mode
    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<ArticulationPointRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Articulation Points execution failed: {:?}", e),
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
                &format!("Articulation Points stats failed: {:?}", e),
            ),
        },
        "mutate" => match request.get("propertyName").and_then(|v| v.as_str()) {
            Some(property_name) => match facade.mutate(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Articulation Points mutate failed: {:?}", e),
                ),
            },
            None => err(
                op,
                "INVALID_REQUEST",
                "Missing 'propertyName' parameter for mutate mode",
            ),
        },
        "write" => match request.get("propertyName").and_then(|v| v.as_str()) {
            Some(property_name) => match facade.write(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Articulation Points write failed: {:?}", e),
                ),
            },
            None => err(
                op,
                "INVALID_REQUEST",
                "Missing 'propertyName' parameter for write mode",
            ),
        },
        "estimate_memory" => {
            let memory = facade.estimate_memory();
            json!({
                "ok": true,
                "op": op,
                "data": {
                    "minBytes": memory.min(),
                    "maxBytes": memory.max()
                }
            })
        }
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
