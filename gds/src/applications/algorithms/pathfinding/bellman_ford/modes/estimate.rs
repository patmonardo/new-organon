use crate::applications::algorithms::pathfinding::bellman_ford::request::BellmanFordRequest;
use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &BellmanFordRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let mut builder = graph_resources
                .facade()
                .bellman_ford()
                .source(request.source)
                .weight_property(&request.weight_property)
                .direction(&request.direction)
                .track_negative_cycles(request.track_negative_cycles)
                .track_paths(request.track_paths)
                .concurrency(request.common.concurrency.value());

            if !request.relationship_types.is_empty() {
                builder = builder.relationship_types(request.relationship_types.clone());
            }

            let memory = builder.estimate_memory();
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
