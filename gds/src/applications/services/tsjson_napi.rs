use crate::applications::services::graph_store_dispatch;
use crate::applications::services::tsjson_support::{err, ok, parse_facade_context};

fn handle_graph_store(request: &serde_json::Value) -> serde_json::Value {
    let ctx = match parse_facade_context(request) {
        Ok(v) => v,
        Err(e) => return e,
    };
    graph_store_dispatch::handle_graph_store(request, &ctx)
}

fn handle_graph_store_catalog(request: &serde_json::Value) -> serde_json::Value {
    let ctx = match parse_facade_context(request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    super::applications_dispatch::handle_graph_store_catalog(
        request,
        &ctx.user,
        &ctx.db,
        ctx.catalog,
    )
}

// `form_eval` was removed from the TS-JSON FFI boundary to keep the
// interface minimal. Complex form evaluation belongs in the application
// layer; callers should use the application APIs directly.

fn handle_algorithms(request: &serde_json::Value) -> serde_json::Value {
    use super::applications_dispatch;

    let ctx = match parse_facade_context(request) {
        Ok(v) => v,
        Err(e) => return e,
    };

    applications_dispatch::handle_algorithms(request, ctx.catalog)
}

/// TS-JSON boundary for GDS.
///
/// This module is intentionally small and FFI-friendly:
/// - accepts/returns JSON strings
/// - uses stable operation names (`op`)
/// - returns handles for large results instead of materializing data
///
/// The internal Rust "applications" layer is free to mirror Java GDS closely.
pub fn invoke(request_json: String) -> String {
    let request: serde_json::Value = match serde_json::from_str(&request_json) {
        Ok(v) => v,
        Err(e) => {
            return err("", "INVALID_JSON", &format!("Invalid JSON request: {e}")).to_string();
        }
    };

    let op = request.get("op").and_then(|v| v.as_str()).unwrap_or("");

    // Prefer facade-based routing when present.
    if let Some(facade) = request.get("facade").and_then(|v| v.as_str()) {
        let response = match facade {
            "graph_store" => handle_graph_store(&request),
            "graph_store_catalog" => handle_graph_store_catalog(&request),
            "algorithms" => handle_algorithms(&request),
            _ => err(op, "UNSUPPORTED_FACADE", "Unsupported facade."),
        };
        return response.to_string();
    }

    let response = match op {
        "ping" => {
            let nonce = request
                .get("nonce")
                .cloned()
                .unwrap_or(serde_json::Value::Null);
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

    response.to_string()
}

/// Convenience: returns the Rust crate version.
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::applications::graph_store_catalog::loaders::GraphStoreCatalogService;
    use crate::applications::services::tsjson_support::{TsjsonUser, TSJSON_CATALOG_SERVICE};
    use crate::types::catalog::GraphCatalog;
    use crate::types::graph_store::DatabaseId;
    use std::sync::Arc;

    use crate::types::graph_store::{DefaultGraphStore, GraphStore};
    use crate::types::random::{RandomGraphConfig, Randomizable};
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    fn test_catalog(username: &str, is_admin: bool, database_id: &str) -> Arc<dyn GraphCatalog> {
        let user = TsjsonUser::new(username.to_string(), is_admin);
        let db = DatabaseId::new(database_id);
        TSJSON_CATALOG_SERVICE.clone().graph_catalog(&user, &db)
    }

    #[test]
    fn invoke_graph_store_catalog_list_graphs_round_trip() {
        let config = RandomGraphConfig {
            graph_name: "graph1".to_string(),
            database_name: "db1".to_string(),
            ..Default::default()
        };
        let mut rng = StdRng::seed_from_u64(0);
        let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
        let catalog = test_catalog("alice", true, "db1");
        catalog.set("graph1", Arc::new(store));

        let request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "listGraphs",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1"
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            response.get("op").and_then(|v| v.as_str()),
            Some("listGraphs")
        );

        let entries = response
            .get("data")
            .and_then(|v| v.get("entries"))
            .and_then(|v| v.as_array())
            .unwrap();

        assert!(entries
            .iter()
            .any(|e| e.get("name").and_then(|v| v.as_str()) == Some("graph1")));

        let _ = GraphCatalog::drop(catalog.as_ref(), &["graph1"], false);
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
        let catalog = test_catalog("alice", true, "db1");
        catalog.set("graph2", Arc::new(store));

        let request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "graphMemoryUsage",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "graph2"
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            response.get("op").and_then(|v| v.as_str()),
            Some("graphMemoryUsage")
        );

        let data = response.get("data").unwrap();
        assert_eq!(
            data.get("graphName").and_then(|v| v.as_str()),
            Some("graph2")
        );
        assert_eq!(
            data.get("nodeCount").and_then(|v| v.as_u64()),
            Some(expected_nodes)
        );
        assert_eq!(
            data.get("relationshipCount").and_then(|v| v.as_u64()),
            Some(expected_rels)
        );

        let _ = GraphCatalog::drop(catalog.as_ref(), &["graph2"], false);
    }

    #[test]
    fn invoke_graph_store_catalog_drop_graph_round_trip() {
        let config = RandomGraphConfig {
            graph_name: "graph_drop_1".to_string(),
            database_name: "db1".to_string(),
            ..Default::default()
        };
        let mut rng = StdRng::seed_from_u64(30);
        let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
        let catalog = test_catalog("alice", true, "db1");
        catalog.set("graph_drop_1", Arc::new(store));

        let request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "dropGraph",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "graph_drop_1",
            "failIfMissing": true
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();
        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            response.get("op").and_then(|v| v.as_str()),
            Some("dropGraph")
        );

        // Verify it is gone.
        let list_request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "listGraphs",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1"
        });
        let list_json = invoke(list_request.to_string());
        let list_response: serde_json::Value = serde_json::from_str(&list_json).unwrap();
        let entries = list_response
            .get("data")
            .and_then(|v| v.get("entries"))
            .and_then(|v| v.as_array())
            .unwrap();
        assert!(!entries
            .iter()
            .any(|e| e.get("name").and_then(|v| v.as_str()) == Some("graph_drop_1")));
    }

    #[test]
    fn invoke_graph_store_catalog_drop_graphs_round_trip() {
        let mut rng = StdRng::seed_from_u64(31);
        let catalog = test_catalog("alice", true, "db1");
        for name in ["graph_drop_a", "graph_drop_b"] {
            let config = RandomGraphConfig {
                graph_name: name.to_string(),
                database_name: "db1".to_string(),
                ..Default::default()
            };
            let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
            catalog.set(name, Arc::new(store));
        }

        let request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "dropGraphs",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphNames": ["graph_drop_a", "graph_drop_b"],
            "failIfMissing": true
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();
        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            response.get("op").and_then(|v| v.as_str()),
            Some("dropGraphs")
        );

        let dropped = response
            .get("data")
            .and_then(|v| v.get("dropped"))
            .and_then(|v| v.as_array())
            .unwrap();
        assert_eq!(dropped.len(), 2);
    }
}
