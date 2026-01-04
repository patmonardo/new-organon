use crate::applications::algorithms::pathfinding::kspanningtree::request::KSpanningTreeRequest;
use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &KSpanningTreeRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let mut builder = graph_resources
                .facade()
                .kspanning_tree()
                .source_node(request.source_node)
                .k(request.k)
                .objective(&request.objective);

            if let Some(ref prop) = request.weight_property {
                builder = builder.weight_property(prop);
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
