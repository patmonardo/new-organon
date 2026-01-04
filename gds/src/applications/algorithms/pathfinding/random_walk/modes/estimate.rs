use crate::applications::algorithms::pathfinding::random_walk::request::RandomWalkRequest;
use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &RandomWalkRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let builder = graph_resources
                .facade()
                .random_walk()
                .walks_per_node(request.walks_per_node)
                .walk_length(request.walk_length)
                .return_factor(request.return_factor)
                .in_out_factor(request.in_out_factor)
                .source_nodes(request.source_nodes.clone())
                .concurrency(request.common.concurrency.value());

            let builder = match request.random_seed {
                Some(seed) => builder.random_seed(seed),
                None => builder,
            };

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
