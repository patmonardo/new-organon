//! KMeans algorithm dispatch handler.
//!
//! Handles JSON requests for KMeans clustering operations,
//! delegating to the facade layer for execution.

use crate::procedures::facades::community::kmeans::{KMeansFacade, KMeansRow};
use crate::procedures::kmeans::KMeansSamplerType;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle KMeans requests
pub fn handle_kmeans(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "kmeans";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let k = request.get("k").and_then(|v| v.as_u64()).unwrap_or(2) as usize;

    let node_property = match request.get("nodeProperty").and_then(|v| v.as_str()) {
        Some(prop) => prop,
        None => return err(op, "INVALID_REQUEST", "Missing 'nodeProperty' parameter"),
    };

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    let max_iterations = request
        .get("maxIterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as u32;

    let random_seed = request
        .get("randomSeed")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let compute_silhouette = request
        .get("computeSilhouette")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let sampler_type = request
        .get("samplerType")
        .and_then(|v| v.as_str())
        .unwrap_or("UNIFORM");

    let sampler = match sampler_type {
        "KMEANSPP" => KMeansSamplerType::KmeansPlusPlus,
        _ => KMeansSamplerType::Uniform,
    };

    let seed_centroids = request
        .get("seedCentroids")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_array())
                .map(|coords| {
                    coords
                        .iter()
                        .filter_map(|c| c.as_f64())
                        .collect::<Vec<f64>>()
                })
                .collect::<Vec<Vec<f64>>>()
        });

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
    let mut facade = KMeansFacade::new(graph_store)
        .k(k)
        .node_property(node_property)
        .concurrency(concurrency)
        .max_iterations(max_iterations)
        .random_seed(random_seed)
        .compute_silhouette(compute_silhouette)
        .sampler_type(sampler);

    if let Some(centroids) = seed_centroids {
        facade = facade.seed_centroids(centroids);
    }

    // Execute based on mode
    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<KMeansRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("KMeans execution failed: {:?}", e),
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
                &format!("KMeans stats failed: {:?}", e),
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
