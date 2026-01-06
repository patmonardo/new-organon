use crate::applications::algorithms::pathfinding::dag_longest_path::request::DagLongestPathRequest;
use crate::applications::algorithms::pathfinding::shared::err;
use crate::core::loading::GraphResources;
use crate::procedures::pathfinding::DagLongestPathRow;
use serde_json::{json, Value};

pub fn run(op: &str, _request: &DagLongestPathRequest, graph_resources: &GraphResources) -> Value {
    let result = graph_resources.facade()
        .dag_longest_path()
        .stream();

    match result {
        Ok(stream) => {
            let rows: Vec<DagLongestPathRow> = stream.collect();
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
        Err(e) => err(op, "EXECUTION_ERROR", &format!("DagLongestPath stream failed: {e}")),
    }
}
