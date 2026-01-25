//! KMeans algorithm dispatch handler.
//!
//! Handles JSON requests for KMeans clustering operations,
//! delegating to the facade layer for execution.

use crate::applications::algorithms::community::{err, timings_json};
use crate::applications::algorithms::machinery::{
    AlgorithmProcessingTemplateConvenience, DefaultAlgorithmProcessingTemplate,
    FnStatsResultBuilder, FnStreamResultBuilder, ProgressTrackerCreator, RequestScopedDependencies,
};
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::loading::CatalogLoader;
use crate::core::loading::GraphResources;
use crate::core::utils::progress::{JobId, ProgressTracker, TaskRegistryFactories, Tasks};
use crate::procedures::community::kmeans::{KMeansFacade, KMeansSamplerType};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle KMeans requests
pub fn handle_kmeans(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "kmeans";

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
        Some(prop) => prop.to_string(),
        None => return err(op, "INVALID_REQUEST", "Missing 'nodeProperty' parameter"),
    };

    let concurrency_value = request
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

    let concurrency = match Concurrency::new(concurrency_value) {
        Some(value) => value,
        None => {
            return err(
                op,
                "INVALID_REQUEST",
                "concurrency must be greater than zero",
            )
        }
    };

    let graph_resources = match CatalogLoader::load_or_err(catalog.as_ref(), graph_name) {
        Ok(resources) => resources,
        Err(e) => return err(op, "GRAPH_NOT_FOUND", &e.to_string()),
    };

    let deps = RequestScopedDependencies::new(
        JobId::new(),
        TaskRegistryFactories::empty(),
        TerminationFlag::running_true(),
    );
    let creator = ProgressTrackerCreator::new(deps);
    let template = DefaultAlgorithmProcessingTemplate::new(creator);
    let convenience = AlgorithmProcessingTemplateConvenience::new(template);

    match mode {
        "stream" => {
            let task = Tasks::leaf("kmeans::stream".to_string()).base().clone();
            let property = node_property.clone();
            let seeds = seed_centroids.clone();
            let sampler_type = sampler;

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Vec<Value>>, String> {
                let mut builder = gr
                    .facade()
                    .kmeans()
                    .k(k)
                    .node_property(property.as_str())
                    .concurrency(concurrency_value)
                    .max_iterations(max_iterations)
                    .random_seed(random_seed)
                    .compute_silhouette(compute_silhouette)
                    .sampler_type(sampler_type);

                if let Some(centroids) = seeds.clone() {
                    builder = builder.seed_centroids(centroids);
                }

                let iter = builder.stream().map_err(|e| e.to_string())?;
                let rows = iter
                    .map(|row| serde_json::to_value(row).map_err(|e| e.to_string()))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Some(rows))
            };

            let result_builder =
                FnStreamResultBuilder::new(|_gr: &GraphResources, rows: Option<Vec<Value>>| {
                    rows.unwrap_or_default().into_iter()
                });

            match convenience.process_stream(
                &graph_resources,
                concurrency,
                task,
                compute,
                result_builder,
            ) {
                Ok(stream) => {
                    let rows: Vec<Value> = stream.collect();
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "stream",
                        "data": rows,
                        "timings": json!({
                            "preProcessingMillis": 0,
                            "computeMillis": 0,
                            "sideEffectMillis": 0
                        })
                    })
                }
                Err(e) => err(op, "EXECUTION_ERROR", &format!("KMeans stream failed: {e}")),
            }
        }
        "stats" => {
            let task = Tasks::leaf("kmeans::stats".to_string()).base().clone();
            let property = node_property;
            let seeds = seed_centroids;
            let sampler_type = sampler;

            let compute = move |gr: &GraphResources,
                                _tracker: &mut dyn ProgressTracker,
                                _termination: &TerminationFlag|
                  -> Result<Option<Value>, String> {
                let mut builder = gr
                    .facade()
                    .kmeans()
                    .k(k)
                    .node_property(property.as_str())
                    .concurrency(concurrency_value)
                    .max_iterations(max_iterations)
                    .random_seed(random_seed)
                    .compute_silhouette(compute_silhouette)
                    .sampler_type(sampler_type);

                if let Some(centroids) = seeds.clone() {
                    builder = builder.seed_centroids(centroids);
                }

                let stats = builder.stats().map_err(|e| e.to_string())?;
                let stats_value = serde_json::to_value(stats).map_err(|e| e.to_string())?;
                Ok(Some(stats_value))
            };

            let builder =
                FnStatsResultBuilder(|_gr: &GraphResources, stats: Option<Value>, timings| {
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "stats",
                        "data": stats,
                        "timings": timings_json(timings)
                    })
                });

            match convenience.process_stats(&graph_resources, concurrency, task, compute, builder) {
                Ok(response) => response,
                Err(e) => err(op, "EXECUTION_ERROR", &format!("KMeans stats failed: {e}")),
            }
        }
        "mutate" => {
            let property_name = request
                .get("mutateProperty")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or_else(|| {
                    request
                        .get("expectedPropertyName")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .unwrap_or_else(|| "community".to_string());

            let mut facade = KMeansFacade::new(Arc::clone(graph_resources.store()))
                .k(k)
                .node_property(node_property.as_str())
                .concurrency(concurrency_value)
                .max_iterations(max_iterations)
                .random_seed(random_seed)
                .compute_silhouette(compute_silhouette)
                .sampler_type(sampler);

            if let Some(centroids) = seed_centroids.clone() {
                facade = facade.seed_centroids(centroids);
            }

            match facade.mutate(&property_name) {
                Ok(result) => {
                    catalog.set(graph_name, result.updated_store);
                    json!({"ok": true, "op": op, "data": {
                        "nodes_updated": result.summary.nodes_updated,
                        "property_name": result.summary.property_name,
                        "execution_time_ms": result.summary.execution_time_ms
                    }})
                }
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("KMeans mutate failed: {:?}", e),
                ),
            }
        }
        "write" => {
            let property_name = request
                .get("writeProperty")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or_else(|| {
                    request
                        .get("expectedPropertyName")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                })
                .unwrap_or_else(|| "community".to_string());

            let mut facade = KMeansFacade::new(Arc::clone(graph_resources.store()))
                .k(k)
                .node_property(node_property.as_str())
                .concurrency(concurrency_value)
                .max_iterations(max_iterations)
                .random_seed(random_seed)
                .compute_silhouette(compute_silhouette)
                .sampler_type(sampler);

            if let Some(centroids) = seed_centroids.clone() {
                facade = facade.seed_centroids(centroids);
            }

            match facade.write(&property_name) {
                Ok(result) => json!({"ok": true, "op": op, "data": result}),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("KMeans write failed: {:?}", e),
                ),
            }
        }
        "estimate" => {
            let mut facade = KMeansFacade::new(Arc::clone(graph_resources.store()))
                .k(k)
                .node_property(node_property.as_str())
                .concurrency(concurrency_value)
                .max_iterations(max_iterations)
                .random_seed(random_seed)
                .compute_silhouette(compute_silhouette)
                .sampler_type(sampler);

            if let Some(centroids) = seed_centroids {
                facade = facade.seed_centroids(centroids);
            }

            match facade.estimate_memory() {
                Ok(range) => json!({"ok": true, "op": op, "data": range}),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("KMeans memory estimation failed: {:?}", e),
                ),
            }
        }
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}
