//! Betweenness centrality algorithm dispatch handler.
//!
//! Handles JSON requests for Betweenness centrality operations,
//! delegating to the facade layer for execution.

use crate::procedures::centrality::betweenness::BetweennessCentralityFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle Betweenness centrality requests
pub fn handle_betweenness(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "betweenness";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let direction = request
        .get("direction")
        .and_then(|v| v.as_str())
        .unwrap_or("both");

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

    let relationship_weight_property = request
        .get("relationshipWeightProperty")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let sampling_strategy = request
        .get("samplingStrategy")
        .and_then(|v| v.as_str())
        .unwrap_or("all");

    let sampling_size = request
        .get("samplingSize")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize);

    let random_seed = request
        .get("randomSeed")
        .and_then(|v| v.as_u64())
        .unwrap_or(42);

    let estimate_submode = request
        .get("submode")
        .and_then(|v| v.as_str());

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
    let facade = BetweennessCentralityFacade::new(graph_store)
        .direction(direction)
        .concurrency(concurrency)
        .relationship_weight_property(relationship_weight_property)
        .sampling_strategy(sampling_strategy)
        .sampling_size(sampling_size)
        .random_seed(random_seed);

    // Execute based on mode
    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<_> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("Betweenness execution failed: {:?}", e),
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
                &format!("Betweenness stats failed: {:?}", e),
            ),
        },
        "mutate" => {
            let property_name = request
                .get("mutateProperty")
                .and_then(|v| v.as_str())
                .unwrap_or("betweenness");
            match facade.mutate(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Betweenness mutate failed: {:?}", e),
                ),
            }
        }
        "write" => {
            let property_name = request
                .get("writeProperty")
                .and_then(|v| v.as_str())
                .unwrap_or("betweenness");
            match facade.write(property_name) {
                Ok(result) => json!({
                    "ok": true,
                    "op": op,
                    "data": result
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("Betweenness write failed: {:?}", e),
                ),
            }
        }
        "estimate" => match estimate_submode {
            Some("memory") => match facade.estimate_memory() {
                memory => json!({
                    "ok": true,
                    "op": op,
                    "data": {
                        "min": memory.min(),
                        "max": memory.max()
                    }
                }),
            },
            Some(other) => err(
                op,
                "INVALID_REQUEST",
                &format!("Invalid estimate submode '{}'. Use 'memory'", other),
            ),
            None => err(
                op,
                "INVALID_REQUEST",
                "Missing 'submode' parameter for estimate mode",
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
