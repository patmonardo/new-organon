use crate::applications::algorithms::pathfinding::shared::err;
use crate::applications::algorithms::pathfinding::spanning_tree::request::SpanningTreeRequest;
use crate::core::loading::GraphResources;
use crate::procedures::pathfinding::SpanningTreeRow;
use serde_json::{json, Value};

pub fn run(op: &str, request: &SpanningTreeRequest, graph_resources: &GraphResources) -> Value {
    let mut builder = graph_resources.facade()
        .spanning_tree()
        .start_node(request.start_node)
        .compute_minimum(request.compute_minimum)
        .weight_property(&request.weight_property)
        .direction(&request.direction);

    if !request.relationship_types.is_empty() {
        builder = builder.relationship_types(request.relationship_types.clone());
    }

    let result = builder.stream();

    match result {
        Ok(stream) => {
            let rows: Vec<SpanningTreeRow> = stream.collect();
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
        Err(e) => err(op, "EXECUTION_ERROR", &format!("SpanningTree stream failed: {e}")),
    }
}
