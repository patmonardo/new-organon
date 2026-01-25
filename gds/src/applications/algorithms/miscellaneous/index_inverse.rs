//! IndexInverse dispatch handler.

use crate::applications::algorithms::miscellaneous::err;
use crate::concurrency::Concurrency;
use crate::core::loading::CatalogLoader;
use crate::types::catalog::GraphCatalog;
use crate::types::prelude::GraphStore;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::Instant;

pub fn handle_index_inverse(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = "index_inverse";

    let graph_name = match request.get("graphName").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => return err(op, "INVALID_REQUEST", "Missing 'graphName' parameter"),
    };

    let mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("mutate");

    let out_name = request
        .get("mutateGraphName")
        .or_else(|| request.get("writeGraphName"))
        .or_else(|| request.get("targetGraphName"))
        .or_else(|| request.get("outputGraphName"))
        .and_then(|v| v.as_str())
        .unwrap_or("index_inverse")
        .to_string();

    let concurrency_value = request
        .get("concurrency")
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .filter(|v| *v > 0)
        .unwrap_or(4);

    if Concurrency::new(concurrency_value).is_none() {
        return err(
            op,
            "INVALID_REQUEST",
            "concurrency must be greater than zero",
        );
    }

    let relationship_types: Vec<String> = match request.get("relationshipTypes") {
        Some(Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect(),
        Some(Value::String(s)) => vec![s.to_string()],
        _ => vec!["*".to_string()],
    };

    let graph_resources = match CatalogLoader::load_or_err(catalog.as_ref(), graph_name) {
        Ok(resources) => resources,
        Err(e) => return err(op, "GRAPH_NOT_FOUND", &e.to_string()),
    };

    match mode {
        "stats" => {
            let facade = graph_resources
                .facade()
                .index_inverse()
                .concurrency(concurrency_value)
                .relationship_types(relationship_types.clone());

            match facade.stats(&out_name) {
                Ok(stats) => json!({
                    "ok": true,
                    "op": op,
                    "mode": "stats",
                    "data": stats,
                }),
                Err(e) => err(
                    op,
                    "EXECUTION_ERROR",
                    &format!("indexInverse stats failed: {e}"),
                ),
            }
        }
        "mutate" => {
            let facade = graph_resources
                .facade()
                .index_inverse()
                .concurrency(concurrency_value)
                .relationship_types(relationship_types.clone());

            match facade.to_store(&out_name) {
                Ok(store) => {
                    let node_count = GraphStore::node_count(&store) as u64;
                    let relationship_count = GraphStore::relationship_count(&store) as u64;
                    catalog.set(&out_name, Arc::new(store));
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "mutate",
                        "data": {
                            "graphName": out_name,
                            "nodeCount": node_count,
                            "relationshipCount": relationship_count,
                        }
                    })
                }
                Err(e) => err(op, "EXECUTION_ERROR", &format!("indexInverse failed: {e}")),
            }
        }
        "write" => {
            let start = Instant::now();
            let facade = graph_resources
                .facade()
                .index_inverse()
                .concurrency(concurrency_value)
                .relationship_types(relationship_types.clone());

            match facade.to_store(&out_name) {
                Ok(store) => {
                    let node_count = GraphStore::node_count(&store) as u64;
                    let relationship_count = GraphStore::relationship_count(&store) as u64;
                    let execution_time_ms = start.elapsed().as_millis() as u64;
                    catalog.set(&out_name, Arc::new(store));
                    json!({
                        "ok": true,
                        "op": op,
                        "mode": "write",
                        "data": {
                            "graphName": out_name,
                            "nodeCount": node_count,
                            "relationshipCount": relationship_count,
                            "writeResult": {
                                "nodesWritten": node_count,
                                "propertyName": out_name,
                                "executionTimeMs": execution_time_ms,
                            }
                        }
                    })
                }
                Err(e) => err(op, "EXECUTION_ERROR", &format!("indexInverse failed: {e}")),
            }
        }
        other => err(op, "INVALID_REQUEST", &format!("Invalid mode '{other}'")),
    }
}
