use once_cell::sync::Lazy;

use crate::config::GraphStoreConfig;
use crate::core::User;
use crate::types::catalog::{GraphCatalog, InMemoryGraphCatalog};
use crate::types::graph::id_map::SimpleIdMap;
use crate::types::graph::RelationshipTopology;
use crate::types::graph_store::{Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName, GraphStore};
use crate::projection::eval::form::{FormProcessor, FormRequest};
use crate::projection::eval::procedure::ExecutionContext;
use crate::projection::RelationshipType;
use crate::form::program::{Context as FormContext, Morph as FormMorph, Shape as FormShapeMeta, FormShape};
use crate::types::schema::GraphSchema;
use crate::applications::graph_store_catalog::facade::{ApplicationsFacade, GraphCatalogApplications, GraphStoreCatalogEntry};
use crate::applications::graph_store_catalog::results::GraphMemoryUsage as AppGraphMemoryUsage;

use std::collections::HashMap;

static TSJSON_CATALOG: Lazy<std::sync::Arc<InMemoryGraphCatalog>> =
    Lazy::new(|| std::sync::Arc::new(InMemoryGraphCatalog::new()));

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

/// GraphStoreCatalog facade backed by the TSJSON in-memory catalog.
///
/// This lets the transport layer route through the *facade contract* even while the
/// full GraphStoreCatalog application subsystem is still being implemented.
struct TsjsonCatalogBackedGraphCatalogApplications {
    catalog: std::sync::Arc<InMemoryGraphCatalog>,
}

impl TsjsonCatalogBackedGraphCatalogApplications {
    fn new(catalog: std::sync::Arc<InMemoryGraphCatalog>) -> Self {
        Self { catalog }
    }
}

impl GraphCatalogApplications for TsjsonCatalogBackedGraphCatalogApplications {
    fn list_graphs(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
    ) -> Vec<GraphStoreCatalogEntry> {
        self.catalog
            .list(None, false)
            .into_iter()
            .map(|e| GraphStoreCatalogEntry::new(e.name, e.node_count, e.relationship_count))
            .collect()
    }

    fn graph_memory_usage(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        graph_name: &str,
    ) -> AppGraphMemoryUsage {
        match self.catalog.size_of(graph_name) {
            Ok(mu) => AppGraphMemoryUsage::new(
                graph_name.to_string(),
                format!("{} bytes", mu.bytes),
                mu.bytes,
                HashMap::new(),
                mu.nodes,
                mu.relationships,
            ),
            Err(_) => AppGraphMemoryUsage::new(
                graph_name.to_string(),
                "0 bytes".to_string(),
                0,
                HashMap::new(),
                0,
                0,
            ),
        }
    }

    fn drop_graph(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        graph_name: &str,
        fail_if_missing: bool,
    ) -> Result<GraphStoreCatalogEntry, String> {
        let dropped =
            GraphCatalog::drop(self.catalog.as_ref(), &[graph_name], fail_if_missing)
                .map_err(|e| e.to_string())?;
        dropped
            .into_iter()
            .next()
            .map(|d| GraphStoreCatalogEntry::new(d.name, d.node_count, d.relationship_count))
            .ok_or_else(|| "Graph not found".to_string())
    }

    fn drop_graphs(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        graph_names: &[String],
        fail_if_missing: bool,
    ) -> Result<Vec<GraphStoreCatalogEntry>, String> {
        let refs: Vec<&str> = graph_names.iter().map(|s| s.as_str()).collect();
        let dropped =
            GraphCatalog::drop(self.catalog.as_ref(), &refs, fail_if_missing)
                .map_err(|e| e.to_string())?;
        Ok(dropped
            .into_iter()
            .map(|d| GraphStoreCatalogEntry::new(d.name, d.node_count, d.relationship_count))
            .collect())
    }

    fn drop_node_properties(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _node_properties: &[String],
        _fail_if_missing: bool,
    ) -> Result<u64, String> {
        Err("drop_node_properties not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn drop_relationships(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _relationship_type: &str,
    ) -> Result<crate::types::graph_store::DeletionResult, String> {
        Err("drop_relationships not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn stream_node_properties(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _node_properties: &[String],
    ) -> Result<Vec<crate::applications::graph_store_catalog::facade::NodePropertyResult>, String> {
        Err("stream_node_properties not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn stream_relationship_properties(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _relationship_properties: &[String],
    ) -> Result<Vec<crate::applications::graph_store_catalog::facade::RelationshipPropertyResult>, String> {
        Err("stream_relationship_properties not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn stream_relationships(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _relationship_types: &[String],
    ) -> Result<Vec<crate::applications::graph_store_catalog::facade::RelationshipResult>, String> {
        Err("stream_relationships not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn write_node_properties(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _node_properties: &[String],
    ) -> Result<crate::applications::graph_store_catalog::facade::WriteResult, String> {
        Err("write_node_properties not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn write_node_labels(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _node_labels: &[String],
    ) -> Result<crate::applications::graph_store_catalog::facade::WriteResult, String> {
        Err("write_node_labels not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn write_relationship_properties(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _relationship_properties: &[String],
    ) -> Result<crate::applications::graph_store_catalog::facade::WriteResult, String> {
        Err("write_relationship_properties not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn write_relationships(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _relationship_type: &str,
    ) -> Result<crate::applications::graph_store_catalog::facade::WriteResult, String> {
        Err("write_relationships not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn export_to_csv(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _export_path: &str,
    ) -> Result<crate::applications::graph_store_catalog::facade::ExportResult, String> {
        Err("export_to_csv not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn export_to_database(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _target_database: &str,
    ) -> Result<crate::applications::graph_store_catalog::facade::ExportResult, String> {
        Err("export_to_database not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn project_native(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _projection_config: &crate::applications::graph_store_catalog::facade::NativeProjectionConfig,
    ) -> Result<crate::applications::graph_store_catalog::facade::ProjectionResult, String> {
        Err("project_native not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn project_generic(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _projection_config: &crate::applications::graph_store_catalog::facade::GenericProjectionConfig,
    ) -> Result<crate::applications::graph_store_catalog::facade::ProjectionResult, String> {
        Err("project_generic not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn generate_graph(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _generation_config: &crate::applications::graph_store_catalog::facade::GraphGenerationConfig,
    ) -> Result<crate::applications::graph_store_catalog::facade::GenerationResult, String> {
        Err("generate_graph not implemented for TSJSON catalog-backed facade yet".to_string())
    }

    fn sample_graph(
        &self,
        _user: &dyn User,
        _database_id: &DatabaseId,
        _graph_name: &str,
        _sampling_config: &crate::applications::graph_store_catalog::facade::SamplingConfig,
    ) -> Result<crate::applications::graph_store_catalog::facade::SamplingResult, String> {
        Err("sample_graph not implemented for TSJSON catalog-backed facade yet".to_string())
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

    // Route through the facade contract (ApplicationsFacade), backed by the in-memory TSJSON catalog.
    // We construct this per-call to avoid requiring global Send/Sync bounds on the facade trait objects.
    let apps_facade = ApplicationsFacade::with_graph_store_catalog_applications(Box::new(
        TsjsonCatalogBackedGraphCatalogApplications::new(TSJSON_CATALOG.clone()),
    ));

    match op {
        // Mirrors logic/src/absolute/form/gds.application.ts
        "list_graphs" => {
            let entries = apps_facade
                .graph_store_catalog()
                .list_graphs(&user, &db)
                .into_iter()
                .map(|e| serde_json::json!({
                    "name": e.graph_name(),
                    "nodeCount": e.node_count(),
                    "relationshipCount": e.relationship_count(),
                }))
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
            let mut rels_by_type: HashMap<String, Vec<(i64, i64)>> = HashMap::new();
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
                        .push((source_mapped, target_mapped));
                }
            }

            let mut relationship_topologies = std::collections::HashMap::new();

            for (rel_type, edges) in rels_by_type.into_iter() {
                let mut adjacency: Vec<Vec<i64>> = vec![Vec::new(); original_node_ids.len()];
                for (s, t) in edges.into_iter() {
                    adjacency[s as usize].push(t);
                }

                let topology = RelationshipTopology::new(adjacency, None);
                relationship_topologies.insert(RelationshipType::of(&rel_type), topology);
            }

            let database_id = request
                .get("databaseId")
                .and_then(|v| v.as_str())
                .unwrap_or("db");

            let database_info = DatabaseInfo::new(
                DatabaseId::new(database_id),
                DatabaseLocation::remote("tsjson", 0, None, None),
            );

            let store = DefaultGraphStore::new(
                GraphStoreConfig::default(),
                GraphName::new(graph_name),
                database_info,
                GraphSchema::empty(),
                Capabilities::default(),
                SimpleIdMap::from_original_ids(original_node_ids.into_iter()),
                relationship_topologies,
            );

            let node_count = GraphStore::node_count(&store) as u64;
            let relationship_count_actual = GraphStore::relationship_count(&store) as u64;

            TSJSON_CATALOG.set(graph_name, std::sync::Arc::new(store));

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

            // Use the same shared catalog as other TSJSON facades.
            let catalog: std::sync::Arc<dyn GraphCatalog> = TSJSON_CATALOG.clone();
            let mut ctx = ExecutionContext::new(username);
            ctx.set_admin(is_admin);
            ctx.set_catalog(catalog);

            let mut processor = FormProcessor::new(ctx);
            match processor.evaluate(&form_request) {
                Ok(result) => {
                    // If an output graph name is provided, persist the ResultStore into the shared catalog
                    // so it becomes addressable by subsequent GraphStore/GraphCatalog calls.
                    let mut persisted_output = false;
                    if let Some(output_name) = form_request.output_graph_name.as_deref() {
                        TSJSON_CATALOG.set(output_name, result.graph.clone());
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

        let _ = GraphCatalog::drop(TSJSON_CATALOG.as_ref(), &["graph1"], false);
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

        let _ = GraphCatalog::drop(TSJSON_CATALOG.as_ref(), &["graph2"], false);
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
        TSJSON_CATALOG.set("graph_drop_1", Arc::new(store));

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
        for name in ["graph_drop_a", "graph_drop_b"] {
            let config = RandomGraphConfig {
                graph_name: name.to_string(),
                database_name: "db1".to_string(),
                ..Default::default()
            };
            let store = DefaultGraphStore::random_with_rng(&config, &mut rng).unwrap();
            TSJSON_CATALOG.set(name, Arc::new(store));
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
        TSJSON_CATALOG.set("graph_form_eval", Arc::new(store));

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

        let _ = GraphCatalog::drop(
            TSJSON_CATALOG.as_ref(),
            &["graph_form_eval"],
            false,
        );
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
        TSJSON_CATALOG.set("graph_form_eval_in", Arc::new(store));

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

        let _ = GraphCatalog::drop(
            TSJSON_CATALOG.as_ref(),
            &["graph_form_eval_in", "graph_form_eval_out"],
            false,
        );
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
        TSJSON_CATALOG.set("graph_form_eval_shine", Arc::new(store));

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

        let _ = GraphCatalog::drop(
            TSJSON_CATALOG.as_ref(),
            &["graph_form_eval_shine"],
            false,
        );
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

        let _ = GraphCatalog::drop(TSJSON_CATALOG.as_ref(), &["graph_stash_put"], false);
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

        let _ = GraphCatalog::drop(TSJSON_CATALOG.as_ref(), &["graph_kind_marker"], false);
    }
}
