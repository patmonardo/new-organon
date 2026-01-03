//! CELF algorithm dispatch handler.
//!
//! Handles JSON requests for Cost-Effective Lazy Forward influence maximization operations,
//! delegating to the facade layer for execution.

use crate::procedures::centrality::celf::{CELFFacade, CELFRow};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle CELF requests
pub fn handle_celf(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "celf";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    // Parse CELF configuration parameters
    let seed_set_size = request
        .get("seedSetSize")
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as usize;

    let monte_carlo_simulations = request
        .get("monteCarloSimulations")
        .and_then(|v| v.as_u64())
        .unwrap_or(100) as usize;

    let propagation_probability = request
        .get("propagationProbability")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.1);

    let batch_size = request
        .get("batchSize")
        .and_then(|v| v.as_u64())
        .unwrap_or(10) as usize;

    let random_seed = request
        .get("randomSeed")
        .and_then(|v| v.as_u64())
        .unwrap_or(42);

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(4) as usize;

    let estimate_submode = request
        .get("estimateSubmode")
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

    // Create facade with configuration
    let facade = CELFFacade::new(graph_store)
        .seed_set_size(seed_set_size)
        .monte_carlo_simulations(monte_carlo_simulations)
        .propagation_probability(propagation_probability)
        .batch_size(batch_size)
        .random_seed(random_seed)
        .concurrency(concurrency);

    // Execute based on mode
    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<CELFRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("CELF execution failed: {:?}", e),
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
                &format!("CELF stats failed: {:?}", e),
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
                    &format!("CELF mutate failed: {:?}", e),
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
                    &format!("CELF write failed: {:?}", e),
                ),
            }
        }
        "estimate" => {
            match estimate_submode {
                Some("memory") => {
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
                Some(other) => err(
                    op,
                    "INVALID_REQUEST",
                    &format!("Invalid estimate submode '{}'. Use 'memory'", other),
                ),
                None => err(
                    op,
                    "INVALID_REQUEST",
                    "Missing 'estimateSubmode' parameter for estimate mode",
                ),
            }
        }
        _ => err(
            op,
            "INVALID_REQUEST",
            "Invalid mode. Use 'stream', 'stats', 'mutate', 'write', or 'estimate'",
        ),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
