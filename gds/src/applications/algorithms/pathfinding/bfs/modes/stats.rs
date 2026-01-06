use crate::applications::algorithms::pathfinding::bfs::request::BfsRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &BfsRequest, graph_resources: &GraphResources) -> Value {
    let mut builder = graph_resources.facade()
        .bfs()
        .source(request.source)
        .track_paths(request.track_paths);

    if let Some(max_depth) = request.max_depth {
        builder = builder.max_depth(max_depth);
    }

    let result = builder.stats();

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
        Err(e) => err(op, "EXECUTION_ERROR", &format!("BFS stats failed: {e}")),
    }
}
