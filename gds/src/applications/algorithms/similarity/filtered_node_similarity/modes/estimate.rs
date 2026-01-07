use crate::applications::algorithms::similarity::filtered_node_similarity::request::FilteredNodeSimilarityRequest;
use crate::applications::algorithms::similarity::shared::{err, Mode};
use crate::core::loading::GraphResources;
use crate::procedures::similarity::filtered_node_similarity::FilteredNodeSimilarityBuilder;
use serde_json::{json, Value};
use std::sync::Arc;

pub fn run(
    op: &str,
    request: &FilteredNodeSimilarityRequest,
    graph_resources: &GraphResources,
) -> Value {
    if request.common.mode != Mode::Estimate {
        return err(op, "INVALID_REQUEST", "Invalid mode");
    }

    match request.common.estimate_submode.as_deref() {
        Some("memory") | None => {
            let mut builder =
                FilteredNodeSimilarityBuilder::new(Arc::clone(graph_resources.store()))
                    .metric(request.metric)
                    .similarity_cutoff(request.similarity_cutoff)
                    .top_k(request.top_k)
                    .top_n(request.top_n)
                    .concurrency(request.common.concurrency.value());

            if let Some(ref prop) = request.weight_property {
                builder = builder.weight_property(prop.clone());
            }
            if let Some(ref label) = request.source_node_label {
                builder = builder.source_node_label(label.clone());
            }
            if let Some(ref label) = request.target_node_label {
                builder = builder.target_node_label(label.clone());
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
