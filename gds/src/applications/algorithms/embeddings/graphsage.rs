//! GraphSAGE embedding algorithm dispatch handler.
//!
//! Handles JSON requests for GraphSAGE embedding operations,
//! delegating to the facade layer for execution.

use crate::procedures::embeddings::GraphSageBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle GraphSAGE embedding requests
pub fn handle_graphsage(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "graphsage";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let model_name = match request.get("modelName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'modelName' parameter"),
    };

    let model_user = request
        .get("modelUser")
        .and_then(|v| v.as_str())
        .unwrap_or("anonymous");

    let batch_size = request
        .get("batchSize")
        .and_then(|v| v.as_u64())
        .unwrap_or(100) as usize;

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(num_cpus::get().max(1) as u64) as usize;

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
    let builder = GraphSageBuilder::new(graph_store)
        .model_name(model_name)
        .model_user(model_user)
        .batch_size(batch_size)
        .concurrency(concurrency);

    // Execute based on mode
    match mode {
        "stream" => match builder.run() {
            Ok(result) => {
                let embeddings: Vec<Value> = result
                    .embeddings
                    .iter()
                    .enumerate()
                    .map(|(node_id, embedding)| {
                        json!({
                            "nodeId": node_id,
                            "embedding": embedding
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
            "mutate mode not yet implemented for GraphSAGE",
        ),
        "write" => err(
            op,
            "NOT_IMPLEMENTED",
            "write mode not yet implemented for GraphSAGE",
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
