use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::applications::algorithms::pathfinding::topological_sort::request::TopologicalSortRequest;
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &TopologicalSortRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let memory = graph_resources
                .facade()
                .topological_sort()
                .compute_max_distance(request.compute_max_distance)
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
