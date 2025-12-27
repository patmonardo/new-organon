//! Degree centrality algorithm dispatch handler.
//!
//! Handles JSON requests for degree centrality operations,
//! delegating to the facade layer for execution.

use crate::procedures::degree_centrality::storage::Orientation;
use crate::procedures::facades::centrality::degree_centrality::DegreeCentralityFacade;
use crate::procedures::facades::traits::CentralityScore;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle degree centrality requests
pub fn handle_degree_centrality(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "degree_centrality";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let normalize = request
        .get("normalize")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let orientation = request
        .get("orientation")
        .and_then(|v| v.as_str())
        .unwrap_or("natural");

    let weighted = request
        .get("weighted")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    // Parse orientation
    let orientation = match orientation {
        "natural" => Orientation::Natural,
        "reverse" => Orientation::Reverse,
        "undirected" => Orientation::Undirected,
        _ => return err(op, "INVALID_REQUEST", "Invalid orientation. Use 'natural', 'reverse', or 'undirected'"),
    };

    // Get graph store
    let graph_store = match catalog.get(graph_name) {
        Some(store) => store,
        None => return err(op, "GRAPH_NOT_FOUND", &format!("Graph '{}' not found", graph_name)),
    };

    // Create facade
    let facade = DegreeCentralityFacade::new(graph_store)
        .normalize(normalize)
        .orientation(orientation)
        .weighted(weighted);

    // Execute based on mode
    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<CentralityScore> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Degree centrality execution failed: {:?}", e)),
        },
        "stats" => match facade.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Degree centrality stats failed: {:?}", e)),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
