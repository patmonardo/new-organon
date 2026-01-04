use crate::applications::algorithms::pathfinding::dijkstra::request::DijkstraRequest;
use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &DijkstraRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let mut builder = graph_resources.facade().dijkstra().source(request.source);
            if !request.targets.is_empty() {
                builder = builder.targets(request.targets.clone());
            }
            builder = builder
                .weight_property(&request.weight_property)
                .direction(&request.direction)
                .track_relationships(request.track_relationships)
                .concurrency(request.common.concurrency.value());

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
