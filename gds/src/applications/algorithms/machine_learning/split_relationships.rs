//! Split relationships dispatch handler.
//!
//! Mirrors the Java `MachineLearningAlgorithms.*splitRelationships` application surface.
//!
//! Note: mutate/write behavior (writing relationships back into the GraphStore) is still
//! pending; we currently return counts only.

use crate::concurrency::Concurrency;
use crate::ml::splitting::SplitRelationshipsConfig;
use crate::procedures::machine_learning::SplitRelationshipsFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle split-relationships requests.
pub fn handle_split_relationships(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "split_relationships";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

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

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("mutate");

    let holdout_fraction = request
        .get("holdoutFraction")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.2);

    let negative_sampling_ratio = request
        .get("negativeSamplingRatio")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0);

    let relationship_weight_property = request
        .get("relationshipWeightProperty")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let holdout_relationship_type = request
        .get("holdoutRelationshipType")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let remaining_relationship_type = request
        .get("remainingRelationshipType")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let random_seed = request.get("randomSeed").and_then(|v| v.as_u64());

    let relationship_types = request
        .get("relationshipTypes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect::<std::collections::HashSet<_>>()
        })
        .unwrap_or_default();

    let concurrency_value = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .unwrap_or(1) as usize;

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

    let config = SplitRelationshipsConfig {
        holdout_fraction,
        negative_sampling_ratio,
        relationship_types,
        relationship_weight_property,
        holdout_relationship_type,
        remaining_relationship_type,
        random_seed,
    };

    let facade = SplitRelationshipsFacade::new(graph_store)
        .config(config)
        .concurrency(concurrency);

    match mode {
        "mutate" => match facade.compute() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": {
                    "selectedRelCount": stats.selected_rel_count,
                    "remainingRelCount": stats.remaining_rel_count
                }
            }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("splitRelationships failed: {e}"),
            ),
        },
        "estimate" | "estimate_memory" => match facade.estimate_memory() {
            Ok(range) => json!({
                "ok": true,
                "op": op,
                "data": {
                    "minBytes": range.min(),
                    "maxBytes": range.max()
                }
            }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("splitRelationships estimation failed: {e}"),
            ),
        },
        "stream" | "stats" | "write" => err(
            op,
            "INVALID_REQUEST",
            "Invalid mode for splitRelationships (expected 'mutate' or 'estimate_memory')",
        ),
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

fn err(op: &str, code: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message }
    })
}
