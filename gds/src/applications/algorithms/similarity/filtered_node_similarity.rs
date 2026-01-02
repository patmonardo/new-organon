//! Filtered Node Similarity algorithm dispatch handler.
//!
//! Handles JSON requests for Filtered Node Similarity operations,
//! delegating to the facade layer for execution.

use crate::procedures::similarity::FilteredNodeSimilarityBuilder;
use crate::projection::NodeLabel;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Filtered Node Similarity requests
pub fn handle_filtered_node_similarity(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "filtered_node_similarity";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let _degree_cutoff = request
        .get("degreeCutoff")
        .and_then(|v| v.as_u64())
        .map(|d| d as usize);

    let similarity_cutoff = request
        .get("similarityCutoff")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.1);

    let _upper_degree_cutoff = request
        .get("upperDegreeCutoff")
        .and_then(|v| v.as_u64())
        .map(|d| d as usize);

    let _lower_degree_cutoff = request
        .get("lowerDegreeCutoff")
        .and_then(|v| v.as_u64())
        .map(|d| d as usize);

    let top_k = request.get("topK").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

    let bottom_k = request
        .get("bottomK")
        .and_then(|v| v.as_u64())
        .map(|k| k as usize);

    let top_n = request
        .get("topN")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let bottom_n = request
        .get("bottomN")
        .and_then(|v| v.as_u64())
        .map(|n| n as usize);

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(4) as usize;

    let source_node_label = request
        .get("sourceNodeLabel")
        .and_then(|v| v.as_str())
        .map(NodeLabel::of);

    let target_node_label = request
        .get("targetNodeLabel")
        .and_then(|v| v.as_str())
        .map(NodeLabel::of);

    if source_node_label.is_none() && target_node_label.is_none() {
        return err(
            op,
            "INVALID_REQUEST",
            "filtered_node_similarity requires sourceNodeLabel and/or targetNodeLabel",
        );
    }

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
    let mut builder = FilteredNodeSimilarityBuilder::new(graph_store)
        .similarity_cutoff(similarity_cutoff)
        .top_k(top_k)
        .concurrency(concurrency);

    // Apply optional parameters
    if let Some(_k) = bottom_k {
        // Note: SimilarityBuilder doesn't have bottom_k, this might be for future use
    }

    if let Some(n) = top_n {
        builder = builder.top_n(n);
    }

    if let Some(_n) = bottom_n {
        // Note: SimilarityBuilder doesn't have bottom_n, this might be for future use
    }

    if let Some(label) = source_node_label {
        builder = builder.source_node_label(label);
    }
    if let Some(label) = target_node_label {
        builder = builder.target_node_label(label);
    }

    // Execute based on mode
    match mode {
        "stream" => match builder.stream() {
            Ok(results) => {
                let result_rows: Vec<_> = results.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": result_rows
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Filtered Node Similarity execution failed: {:?}", e),
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
                &format!("Filtered Node Similarity stats failed: {:?}", e),
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
                Ok(_) => json!({
                    "ok": true,
                    "op": op,
                    "data": { "nodesWritten": 0 }
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Filtered Node Similarity mutate failed: {:?}", e),
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
                Ok(_) => json!({
                    "ok": true,
                    "op": op,
                    "data": { "nodesWritten": 0 }
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Filtered Node Similarity write failed: {:?}", e),
                ),
            }
        }
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
