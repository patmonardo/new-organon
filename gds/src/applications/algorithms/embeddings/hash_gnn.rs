//! HashGNN embedding algorithm dispatch handler.
//!
//! Handles JSON requests for HashGNN embedding operations,
//! delegating to the facade layer for execution.

use crate::procedures::embeddings::{
    BinarizeFeaturesConfig, GenerateFeaturesConfig, HashGNNBuilder,
};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle HashGNN embedding requests
pub fn handle_hash_gnn(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "hash_gnn";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(4) as usize;

    let iterations = request
        .get("iterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    let embedding_density = request
        .get("embeddingDensity")
        .and_then(|v| v.as_u64())
        .unwrap_or(64) as usize;

    let neighbor_influence = request
        .get("neighborInfluence")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let feature_properties = request
        .get("featureProperties")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    let heterogeneous = request
        .get("heterogeneous")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let output_dimension = request
        .get("outputDimension")
        .and_then(|v| v.as_u64())
        .map(|d| d as usize);

    let random_seed = request.get("randomSeed").and_then(|v| v.as_u64());

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stats");

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

    // Create and configure builder
    let mut builder = HashGNNBuilder::new(graph_store)
        .concurrency(concurrency)
        .iterations(iterations)
        .embedding_density(embedding_density)
        .neighbor_influence(neighbor_influence)
        .feature_properties(feature_properties)
        .heterogeneous(heterogeneous)
        .output_dimension(output_dimension);

    if let Some(seed) = random_seed {
        builder = builder.random_seed(Some(seed));
    }

    // Handle binarize_features if provided
    if let Some(binarize_config) = request.get("binarizeFeatures") {
        if let (Some(dimension), Some(threshold)) = (
            binarize_config.get("dimension").and_then(|v| v.as_u64()),
            binarize_config.get("threshold").and_then(|v| v.as_f64()),
        ) {
            let config = BinarizeFeaturesConfig {
                dimension: dimension as usize,
                threshold,
            };
            builder = builder.binarize_features(Some(config));
        }
    }

    // Handle generate_features if provided
    if let Some(generate_config) = request.get("generateFeatures") {
        if let (Some(dimension), Some(density_level)) = (
            generate_config.get("dimension").and_then(|v| v.as_u64()),
            generate_config.get("densityLevel").and_then(|v| v.as_u64()),
        ) {
            let config = GenerateFeaturesConfig {
                dimension: dimension as usize,
                density_level: density_level as usize,
            };
            builder = builder.generate_features(Some(config));
        }
    }

    // Execute based on mode
    match mode {
        "stats" => match builder.stats() {
            Ok(stats) => json!({ "ok": true, "op": op, "data": stats }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Failed to compute embeddings: {}", e),
            ),
        },
        "stream" => err(
            op,
            "NOT_IMPLEMENTED",
            "stream mode not yet implemented for HashGNN",
        ),
        "mutate" => err(
            op,
            "NOT_IMPLEMENTED",
            "mutate mode not yet implemented for HashGNN",
        ),
        "write" => err(
            op,
            "NOT_IMPLEMENTED",
            "write mode not yet implemented for HashGNN",
        ),
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Helper function to create error responses
fn err(op: &str, error_type: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": error_type, "message": message }
    })
}
