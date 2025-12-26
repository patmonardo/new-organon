use once_cell::sync::Lazy;

use crate::config::GraphStoreConfig;
use crate::core::User;
use crate::types::catalog::GraphCatalog;
use crate::types::graph::id_map::SimpleIdMap;
use crate::types::graph::RelationshipTopology;
use crate::types::graph_store::{Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName, GraphStore};
use crate::projection::NodeLabel;
use crate::types::properties::node::impls::default_node_property_values::{
    DefaultDoubleNodePropertyValues, DefaultLongNodePropertyValues,
};
use crate::types::properties::node::NodePropertyValues;
use crate::collections::backends::vec::{VecDouble, VecLong};
use crate::types::properties::relationship::impls::default_relationship_property_values::{
    DefaultDoubleRelationshipPropertyValues, DefaultLongRelationshipPropertyValues,
};
use crate::types::properties::relationship::RelationshipPropertyValues;
use crate::projection::eval::form::{FormProcessor, FormRequest};
use crate::projection::eval::procedure::ExecutionContext;
use crate::projection::RelationshipType;
use crate::form::program::{Context as FormContext, Morph as FormMorph, Shape as FormShapeMeta, FormShape};
use crate::types::schema::GraphSchema;
use crate::applications::services::logging::Log;
use crate::applications::graph_store_catalog::facade::{
    ApplicationsFacade,
};
use crate::applications::graph_store_catalog::facade::DefaultGraphCatalogApplicationsBuilder;
use crate::applications::graph_store_catalog::loaders::{
    GraphStoreCatalogService, PerUserDbGraphStoreCatalogService,
};

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
            roles: if is_admin { vec!["admin".to_string()] } else { vec![] },
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

fn handle_graph_store_catalog(request: &serde_json::Value) -> serde_json::Value {
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

    // Route through the facade contract (ApplicationsFacade), backed by the shared TSJSON catalog.
    //
    // Important: this uses the *real* GraphStoreCatalog applications facade implementation
    // (`DefaultGraphCatalogApplications`) rather than a transport-local stub.
    //
    // We construct this per-call to keep the FFI boundary simple (no long-lived globals required).
    let service = TSJSON_CATALOG_SERVICE.clone();
    let apps = DefaultGraphCatalogApplicationsBuilder::new(Log::new())
        .with_graph_store_catalog_service(service)
        .build();
    let apps_facade =
        ApplicationsFacade::with_graph_store_catalog_applications(Box::new(apps));

    match op {
        "graph_exists" => {
            let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: graphName");
            };

            let exists = apps_facade
                .graph_store_catalog()
                .graph_exists(&user, &db, graph_name);
            ok(op, serde_json::json!({ "graphName": graph_name, "exists": exists }))
        }
        // Mirrors logic/src/absolute/form/gds.application.ts
        "list_graphs" => {
            let graph_name = request
                .get("graphName")
                .and_then(|v| v.as_str())
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());
            let include_degree_distribution = request
                .get("includeDegreeDistribution")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            let entries = apps_facade
                .graph_store_catalog()
                .list_graphs(&user, &db, graph_name, include_degree_distribution)
                .into_iter()
                .map(|e| {
                    let mut obj = serde_json::Map::new();
                    obj.insert("name".to_string(), serde_json::Value::String(e.graph_name().to_string()));
                    obj.insert(
                        "nodeCount".to_string(),
                        serde_json::Value::Number(serde_json::Number::from(e.node_count())),
                    );
                    obj.insert(
                        "relationshipCount".to_string(),
                        serde_json::Value::Number(serde_json::Number::from(e.relationship_count())),
                    );
                    if let Some(dist) = e.degree_distribution() {
                        let mut dd = serde_json::Map::new();
                        for (deg, count) in dist.iter() {
                            dd.insert(
                                deg.to_string(),
                                serde_json::Value::Number(serde_json::Number::from(*count)),
                            );
                        }
                        obj.insert("degreeDistribution".to_string(), serde_json::Value::Object(dd));
                    }
                    serde_json::Value::Object(obj)
                })
                .collect::<Vec<_>>();

            ok(op, serde_json::json!({ "entries": entries }))
        }
        "drop_graph" => {
            let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: graphName");
            };
            let fail_if_missing = request
                .get("failIfMissing")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            match apps_facade
                .graph_store_catalog()
                .drop_graph(&user, &db, graph_name, fail_if_missing)
            {
                Ok(d) => ok(
                    op,
                    serde_json::json!({
                        "dropped": [{
                            "name": d.graph_name(),
                            "nodeCount": d.node_count(),
                            "relationshipCount": d.relationship_count()
                        }]
                    }),
                ),
                Err(message) => {
                    if message.to_lowercase().contains("not found") {
                        err(op, "NOT_FOUND", "Graph not found")
                    } else {
                        err(op, "ERROR", &message)
                    }
                }
            }
        }
        "drop_graphs" => {
            let graph_names = request
                .get("graphNames")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let fail_if_missing = request
                .get("failIfMissing")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            match apps_facade
                .graph_store_catalog()
                .drop_graphs(&user, &db, &graph_names, fail_if_missing)
            {
                Ok(dropped) => ok(
                    op,
                    serde_json::json!({
                        "dropped": dropped.into_iter().map(|d| serde_json::json!({
                            "name": d.graph_name(),
                            "nodeCount": d.node_count(),
                            "relationshipCount": d.relationship_count()
                        })).collect::<Vec<_>>()
                    }),
                ),
                Err(message) => {
                    if message.to_lowercase().contains("not found") {
                        err(op, "NOT_FOUND", "Graph not found")
                    } else {
                        err(op, "ERROR", &message)
                    }
                }
            }
        }
        "drop_node_properties" => {
            err(op, "UNIMPLEMENTED", "drop_node_properties not wired in TSJSON facade yet.")
        }
        "stream_relationships" => {
            let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: graphName");
            };

            let relationship_types = request
                .get("relationshipTypes")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            match apps_facade
                .graph_store_catalog()
                .stream_relationships(&user, &db, graph_name, &relationship_types)
            {
                Ok(rows) => ok(
                    op,
                    serde_json::json!({
                        "graphName": graph_name,
                        "relationships": rows.into_iter().map(|r| serde_json::json!({
                            "sourceNodeId": r.source_node_id,
                            "targetNodeId": r.target_node_id,
                            "relationshipType": r.relationship_type,
                        })).collect::<Vec<_>>()
                    }),
                ),
                Err(message) => err(op, "ERROR", &message),
            }
        }
        "stream_node_properties" => {
            let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: graphName");
            };
            let node_properties = request
                .get("nodeProperties")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            let node_labels = request
                .get("nodeLabels")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            let list_node_labels = request
                .get("listNodeLabels")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            match apps_facade
                .graph_store_catalog()
                .stream_node_properties(
                    &user,
                    &db,
                    graph_name,
                    &node_properties,
                    &node_labels,
                    list_node_labels,
                )
            {
                Ok(rows) => ok(
                    op,
                    serde_json::json!({
                        "graphName": graph_name,
                        "rows": rows.into_iter().map(|r| serde_json::json!({
                            "nodeId": r.node_id,
                            "nodeProperty": r.node_property,
                            "propertyValue": r.property_value,
                            "nodeLabels": r.node_labels,
                        })).collect::<Vec<_>>()
                    }),
                ),
                Err(message) => err(op, "ERROR", &message),
            }
        }
        "stream_relationship_properties" => {
            let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: graphName");
            };
            let relationship_properties = request
                .get("relationshipProperties")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            let relationship_types = request
                .get("relationshipTypes")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            match apps_facade.graph_store_catalog().stream_relationship_properties(
                &user,
                &db,
                graph_name,
                &relationship_properties,
                &relationship_types,
            ) {
                Ok(rows) => ok(
                    op,
                    serde_json::json!({
                        "graphName": graph_name,
                        "rows": rows.into_iter().map(|r| serde_json::json!({
                            "sourceNodeId": r.source_node_id,
                            "targetNodeId": r.target_node_id,
                            "relationshipType": r.relationship_type,
                            "relationshipProperty": r.relationship_property,
                            "propertyValue": r.property_value,
                        })).collect::<Vec<_>>()
                    }),
                ),
                Err(message) => err(op, "ERROR", &message),
            }
        }
        "graph_memory_usage" => {
            let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: graphName");
            };

            let mu = apps_facade
                .graph_store_catalog()
                .graph_memory_usage(&user, &db, graph_name);
            ok(
                op,
                serde_json::json!({
                    "graphName": graph_name,
                    "bytes": mu.size_in_bytes,
                    "nodes": mu.node_count,
                    "relationships": mu.relationship_count,
                }),
            )
        }
        "project_native" => {
            let projection = request.get("projectionConfig").unwrap_or(&serde_json::Value::Null);
            let Some(graph_name) = projection.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: projectionConfig.graphName");
            };
            let source_graph_name = projection
                .get("sourceGraphName")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let fictitious_loading = projection
                .get("fictitiousLoading")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let node_labels = projection
                .get("nodeLabels")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let node_properties = projection
                .get("nodeProperties")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let relationship_types = projection
                .get("relationshipTypes")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let relationship_properties = projection
                .get("relationshipProperties")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let relationship_property_selectors: std::collections::HashMap<String, String> = projection
                .get("relationshipPropertySelectors")
                .and_then(|v| v.as_object())
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect()
                })
                .unwrap_or_default();
            let weight_property = projection
                .get("weightProperty")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let cfg = crate::applications::graph_store_catalog::facade::NativeProjectionConfig {
                graph_name: graph_name.to_string(),
                source_graph_name,
                node_labels,
                node_properties,
                relationship_types,
                relationship_properties,
                relationship_property_selectors,
                weight_property,
                fictitious_loading,
            };

            match apps_facade
                .graph_store_catalog()
                .project_native(&user, &db, &cfg)
            {
                Ok(r) => ok(
                    op,
                    serde_json::json!({
                        "graphName": r.graph_name(),
                        "nodeCount": r.nodes_projected(),
                        "relationshipCount": r.relationships_projected(),
                        "projectMillis": r.projection_time_ms(),
                    }),
                ),
                Err(message) => err(op, "ERROR", &message),
            }
        }
        "project_generic" => {
            let projection = request.get("projectionConfig").unwrap_or(&serde_json::Value::Null);
            let Some(graph_name) = projection.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: projectionConfig.graphName");
            };
            let source_graph_name = projection
                .get("sourceGraphName")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let fictitious_loading = projection
                .get("fictitiousLoading")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let node_labels = projection
                .get("nodeLabels")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let node_properties = projection
                .get("nodeProperties")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let relationship_types = projection
                .get("relationshipTypes")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let relationship_properties = projection
                .get("relationshipProperties")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .filter(|s| !s.trim().is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let relationship_property_selectors: std::collections::HashMap<String, String> = projection
                .get("relationshipPropertySelectors")
                .and_then(|v| v.as_object())
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect()
                })
                .unwrap_or_default();
            let weight_property = projection
                .get("weightProperty")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let cfg = crate::applications::graph_store_catalog::facade::GenericProjectionConfig {
                graph_name: graph_name.to_string(),
                source_graph_name,
                node_labels,
                node_properties,
                relationship_types,
                relationship_properties,
                relationship_property_selectors,
                weight_property,
                fictitious_loading,
            };

            match apps_facade
                .graph_store_catalog()
                .project_generic(&user, &db, &cfg)
            {
                Ok(r) => ok(
                    op,
                    serde_json::json!({
                        "graphName": r.graph_name(),
                        "nodeCount": r.nodes_projected(),
                        "relationshipCount": r.relationships_projected(),
                        "projectMillis": r.projection_time_ms(),
                    }),
                ),
                Err(message) => err(op, "ERROR", &message),
            }
        }
        _ => err(
            op,
            "UNSUPPORTED_OP",
            "Unsupported graph_store_catalog operation.",
        ),
    }
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

            let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: graphName");
            };

            let Some(snapshot) = request.get("snapshot") else {
                return err(op, "INVALID_REQUEST", "Missing required field: snapshot");
            };

            let nodes_value = snapshot.get("nodes").and_then(|v| v.as_array());
            let Some(nodes) = nodes_value else {
                return err(op, "INVALID_REQUEST", "snapshot.nodes must be a non-empty integer array");
            };
            if nodes.is_empty() {
                return err(op, "INVALID_REQUEST", "snapshot.nodes must be a non-empty integer array");
            }

            let mut original_node_ids: Vec<i64> = Vec::with_capacity(nodes.len());
            for v in nodes.iter() {
                let Some(n) = v.as_i64() else {
                    return err(op, "INVALID_REQUEST", "snapshot.nodes must be a non-empty integer array");
                };
                original_node_ids.push(n);
            }

            let mut index_by_original: HashMap<i64, i64> = HashMap::with_capacity(original_node_ids.len());
            for (idx, original) in original_node_ids.iter().copied().enumerate() {
                index_by_original.insert(original, idx as i64);
            }

            // Group relationships by type.
            #[derive(Clone, Debug)]
            struct RelEdge {
                source: i64,
                target: i64,
                props: std::collections::HashMap<String, serde_json::Value>,
            }
            let mut rels_by_type: HashMap<String, Vec<RelEdge>> = HashMap::new();
            if let Some(rels) = snapshot.get("relationships").and_then(|v| v.as_array()) {
                for rel in rels.iter() {
                    let Some(rel_type) = rel.get("type").and_then(|v| v.as_str()) else {
                        return err(op, "INVALID_REQUEST", "snapshot.relationships[*].type must be a non-empty string");
                    };
                    if rel_type.trim().is_empty() {
                        return err(op, "INVALID_REQUEST", "snapshot.relationships[*].type must be a non-empty string");
                    }
                    let Some(source_original) = rel.get("source").and_then(|v| v.as_i64()) else {
                        return err(op, "INVALID_REQUEST", "snapshot.relationships[*].source must be an integer");
                    };
                    let Some(target_original) = rel.get("target").and_then(|v| v.as_i64()) else {
                        return err(op, "INVALID_REQUEST", "snapshot.relationships[*].target must be an integer");
                    };

                    let Some(source_mapped) = index_by_original.get(&source_original).copied() else {
                        return err(op, "INVALID_REQUEST", "snapshot.relationships[*].source not found in snapshot.nodes");
                    };
                    let Some(target_mapped) = index_by_original.get(&target_original).copied() else {
                        return err(op, "INVALID_REQUEST", "snapshot.relationships[*].target not found in snapshot.nodes");
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

            let mut relationship_topologies = std::collections::HashMap::new();
            // Temporary per-type relationship properties collected by source node adjacency order.
            let mut rel_props_by_type: HashMap<String, HashMap<String, Vec<Vec<serde_json::Value>>>> =
                HashMap::new();

            for (rel_type, edges) in rels_by_type.into_iter() {
                let mut adjacency: Vec<Vec<i64>> = vec![Vec::new(); original_node_ids.len()];

                // Collect property keys for this relationship type.
                let mut keys: std::collections::HashSet<String> = std::collections::HashSet::new();
                for e in edges.iter() {
                    for k in e.props.keys() {
                        keys.insert(k.clone());
                    }
                }

                // Init per-key per-source buffers.
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
                        for (k, per_source) in by_key.iter_mut() {
                            let v = e
                                .props
                                .get(k)
                                .cloned()
                                .unwrap_or(serde_json::Value::Null);
                            per_source[e.source as usize].push(v);
                        }
                    }
                }

                let topology = RelationshipTopology::new(adjacency, None);
                relationship_topologies.insert(RelationshipType::of(&rel_type), topology);
            }

            let database_id = request
                .get("databaseId")
                .and_then(|v| v.as_str())
                .unwrap_or("db");
            let db = DatabaseId::new(database_id);

            let database_info = DatabaseInfo::new(
                DatabaseId::new(database_id),
                DatabaseLocation::remote("tsjson", 0, None, None),
            );

            let mut store = DefaultGraphStore::new(
                GraphStoreConfig::default(),
                GraphName::new(graph_name),
                database_info,
                GraphSchema::empty(),
                Capabilities::default(),
                SimpleIdMap::from_original_ids(original_node_ids.into_iter()),
                relationship_topologies,
            );

            // Persist relationship properties, if present.
            for (rel_type, by_key) in rel_props_by_type.into_iter() {
                let rel_type_id = RelationshipType::of(&rel_type);
                // Flatten per-key values into (source order, adjacency order) vectors.
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
                            // Null / missing => NaN placeholder
                            is_all_longs = false;
                            all_doubles.push(f64::NAN);
                        }
                    }

                    let element_count = all_doubles.len();
                    let pv: std::sync::Arc<dyn RelationshipPropertyValues> = if is_all_longs {
                        // Use Long property values when all values are integral.
                        let backend = crate::collections::backends::vec::VecLong::from(all_longs);
                        std::sync::Arc::new(
                            DefaultLongRelationshipPropertyValues::<crate::collections::backends::vec::VecLong>::from_collection(
                                backend,
                                element_count,
                            ),
                        )
                    } else {
                        let backend = crate::collections::backends::vec::VecDouble::from(all_doubles);
                        std::sync::Arc::new(
                            DefaultDoubleRelationshipPropertyValues::<crate::collections::backends::vec::VecDouble>::from_collection(
                                backend,
                                element_count,
                            ),
                        )
                    };

                    if let Err(e) = store.add_relationship_property(rel_type_id.clone(), key.clone(), pv) {
                        return err(
                            op,
                            "ERROR",
                            &format!("Failed to add relationship property '{key}' for type '{rel_type}': {e}"),
                        );
                    }
                }
            }

            // Optional: persist node properties (Java parity: nodeProperties(key) on Graph view).
            if let Some(props_obj) = snapshot.get("nodeProperties").and_then(|v| v.as_object()) {
                let node_count = GraphStore::node_count(&store) as usize;
                for (key, val) in props_obj.iter() {
                    let Some(arr) = val.as_array() else {
                        return err(op, "INVALID_REQUEST", "snapshot.nodeProperties values must be arrays");
                    };
                    if arr.len() != node_count {
                        return err(
                            op,
                            "INVALID_REQUEST",
                            "snapshot.nodeProperties[*] arrays must match snapshot.nodes length",
                        );
                    }

                    // Try longs first.
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

                    let pv: std::sync::Arc<dyn NodePropertyValues> = if is_all_longs {
                        std::sync::Arc::new(DefaultLongNodePropertyValues::<VecLong>::from_collection(
                            VecLong::from(all_longs),
                            node_count,
                        ))
                    } else {
                        std::sync::Arc::new(
                            DefaultDoubleNodePropertyValues::<VecDouble>::from_collection(
                                VecDouble::from(all_doubles),
                                node_count,
                            ),
                        )
                    };

                    let labels = std::collections::HashSet::from([NodeLabel::all_nodes()]);
                    if let Err(e) = store.add_node_property(labels, key.to_string(), pv) {
                        return err(op, "ERROR", &format!("Failed to add node property '{key}': {e}"));
                    }
                }
            }

            let node_count = GraphStore::node_count(&store) as u64;
            let relationship_count_actual = GraphStore::relationship_count(&store) as u64;

            let catalog = TSJSON_CATALOG_SERVICE.clone().graph_catalog(&user, &db);
            catalog.set(graph_name, std::sync::Arc::new(store));

            ok(
                op,
                serde_json::json!({
                    "graphName": graph_name,
                    "nodeCount": node_count,
                    "relationshipCount": relationship_count_actual,
                }),
            )
        }
        _ => err(op, "UNSUPPORTED_OP", "Unsupported graph_store operation."),
    }
}

fn handle_form_eval(request: &serde_json::Value) -> serde_json::Value {
    let op = request.get("op").and_then(|v| v.as_str()).unwrap_or("");

    match op {
        "evaluate" => {
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
            let user = TsjsonUser::new(username.clone(), is_admin);

            let database_id = request
                .get("databaseId")
                .and_then(|v| v.as_str())
                .unwrap_or("db");
            let db = DatabaseId::new(database_id);

            let Some(graph_name) = request.get("graphName").and_then(|v| v.as_str()) else {
                return err(op, "INVALID_REQUEST", "Missing required field: graphName");
            };

            // Extract operator pipeline: program.morph.patterns
            let patterns = request
                .get("program")
                .and_then(|v| v.get("morph"))
                .and_then(|v| v.get("patterns"))
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            if patterns.is_empty() {
                return err(
                    op,
                    "INVALID_REQUEST",
                    "program.morph.patterns must be a non-empty string array",
                );
            }

            fn parse_string_vec(value: Option<&serde_json::Value>) -> Vec<String> {
                value
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|x| x.as_str().map(|s| s.to_string()))
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default()
            }

            fn parse_string_map(value: Option<&serde_json::Value>) -> std::collections::HashMap<String, String> {
                let mut out = std::collections::HashMap::new();
                let Some(obj) = value.and_then(|v| v.as_object()) else {
                    return out;
                };
                for (k, v) in obj.iter() {
                    if let Some(s) = v.as_str() {
                        out.insert(k.clone(), s.to_string());
                    }
                }
                out
            }

            // Artifacts: accept an object map; otherwise default empty.
            let mut artifacts = crate::projection::eval::form::FormArtifacts::new();
            if let Some(obj) = request.get("artifacts").and_then(|v| v.as_object()) {
                for (k, v) in obj.iter() {
                    artifacts.insert(k.to_string(), v.clone());
                }
            }

            // User context (for auditing); defaults are safe.
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

            // Shape + Context are optional today, but we thread them through so
            // operators can witness them (e.g. Shine/Reflection stages).
            let shape_json = request.get("program").and_then(|v| v.get("shape"));
            let shape = FormShapeMeta::new(
                parse_string_vec(shape_json.and_then(|v| v.get("required_fields"))),
                parse_string_vec(shape_json.and_then(|v| v.get("optional_fields"))),
                parse_string_map(shape_json.and_then(|v| v.get("type_constraints"))),
                parse_string_map(shape_json.and_then(|v| v.get("validation_rules"))),
            );

            let context_json = request.get("program").and_then(|v| v.get("context"));
            let context = FormContext::new(
                parse_string_vec(context_json.and_then(|v| v.get("dependencies"))),
                parse_string_vec(context_json.and_then(|v| v.get("execution_order"))),
                context_json
                    .and_then(|v| v.get("runtime_strategy"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("default")
                    .to_string(),
                parse_string_vec(context_json.and_then(|v| v.get("conditions"))),
            );

            let program = FormShape::new(shape, context, FormMorph::new(patterns.clone()));

            let mut form_request = FormRequest::new(graph_name.to_string(), program);
            form_request.artifacts = artifacts;
            form_request.output_graph_name = request
                .get("outputGraphName")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            // Use the same per-user/per-db catalog service as other TSJSON facades.
            let catalog: std::sync::Arc<dyn GraphCatalog> =
                TSJSON_CATALOG_SERVICE.clone().graph_catalog(&user, &db);
            let mut ctx = ExecutionContext::new(username);
            ctx.set_admin(is_admin);
            ctx.set_catalog(catalog.clone());

            let mut processor = FormProcessor::new(ctx);
            match processor.evaluate(&form_request) {
                Ok(result) => {
                    // If an output graph name is provided, persist the ResultStore into the shared catalog
                    // so it becomes addressable by subsequent GraphStore/GraphCatalog calls.
                    let mut persisted_output = false;
                    if let Some(output_name) = form_request.output_graph_name.as_deref() {
                        catalog.set(output_name, result.graph.clone());
                        persisted_output = true;
                    }

                    let node_count = GraphStore::node_count(result.graph.as_ref()) as u64;
                    let relationship_count = GraphStore::relationship_count(result.graph.as_ref()) as u64;

                    ok(
                        op,
                        serde_json::json!({
                            "graphName": graph_name,
                            "outputGraphName": form_request.output_graph_name,
                            "persistedOutputGraph": persisted_output,
                            "operator": result.operator,
                            "execution_time_ms": result.execution_time.as_millis(),
                            "nodeCount": node_count,
                            "relationshipCount": relationship_count,
                            "proof": result.proof,
                        }),
                    )
                }
                Err(e) => err(op, "FORM_EVAL_ERROR", &e.to_string()),
            }
        }
        _ => err(op, "UNSUPPORTED_OP", "Unsupported form_eval operation."),
    }
}

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
    use super::pathfinding_dispatch;
    let catalog = TSJSON_CATALOG_SERVICE.clone().graph_catalog(&user, &db);

    match op {
        // ============================================================================
        // Unified Pathfinding Algorithms (mode parameter)
        // ============================================================================
        "bfs" => pathfinding_dispatch::handle_bfs(request, catalog),
        "dfs" => pathfinding_dispatch::handle_dfs(request, catalog),
        "dijkstra" => pathfinding_dispatch::handle_dijkstra(request, catalog),
        "bellman_ford" => pathfinding_dispatch::handle_bellman_ford(request, catalog),
        "astar" => pathfinding_dispatch::handle_astar(request, catalog),
        "delta_stepping" => pathfinding_dispatch::handle_delta_stepping(request, catalog),
        "yens" => pathfinding_dispatch::handle_yens(request, catalog),
        "all_shortest_paths" => pathfinding_dispatch::handle_all_shortest_paths(request, catalog),
        "spanning_tree" => pathfinding_dispatch::handle_spanning_tree(request, catalog),
        "topological_sort" => pathfinding_dispatch::handle_topological_sort(request, catalog),
        "random_walk" => pathfinding_dispatch::handle_random_walk(request, catalog),

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
            "graph_store_catalog" => handle_graph_store_catalog(&request),
            "graph_store" => handle_graph_store(&request),
            "form_eval" => handle_form_eval(&request),
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
        assert!(
            !entries.iter().any(|e| e.get("name").and_then(|v| v.as_str()) == Some("graph_drop_1"))
        );
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

    #[test]
    fn invoke_form_eval_pass_through_round_trip() {
        let config = RandomGraphConfig {
            graph_name: "graph_form_eval".to_string(),
            database_name: "db1".to_string(),
            ..Default::default()
        };
        let mut rng = StdRng::seed_from_u64(2);
        let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
        let catalog = test_catalog("alice", true, "db1");
        catalog.set("graph_form_eval", Arc::new(store));

        let request = serde_json::json!({
            "facade": "form_eval",
            "op": "evaluate",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "graph_form_eval",
            "program": { "morph": { "patterns": ["passThrough"] } },
            "artifacts": {}
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(response.get("op").and_then(|v| v.as_str()), Some("evaluate"));

        let data = response.get("data").unwrap();
        assert_eq!(data.get("graphName").and_then(|v| v.as_str()), Some("graph_form_eval"));

        // passThrough operator proof marker
        let proof_kind = data
            .get("proof")
            .and_then(|v| v.get("final"))
            .and_then(|v| v.get("proof"))
            .and_then(|v| v.get("kind"))
            .and_then(|v| v.as_str());
        assert_eq!(proof_kind, Some("passThrough"));

        let _ = GraphCatalog::drop(catalog.as_ref(), &["graph_form_eval"], false);
    }

    #[test]
    fn invoke_form_eval_persists_output_graph_into_catalog_when_named() {
        let config = RandomGraphConfig {
            graph_name: "graph_form_eval_in".to_string(),
            database_name: "db1".to_string(),
            ..Default::default()
        };
        let mut rng = StdRng::seed_from_u64(22);
        let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
        let catalog = test_catalog("alice", true, "db1");
        catalog.set("graph_form_eval_in", Arc::new(store));

        let request = serde_json::json!({
            "facade": "form_eval",
            "op": "evaluate",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "graph_form_eval_in",
            "outputGraphName": "graph_form_eval_out",
            "program": { "morph": { "patterns": ["passThrough"] } },
            "artifacts": {}
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        let data = response.get("data").unwrap();
        assert_eq!(
            data.get("outputGraphName").and_then(|v| v.as_str()),
            Some("graph_form_eval_out")
        );
        assert_eq!(
            data.get("persistedOutputGraph").and_then(|v| v.as_bool()),
            Some(true)
        );

        // Verify it shows up in catalog listing.
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
        assert!(
            entries.iter().any(|e| e.get("name").and_then(|v| v.as_str()) == Some("graph_form_eval_out"))
        );

        let _ = GraphCatalog::drop(catalog.as_ref(), &["graph_form_eval_in", "graph_form_eval_out"], false);
    }

    #[test]
    fn invoke_form_eval_essence_shine_reflection_round_trip() {
        let config = RandomGraphConfig {
            graph_name: "graph_form_eval_shine".to_string(),
            database_name: "db1".to_string(),
            ..Default::default()
        };
        let mut rng = StdRng::seed_from_u64(3);
        let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
        let catalog = test_catalog("alice", true, "db1");
        catalog.set("graph_form_eval_shine", Arc::new(store));

        let request = serde_json::json!({
            "facade": "form_eval",
            "op": "evaluate",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "graph_form_eval_shine",
            "program": {
                "shape": {
                    "validation_rules": {
                        "moment": "shine",
                        "hegel": "Essence→Shine",
                        "yoga": "YS IV.3 nirmāṇa-cittāni asmitā-mātra"
                    }
                },
                "morph": { "patterns": ["essence", "shine", "reflection"] }
            },
            "artifacts": {}
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();

        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(response.get("op").and_then(|v| v.as_str()), Some("evaluate"));

        let data = response.get("data").unwrap();

        let essence_kind = data
            .get("proof")
            .and_then(|v| v.get("steps"))
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("proof"))
            .and_then(|v| v.get("kind"))
            .and_then(|v| v.as_str());
        assert_eq!(essence_kind, Some("essence"));

        let shine_kind = data
            .get("proof")
            .and_then(|v| v.get("steps"))
            .and_then(|v| v.get(1))
            .and_then(|v| v.get("proof"))
            .and_then(|v| v.get("kind"))
            .and_then(|v| v.as_str());
        assert_eq!(shine_kind, Some("shine"));

        let shine_presupposes = data
            .get("proof")
            .and_then(|v| v.get("steps"))
            .and_then(|v| v.get(1))
            .and_then(|v| v.get("proof"))
            .and_then(|v| v.get("presupposes"))
            .and_then(|v| v.as_str());
        assert_eq!(shine_presupposes, Some("essence"));

        let reflection_kind = data
            .get("proof")
            .and_then(|v| v.get("final"))
            .and_then(|v| v.get("proof"))
            .and_then(|v| v.get("kind"))
            .and_then(|v| v.as_str());
        assert_eq!(reflection_kind, Some("reflection"));

        let reflection_presupposes = data
            .get("proof")
            .and_then(|v| v.get("final"))
            .and_then(|v| v.get("proof"))
            .and_then(|v| v.get("presupposes"))
            .and_then(|v| v.as_str());
        assert_eq!(reflection_presupposes, Some("shine"));

        let _ = GraphCatalog::drop(catalog.as_ref(), &["graph_form_eval_shine"], false);
    }

    #[test]
    fn invoke_graph_store_put_round_trip() {
        let request = serde_json::json!({
            "facade": "graph_store",
            "op": "put",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "graph_stash_put",
            "snapshot": {
                "nodes": [0, 1, 2],
                "relationships": [
                    { "type": "KNOWS", "source": 0, "target": 1 },
                    { "type": "KNOWS", "source": 1, "target": 2 }
                ]
            }
        });

        let response_json = invoke(request.to_string());
        let response: serde_json::Value = serde_json::from_str(&response_json).unwrap();
        assert_eq!(response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(response.get("op").and_then(|v| v.as_str()), Some("put"));

        let data = response.get("data").unwrap();
        assert_eq!(data.get("graphName").and_then(|v| v.as_str()), Some("graph_stash_put"));
        assert_eq!(data.get("nodeCount").and_then(|v| v.as_u64()), Some(3));
        assert_eq!(data.get("relationshipCount").and_then(|v| v.as_u64()), Some(2));

        // Verify it shows up in catalog listing.
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
        assert!(entries.iter().any(|e| e.get("name").and_then(|v| v.as_str()) == Some("graph_stash_put")));

        let catalog = test_catalog("alice", true, "db1");
        let _ = GraphCatalog::drop(catalog.as_ref(), &["graph_stash_put"], false);
    }

    #[test]
    fn invoke_accepts_application_form_kind_marker() {
        // This test demonstrates the "wire" ApplicationForm marker:
        // - Client ENC: includes kind="ApplicationForm"
        // - Kernel DEC: ignores the marker for routing, but still executes correctly
        // - Kernel ENC: returns standard TS-JSON envelope
        let put_request = serde_json::json!({
            "kind": "ApplicationForm",
            "facade": "graph_store",
            "op": "put",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1",
            "graphName": "graph_kind_marker",
            "snapshot": {
                "nodes": [0, 1],
                "relationships": [
                    { "type": "KNOWS", "source": 0, "target": 1 }
                ]
            }
        });

        let put_json = invoke(put_request.to_string());
        let put_response: serde_json::Value = serde_json::from_str(&put_json).unwrap();
        assert_eq!(put_response.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(put_response.get("op").and_then(|v| v.as_str()), Some("put"));

        let list_request = serde_json::json!({
            "kind": "ApplicationForm",
            "facade": "graph_store_catalog",
            "op": "list_graphs",
            "user": { "username": "alice", "isAdmin": true },
            "databaseId": "db1"
        });

        let list_json = invoke(list_request.to_string());
        let list_response: serde_json::Value = serde_json::from_str(&list_json).unwrap();
        assert_eq!(list_response.get("ok").and_then(|v| v.as_bool()), Some(true));

        let entries = list_response
            .get("data")
            .and_then(|v| v.get("entries"))
            .and_then(|v| v.as_array())
            .unwrap();
        assert!(entries
            .iter()
            .any(|e| e.get("name").and_then(|v| v.as_str()) == Some("graph_kind_marker")));

        let catalog = test_catalog("alice", true, "db1");
        let _ = GraphCatalog::drop(catalog.as_ref(), &["graph_kind_marker"], false);
    }
}
