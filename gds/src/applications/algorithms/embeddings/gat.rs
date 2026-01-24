//! GAT embedding algorithm dispatch handler.
//!
//! Handles JSON requests for GAT embedding operations,
//! delegating to the facade layer for execution.

use crate::procedures::embeddings::gat::GATBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle GAT embedding requests
pub fn handle_gat(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "gat";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let embedding_dimension = request
        .get("embeddingDimension")
        .and_then(|v| v.as_u64())
        .unwrap_or(64) as usize;

    let num_heads = request
        .get("numHeads")
        .and_then(|v| v.as_u64())
        .unwrap_or(8) as usize;

    let num_layers = request
        .get("numLayers")
        .and_then(|v| v.as_u64())
        .unwrap_or(2) as usize;

    let epochs = request
        .get("epochs")
        .and_then(|v| v.as_u64())
        .unwrap_or(100) as usize;

    let dropout = request
        .get("dropout")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.6);

    let alpha = request.get("alpha").and_then(|v| v.as_f64()).unwrap_or(0.2);

    let random_seed = request.get("randomSeed").and_then(|v| v.as_u64());

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(4) as usize;

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stats");

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
    let mut builder = GATBuilder::new(graph_store)
        .embedding_dimension(embedding_dimension)
        .num_heads(num_heads)
        .num_layers(num_layers)
        .epochs(epochs)
        .dropout(dropout)
        .alpha(alpha)
        .concurrency(concurrency);

    if let Some(seed) = random_seed {
        builder = builder.random_seed(seed);
    }

    // Execute based on mode
    match mode {
        "stats" => match builder.stats() {
            Ok(stats) => json!({ "ok": true, "op": op, "data": stats }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Failed to compute embeddings: {}", e),
            ),
        },
        "stream" => err(
            op,
            "NOT_IMPLEMENTED",
            "stream mode not yet implemented for GAT",
        ),
        "mutate" => err(
            op,
            "NOT_IMPLEMENTED",
            "mutate mode not yet implemented for GAT",
        ),
        "write" => err(
            op,
            "NOT_IMPLEMENTED",
            "write mode not yet implemented for GAT",
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
