//! KGE predict dispatch handler.
//!
//! Mirrors the Java `MachineLearningAlgorithms.*kge` application surface.

use crate::procedures::machine_learning::{KgePredictFacade, ScoreFunction};
use crate::types::catalog::GraphCatalog;
use serde_json::{json, Value};
use std::sync::Arc;

/// Handle KGE predict requests.
pub fn handle_kge(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "kge";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let graph_store = match catalog.get(graph_name) {
        Some(store) => store,
        None => {
            return err(
                op,
                "GRAPH_NOT_FOUND",
                &format!("Graph '{}' not found", graph_name),
            )
        }
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("stream");

    let node_embedding_property = request
        .get("nodeEmbeddingProperty")
        .and_then(|v| v.as_str())
        .unwrap_or("embedding")
        .to_string();

    let relationship_type_embedding: Vec<f64> = request
        .get("relationshipTypeEmbedding")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_f64()).collect())
        .unwrap_or_default();

    let scoring_function = match request
        .get("scoringFunction")
        .and_then(|v| v.as_str())
        .unwrap_or("DISTMULT")
    {
        "TRANSE" => ScoreFunction::Transe,
        "DISTMULT" => ScoreFunction::Distmult,
        other => {
            return err(
                op,
                "INVALID_REQUEST",
                &format!("Unknown scoringFunction '{other}'"),
            )
        }
    };

    let top_k = request.get("topK").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

    let facade = KgePredictFacade::new(graph_store)
        .node_embedding_property(node_embedding_property)
        .relationship_type_embedding(relationship_type_embedding)
        .scoring_function(scoring_function)
        .top_k(top_k);

    match mode {
        "stream" => match facade.stream() {
            Ok(result) => {
                let rows: Vec<Value> = result
                    .map(|row| {
                        json!({
                            "sourceNodeId": row.source_node_id,
                            "targetNodeId": row.target_node_id,
                            "score": row.score
                        })
                    })
                    .collect();

                json!({ "ok": true, "op": op, "data": rows })
            }
            Err(e) => err(op, "EXECUTION_ERROR", &format!("KGE failed: {e}")),
        },
        "stats" => match facade.stats() {
            Ok(stats) => json!({
                "ok": true,
                "op": op,
                "data": {
                    "relationshipsWritten": stats.relationships_written,
                    "linksConsidered": stats.links_considered
                }
            }),
            Err(e) => err(op, "EXECUTION_ERROR", &format!("KGE failed: {e}")),
        },
        "mutate" | "write" | "estimate" => err(
            op,
            "NOT_IMPLEMENTED",
            "mutate/write/estimate_memory are not yet wired for KGE",
        ),
        _ => err(op, "INVALID_REQUEST", "Invalid mode"),
    }
}

fn err(op: &str, code: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message }
    })
}
