//! ApproxMaxKCut algorithm dispatch handler.
//!
//! Handles JSON requests for ApproxMaxKCut community detection operations,
//! delegating to the facade layer for execution.

use crate::procedures::community::approx_max_kcut::{
    ApproxMaxKCutFacade, ApproxMaxKCutRow,
};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle ApproxMaxKCut requests
pub fn handle_approx_max_kcut(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "approxMaxKCut";

    // Parse request parameters
    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let k = request.get("k").and_then(|v| v.as_u64()).unwrap_or(2) as u8;

    let iterations = request
        .get("iterations")
        .and_then(|v| v.as_u64())
        .unwrap_or(8) as usize;

    let random_seed = request
        .get("randomSeed")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let minimize = request
        .get("minimize")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let relationship_weight_property = request
        .get("relationshipWeightProperty")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let min_community_sizes = request
        .get("minCommunitySizes")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64())
                .map(|v| v as usize)
                .collect()
        })
        .unwrap_or(vec![0; k as usize]);

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
    let mut facade = ApproxMaxKCutFacade::new(graph_store)
        .k(k)
        .iterations(iterations)
        .random_seed(random_seed)
        .minimize(minimize)
        .relationship_weight_property(relationship_weight_property);

    if !min_community_sizes.is_empty() {
        facade = facade.min_community_sizes(min_community_sizes);
    }

    // Execute based on mode
    match mode {
        "stream" => match facade.stream() {
            Ok(rows_iter) => {
                let rows: Vec<ApproxMaxKCutRow> = rows_iter.collect();
                json!({
                    "ok": true,
                    "op": op,
                    "data": rows
                })
            }
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("ApproxMaxKCut execution failed: {:?}", e),
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
                &format!("ApproxMaxKCut stats failed: {:?}", e),
            ),
        },
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

/// Common error response builder
fn err(op: &str, code: &str, message: &str) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message } })
}
