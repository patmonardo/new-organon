use crate::applications::algorithms::similarity::knn::request::KnnRequest;
use crate::applications::algorithms::similarity::shared::{err, Mode};
use crate::core::loading::GraphResources;
use crate::procedures::similarity::knn::KnnBuilder;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn run(op: &str, request: &KnnRequest, graph_resources: &GraphResources) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let primary = &request.node_properties[0];
            let mut builder =
                KnnBuilder::new(Arc::clone(graph_resources.store()), primary.name.clone())
                    .k(request.top_k)
                    .similarity_cutoff(request.similarity_cutoff)
                    .metric(primary.metric)
                    .concurrency(request.common.concurrency.value());

            if request.node_properties.len() > 1 {
                for prop in request.node_properties.iter().skip(1) {
                    builder = builder.add_property(prop.name.clone(), prop.metric);
                }
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
