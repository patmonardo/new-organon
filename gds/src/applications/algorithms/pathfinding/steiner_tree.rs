//! Steiner Tree pathfinding algorithm dispatch handler.
//!
//! Handles JSON requests for Steiner Tree operations,
//! delegating to the facade layer for execution.

use crate::procedures::pathfinding::steiner_tree::SteinerTreeBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Steiner Tree requests
pub fn handle_steiner_tree(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "steiner_tree";

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

    let target_nodes = if let Some(nodes) = request.get("targetNodes").and_then(|v| v.as_array()) {
        nodes.iter().filter_map(|v| v.as_u64()).collect::<Vec<_>>()
    } else {
        return err(op, "INVALID_REQUEST", "Missing 'targetNodes' parameter");
    };

    if target_nodes.is_empty() {
        return err(op, "INVALID_REQUEST", "'targetNodes' must not be empty");
    }

    let relationship_weight_property = request
        .get("relationshipWeightProperty")
        .and_then(|v| v.as_str());

    let delta = request.get("delta").and_then(|v| v.as_f64()).unwrap_or(1.0);

    let apply_rerouting = request
        .get("applyRerouting")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(4) as usize;

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
    let mut builder = SteinerTreeBuilder::new(graph_store)
        .source_node(source_node)
        .target_nodes(target_nodes)
        .delta(delta)
        .apply_rerouting(apply_rerouting)
        .concurrency(concurrency);

    if let Some(prop) = relationship_weight_property {
        builder = builder.relationship_weight_property(prop);
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
                &format!("Steiner Tree execution failed: {:?}", e),
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
                &format!("Steiner Tree stats failed: {:?}", e),
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
