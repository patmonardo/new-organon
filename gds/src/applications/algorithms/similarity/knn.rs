//! KNN similarity algorithm dispatch handler.
//!
//! Handles JSON requests for KNN similarity operations,
//! delegating to the facade layer for execution.

use crate::procedures::similarity::knn::KnnBuilder;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle KNN similarity requests
pub fn handle_knn(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "knn";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let node_properties = match request.get("nodeProperties").and_then(|v| v.as_array()) {
        Some(props) => {
            let mut properties = Vec::new();
            for prop in props {
                if let Some(prop_str) = prop.as_str() {
                    properties.push(prop_str.to_string());
                }
            }
            if properties.is_empty() {
                return err(
                    op,
                    "INVALID_REQUEST",
                    "nodeProperties array cannot be empty",
                );
            }
            properties
        }
        None => return err(op, "INVALID_REQUEST", "Missing 'nodeProperties' parameter"),
    };

    let top_k = request.get("topK").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

    let _sample_rate = request
        .get("sampleRate")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let _perturbation_rate = request
        .get("perturbationRate")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    let max_iterations = request
        .get("maxIterations")
        .and_then(|v| v.as_u64())
        .map(|i| i as usize);

    let similarity_cutoff = request
        .get("similarityCutoff")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.0);

    let degree_cutoff = request
        .get("degreeCutoff")
        .and_then(|v| v.as_u64())
        .map(|d| d as usize);

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

    // Create builder - use first property as primary, add others
    let mut builder = KnnBuilder::new(graph_store, &node_properties[0])
        .k(top_k)
        .similarity_cutoff(similarity_cutoff)
        .concurrency(concurrency);

    // Add additional properties if any
    for prop in &node_properties[1..] {
        builder = builder.add_property(
            prop,
            crate::algo::similarity::knn::SimilarityMetric::Default,
        );
    }

    // Apply optional parameters
    if let Some(_iterations) = max_iterations {
        // Note: KnnBuilder doesn't have max_iterations, this might be for future use
        // or we might need to extend the builder
    }

    if let Some(_degree) = degree_cutoff {
        // Note: KnnBuilder doesn't have degree_cutoff, this might be for future use
    }

    if let Some(_seed) = random_seed {
        // Note: KnnBuilder doesn't have random_seed, this might be for future use
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
                &format!("KNN execution failed: {:?}", e),
            ),
        },
        "stats" => match builder.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("KNN stats failed: {:?}", e)),
        },
        "mutate" => {
            let _property_name = match request.get("property_name").and_then(|v| v.as_str()) {
                Some(name) => name,
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'property_name' for mutate mode",
                    )
                }
            };
            // Note: mutate not implemented in KnnBuilder yet
            err(
                op,
                "NOT_IMPLEMENTED",
                "mutate mode not yet implemented for KNN",
            )
        }
        "write" => {
            let _property_name = match request.get("property_name").and_then(|v| v.as_str()) {
                Some(name) => name,
                None => {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "Missing 'property_name' for write mode",
                    )
                }
            };
            // Note: write not implemented in KnnBuilder yet
            err(
                op,
                "NOT_IMPLEMENTED",
                "write mode not yet implemented for KNN",
            )
        }
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
