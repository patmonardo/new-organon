//! Random Walk pathfinding algorithm dispatch handler.
//!
//! Handles JSON requests for Random Walk operations,
//! delegating to the facade layer for execution.

use crate::procedures::pathfinding::random_walk::RandomWalkBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Random Walk requests
pub fn handle_random_walk(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "random_walk";

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

    let source_nodes = if let Some(nodes) = request.get("sourceNodes").and_then(|v| v.as_array()) {
        nodes.iter().filter_map(|v| v.as_u64()).collect::<Vec<_>>()
    } else {
        vec![]
    };

    let random_seed = request.get("randomSeed").and_then(|v| v.as_u64());

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
    let mut builder = RandomWalkBuilder::new(graph_store)
        .walks_per_node(walks_per_node)
        .walk_length(walk_length)
        .return_factor(return_factor)
        .in_out_factor(in_out_factor)
        .source_nodes(source_nodes)
        .concurrency(concurrency);

    if let Some(seed) = random_seed {
        builder = builder.random_seed(seed);
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
                &format!("Random Walk execution failed: {:?}", e),
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
                &format!("Random Walk stats failed: {:?}", e),
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
