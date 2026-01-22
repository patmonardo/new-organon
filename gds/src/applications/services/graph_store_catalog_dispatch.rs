use crate::applications::graph_store_catalog::applications::complex_applications::{
    EstimateCommonNeighbourAwareRandomWalkApplication, GenerateGraphApplication,
    GraphSamplingApplication, ListGraphApplication, SubGraphProjectApplication,
};
use crate::applications::graph_store_catalog::applications::project_applications::NativeProjectApplication;
use crate::applications::graph_store_catalog::applications::simple_applications::{
    DropGraphApplication, DropGraphPropertyApplication, DropNodePropertiesApplication,
    DropRelationshipsApplication, GraphMemoryUsageApplication, NodeFilterParser,
    NodeLabelMutatorApplication,
};
use crate::applications::graph_store_catalog::applications::stream_applications::{
    StreamGraphPropertiesApplication, StreamNodePropertiesApplication,
    StreamRelationshipPropertiesApplication, StreamRelationshipsApplication,
};
use crate::applications::graph_store_catalog::applications::write_applications::{
    WriteNodeLabelApplication, WriteNodePropertiesApplication,
    WriteRelationshipPropertiesApplication, WriteRelationshipsApplication,
};
use crate::applications::graph_store_catalog::configs::{
    GraphGenerationConfig, GraphGenerationRelationshipConfig, MutateLabelConfig,
    NativeProjectionConfig, SamplingConfig,
};
use crate::applications::graph_store_catalog::loaders::graph_store_loader::GraphStoreCatalogService;
use crate::applications::graph_store_catalog::services::progress_tracker_factory::{
    default_task_registry_factory, default_user_log_registry_factory,
};
use crate::applications::graph_store_catalog::services::GraphListingService;
use crate::applications::services::logging::Log;
use crate::applications::services::tsjson_support::TSJSON_CATALOG_SERVICE;
use crate::core::User;
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::{DatabaseId, DefaultGraphStore, GraphStore};
use serde_json::{json, Value};
use std::sync::Arc;

fn op(request: &Value) -> &str {
    request.get("op").and_then(|v| v.as_str()).unwrap_or("")
}

fn ok(op: &str, data: Value) -> Value {
    json!({ "ok": true, "op": op, "data": data })
}

fn err(op: &str, code: &str, message: impl Into<String>) -> Value {
    json!({ "ok": false, "op": op, "error": { "code": code, "message": message.into() } })
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

fn get_bool(request: &Value, key: &str, default: bool) -> bool {
    request
        .get(key)
        .and_then(|v| v.as_bool())
        .unwrap_or(default)
}

fn optional_string_vec(request: &Value, key: &str) -> Vec<String> {
    request
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::trim))
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub fn handle_graph_store_catalog(
    request: &Value,
    user: &dyn User,
    db: &DatabaseId,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    let operation = op(request);
    if operation.is_empty() {
        return err(operation, "INVALID_REQUEST", "op missing or empty");
    }

    match operation {
        "graphExists" => handle_graph_exists(operation, request, &catalog),
        "listGraphs" => handle_list_graphs(operation, request, user, db),
        "graphMemoryUsage" => handle_graph_memory_usage(operation, request, user, db),
        "dropGraph" => handle_drop_graph(operation, request, user, db),
        "dropGraphs" => handle_drop_graphs(operation, request, user, db),
        "dropNodeProperties" => handle_drop_node_properties(operation, request, user, db),
        "dropRelationships" => handle_drop_relationships(operation, request, user, db),
        "dropGraphProperty" => handle_drop_graph_property(operation, request, user, db),
        "mutateLabel" => handle_mutate_label(operation, request, user, db),
        "writeNodeProperties" => handle_write_node_properties(operation, request, user, db),
        "writeRelationshipProperties" => {
            handle_write_relationship_properties(operation, request, user, db)
        }
        "writeRelationships" => handle_write_relationships(operation, request, user, db),
        "writeNodeLabel" => handle_write_node_label(operation, request, user, db),
        "streamGraphProperty" => handle_stream_graph_property(operation, request, user, db),
        "streamNodeProperties" => handle_stream_node_properties(operation, request, user, db),
        "streamRelationshipProperties" => {
            handle_stream_relationship_properties(operation, request, user, db)
        }
        "streamRelationships" => handle_stream_relationships(operation, request, user, db),
        "generateGraph" => handle_generate_graph(operation, request, user, db),
        "sampleGraph" => handle_sample_graph(operation, request, user, db),
        "estimateCommonNeighbourAwareRandomWalk" => {
            handle_estimate_common_neighbour_aware_random_walk(operation, request, user, db)
        }
        "subGraphProject" => handle_sub_graph_project(operation, request, user, db),
        "estimateNativeProject" => handle_estimate_native_project(operation, request, user, db),
        "estimateCypherProject"
        | "cypherProject"
        | "exportToCsvEstimate"
        | "exportToCsv"
        | "exportToDatabase" => err(
            operation,
            "UNSUPPORTED_OP",
            "Operation is not implemented in the Rust TSJSON dispatcher yet.",
        ),
        _ => err(
            operation,
            "UNSUPPORTED_OP",
            "Unsupported graph_store_catalog operation.",
        ),
    }
}

fn resolve_graph_store(
    op: &str,
    user: &dyn User,
    db: &DatabaseId,
    graph_name: &str,
) -> Result<Arc<DefaultGraphStore>, Value> {
    TSJSON_CATALOG_SERVICE
        .get_graph_store(user, db, graph_name)
        .map_err(|e| err(op, "CATALOG_ERROR", e))
}

fn handle_graph_exists(op: &str, request: &Value, catalog: &Arc<dyn GraphCatalog>) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let exists = catalog.get(graph_name).is_some();
    ok(op, json!({ "graphName": graph_name, "exists": exists }))
}

fn handle_list_graphs(_op: &str, request: &Value, user: &dyn User, db: &DatabaseId) -> Value {
    let filter = request
        .get("graphName")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|v| !v.is_empty());
    let include_degree_distribution = get_bool(request, "includeDegreeDistribution", false);
    let listing_service = GraphListingService::new(TSJSON_CATALOG_SERVICE.clone());
    let app = ListGraphApplication::new(listing_service);
    app.compute(filter, include_degree_distribution, user, db)
}

fn handle_graph_memory_usage(op: &str, request: &Value, user: &dyn User, db: &DatabaseId) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let app = GraphMemoryUsageApplication::new(TSJSON_CATALOG_SERVICE.clone());
    let mu = app.compute(user, db, graph_name);
    ok(
        op,
        json!({
            "graphName": graph_name,
            "memoryUsage": mu.memory_usage,
            "sizeInBytes": mu.size_in_bytes,
            "nodeCount": mu.node_count,
            "relationshipCount": mu.relationship_count,
        }),
    )
}

fn handle_drop_graph(op: &str, request: &Value, user: &dyn User, db: &DatabaseId) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let fail_if_missing = get_bool(request, "failIfMissing", false);
    let app = DropGraphApplication::new(TSJSON_CATALOG_SERVICE.clone());
    match app.compute(&[graph_name.to_string()], fail_if_missing, db, user, None) {
        Ok(dropped) => ok(
            op,
            json!({
                "dropped": dropped
                    .into_iter()
                    .map(|d| json!({
                        "name": d.name,
                        "nodeCount": d.node_count,
                        "relationshipCount": d.relationship_count,
                    }))
                    .collect::<Vec<_>>(),
            }),
        ),
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_drop_graphs(op: &str, request: &Value, user: &dyn User, db: &DatabaseId) -> Value {
    let names = match require_string_vec(op, request, "graphNames") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let fail_if_missing = get_bool(request, "failIfMissing", false);
    let app = DropGraphApplication::new(TSJSON_CATALOG_SERVICE.clone());
    match app.compute(&names, fail_if_missing, db, user, None) {
        Ok(dropped) => ok(
            op,
            json!({
                "dropped": dropped
                    .into_iter()
                    .map(|d| json!({
                        "name": d.name,
                        "nodeCount": d.node_count,
                        "relationshipCount": d.relationship_count,
                    }))
                    .collect::<Vec<_>>(),
            }),
        ),
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_drop_node_properties(
    op: &str,
    request: &Value,
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

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    if fail_if_missing {
        for key in node_properties.iter() {
            if !store.has_node_property(key) {
                return err(op, "NOT_FOUND", format!("Unknown node property '{key}'"));
            }
        }
    }

    let app = DropNodePropertiesApplication::new(Log::default());
    match app.compute(
        &default_task_registry_factory(),
        &default_user_log_registry_factory(),
        &node_properties,
        store.as_ref(),
    ) {
        Ok((updated_store, removed)) => {
            TSJSON_CATALOG_SERVICE
                .graph_catalog(user, db)
                .set(graph_name, Arc::new(updated_store));
            ok(
                op,
                json!({
                    "graphName": graph_name,
                    "nodeProperties": node_properties,
                    "removed": removed,
                }),
            )
        }
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_drop_relationships(op: &str, request: &Value, user: &dyn User, db: &DatabaseId) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let relationship_type = match require_str(op, request, "relationshipType") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    if !store.has_relationship_type(&RelationshipType::of(relationship_type)) {
        return err(
            op,
            "NOT_FOUND",
            format!("Unknown relationship type '{relationship_type}'"),
        );
    }

    let app = DropRelationshipsApplication::new(Log::default());
    match app.compute(
        &default_task_registry_factory(),
        &default_user_log_registry_factory(),
        store.as_ref(),
        relationship_type,
    ) {
        Ok((updated_store, deletion_result)) => {
            TSJSON_CATALOG_SERVICE
                .graph_catalog(user, db)
                .set(graph_name, Arc::new(updated_store));

            let deleted_relationships = deletion_result.deleted_relationship_count().unwrap_or(0);
            let deleted_nodes = deletion_result.deleted_node_count().unwrap_or(0);

            ok(
                op,
                json!({
                    "graphName": graph_name,
                    "relationshipType": relationship_type,
                    "deletedRelationshipCount": deleted_relationships,
                    "deletedNodeCount": deleted_nodes,
                }),
            )
        }
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_drop_graph_property(
    op: &str,
    request: &Value,
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

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let app = DropGraphPropertyApplication::new();
    match app.compute(store.as_ref(), graph_property, fail_if_missing) {
        Ok((updated_store, removed)) => {
            TSJSON_CATALOG_SERVICE
                .graph_catalog(user, db)
                .set(graph_name, Arc::new(updated_store));

            ok(
                op,
                json!({
                    "graphName": graph_name,
                    "graphProperty": graph_property,
                    "removed": removed,
                }),
            )
        }
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_mutate_label(op: &str, request: &Value, user: &dyn User, db: &DatabaseId) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let node_label = match require_str(op, request, "nodeLabel") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let cfg_value = match request.get("mutateLabelConfig") {
        Some(v) => v,
        None => return err(op, "INVALID_REQUEST", "mutateLabelConfig missing"),
    };

    let config = match MutateLabelConfig::from_json(cfg_value) {
        Ok(c) => c,
        Err(e) => return err(op, "INVALID_REQUEST", e),
    };

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let expr = match NodeFilterParser::new().parse_and_validate(store.as_ref(), &config.node_filter)
    {
        Ok(e) => e,
        Err(e) => return err(op, "INVALID_REQUEST", e),
    };

    let app = NodeLabelMutatorApplication::new();
    let result = app.compute(store.as_ref(), graph_name, node_label, &config, &expr);

    match serde_json::to_value(result) {
        Ok(v) => ok(op, v),
        Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
    }
}

fn handle_write_node_properties(
    op: &str,
    request: &Value,
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
    let node_labels = optional_string_vec(request, "nodeLabels")
        .into_iter()
        .map(|s| NodeLabel::of(s.as_str()))
        .collect::<Vec<_>>();

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let app = WriteNodePropertiesApplication::new(Log::default());
    match app.compute(store.as_ref(), &node_properties, &node_labels) {
        Ok(result) => match serde_json::to_value(result) {
            Ok(v) => ok(op, v),
            Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
        },
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_write_relationship_properties(
    op: &str,
    request: &Value,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let relationship_properties = match require_string_vec(op, request, "relationshipProperties") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let app = WriteRelationshipPropertiesApplication::new(Log::default());
    match app.compute(store.as_ref(), &relationship_properties) {
        Ok(result) => match serde_json::to_value(result) {
            Ok(v) => ok(op, v),
            Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
        },
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_write_relationships(
    op: &str,
    request: &Value,
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

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let app = WriteRelationshipsApplication::new(Log::default());
    match app.compute(store.as_ref(), relationship_type) {
        Ok(result) => match serde_json::to_value(result) {
            Ok(v) => ok(op, v),
            Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
        },
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_write_node_label(op: &str, request: &Value, user: &dyn User, db: &DatabaseId) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let node_labels = match require_string_vec(op, request, "nodeLabels") {
        Ok(v) => v,
        Err(e) => return e,
    };

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let app = WriteNodeLabelApplication::new(Log::default());
    match app.compute(store.as_ref(), &node_labels) {
        Ok(result) => match serde_json::to_value(result) {
            Ok(v) => ok(op, v),
            Err(e) => err(op, "SERIALIZATION_ERROR", e.to_string()),
        },
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_stream_graph_property(
    op: &str,
    request: &Value,
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

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let app = StreamGraphPropertiesApplication;
    match app.compute(store.as_ref(), graph_property) {
        Ok(results) => ok(op, json!({ "graphName": graph_name, "results": results })),
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_stream_node_properties(
    op: &str,
    request: &Value,
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
    if node_properties.is_empty() {
        return err(op, "INVALID_REQUEST", "nodeProperties must not be empty");
    }
    let node_labels = optional_string_vec(request, "nodeLabels")
        .into_iter()
        .map(|s| NodeLabel::of(s.as_str()))
        .collect::<Vec<_>>();
    let list_node_labels = get_bool(request, "listNodeLabels", false);

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let app = StreamNodePropertiesApplication;
    match app.compute(
        store.as_ref(),
        &node_properties,
        &node_labels,
        list_node_labels,
    ) {
        Ok(results) => ok(op, json!({ "graphName": graph_name, "results": results })),
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_stream_relationship_properties(
    op: &str,
    request: &Value,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let relationship_properties = match require_string_vec(op, request, "relationshipProperties") {
        Ok(v) => v,
        Err(e) => return e,
    };
    if relationship_properties.is_empty() {
        return err(
            op,
            "INVALID_REQUEST",
            "relationshipProperties must not be empty",
        );
    }
    let relationship_types = optional_string_vec(request, "relationshipTypes")
        .into_iter()
        .map(|s| RelationshipType::of(s.as_str()))
        .collect::<Vec<_>>();

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let app = StreamRelationshipPropertiesApplication;
    match app.compute(
        store.as_ref(),
        &relationship_properties,
        &relationship_types,
    ) {
        Ok(results) => ok(op, json!({ "graphName": graph_name, "results": results })),
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_stream_relationships(
    op: &str,
    request: &Value,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let relationship_types = optional_string_vec(request, "relationshipTypes")
        .into_iter()
        .map(|s| RelationshipType::of(s.as_str()))
        .collect::<Vec<_>>();

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let app = StreamRelationshipsApplication;
    match app.compute(store.as_ref(), &relationship_types) {
        Ok(results) => ok(op, json!({ "graphName": graph_name, "results": results })),
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_generate_graph(op: &str, request: &Value, user: &dyn User, db: &DatabaseId) -> Value {
    let generation_cfg = match request.get("generationConfig") {
        Some(v) => v,
        None => return err(op, "INVALID_REQUEST", "generationConfig missing"),
    };

    let cfg = match parse_generation_config(generation_cfg) {
        Ok(c) => c,
        Err(e) => return err(op, "INVALID_REQUEST", e),
    };

    let app = GenerateGraphApplication::new(Log::default(), TSJSON_CATALOG_SERVICE.clone());
    match app.compute(user, db, &cfg) {
        Ok(res) => ok(
            op,
            json!({
                "graphName": res.graph_name,
                "nodesGenerated": res.nodes_generated,
                "relationshipsGenerated": res.relationships_generated,
                "generateMillis": res.generate_millis,
            }),
        ),
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn parse_generation_config(value: &Value) -> Result<GraphGenerationConfig, String> {
    let graph_name = value
        .get("graphName")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());
    let node_count = value.get("nodeCount").and_then(|v| v.as_u64());
    let node_labels = optional_string_vec(value, "nodeLabels");
    let relationships = value
        .get("relationships")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let relationship_type = item
                        .get("relationshipType")
                        .and_then(|v| v.as_str())
                        .map(str::trim)
                        .filter(|s| !s.is_empty())?;
                    let probability = item
                        .get("probability")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.1);
                    Some(GraphGenerationRelationshipConfig {
                        relationship_type: relationship_type.to_string(),
                        probability,
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let directed = value.get("directed").and_then(|v| v.as_bool());
    let inverse_indexed = value.get("inverseIndexed").and_then(|v| v.as_bool());
    let seed = value.get("seed").and_then(|v| v.as_u64());

    Ok(GraphGenerationConfig {
        graph_name,
        node_count,
        node_labels,
        relationships,
        directed,
        inverse_indexed,
        seed,
    })
}

fn handle_sample_graph(op: &str, request: &Value, user: &dyn User, db: &DatabaseId) -> Value {
    let origin_graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let sampling_cfg_value = match request.get("samplingConfig") {
        Some(v) => v,
        None => &serde_json::Value::Null,
    };

    let cfg = parse_sampling_config(sampling_cfg_value);

    let app = GraphSamplingApplication::new(Log::default(), TSJSON_CATALOG_SERVICE.clone());
    match app.compute(user, db, origin_graph_name, &cfg) {
        Ok(res) => ok(
            op,
            json!({
                "graphName": res.graph_name,
                "originNodeCount": res.origin_node_count,
                "sampledNodeCount": res.sampled_node_count,
                "originRelationshipCount": res.origin_relationship_count,
                "sampledRelationshipCount": res.sampled_relationship_count,
            }),
        ),
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn parse_sampling_config(value: &Value) -> SamplingConfig {
    SamplingConfig {
        sample_node_count: value
            .get("sampleNodeCount")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize),
        sample_ratio: value.get("sampleRatio").and_then(|v| v.as_f64()),
        sampled_graph_name: value
            .get("sampledGraphName")
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty()),
        seed: value.get("seed").and_then(|v| v.as_u64()),
    }
}

fn handle_estimate_common_neighbour_aware_random_walk(
    op: &str,
    request: &Value,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let graph_name = match require_str(op, request, "graphName") {
        Ok(v) => v,
        Err(e) => return e,
    };
    let config = request
        .get("configuration")
        .cloned()
        .unwrap_or_else(|| json!({}));

    let store = match resolve_graph_store(op, user, db, graph_name) {
        Ok(s) => s,
        Err(e) => return e,
    };

    let app = EstimateCommonNeighbourAwareRandomWalkApplication::new();
    let res = app.compute(store.as_ref(), &config);
    ok(op, json!({ "graphName": graph_name, "estimate": res }))
}

fn handle_sub_graph_project(op: &str, request: &Value, user: &dyn User, db: &DatabaseId) -> Value {
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

    let cfg_value = request
        .get("configuration")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let sample_node_count = cfg_value
        .get("sampleNodeCount")
        .and_then(|v| v.as_u64())
        .map(|v| v as usize);
    let sample_ratio = cfg_value.get("sampleRatio").and_then(|v| v.as_f64());
    let seed = cfg_value.get("seed").and_then(|v| v.as_u64());

    let app = SubGraphProjectApplication::new(Log::default(), TSJSON_CATALOG_SERVICE.clone());
    match app.compute(
        user,
        db,
        graph_name,
        origin_graph_name,
        node_filter,
        relationship_filter,
        sample_node_count,
        sample_ratio,
        seed,
    ) {
        Ok(res) => ok(op, json!(res)),
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn handle_estimate_native_project(
    op: &str,
    request: &Value,
    user: &dyn User,
    db: &DatabaseId,
) -> Value {
    let projection_cfg_value = match request.get("projectionConfig") {
        Some(v) => v,
        None => return err(op, "INVALID_REQUEST", "projectionConfig missing"),
    };

    let cfg = match parse_native_projection_config(projection_cfg_value) {
        Ok(c) => c,
        Err(e) => return err(op, "INVALID_REQUEST", e),
    };

    let app = NativeProjectApplication::default();
    match app.project(TSJSON_CATALOG_SERVICE.clone(), user, db, &cfg) {
        Ok(res) => ok(
            op,
            json!({
                "graphName": res.graph_name,
                "nodeCount": res.node_count,
                "relationshipCount": res.relationship_count,
                "projectMillis": res.project_millis,
            }),
        ),
        Err(e) => err(op, "CATALOG_ERROR", e),
    }
}

fn parse_native_projection_config(value: &Value) -> Result<NativeProjectionConfig, String> {
    let graph_name = value
        .get("graphName")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "projectionConfig.graphName is required".to_string())?
        .to_string();

    let source_graph_name = value
        .get("sourceGraphName")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let relationship_types = optional_string_vec(value, "relationshipTypes");
    let node_labels = optional_string_vec(value, "nodeLabels");
    let node_properties = optional_string_vec(value, "nodeProperties");
    let relationship_properties = optional_string_vec(value, "relationshipProperties");

    let relationship_property_selectors = value
        .get("relationshipPropertySelectors")
        .and_then(|v| v.as_object())
        .map(|m| {
            m.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect::<std::collections::HashMap<String, String>>()
        })
        .unwrap_or_default();

    let weight_property = value
        .get("weightProperty")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let fictitious_loading = value
        .get("fictitiousLoading")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    Ok(NativeProjectionConfig {
        graph_name,
        source_graph_name,
        relationship_types,
        node_labels,
        node_properties,
        relationship_properties,
        relationship_property_selectors,
        weight_property,
        fictitious_loading,
    })
}
