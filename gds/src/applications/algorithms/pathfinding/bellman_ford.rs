//! Bellman-Ford pathfinding algorithm dispatch handler.
//!
//! Handles JSON requests for Bellman-Ford pathfinding operations,
//! delegating to the facade layer for execution.

use crate::procedures::pathfinding::bellman_ford::BellmanFordBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Bellman-Ford pathfinding requests
pub fn handle_bellman_ford(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "bellman_ford";

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

    let weight_property = request
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("weight");

    let direction = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing");

    let track_negative_cycles = request
        .get("trackNegativeCycles")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let track_paths = request
        .get("trackPaths")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(4) as usize;

    let estimate_submode = request
        .get("submode")
        .and_then(|v| v.as_str());

    let relationship_types =
        if let Some(types) = request.get("relationshipTypes").and_then(|v| v.as_array()) {
            types
                .iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        } else {
            vec![]
        };

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
    let mut builder = BellmanFordBuilder::new(graph_store)
        .source(source)
        .weight_property(weight_property)
        .direction(direction)
        .track_negative_cycles(track_negative_cycles)
        .track_paths(track_paths)
        .concurrency(concurrency);

    if !relationship_types.is_empty() {
        builder = builder.relationship_types(relationship_types);
    }

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
                &format!("Bellman-Ford execution failed: {:?}", e),
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
                &format!("Bellman-Ford stats failed: {:?}", e),
            ),
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
                    &format!("Bellman-Ford mutate failed: {:?}", e),
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
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Bellman-Ford write failed: {:?}", e),
                ),
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
                "Missing 'submode' parameter for estimate mode",
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
