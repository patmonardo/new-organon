//! BFS pathfinding algorithm dispatch handler.
//!
//! Handles JSON requests for BFS pathfinding operations,
//! delegating to the facade layer for execution.

use crate::procedures::pathfinding::bfs::BfsBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle BFS pathfinding requests
pub fn handle_bfs(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "bfs";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let source = match request.get("sourceNode").and_then(|v| v.as_u64()) {
        Some(s) => s,
        None => {
            return err(
                op,
                "INVALID_REQUEST",
                "Missing or invalid 'sourceNode' parameter",
            )
        }
    };

    let targets = if let Some(target) = request.get("targetNode").and_then(|v| v.as_u64()) {
        vec![target]
    } else if let Some(targets) = request.get("targetNodes").and_then(|v| v.as_array()) {
        targets
            .iter()
            .filter_map(|v| v.as_u64())
            .collect::<Vec<_>>()
    } else {
        vec![] // BFS can work without specific targets
    };

    let max_depth = request
        .get("maxDepth")
        .and_then(|v| v.as_u64())
        .map(|d| d as u32);

    let track_paths = request
        .get("trackPaths")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    let estimate_submode = request
        .get("estimateSubmode")
        .and_then(|v| v.as_str());

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
    let mut builder = BfsBuilder::new(graph_store)
        .source(source)
        .concurrency(concurrency);

    if !targets.is_empty() {
        builder = builder.targets(targets);
    }

    if let Some(depth) = max_depth {
        builder = builder.max_depth(depth);
    }

    builder = builder.track_paths(track_paths);

    // Execute based on mode
    match mode {
        "stream" => match builder.stream() {
            Ok(paths) => {
                let path_results: Vec<_> = paths.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": path_results
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("BFS execution failed: {:?}", e),
            ),
        },
        "stats" => match builder.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("BFS stats failed: {:?}", e)),
        },
        "mutate" => {
            let property_name = match request.get("property_name").and_then(|v| v.as_str()) {
                Some(name) => name,
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'property_name' for mutate mode",
                    )
                }
            };
            match builder.mutate(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("BFS mutate failed: {:?}", e),
                ),
            }
        }
        "write" => {
            let property_name = match request.get("property_name").and_then(|v| v.as_str()) {
                Some(name) => name,
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'property_name' for write mode",
                    )
                }
            };
            match builder.write(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(op, "EXECUTION_ERROR", &format!("BFS write failed: {:?}", e)),
            }
        }
        "estimate" => match estimate_submode {
            Some("memory") => {
                // Memory estimation mode - returns memory range without executing algorithm
                let memory_range = builder.estimate_memory();
                json!({
                    "ok": true,
                    "op": op,
                    "data": {
                        "memoryBytes": {
                            "min": memory_range.min(),
                            "max": memory_range.max()
                        }
                    }
                })
            }
            Some(other) => err(
                op,
                "INVALID_REQUEST",
                &format!("Invalid estimate submode '{}'. Use 'memory'", other),
            ),
            None => err(
                op,
                "INVALID_REQUEST",
                "Missing 'estimateSubmode' parameter for estimate mode",
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
