use crate::applications::graph_store_catalog::facade::GraphCatalogApplications;
use crate::core::User;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::DatabaseId;
use serde_json::json;
use std::sync::Arc;

pub fn handle_graph_store_catalog_dispatch(
    request: &serde_json::Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
    _catalog: Arc<dyn GraphCatalog>,
) -> serde_json::Value {
    let op = request.get("op").and_then(|v| v.as_str()).unwrap_or("");

    match op {
        "graph_exists" => {
            let graph_name = request
                .get("graphName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let exists = catalog_apps.graph_exists(user, db, graph_name);
            json!({ "ok": true, "op": op, "data": { "graphName": graph_name, "exists": exists } })
        }
        "list_graphs" => {
            let graph_name = request.get("graphName").and_then(|v| v.as_str());
            let include_degree_distribution = request
                .get("includeDegreeDistribution")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            catalog_apps.list_graphs_json(user, db, graph_name, include_degree_distribution)
        }
        "drop_graph" => {
            let graph_name = request
                .get("graphName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let fail_if_missing = request
                .get("failIfMissing")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            match catalog_apps.drop_graph(user, db, graph_name, fail_if_missing) {
                Ok(e) => {
                    json!({ "ok": true, "op": op, "data": { "dropped": [ { "name": e.graph_name(), "nodeCount": e.node_count(), "relationshipCount": e.relationship_count() } ] } })
                }
                Err(msg) => {
                    json!({ "ok": false, "op": op, "error": { "code": "ERROR", "message": msg } })
                }
            }
        }
        "drop_graphs" => {
            let names = request
                .get("graphNames")
                .and_then(|v| v.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let fail_if_missing = request
                .get("failIfMissing")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            match catalog_apps.drop_graphs(user, db, &names, fail_if_missing) {
                Ok(v) => {
                    json!({ "ok": true, "op": op, "data": { "dropped": v.iter().map(|e| json!({ "name": e.graph_name(), "nodeCount": e.node_count(), "relationshipCount": e.relationship_count() })).collect::<Vec<_>>() } })
                }
                Err(msg) => {
                    json!({ "ok": false, "op": op, "error": { "code": "ERROR", "message": msg } })
                }
            }
        }
        "drop_node_properties" => {
            let graph_name = request
                .get("graphName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let node_properties = request
                .get("nodeProperties")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let fail_if_missing = request
                .get("failIfMissing")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            match catalog_apps.drop_node_properties(
                user,
                db,
                graph_name,
                &node_properties,
                fail_if_missing,
            ) {
                Ok(count) => {
                    json!({ "ok": true, "op": op, "data": { "propertiesRemoved": count } })
                }
                Err(msg) => {
                    json!({ "ok": false, "op": op, "error": { "code": "ERROR", "message": msg } })
                }
            }
        }
        "drop_relationships" => {
            let graph_name = request
                .get("graphName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let relationship_type = request
                .get("relationshipType")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            match catalog_apps.drop_relationships(user, db, graph_name, relationship_type) {
                Ok(result) => {
                    json!({ "ok": true, "op": op, "data": { "relationshipsRemoved": result.deleted_relationship_count().unwrap_or(0) } })
                }
                Err(msg) => {
                    json!({ "ok": false, "op": op, "error": { "code": "ERROR", "message": msg } })
                }
            }
        }
        "graph_memory_usage" => {
            let graph_name = request
                .get("graphName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let mu = catalog_apps.graph_memory_usage(user, db, graph_name);
            json!({
                "ok": true,
                "op": op,
                "data": {
                    "graphName": mu.graph_name,
                    "memoryUsage": mu.memory_usage,
                    "sizeInBytes": mu.size_in_bytes,
                    "detailSizeInBytes": mu.detail_size_in_bytes,
                    "nodeCount": mu.node_count,
                    "relationshipCount": mu.relationship_count,
                    // Backwards-compatible keys expected by TS-JSON tests
                    "nodes": mu.node_count,
                    "relationships": mu.relationship_count
                }
            })
        }
        "stream_node_properties" => {
            let graph_name = request
                .get("graphName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let node_properties = request
                .get("nodeProperties")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let node_labels = request
                .get("nodeLabels")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let list_node_labels = request
                .get("listNodeLabels")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);

            match catalog_apps.stream_node_properties(
                user,
                db,
                graph_name,
                &node_properties,
                &node_labels,
                list_node_labels,
            ) {
                Ok(results) => json!({ "ok": true, "op": op, "data": { "results": results } }),
                Err(msg) => {
                    json!({ "ok": false, "op": op, "error": { "code": "STREAM_FAILED", "message": msg } })
                }
            }
        }
        "stream_relationship_properties" => {
            let graph_name = request
                .get("graphName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let relationship_properties = request
                .get("relationshipProperties")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let relationship_types = request
                .get("relationshipTypes")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            match catalog_apps.stream_relationship_properties(
                user,
                db,
                graph_name,
                &relationship_properties,
                &relationship_types,
            ) {
                Ok(results) => json!({ "ok": true, "op": op, "data": { "results": results } }),
                Err(msg) => {
                    json!({ "ok": false, "op": op, "error": { "code": "STREAM_FAILED", "message": msg } })
                }
            }
        }
        "stream_relationships" => {
            let graph_name = request
                .get("graphName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let relationship_types = request
                .get("relationshipTypes")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            match catalog_apps.stream_relationships(user, db, graph_name, &relationship_types) {
                Ok(results) => json!({ "ok": true, "op": op, "data": { "results": results } }),
                Err(msg) => {
                    json!({ "ok": false, "op": op, "error": { "code": "STREAM_FAILED", "message": msg } })
                }
            }
        }
        "generate_graph" => {
            // For now, use a placeholder config - this would need proper config parsing
            let generation_config =
                crate::applications::graph_store_catalog::facade::GraphGenerationConfig;
            match catalog_apps.generate_graph(user, db, &generation_config) {
                Ok(result) => {
                    json!({ "ok": true, "op": op, "data": { "graphName": result.graph_name(), "nodesGenerated": result.nodes_generated(), "relationshipsGenerated": result.relationships_generated(), "generationTimeMs": result.generation_time_ms() } })
                }
                Err(msg) => {
                    json!({ "ok": false, "op": op, "error": { "code": "GENERATION_FAILED", "message": msg } })
                }
            }
        }
        "sample_graph" => {
            let graph_name = request
                .get("graphName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            // For now, use a placeholder config - this would need proper config parsing
            let sampling_config = crate::applications::graph_store_catalog::facade::SamplingConfig;
            match catalog_apps.sample_graph(user, db, graph_name, &sampling_config) {
                Ok(result) => {
                    json!({ "ok": true, "op": op, "data": { "sampledGraphName": result.sampled_graph_name(), "originalNodes": result.original_nodes(), "sampledNodes": result.sampled_nodes(), "originalRelationships": result.original_relationships(), "sampledRelationships": result.sampled_relationships() } })
                }
                Err(msg) => {
                    json!({ "ok": false, "op": op, "error": { "code": "SAMPLING_FAILED", "message": msg } })
                }
            }
        }
        _ => {
            json!({ "ok": false, "op": op, "error": { "code": "UNSUPPORTED_OP", "message": "Unsupported graph_store_catalog operation." } })
        }
    }
}
