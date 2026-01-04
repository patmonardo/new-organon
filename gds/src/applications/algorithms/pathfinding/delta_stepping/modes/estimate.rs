use crate::applications::algorithms::pathfinding::delta_stepping::request::DeltaSteppingRequest;
use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &DeltaSteppingRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let mut builder = graph_resources
                .facade()
                .delta_stepping()
                .source(request.source)
                .delta(request.delta)
                .weight_property(&request.weight_property)
                .direction(&request.direction)
                .store_predecessors(request.store_predecessors)
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
