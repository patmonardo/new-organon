//! Local Clustering Coefficient algorithm dispatch handler.
//!
//! Handles JSON requests for Local Clustering Coefficient operations,
//! delegating to the facade layer for execution.

use crate::procedures::facades::community::local_clustering_coefficient::{LocalClusteringCoefficientBuilder, LocalClusteringCoefficientRow};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Local Clustering Coefficient requests
pub fn handle_local_clustering_coefficient(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "localClusteringCoefficient";

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

    let max_degree = request
        .get("maxDegree")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize);

    let seed_property = request
        .get("seedProperty")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    // Get graph store
    let graph_store = match catalog.get(graph_name) {
        Some(store) => store,
        None => return err(op, "GRAPH_NOT_FOUND", &format!("Graph '{}' not found", graph_name)),
    };

    // Create builder
    let mut builder = LocalClusteringCoefficientBuilder::new(graph_store).concurrency(concurrency);

    if let Some(max_deg) = max_degree {
        builder = builder.max_degree(max_deg as u64);
    }

    if let Some(seed) = seed_property {
        builder = builder.seed_property(&seed);
    }

    // Execute based on mode
    match mode {
        "stream" => match builder.stream() {
            Ok(rows_iter) => {
                let rows: Vec<LocalClusteringCoefficientRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Local Clustering Coefficient execution failed: {:?}", e)),
        },
        "stats" => match builder.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("Local Clustering Coefficient stats failed: {:?}", e)),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
