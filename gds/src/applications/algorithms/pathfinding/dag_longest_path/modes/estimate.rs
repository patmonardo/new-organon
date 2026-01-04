use crate::applications::algorithms::pathfinding::dag_longest_path::request::DagLongestPathRequest;
use crate::applications::algorithms::pathfinding::shared::{err, Mode};
use crate::core::loading::GraphResources;
use serde_json::{json, Value};

pub fn run(op: &str, request: &DagLongestPathRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let memory = graph_resources
                .facade()
                .dag_longest_path()
                .concurrency(request.common.concurrency.value())
                .estimate_memory()
                .map_err(|e| e.to_string());

            match memory {
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
                Err(e) => err(op, "EXECUTION_ERROR", &format!("DagLongestPath estimate failed: {e}")),
            }
        }
        Some(other) => err(
            op,
            "INVALID_REQUEST",
            &format!("Invalid estimate submode '{other}'. Use 'memory'"),
        ),
    }
}
