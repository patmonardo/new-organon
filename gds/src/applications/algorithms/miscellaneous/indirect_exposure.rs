//! IndirectExposure dispatch handler.

use crate::procedures::miscellaneous::IndirectExposureFacade;
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn handle_indirect_exposure(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "indirect_exposure";

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
        .unwrap_or("stats");

    let sanctioned_property = request
        .get("sanctionedProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("sanctioned");

    let relationship_weight_property = request
        .get("relationshipWeightProperty")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let max_iterations = request
        .get("maxIterations")
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .filter(|v| *v > 0)
        .unwrap_or(20);

    let concurrency = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .filter(|v| *v > 0)
        .unwrap_or(4);

    let facade = IndirectExposureFacade::new(graph_store)
        .sanctioned_property(sanctioned_property)
        .relationship_weight_property(relationship_weight_property)
        .max_iterations(max_iterations)
        .concurrency(concurrency);

    match mode {
        "stats" => match facade.stats() {
            Ok(result) => json!({
                "ok": true,
                "op": op,
                "data": {
                    "exposures": result.exposures,
                    "roots": result.roots,
                    "parents": result.parents,
                    "hops": result.hops,
                    "iterationsRan": result.iterations_ran,
                    "didConverge": result.did_converge,
                }
            }),
            Err(e) => err(
                op,
                "EXECUTION_ERROR",
                &format!("indirectExposure failed: {e}"),
            ),
        },
        other => err(op, "INVALID_REQUEST", &format!("Invalid mode '{other}'")),
    }
}

fn err(op: &str, code: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message }
    })
}
