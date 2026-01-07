use crate::applications::graph_store_catalog::configs::MutateLabelConfig;
use crate::applications::graph_store_catalog::facade::{
    GraphCatalogApplications, GraphGenerationConfig, NativeProjectionConfig, SamplingConfig,
};
use crate::core::User;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::DatabaseId;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};
use std::sync::Arc;

fn op(request: &Value) -> &str {
    request.get("op").and_then(|v| v.as_str()).unwrap_or("")
}

fn require_str<'a>(op: &str, request: &'a Value, key: &str) -> Result<&'a str, Value> {
    match request.get(key).and_then(|v| v.as_str()).map(str::trim) {
        Some(v) if !v.is_empty() => Ok(v),
        _ => Err(err(
            op,
            "INVALID_REQUEST",
            format!("{key} missing or empty"),
        )),
    }
}

fn get_bool(request: &Value, key: &str, default: bool) -> bool {
    request
        .get(key)
        .and_then(|v| v.as_bool())
        .unwrap_or(default)
}

fn get_string_vec(request: &Value, key: &str) -> Vec<String> {
    request
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_str().map(String::from))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn require_string_vec(op: &str, request: &Value, key: &str) -> Result<Vec<String>, Value> {
    match request.get(key).and_then(|v| v.as_array()) {
        Some(arr) => {
            let values = arr
                .iter()
                .filter_map(|x| x.as_str().map(str::trim))
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect::<Vec<_>>();
            if values.is_empty() {
                Err(err(
                    op,
                    "INVALID_REQUEST",
                    format!("{key} must be a non-empty string array"),
                ))
            } else {
                Ok(values)
            }
        }
        None => Err(err(
            op,
            "INVALID_REQUEST",
            format!("{key} missing or not an array"),
        )),
    }
}

fn get_json_object_or_empty(op: &str, request: &Value, key: &str) -> Result<Value, Value> {
    match request.get(key) {
        None => Ok(json!({})),
        Some(v) if v.is_object() => Ok(v.clone()),
        Some(_) => Err(err(
            op,
            "INVALID_REQUEST",
            format!("{key} must be an object"),
        )),
    }
}

fn parse_required<T: DeserializeOwned>(op: &str, request: &Value, key: &str) -> Result<T, Value> {
    match request.get(key) {
        Some(v) => serde_json::from_value(v.clone())
            .map_err(|e| err(op, "INVALID_CONFIG", format!("{key} invalid: {e}"))),
        None => Err(err(op, "INVALID_REQUEST", format!("{key} missing"))),
    }
}

fn require_u64(op: &str, request: &Value, key: &str) -> Result<u64, Value> {
    match request.get(key).and_then(|v| v.as_u64()) {
        Some(v) => Ok(v),
        None => Err(err(
            op,
            "INVALID_REQUEST",
            format!("{key} missing or not a u64"),
        )),
    }
}

fn ok(op: &str, data: Value) -> Value {
    json!({ "ok": true, "op": op, "data": data })
}

fn err(op: &str, code: &str, message: impl Into<String>) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message.into() } })
}

pub fn handle_graph_store_catalog_dispatch(
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
    _catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let operation = op(request);
    if operation.is_empty() {
        return err(operation, "INVALID_REQUEST", "op missing or empty");
    }

    match operation {
        "graphExists" => handle_graph_exists(operation, request, catalog_apps, user, db),
        "listGraphs" => handle_list_graphs(operation, request, catalog_apps, user, db),
        "dropGraph" => handle_drop_graph(operation, request, catalog_apps, user, db),
        "dropGraphs" => handle_drop_graphs(operation, request, catalog_apps, user, db),
        "dropNodeProperties" => {
            handle_drop_node_properties(operation, request, catalog_apps, user, db)
        }
        "dropRelationships" => {
            handle_drop_relationships(operation, request, catalog_apps, user, db)
        }
        "dropGraphProperty" => {
            handle_drop_graph_property(operation, request, catalog_apps, user, db)
        }
        "streamGraphProperty" => {
            handle_stream_graph_property(operation, request, catalog_apps, user, db)
        }
        "graphMemoryUsage" => handle_graph_memory_usage(operation, request, catalog_apps, user, db),
        "streamNodeProperties" => {
            handle_stream_node_properties(operation, request, catalog_apps, user, db)
        }
        "streamRelationshipProperties" => {
            handle_stream_relationship_properties(operation, request, catalog_apps, user, db)
        }
        "streamRelationships" => {
            handle_stream_relationships(operation, request, catalog_apps, user, db)
        }
        "generateGraph" => handle_generate_graph(operation, request, catalog_apps, user, db),
        "generateGraphStats" => {
            handle_generate_graph_stats(operation, request, catalog_apps, user, db)
        }
        "exportToCsvEstimate" => {
            handle_export_to_csv_estimate(operation, request, catalog_apps, user, db)
        }
        "subGraphProject" => handle_sub_graph_project(operation, request, catalog_apps, user, db),
        "sampleRandomWalkWithRestarts" => {
            handle_sample_random_walk_with_restarts(operation, request, catalog_apps, user, db)
        }
        "sampleCommonNeighbourAwareRandomWalk" => handle_sample_common_neighbour_aware_random_walk(
            operation,
            request,
            catalog_apps,
            user,
            db,
        ),
        "estimateCommonNeighbourAwareRandomWalk" => {
            handle_estimate_common_neighbour_aware_random_walk(
                operation,
                request,
                catalog_apps,
                user,
                db,
            )
        }
        "estimateNativeProject" => {
            handle_estimate_native_project(operation, request, catalog_apps, user, db)
        }
        "cypherProject" => handle_cypher_project(operation, request, catalog_apps, user, db),
        "estimateCypherProject" => {
            handle_estimate_cypher_project(operation, request, catalog_apps, user, db)
        }
        "mutateLabel" => handle_mutate_label(operation, request, catalog_apps, user, db),
        "sampleGraph" => handle_sample_graph(operation, request, catalog_apps, user, db),
        _ => err(
            operation,
            "UNSUPPORTED_OP",
            "Unsupported graph_store_catalog operation.",
        ),
    }
}

fn handle_graph_exists(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let exists = catalog_apps.graph_exists(user, db, graph_name);
    ok(op, json!({ "graphName": graph_name, "exists": exists }))
}

fn handle_list_graphs(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = request.get("graphName").and_then(|v| v.as_str());
    let include_degree_distribution = get_bool(request, "includeDegreeDistribution", false);
    let entries = catalog_apps.list_graphs(user, db, graph_name, include_degree_distribution);
    let json_entries = entries
        .into_iter()
        .map(|e| {
            if include_degree_distribution {
                json!({
                    "name": e.graph_name(),
                    "nodeCount": e.node_count(),
                    "relationshipCount": e.relationship_count(),
                    "degreeDistribution": e.degree_distribution().cloned()
                })
            } else {
                json!({
                    "name": e.graph_name(),
                    "nodeCount": e.node_count(),
                    "relationshipCount": e.relationship_count()
                })
            }
        })
        .collect::<Vec<_>>();
    ok(op, json!({ "entries": json_entries }))
}

fn handle_drop_graph(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let fail_if_missing = get_bool(request, "failIfMissing", false);
    match catalog_apps.drop_graph(user, db, graph_name, fail_if_missing) {
        Ok(e) => ok(
            op,
            json!({
                "dropped": [ { "name": e.graph_name(), "nodeCount": e.node_count(), "relationshipCount": e.relationship_count() } ]
            }),
        ),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_drop_graphs(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let names = match require_string_vec(op, request, "graphNames") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let fail_if_missing = get_bool(request, "failIfMissing", false);
    match catalog_apps.drop_graphs(user, db, &names, fail_if_missing) {
        Ok(v) => ok(
            op,
            json!({
                "dropped": v
                    .iter()
                    .map(|e| json!({ "name": e.graph_name(), "nodeCount": e.node_count(), "relationshipCount": e.relationship_count() }))
                    .collect::<Vec<_>>()
            }),
        ),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_drop_node_properties(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let node_properties = match require_string_vec(op, request, "nodeProperties") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let fail_if_missing = get_bool(request, "failIfMissing", false);
    match catalog_apps.drop_node_properties(user, db, graph_name, &node_properties, fail_if_missing)
    {
        Ok(count) => ok(op, json!({ "propertiesRemoved": count })),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_drop_relationships(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let relationship_type = match require_str(op, request, "relationshipType") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match catalog_apps.drop_relationships(user, db, graph_name, relationship_type) {
        Ok(result) => ok(
            op,
            json!({ "relationshipsRemoved": result.deleted_relationship_count().unwrap_or(0) }),
        ),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_drop_graph_property(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let graph_property = match require_str(op, request, "graphProperty") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let fail_if_missing = get_bool(request, "failIfMissing", false);
    match catalog_apps.drop_graph_property(user, db, graph_name, graph_property, fail_if_missing) {
        Ok(count) => ok(op, json!({ "propertiesRemoved": count })),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_stream_graph_property(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let graph_property = match require_str(op, request, "graphProperty") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match catalog_apps.stream_graph_property(user, db, graph_name, graph_property) {
        Ok(results) => ok(op, json!({ "results": results })),
        Err(msg) => err(op, "STREAM_FAILED", msg),
    }
}

fn handle_graph_memory_usage(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let mu = catalog_apps.graph_memory_usage(user, db, graph_name);
    ok(
        op,
        json!({
            "graphName": mu.graph_name,
            "memoryUsage": mu.memory_usage,
            "sizeInBytes": mu.size_in_bytes,
            "detailSizeInBytes": mu.detail_size_in_bytes,
            "nodeCount": mu.node_count,
            "relationshipCount": mu.relationship_count
        }),
    )
}

fn handle_stream_node_properties(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let node_properties = get_string_vec(request, "nodeProperties");
    let node_labels = get_string_vec(request, "nodeLabels");
    let list_node_labels = get_bool(request, "listNodeLabels", false);
    match catalog_apps.stream_node_properties(
        user,
        db,
        graph_name,
        &node_properties,
        &node_labels,
        list_node_labels,
    ) {
        Ok(results) => ok(op, json!({ "results": results })),
        Err(msg) => err(op, "STREAM_FAILED", msg),
    }
}

fn handle_stream_relationship_properties(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let relationship_properties = get_string_vec(request, "relationshipProperties");
    let relationship_types = get_string_vec(request, "relationshipTypes");
    match catalog_apps.stream_relationship_properties(
        user,
        db,
        graph_name,
        &relationship_properties,
        &relationship_types,
    ) {
        Ok(results) => ok(op, json!({ "results": results })),
        Err(msg) => err(op, "STREAM_FAILED", msg),
    }
}

fn handle_stream_relationships(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let relationship_types = get_string_vec(request, "relationshipTypes");
    match catalog_apps.stream_relationships(user, db, graph_name, &relationship_types) {
        Ok(results) => ok(op, json!({ "results": results })),
        Err(msg) => err(op, "STREAM_FAILED", msg),
    }
}

fn handle_generate_graph(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let generation_config: GraphGenerationConfig =
        match parse_required(op, request, "generationConfig") {
            Ok(v) => v,
            Err(e) => return e,
        };
    match catalog_apps.generate_graph(user, db, &generation_config) {
        Ok(result) => ok(
            op,
            json!({
                "graphName": result.graph_name(),
                "nodesGenerated": result.nodes_generated(),
                "relationshipsGenerated": result.relationships_generated(),
                "generationTimeMs": result.generation_time_ms()
            }),
        ),
        Err(msg) => err(op, "GENERATION_FAILED", msg),
    }
}

fn handle_generate_graph_stats(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let node_count = match require_u64(op, request, "nodeCount") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let average_degree = match require_u64(op, request, "averageDegree") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let configuration = match get_json_object_or_empty(op, request, "configuration") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match catalog_apps.generate_graph_stats(
        user,
        db,
        graph_name,
        node_count,
        average_degree,
        &configuration,
    ) {
        Ok(stats) => ok(op, json!(stats)),
        Err(msg) => err(op, "GENERATION_FAILED", msg),
    }
}

fn handle_export_to_csv_estimate(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match catalog_apps.export_to_csv_estimate(user, db, graph_name) {
        Ok(result) => ok(op, json!(result)),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_sub_graph_project(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let origin_graph_name = match require_str(op, request, "originGraphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let node_filter = match require_str(op, request, "nodeFilter") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let relationship_filter = match require_str(op, request, "relationshipFilter") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let configuration = match get_json_object_or_empty(op, request, "configuration") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match catalog_apps.sub_graph_project(
        user,
        db,
        graph_name,
        origin_graph_name,
        node_filter,
        relationship_filter,
        &configuration,
    ) {
        Ok(result) => ok(op, json!(result)),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_sample_random_walk_with_restarts(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let origin_graph_name = match require_str(op, request, "originGraphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let configuration = match get_json_object_or_empty(op, request, "configuration") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match catalog_apps.sample_random_walk_with_restarts(
        user,
        db,
        graph_name,
        origin_graph_name,
        &configuration,
    ) {
        Ok(result) => ok(op, json!(result)),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_sample_common_neighbour_aware_random_walk(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let origin_graph_name = match require_str(op, request, "originGraphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let configuration = match get_json_object_or_empty(op, request, "configuration") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match catalog_apps.sample_common_neighbour_aware_random_walk(
        user,
        db,
        graph_name,
        origin_graph_name,
        &configuration,
    ) {
        Ok(result) => ok(op, json!(result)),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_estimate_common_neighbour_aware_random_walk(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let configuration = match get_json_object_or_empty(op, request, "configuration") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match catalog_apps.estimate_common_neighbour_aware_random_walk(
        user,
        db,
        graph_name,
        &configuration,
    ) {
        Ok(result) => ok(op, json!(result)),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_estimate_native_project(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let projection_config: NativeProjectionConfig =
        match parse_required(op, request, "projectionConfig") {
            Ok(cfg) => cfg,
            Err(e) => return e,
        };

    match catalog_apps.estimate_project_native(user, db, &projection_config) {
        Ok(result) => ok(op, json!(result)),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_cypher_project(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let node_query = match require_str(op, request, "nodeQuery") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let relationship_query = match require_str(op, request, "relationshipQuery") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let configuration = match get_json_object_or_empty(op, request, "configuration") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match catalog_apps.project_cypher(
        user,
        db,
        graph_name,
        node_query,
        relationship_query,
        &configuration,
    ) {
        Ok(result) => ok(op, json!(result)),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_estimate_cypher_project(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let node_query = match require_str(op, request, "nodeQuery") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let relationship_query = match require_str(op, request, "relationshipQuery") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let configuration = match get_json_object_or_empty(op, request, "configuration") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match catalog_apps.estimate_project_cypher(
        user,
        db,
        node_query,
        relationship_query,
        &configuration,
    ) {
        Ok(result) => ok(op, json!(result)),
        Err(msg) => err(op, "ERROR", msg),
    }
}

fn handle_mutate_label(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let node_label = match require_str(op, request, "nodeLabel") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let cfg_json = match request.get("mutateLabelConfig") {
        Some(v) if v.is_object() => v.clone(),
        Some(_) => return err(op, "INVALID_REQUEST", "mutateLabelConfig must be an object"),
        None => return err(op, "INVALID_REQUEST", "mutateLabelConfig missing"),
    };
    match MutateLabelConfig::from_json(&cfg_json) {
        Ok(config) => match catalog_apps.mutate_label(user, db, graph_name, node_label, &config) {
            Ok(result) => ok(op, json!(result)),
            Err(msg) => err(op, "ERROR", msg),
        },
        Err(msg) => err(op, "INVALID_CONFIG", msg),
    }
}

fn handle_sample_graph(
    op: &str,
    request: &Value,
    catalog_apps: &dyn GraphCatalogApplications,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let sampling_config: SamplingConfig = match parse_required(op, request, "samplingConfig") {
        Ok(v) => v,
        Err(e) => return e,
    };
    match catalog_apps.sample_graph(user, db, graph_name, &sampling_config) {
        Ok(result) => ok(
            op,
            json!({
                "sampledGraphName": result.sampled_graph_name(),
                "originalNodes": result.original_nodes(),
                "sampledNodes": result.sampled_nodes(),
                "originalRelationships": result.original_relationships(),
                "sampledRelationships": result.sampled_relationships()
            }),
        ),
        Err(msg) => err(op, "SAMPLING_FAILED", msg),
    }
}
