//! Split relationships dispatch handler.
//!
//! Mirrors the Java `MachineLearningAlgorithms.*splitRelationships` application surface.
//!
//! Note: the full Java-parity mutate behavior (writing two relationship types back into the
//! GraphStore) is not yet wired through the Rust `types::graph_store` layer. For now, this
//! handler returns NOT_IMPLEMENTED.

use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle split-relationships requests.
pub fn handle_split_relationships(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "split_relationships";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    // Validate graph exists
    if catalog.get(graph_name).is_none() {
        return err(
            op,
            "GRAPH_NOT_FOUND",
            &format!("Graph '{}' not found", graph_name),
        );
    }

    let mode = request.get("mode").and_then(|v| v.as_str()).unwrap_or("mutate");

    match mode {
        "mutate" | "estimate_memory" => err(
            op,
            "NOT_IMPLEMENTED",
            "splitRelationships is not yet implemented in Rust gds/",
        ),
        "stream" | "stats" | "write" => err(
            op,
            "INVALID_REQUEST",
            "Invalid mode for splitRelationships (expected 'mutate' or 'estimate_memory')",
        ),
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

fn err(op: &str, code: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message }
    })
}
