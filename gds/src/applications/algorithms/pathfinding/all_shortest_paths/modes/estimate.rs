use crate::applications::algorithms::pathfinding::all_shortest_paths::request::AllShortestPathsRequest;
use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &AllShortestPathsRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let mut builder = graph_resources
                .facade()
                .all_shortest_paths()
                .weighted(request.weighted)
                .direction(&request.direction)
                .weight_property(&request.weight_property)
                .concurrency(request.common.concurrency.value());

            if !request.relationship_types.is_empty() {
                builder = builder.relationship_types(request.relationship_types.clone());
            }
            if let Some(max) = request.max_results {
                builder = builder.max_results(max);
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
