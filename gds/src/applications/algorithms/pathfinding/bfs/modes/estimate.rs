use crate::applications::algorithms::pathfinding::bfs::request::BfsRequest;
use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &BfsRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let mut builder = graph_resources.facade().bfs().source(request.source);
            if !request.targets.is_empty() {
                builder = builder.targets(request.targets.clone());
            }
            if let Some(max_depth) = request.max_depth {
                builder = builder.max_depth(max_depth);
            }
            builder = builder.track_paths(request.track_paths);
            builder = builder.concurrency(request.common.concurrency.value());
            if let Some(delta) = request.delta {
                builder = builder.delta(delta);
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
