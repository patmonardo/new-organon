use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::applications::algorithms::pathfinding::spanning_tree::request::SpanningTreeRequest;
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &SpanningTreeRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let memory = graph_resources
                .facade()
                .spanning_tree()
                .start_node(request.start_node)
                .compute_minimum(request.compute_minimum)
                .weight_property(&request.weight_property)
                .relationship_types(request.relationship_types.clone())
                .direction(&request.direction)
                .concurrency(request.common.concurrency.value())
                .estimate_memory();

            json!({
                "ok": true,
                "op": op,
                "mode": "estimate",
                "submode": "memory",
                "data": {
                    "minBytes": memory.min(),
                    "maxBytes": memory.max()
                }
            })
        }
        Some(other) => err(
            op,
            "INVALID_REQUEST",
            &format!("Invalid estimate submode '{other}'. Use 'memory'"),
        ),
    }
}
