//! Delta Stepping pathfinding algorithm dispatch handler.
//!
//! Handles JSON requests for Delta Stepping pathfinding operations,
//! delegating to the facade layer for execution.

use crate::procedures::facades::pathfinding::delta_stepping::DeltaSteppingBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Delta Stepping requests
pub fn handle_delta_stepping(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "delta_stepping";

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

    let delta = request.get("delta").and_then(|v| v.as_f64()).unwrap_or(1.0);

    let weight_property = request
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("weight");

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

    let direction = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing");

    let store_predecessors = request
        .get("storePredecessors")
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
    let builder = DeltaSteppingBuilder::new(graph_store)
        .source(source)
        .delta(delta)
        .weight_property(weight_property)
        .relationship_types(relationship_types)
        .direction(direction)
        .store_predecessors(store_predecessors)
        .concurrency(concurrency);

    // Execute based on mode
    match mode {
        "stream" => match builder.stream() {
            Ok(paths_iter) => {
                let paths: Vec<_> = paths_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": paths
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Delta Stepping execution failed: {:?}", e),
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
                &format!("Delta Stepping stats failed: {:?}", e),
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
                    &format!("Delta Stepping mutate failed: {:?}", e),
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
                    &format!("Delta Stepping write failed: {:?}", e),
                ),
            }
        }
        "estimate" => {
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
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
