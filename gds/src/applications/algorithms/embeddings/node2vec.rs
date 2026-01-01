//! Node2Vec embedding algorithm dispatch handler.
//!
//! Handles JSON requests for Node2Vec embedding operations,
//! delegating to the facade layer for execution.

use crate::procedures::embeddings::Node2VecBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Node2Vec embedding requests
pub fn handle_node2vec(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "node2vec";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let walks_per_node = request
        .get("walksPerNode")
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as usize;

    let walk_length = request
        .get("walkLength")
        .and_then(|v| v.as_u64())
        .unwrap_or(80) as usize;

    let return_factor = request
        .get("returnFactor")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let in_out_factor = request
        .get("inOutFactor")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let positive_sampling_factor = request
        .get("positiveSamplingFactor")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.001);

    let negative_sampling_exponent = request
        .get("negativeSamplingExponent")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.75);

    let embedding_dimension = request
        .get("embeddingDimension")
        .and_then(|v| v.as_u64())
        .unwrap_or(128) as usize;

    let iterations = request
        .get("iterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    let random_seed = request.get("randomSeed").and_then(|v| v.as_u64());

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
    let mut builder = Node2VecBuilder::new(graph_store)
        .walks_per_node(walks_per_node)
        .walk_length(walk_length)
        .return_factor(return_factor)
        .in_out_factor(in_out_factor)
        .positive_sampling_factor(positive_sampling_factor)
        .negative_sampling_exponent(negative_sampling_exponent)
        .embedding_dimension(embedding_dimension)
        .iterations(iterations);

    if let Some(seed) = random_seed {
        builder = builder.random_seed(Some(seed));
    }

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
            "mutate mode not yet implemented for Node2Vec",
        ),
        "write" => err(
            op,
            "NOT_IMPLEMENTED",
            "write mode not yet implemented for Node2Vec",
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
