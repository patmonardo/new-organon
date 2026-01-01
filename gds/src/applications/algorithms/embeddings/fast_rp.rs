//! FastRP embedding algorithm dispatch handler.
//!
//! Handles JSON requests for FastRP embedding operations,
//! delegating to the facade layer for execution.

use crate::procedures::embeddings::FastRPBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle FastRP embedding requests
pub fn handle_fast_rp(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "fast_rp";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let embedding_dimension = request
        .get("embeddingDimension")
        .and_then(|v| v.as_u64())
        .unwrap_or(128) as usize;

    let property_dimension = request
        .get("propertyDimension")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

    let iteration_weights = request
        .get("iterationWeights")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_f64().map(|f| f as f32))
                .collect::<Vec<f32>>()
        })
        .unwrap_or_else(|| vec![0.0, 1.0, 1.0]);

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

    // Create and configure builder
    let builder = FastRPBuilder::new(graph_store)
        .embedding_dimension(embedding_dimension)
        .property_dimension(property_dimension)
        .iteration_weights(iteration_weights);

    // Execute based on mode
    match mode {
        "stream" => match builder.stream() {
            Ok(iter) => {
                let embeddings: Vec<Value> = iter
                    .map(|row| {
                        json!({
                            "nodeId": row.node_id,
                            "embedding": row.embedding
                        })
                    })
                    .collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": embeddings
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Failed to compute embeddings: {}", e),
            ),
        },
        "stats" => match builder.stats() {
            Ok(stats) => json!({ "ok": true, "op": op, "data": stats }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Failed to compute embeddings: {}", e),
            ),
        },
        "mutate" => err(
            op,
            "NOT_IMPLEMENTED",
            "mutate mode not yet implemented for FastRP",
        ),
        "write" => err(
            op,
            "NOT_IMPLEMENTED",
            "write mode not yet implemented for FastRP",
        ),
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Helper function to create error responses
fn err(op: &str, error_type: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": error_type, "message": message }
    })
}
