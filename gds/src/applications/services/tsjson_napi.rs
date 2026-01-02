use once_cell::sync::Lazy;

use crate::applications::graph_store_catalog::facade::ApplicationsFacade;
use crate::applications::graph_store_catalog::facade::DefaultGraphCatalogApplicationsBuilder;
use crate::applications::graph_store_catalog::loaders::{
    GraphStoreCatalogService, PerUserDbGraphStoreCatalogService,
};
use crate::applications::services::logging::Log;
use crate::collections::backends::vec::{VecDouble, VecLong};
use crate::config::GraphStoreConfig;
use crate::core::User;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::SimpleIdMap;
use crate::types::graph::RelationshipTopology;
use crate::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
    GraphStore,
};
use crate::types::properties::node::impls::default_node_property_values::{
    DefaultDoubleNodePropertyValues, DefaultLongNodePropertyValues,
};
use crate::types::properties::node::NodePropertyValues;
use crate::types::properties::relationship::impls::default_relationship_property_values::{
    DefaultDoubleRelationshipPropertyValues, DefaultLongRelationshipPropertyValues,
};
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::types::schema::GraphSchema;

use std::collections::HashMap;
use std::sync::Arc;

static TSJSON_CATALOG_SERVICE: Lazy<Arc<PerUserDbGraphStoreCatalogService>> =
    Lazy::new(|| Arc::new(PerUserDbGraphStoreCatalogService::new()));

#[derive(Clone, Debug)]
struct TsjsonUser {
    username: String,
    roles: Vec<String>,
    permissions: Vec<String>,
}

impl TsjsonUser {
    fn new(username: String, is_admin: bool) -> Self {
        Self {
            username,
            roles: if is_admin {
                vec!["admin".to_string()]
            } else {
                vec![]
            },
            permissions: vec![],
        }
    }
}

impl User for TsjsonUser {
    fn username(&self) -> &str {
        &self.username
    }
    fn roles(&self) -> &[String] {
        &self.roles
    }
    fn is_authenticated(&self) -> bool {
        true
    }
    fn permissions(&self) -> &[String] {
        &self.permissions
    }
}

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

fn handle_graph_store(request: &serde_json::Value) -> serde_json::Value {
    let op = request.get("op").and_then(|v| v.as_str()).unwrap_or("");

    match op {
        "put" => {
            let username = request
                .get("user")
                .and_then(|v| v.get("username"))
                .and_then(|v| v.as_str())
                .unwrap_or("anonymous")
                .to_string();
            let is_admin = request
                .get("user")
                .and_then(|v| v.get("isAdmin"))
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let user = TsjsonUser::new(username, is_admin);

            let database_id = request
                .get("databaseId")
                .and_then(|v| v.as_str())
                .unwrap_or("db");
            let db = DatabaseId::new(database_id);

            let service = TSJSON_CATALOG_SERVICE.clone();
            let catalog = service.graph_catalog(&user, &db);

            let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: graphName");
            };

            let Some(snapshot) = request.get("snapshot") else {
                return err(op, "INVALID_REQUEST", "Missing required field: snapshot");
            };

            let nodes_value = snapshot.get("nodes").and_then(|v| v.as_array());
            let Some(nodes) = nodes_value else {
                return err(
                    op,
                    "INVALID_REQUEST",
                    "snapshot.nodes must be a non-empty integer array",
                );
            };
            if nodes.is_empty() {
                return err(
                    op,
                    "INVALID_REQUEST",
                    "snapshot.nodes must be a non-empty integer array",
                );
            }

            let mut original_node_ids: Vec<i64> = Vec::with_capacity(nodes.len());
            for v in nodes.iter() {
                let Some(n) = v.as_i64() else {
                    return err(
                        op,
                        "INVALID_REQUEST",
                        "snapshot.nodes must be a non-empty integer array",
                    );
                };
                original_node_ids.push(n);
            }

            let mut index_by_original: HashMap<i64, i64> =
                HashMap::with_capacity(original_node_ids.len());
            for (idx, original) in original_node_ids.iter().copied().enumerate() {
                index_by_original.insert(original, idx as i64);
            }

            #[derive(Clone, Debug)]
            struct RelEdge {
                source: i64,
                target: i64,
                props: HashMap<String, serde_json::Value>,
            }
            let mut rels_by_type: HashMap<String, Vec<RelEdge>> = HashMap::new();
            if let Some(rels) = snapshot.get("relationships").and_then(|v| v.as_array()) {
                for rel in rels.iter() {
                    let Some(rel_type) = rel.get("type").and_then(|v| v.as_str()) else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].type must be a non-empty string",
                        );
                    };
                    if rel_type.trim().is_empty() {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].type must be a non-empty string",
                        );
                    }
                    let Some(source_original) = rel.get("source").and_then(|v| v.as_i64()) else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].source must be an integer",
                        );
                    };
                    let Some(target_original) = rel.get("target").and_then(|v| v.as_i64()) else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].target must be an integer",
                        );
                    };

                    let Some(source_mapped) = index_by_original.get(&source_original).copied()
                    else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].source not found in snapshot.nodes",
                        );
                    };
                    let Some(target_mapped) = index_by_original.get(&target_original).copied()
                    else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.relationships[*].target not found in snapshot.nodes",
                        );
                    };

                    rels_by_type
                        .entry(rel_type.to_string())
                        .or_default()
                        .push(RelEdge {
                            source: source_mapped,
                            target: target_mapped,
                            props: rel
                                .get("properties")
                                .and_then(|v| v.as_object())
                                .map(|o| o.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                                .unwrap_or_default(),
                        });
                }
            }

            let mut relationship_topologies = HashMap::new();
            let mut rel_props_by_type: HashMap<
                String,
                HashMap<String, Vec<Vec<serde_json::Value>>>,
            > = HashMap::new();

            for (rel_type, edges) in rels_by_type.into_iter() {
                let mut adjacency: Vec<Vec<i64>> = vec![Vec::new(); original_node_ids.len()];

                let mut keys: std::collections::HashSet<String> = std::collections::HashSet::new();
                for e in edges.iter() {
                    for k in e.props.keys() {
                        keys.insert(k.clone());
                    }
                }

                if !keys.is_empty() {
                    let mut by_key: HashMap<String, Vec<Vec<serde_json::Value>>> = HashMap::new();
                    for k in keys.iter() {
                        by_key.insert(k.clone(), vec![Vec::new(); original_node_ids.len()]);
                    }
                    rel_props_by_type.insert(rel_type.clone(), by_key);
                }

                for e in edges.into_iter() {
                    adjacency[e.source as usize].push(e.target);
                    if let Some(by_key) = rel_props_by_type.get_mut(&rel_type) {
                        for (_k, per_source) in by_key.iter_mut() {
                            let v = e.props.get(_k).cloned().unwrap_or(serde_json::Value::Null);
                            per_source[e.source as usize].push(v);
                        }
                    }
                }

                let topology = RelationshipTopology::new(adjacency, None);
                relationship_topologies.insert(RelationshipType::of(&rel_type), topology);
            }

            let database_info = DatabaseInfo::new(
                DatabaseId::new(&db),
                DatabaseLocation::remote("tsjson", 0, None, None),
            );

            let mut store = DefaultGraphStore::new(
                GraphStoreConfig::default(),
                GraphName::new(graph_name),
                database_info,
                GraphSchema::empty(),
                Capabilities::default(),
                SimpleIdMap::from_original_ids(original_node_ids),
                relationship_topologies,
            );

            // persist rel props
            for (rel_type, by_key) in rel_props_by_type.into_iter() {
                let rel_type_id = RelationshipType::of(&rel_type);
                for (key, per_source) in by_key.into_iter() {
                    let mut flat: Vec<serde_json::Value> = Vec::new();
                    for src in per_source.into_iter() {
                        flat.extend(src);
                    }
                    if flat.is_empty() {
                        continue;
                    }

                    let mut all_longs: Vec<i64> = Vec::with_capacity(flat.len());
                    let mut all_doubles: Vec<f64> = Vec::with_capacity(flat.len());
                    let mut is_all_longs = true;
                    for v in flat.iter() {
                        if let Some(i) = v.as_i64() {
                            all_longs.push(i);
                            all_doubles.push(i as f64);
                        } else if let Some(f) = v.as_f64() {
                            is_all_longs = false;
                            all_doubles.push(f);
                        } else {
                            is_all_longs = false;
                            all_doubles.push(f64::NAN);
                        }
                    }

                    let element_count = all_doubles.len();
                    let pv: Arc<dyn RelationshipPropertyValues> = if is_all_longs {
                        let backend = VecLong::from(all_longs);
                        Arc::new(
                            DefaultLongRelationshipPropertyValues::<VecLong>::from_collection(
                                backend,
                                element_count,
                            ),
                        )
                    } else {
                        let backend = VecDouble::from(all_doubles);
                        Arc::new(
                            DefaultDoubleRelationshipPropertyValues::<VecDouble>::from_collection(
                                backend,
                                element_count,
                            ),
                        )
                    };

                    if let Err(e) =
                        store.add_relationship_property(rel_type_id.clone(), key.clone(), pv)
                    {
                        return err(op, "ERROR", &format!("Failed to add relationship property '{key}' for type '{rel_type}': {e}"));
                    }
                }
            }

            // node properties
            if let Some(props_obj) = snapshot.get("nodeProperties").and_then(|v| v.as_object()) {
                let node_count = GraphStore::node_count(&store);
                for (key, val) in props_obj.iter() {
                    let Some(arr) = val.as_array() else {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.nodeProperties values must be arrays",
                        );
                    };
                    if arr.len() != node_count {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.nodeProperties[*] arrays must match snapshot.nodes length",
                        );
                    }

                    let mut all_longs = Vec::with_capacity(arr.len());
                    let mut all_doubles = Vec::with_capacity(arr.len());
                    let mut is_all_longs = true;
                    let mut is_all_numbers = true;
                    for v in arr.iter() {
                        if let Some(i) = v.as_i64() {
                            all_longs.push(i);
                            all_doubles.push(i as f64);
                        } else if let Some(f) = v.as_f64() {
                            is_all_longs = false;
                            all_doubles.push(f);
                        } else {
                            is_all_numbers = false;
                            break;
                        }
                    }
                    if !is_all_numbers {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.nodeProperties arrays must contain only numbers",
                        );
                    }

                    let pv: Arc<dyn NodePropertyValues> = if is_all_longs {
                        Arc::new(DefaultLongNodePropertyValues::<VecLong>::from_collection(
                            VecLong::from(all_longs),
                            node_count,
                        ))
                    } else {
                        Arc::new(
                            DefaultDoubleNodePropertyValues::<VecDouble>::from_collection(
                                VecDouble::from(all_doubles),
                                node_count,
                            ),
                        )
                    };

                    let labels =
                        std::collections::HashSet::from(
                            [crate::projection::NodeLabel::all_nodes()],
                        );
                    if let Err(e) = store.add_node_property(labels, key.to_string(), pv) {
                        return err(
                            op,
                            "ERROR",
                            &format!("Failed to add node property '{key}': {e}"),
                        );
                    }
                }
            }

            let node_count = GraphStore::node_count(&store) as u64;
            let relationship_count_actual = GraphStore::relationship_count(&store) as u64;

            catalog.set(graph_name, Arc::new(store));

            ok(
                op,
                serde_json::json!({ "graphName": graph_name, "nodeCount": node_count, "relationshipCount": relationship_count_actual }),
            )
        }
        _ => err(op, "UNSUPPORTED_OP", "Unsupported graph_store operation."),
    }
}

fn handle_graph_store_catalog(request: &serde_json::Value) -> serde_json::Value {
    use super::graph_store_catalog_dispatch;

    let username = request
        .get("user")
        .and_then(|v| v.get("username"))
        .and_then(|v| v.as_str())
        .unwrap_or("anonymous")
        .to_string();
    let is_admin = request
        .get("user")
        .and_then(|v| v.get("isAdmin"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let user = TsjsonUser::new(username, is_admin);

    let database_id = request
        .get("databaseId")
        .and_then(|v| v.as_str())
        .unwrap_or("db");
    let db = DatabaseId::new(database_id);

    let service = TSJSON_CATALOG_SERVICE.clone();
    let apps = DefaultGraphCatalogApplicationsBuilder::new(Log::new())
        .with_graph_store_catalog_service(service)
        .build();
    let apps_facade = ApplicationsFacade::with_graph_store_catalog_applications(Box::new(apps));

    let catalog = TSJSON_CATALOG_SERVICE.clone().graph_catalog(&user, &db);
    graph_store_catalog_dispatch::handle_graph_store_catalog_dispatch(
        request,
        apps_facade.graph_store_catalog(),
        &user,
        &db,
        catalog,
    )
}

// `form_eval` was removed from the TS-JSON FFI boundary to keep the
// interface minimal. Complex form evaluation belongs in the application
// layer; callers should use the application APIs directly.

fn handle_algorithms(request: &serde_json::Value) -> serde_json::Value {
    let op = request.get("op").and_then(|v| v.as_str()).unwrap_or("");

    let username = request
        .get("user")
        .and_then(|v| v.get("username"))
        .and_then(|v| v.as_str())
        .unwrap_or("anonymous")
        .to_string();
    let is_admin = request
        .get("user")
        .and_then(|v| v.get("isAdmin"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let user = TsjsonUser::new(username, is_admin);

    let database_id = request
        .get("databaseId")
        .and_then(|v| v.as_str())
        .unwrap_or("db");
    let db = DatabaseId::new(database_id);

    // New unified handlers with mode parameter (preferred)
    use super::algorithms_dispatch;
    let catalog = TSJSON_CATALOG_SERVICE.clone().graph_catalog(&user, &db);

    match op {
        // ============================================================================
        // Unified Pathfinding Algorithms (mode parameter)
        // ============================================================================
        "bfs" => algorithms_dispatch::handle_bfs(request, catalog),
        "dfs" => algorithms_dispatch::handle_dfs(request, catalog),
        "dijkstra" => algorithms_dispatch::handle_dijkstra(request, catalog),
        "bellman_ford" => algorithms_dispatch::handle_bellman_ford(request, catalog),
        "astar" => algorithms_dispatch::handle_astar(request, catalog),
        "delta_stepping" => algorithms_dispatch::handle_delta_stepping(request, catalog),
        "dag_longest_path" => algorithms_dispatch::handle_dag_longest_path(request, catalog),
        "kspanningtree" => algorithms_dispatch::handle_kspanningtree(request, catalog),
        "yens" => algorithms_dispatch::handle_yens(request, catalog),
        "all_shortest_paths" => algorithms_dispatch::handle_all_shortest_paths(request, catalog),
        "spanning_tree" => algorithms_dispatch::handle_spanning_tree(request, catalog),
        "steiner_tree" => algorithms_dispatch::handle_steiner_tree(request, catalog),
        "topological_sort" => algorithms_dispatch::handle_topological_sort(request, catalog),
        "random_walk" => algorithms_dispatch::handle_random_walk(request, catalog),
        "pagerank" => algorithms_dispatch::handle_pagerank(request, catalog),
        "articulation_points" => algorithms_dispatch::handle_articulation_points(request, catalog),
        "betweenness" => algorithms_dispatch::handle_betweenness(request, catalog),
        "bridges" => algorithms_dispatch::handle_bridges(request, catalog),
        "celf" => algorithms_dispatch::handle_celf(request, catalog),
        "closeness" => algorithms_dispatch::handle_closeness(request, catalog),
        "degree_centrality" => algorithms_dispatch::handle_degree_centrality(request, catalog),
        "harmonic" => algorithms_dispatch::handle_harmonic(request, catalog),
        "hits" => algorithms_dispatch::handle_hits(request, catalog),

        // =========================================================================
        // Similarity Algorithms
        // =========================================================================
        "knn" => algorithms_dispatch::handle_knn(request, catalog),
        "node_similarity" => algorithms_dispatch::handle_node_similarity(request, catalog),
        "filtered_knn" => algorithms_dispatch::handle_filtered_knn(request, catalog),
        "filtered_node_similarity" => {
            algorithms_dispatch::handle_filtered_node_similarity(request, catalog)
        }

        // ============================================================================
        // Embedding Algorithms
        // ============================================================================
        "fast_rp" => algorithms_dispatch::handle_fast_rp(request, catalog),
        "gat" => algorithms_dispatch::handle_gat(request, catalog),
        "graphsage" => algorithms_dispatch::handle_graphsage(request, catalog),
        "hash_gnn" => algorithms_dispatch::handle_hash_gnn(request, catalog),
        "node2vec" => algorithms_dispatch::handle_node2vec(request, catalog),

        // =========================================================================
        // Miscellaneous Algorithms
        // =========================================================================
        "to_undirected" => algorithms_dispatch::handle_to_undirected(request, catalog),
        "scale_properties" => algorithms_dispatch::handle_scale_properties(request, catalog),
        "index_inverse" => algorithms_dispatch::handle_index_inverse(request, catalog),
        "collapse_path" => algorithms_dispatch::handle_collapse_path(request, catalog),

        _ => err(op, "UNSUPPORTED_OP", "Unsupported algorithms operation."),
    }
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
    use crate::types::catalog::GraphCatalog;
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
            "op": "list_graphs",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1"
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            response.get("op").and_then(|v| v.as_str()),
            Some("list_graphs")
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
            "op": "graph_memory_usage",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "graph2"
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(
            response.get("op").and_then(|v| v.as_str()),
            Some("graph_memory_usage")
        );

        let data = response.get("data").unwrap();
        assert_eq!(
            data.get("graphName").and_then(|v| v.as_str()),
            Some("graph2")
        );
        assert_eq!(
            data.get("nodes").and_then(|v| v.as_u64()),
            Some(expected_nodes)
        );
        assert_eq!(
            data.get("relationships").and_then(|v| v.as_u64()),
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
            "op": "drop_graph",
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
            Some("drop_graph")
        );

        // Verify it is gone.
        let list_request = serde_json::json!({
            "facade": "graph_store_catalog",
            "op": "list_graphs",
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
            "op": "drop_graphs",
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
            Some("drop_graphs")
        );

        let dropped = response
            .get("data")
            .and_then(|v| v.get("dropped"))
            .and_then(|v| v.as_array())
            .unwrap();
        assert_eq!(dropped.len(), 2);
    }
}
