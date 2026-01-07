use crate::applications::algorithms::pathfinding::shared::err;
use crate::applications::algorithms::pathfinding::topological_sort::request::TopologicalSortRequest;
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &TopologicalSortRequest, graph_resources: &GraphResources) -> Value {
    let result = graph_resources
        .facade()
        .topological_sort()
        .compute_max_distance(request.compute_max_distance)
        .stats();

    match result {
        Ok(stats) => json!({
            "ok": true,
            "op": op,
            "mode": "stats",
            "data": stats,
            "timings": json!({
                "pre_processing_millis": 0,
                "compute_millis": stats.execution_time_ms as i64,
                "side_effect_millis": 0
            })
        }),
        Err(e) => err(
            op,
            "EXECUTION_ERROR",
            &format!("TopologicalSort stats failed: {e}"),
        ),
    }
}
