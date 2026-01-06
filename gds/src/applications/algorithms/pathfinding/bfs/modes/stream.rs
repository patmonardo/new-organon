use crate::applications::algorithms::pathfinding::bfs::request::BfsRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::core::loading::GraphResources;
use crate::procedures::traits::PathResult;
use serde_json::{json, Value};

pub fn run(op: &str, request: &BfsRequest, graph_resources: &GraphResources) -> Value {
    let mut builder = graph_resources.facade()
        .bfs()
        .source(request.source)
        .track_paths(request.track_paths);

    if let Some(max_depth) = request.max_depth {
        builder = builder.max_depth(max_depth);
    }

    let result = builder.stream();

    match result {
        Ok(stream) => {
            let rows: Vec<PathResult> = stream.collect();
            json!({
                "ok": true,
                "op": op,
                "mode": "stream",
                "data": rows,
                "timings": json!({
                    "pre_processing_millis": 0,
                    "compute_millis": 0,
                    "side_effect_millis": 0
                })
            })
        },
        Err(e) => err(op, "EXECUTION_ERROR", &format!("BFS stream failed: {e}")),
    }
}
