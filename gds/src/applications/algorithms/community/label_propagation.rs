//! Label Propagation algorithm dispatch handler.
//!
//! Handles JSON requests for Label Propagation community detection operations,
//! delegating to the facade layer for execution.

use crate::procedures::community::label_propagation::{
    LabelPropagationFacade, LabelPropagationRow,
};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Label Propagation requests
pub fn handle_label_propagation(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "labelPropagation";

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

    let max_iterations = request
        .get("maxIterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(10);

    let node_weight_property = request
        .get("nodeWeightProperty")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let seed_property = request
        .get("seedProperty")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

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
    let mut facade = LabelPropagationFacade::new(graph_store)
        .concurrency(concurrency)
        .max_iterations(max_iterations);

    if let Some(node_weight) = node_weight_property {
        facade = facade.node_weight_property(&node_weight);
    }

    if let Some(seed) = seed_property {
        facade = facade.seed_property(&seed);
    }

    // Execute based on mode
    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<LabelPropagationRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Label Propagation execution failed: {:?}", e),
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
                &format!("Label Propagation stats failed: {:?}", e),
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
                &format!("Label Propagation mutate failed: {:?}", e),
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
                &format!("Label Propagation write failed: {:?}", e),
            ),
        },
        "estimate" => match facade.estimate_memory() {
            Ok(range) => json!({
                "ok": true,
                "op": op,
                "data": range
            }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Label Propagation memory estimation failed: {:?}", e),
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
