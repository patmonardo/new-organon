//! PageRank centrality algorithm dispatch handler.
//!
//! Handles JSON requests for PageRank operations,
//! delegating to the facade layer for execution.

use crate::procedures::centrality::pagerank::PageRankFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle PageRank requests
pub fn handle_pagerank(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "pagerank";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    let direction = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("outgoing");

    let source_nodes = request
        .get("sourceNodes")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_u64()).collect())
        .unwrap_or(vec![]);

    let iterations = request
        .get("iterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(20) as u32;

    let damping_factor = request
        .get("dampingFactor")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.85);

    let tolerance = request
        .get("tolerance")
        .and_then(|v| v.as_f64())
        .unwrap_or(1e-4);

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

    // Create facade
    let mut facade = PageRankFacade::new(graph_store)
        .concurrency(concurrency)
        .direction(direction)
        .iterations(iterations)
        .damping_factor(damping_factor)
        .tolerance(tolerance);

    if !source_nodes.is_empty() {
        facade = facade.source_nodes(source_nodes);
    }

    // Execute based on mode
    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<serde_json::Value> = rows_iter
                    .map(|score| {
                        json!({
                            "node_id": score.node_id,
                            "score": score.score
                        })
                    })
                    .collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("PageRank execution failed: {:?}", e),
            ),
        },
        "stats" => match facade.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("PageRank stats failed: {:?}", e),
            ),
        },
        "mutate" => {
            let property_name = match request.get("mutateProperty").and_then(|v| v.as_str()) {
                Some(name) => name,
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'mutateProperty' parameter for mutate mode",
                    )
                }
            };
            match facade.mutate(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("PageRank mutate failed: {:?}", e),
                ),
            }
        }
        "write" => {
            let property_name = match request.get("writeProperty").and_then(|v| v.as_str()) {
                Some(name) => name,
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'writeProperty' parameter for write mode",
                    )
                }
            };
            match facade.write(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("PageRank write failed: {:?}", e),
                ),
            }
        }
        "estimate_memory" => {
            let memory = facade.estimate_memory();
            json!({
                "ok": true,
                "op": op,
                "data": {
                    "min_bytes": memory.min(),
                    "max_bytes": memory.max()
                }
            })
        }
        _ => err(
            op,
            "INVALID_REQUEST",
            "Invalid mode. Use 'stream', 'stats', 'mutate', 'write', or 'estimate_memory'",
        ),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
