//! Filtered KNN similarity algorithm dispatch handler.
//!
//! Handles JSON requests for Filtered KNN similarity operations,
//! delegating to the facade layer for execution.

use crate::procedures::similarity::filtered_knn::FilteredKnnBuilder;
use crate::procedures::similarity::knn::SimilarityMetric;
use crate::projection::NodeLabel;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Filtered KNN similarity requests
pub fn handle_filtered_knn(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "filtered_knn";

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

    let source_node_label = request
        .get("sourceNodeLabel")
        .and_then(|v| v.as_str())
        .map(NodeLabel::of);

    let target_node_label = request
        .get("targetNodeLabel")
        .and_then(|v| v.as_str())
        .map(NodeLabel::of);

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
    let mut builder = FilteredKnnBuilder::new(graph_store, &node_properties[0])
        .k(top_k)
        .similarity_cutoff(similarity_cutoff)
        .concurrency(concurrency);

    // Add additional properties if any
    for prop in &node_properties[1..] {
        builder = builder.add_property(prop, SimilarityMetric::Default);
    }

    // Add node label filters
    if let Some(_label) = source_node_label {
        // Note: FilteredKnnBuilder doesn't have source_node_labels method yet
        // This would need to be added to the builder
    }

    if let Some(_label) = target_node_label {
        // Note: FilteredKnnBuilder doesn't have target_node_labels method yet
        // This would need to be added to the builder
    }

    // Apply optional parameters
    if let Some(_iterations) = max_iterations {
        // Note: FilteredKnnBuilder doesn't have max_iterations, this might be for future use
    }

    if let Some(_degree) = degree_cutoff {
        // Note: FilteredKnnBuilder doesn't have degree_cutoff, this might be for future use
    }

    if let Some(_seed) = random_seed {
        // Note: FilteredKnnBuilder doesn't have random_seed, this might be for future use
    }

    // Execute based on mode
    match mode {
        "stream" => {
            // Note: FilteredKnnBuilder doesn't have stream method yet
            // This would need to be implemented
            err(
                op,
                "NOT_IMPLEMENTED",
                "stream mode not yet implemented for Filtered KNN",
            )
        }
        "stats" => match builder.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": stats
            }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Filtered KNN stats failed: {:?}", e),
            ),
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
            // Note: mutate not implemented in FilteredKnnBuilder yet
            err(
                op,
                "NOT_IMPLEMENTED",
                "mutate mode not yet implemented for Filtered KNN",
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
            // Note: write not implemented in FilteredKnnBuilder yet
            err(
                op,
                "NOT_IMPLEMENTED",
                "write mode not yet implemented for Filtered KNN",
            )
        }
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
