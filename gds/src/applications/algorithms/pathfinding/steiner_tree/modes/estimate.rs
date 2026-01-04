use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::applications::algorithms::pathfinding::steiner_tree::request::SteinerTreeRequest;
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &SteinerTreeRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let mut builder = graph_resources
                .facade()
                .steiner_tree()
                .source_node(request.source_node)
                .target_nodes(request.target_nodes.clone())
                .delta(request.delta)
                .apply_rerouting(request.apply_rerouting)
                .concurrency(request.common.concurrency.value());

            if let Some(ref prop) = request.relationship_weight_property {
                builder = builder.relationship_weight_property(prop);
            }

            match builder.estimate_memory().map_err(|e| e.to_string()) {
                Ok(memory) => json!({
                    "ok": true,
                    "op": op,
                    "mode": "estimate",
                    "submode": "memory",
                    "data": {
                        "minBytes": memory.min(),
                        "maxBytes": memory.max()
                    }
                }),
                Err(e) => err(op, "EXECUTION_ERROR", &format!("SteinerTree estimate failed: {e}")),
            }
        }
        Some(other) => err(
            op,
            "INVALID_REQUEST",
            &format!("Invalid estimate submode '{other}'. Use 'memory'"),
        ),
    }
}
