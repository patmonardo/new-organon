use crate::applications::algorithms::pathfinding::dag_longest_path::request::DagLongestPathRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, _request: &DagLongestPathRequest, graph_resources: &GraphResources) -> Value {
    let result = graph_resources.facade()
        .dag_longest_path()
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
        Err(e) => err(op, "EXECUTION_ERROR", &format!("DagLongestPath stats failed: {e}")),
    }
}
