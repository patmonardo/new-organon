use napi_derive::napi;
use once_cell::sync::Lazy;

use crate::types::catalog::{CatalogError, GraphCatalog, InMemoryGraphCatalog};

static TSJSON_CATALOG: Lazy<InMemoryGraphCatalog> = Lazy::new(InMemoryGraphCatalog::new);

fn ok(op: &str, data: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "ok": true,
        "op": op,
        "data": data,
    })
}

fn err(op: &str, code: &str, message: &str) -> serde_json::Value {
    serde_json::json!({
        "ok": false,
        "op": op,
        "error": {
            "code": code,
            "message": message,
        }
    })
}

fn handle_graph_store_catalog(request: &serde_json::Value) -> serde_json::Value {
    let op = request
        .get("op")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    match op {
        // Mirrors logic/src/absolute/form/gds.application.ts
        "list_graphs" => {
            // Optional extension: includeDegreeDist. Default false.
            let include_degree_dist = request
                .get("includeDegreeDist")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let entries = TSJSON_CATALOG
                .list(None, include_degree_dist)
                .into_iter()
                .map(|e| {
                    serde_json::json!({
                        "name": e.name,
                        "nodeCount": e.node_count,
                        "relationshipCount": e.relationship_count,
                        "degreeDistribution": e.degree_distribution,
                    })
                })
                .collect::<Vec<_>>();

            ok(op, serde_json::json!({ "entries": entries }))
        }
        "graph_memory_usage" => {
            let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: graphName");
            };

            match TSJSON_CATALOG.size_of(graph_name) {
                Ok(mu) => ok(
                    op,
                    serde_json::json!({
                        "graphName": graph_name,
                        "bytes": mu.bytes,
                        "nodes": mu.nodes,
                        "relationships": mu.relationships,
                    }),
                ),
                Err(CatalogError::NotFound(_)) => err(op, "NOT_FOUND", "Graph not found"),
                Err(CatalogError::AlreadyExists(_)) => {
                    err(op, "INTERNAL", "Unexpected catalog error")
                }
            }
        }
        _ => err(op, "UNSUPPORTED_OP", "Unsupported graph_store_catalog operation."),
    }
}

/// TS-JSON / NAPI boundary for GDS.
///
/// This module is intentionally small and FFI-friendly:
/// - accepts/returns JSON strings
/// - uses stable operation names (`op`)
/// - returns handles for large results instead of materializing data
///
/// The internal Rust "applications" layer is free to mirror Java GDS closely.
#[napi]
pub fn invoke(request_json: String) -> napi::Result<String> {
    let request: serde_json::Value = serde_json::from_str(&request_json)
        .map_err(|e| napi::Error::from_reason(format!("Invalid JSON request: {e}")))?;

    let op = request
        .get("op")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Prefer facade-based routing when present.
    if let Some(facade) = request.get("facade").and_then(|v| v.as_str()) {
        let response = match facade {
            "graph_store_catalog" => handle_graph_store_catalog(&request),
            _ => err(op, "UNSUPPORTED_FACADE", "Unsupported facade."),
        };
        return Ok(response.to_string());
    }

    let response = match op {
        "ping" => {
            let nonce = request.get("nonce").cloned().unwrap_or(serde_json::Value::Null);
            ok("ping", serde_json::json!({ "nonce": nonce }))
        }
        "version" => ok(
            "version",
            serde_json::json!({
                "crate": "gds",
                "version": env!("CARGO_PKG_VERSION")
            }),
        ),
        _ => err(op, "UNSUPPORTED_OP", "Unsupported operation."),
    };

    Ok(response.to_string())
}

/// Convenience: returns the Rust crate version.
#[napi]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use crate::types::graph_store::{DefaultGraphStore, GraphStore};
    use crate::types::random::{RandomGraphConfig, Randomizable};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn invoke_graph_store_catalog_list_graphs_round_trip() {
        let config = RandomGraphConfig {
            graph_name: "graph1".to_string(),
            database_name: "db1".to_string(),
            ..Default::default()
        };
        let mut rng = StdRng::seed_from_u64(0);
        let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
        TSJSON_CATALOG.set("graph1", Arc::new(store));

        let request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "list_graphs",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1"
        });

        let response_json = invoke(request.to_string()).unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(response.get("op").and_then(|v| v.as_str()), Some("list_graphs"));

        let entries = response
            .get("data")
            .and_then(|v| v.get("entries"))
            .and_then(|v| v.as_array())
            .unwrap();

        assert!(entries
            .iter()
            .any(|e| e.get("name").and_then(|v| v.as_str()) == Some("graph1")));

        let _ = TSJSON_CATALOG.drop(&["graph1"], false);
    }

    #[test]
    fn invoke_graph_store_catalog_graph_memory_usage_round_trip() {
        let config = RandomGraphConfig {
            graph_name: "graph2".to_string(),
            database_name: "db1".to_string(),
            ..Default::default()
        };
        let mut rng = StdRng::seed_from_u64(1);
        let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
        let expected_nodes = GraphStore::node_count(&store) as u64;
        let expected_rels = GraphStore::relationship_count(&store) as u64;
        TSJSON_CATALOG.set("graph2", Arc::new(store));

        let request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "graph_memory_usage",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "graph2"
        });

        let response_json = invoke(request.to_string()).unwrap();
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(response.get("op").and_then(|v| v.as_str()), Some("graph_memory_usage"));

        let data = response.get("data").unwrap();
        assert_eq!(data.get("graphName").and_then(|v| v.as_str()), Some("graph2"));
        assert_eq!(data.get("nodes").and_then(|v| v.as_u64()), Some(expected_nodes));
        assert_eq!(data.get("relationships").and_then(|v| v.as_u64()), Some(expected_rels));

        let _ = TSJSON_CATALOG.drop(&["graph2"], false);
    }
}
