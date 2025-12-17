//! Form ISA - Apodictic (ResultStore synthesis)
//!
//! The Form ISA lives inside `projection/eval/` (not `logic/`).
//!
//! It is intentionally minimal today:
//! - **Input**: a base graph (GraphStore) + artifacts produced by Procedure/ML stages
//! - **Output**: a graph (ResultStore) — i.e. the apodictic/singular return value

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde_json::Value as JsonValue;

use crate::config::GraphStoreConfig;
use crate::projection::eval::procedure::ExecutionContext;
use crate::projection::NodeLabel;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::{IdMap, OriginalNodeId, SimpleIdMap};
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::{GraphName, GraphStore};
use crate::types::graph::RelationshipTopology;

/// Artifacts passed into Form evaluation.
///
/// This is the “Problematic” content carried forward from Procedure/ML.
pub type FormArtifacts = HashMap<String, JsonValue>;

/// Form evaluation errors.
#[derive(Debug, thiserror::Error)]
pub enum FormError {
    #[error("Context error: {0}")]
    Context(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Unknown form operator: {0}")]
    UnknownOperator(String),

    #[error("Execution error: {0}")]
    Execution(String),
}

/// Input to a concrete Form operator.
#[derive(Debug)]
pub struct FormInput<'a> {
    pub base_graph: Arc<DefaultGraphStore>,
    pub program: &'a crate::form::core::FormShape,
    pub artifacts: &'a FormArtifacts,
}

/// The apodictic result of Form evaluation.
#[derive(Debug, Clone)]
pub struct FormResult {
    pub graph: Arc<DefaultGraphStore>,
    pub execution_time: Duration,
    pub operator: String,
    /// A structured trace/proof describing how the ResultStore was projected.
    pub proof: JsonValue,
}

/// Output produced by a concrete Form operator.
#[derive(Debug, Clone)]
pub struct FormOperatorOutput {
    pub graph: Arc<DefaultGraphStore>,
    pub proof: JsonValue,
}

/// A Form operator projects a ResultStore (GraphStore) from a base graph and artifacts.
pub trait FormOperator: Send + Sync {
    fn name(&self) -> &str;

    fn evaluate(
        &self,
        input: FormInput<'_>,
        context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput, FormError>;
}

/// A complete Form evaluation request.
#[derive(Debug, Clone)]
pub struct FormRequest {
    /// Load the base graph from the procedure ExecutionContext catalog.
    pub graph_name: String,

    /// The Form program (Shape + Context + Morph).
    pub program: crate::form::core::FormShape,

    /// Cross-stage artifacts (Procedure/ML outputs, configs, model handles).
    pub artifacts: FormArtifacts,

    /// If provided, store the resulting graph into the catalog under this name.
    pub output_graph_name: Option<String>,
}

impl FormRequest {
    pub fn new(graph_name: impl Into<String>, program: crate::form::core::FormShape) -> Self {
        Self {
            graph_name: graph_name.into(),
            program,
            artifacts: FormArtifacts::new(),
            output_graph_name: None,
        }
    }
}

/// Resolve the operator name from a FormShape.
///
/// Convention: the first entry in `morph.patterns` selects the operator.
pub fn operator_name_from_form_shape(shape: &crate::form::core::FormShape) -> Option<&str> {
    shape.morph.patterns.first().map(|s| s.as_str())
}

/// A minimal “identity” form: returns the base graph as the ResultStore.
#[derive(Debug, Default)]
pub struct PassThroughFormOperator;

impl FormOperator for PassThroughFormOperator {
    fn name(&self) -> &str {
        "passThrough"
    }

    fn evaluate(
        &self,
        input: FormInput<'_>,
        _context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput, FormError> {
        Ok(FormOperatorOutput {
            graph: input.base_graph,
            proof: serde_json::json!({ "kind": "passThrough" }),
        })
    }
}

/// Commits a selected subgraph into a new `DefaultGraphStore`.
///
/// This is the “B” operator: it turns a problematic selection into a definite
/// ResultStore by projecting an induced subgraph.
///
/// Artifacts contract (JSON):
///
/// - Either `{"selection": {"node_ids": [<i64>...]}}`
/// - Or top-level `{"node_ids": [<i64>...]}`
///
/// The ids are **original node ids** (the IdMap “external” ids).
#[derive(Debug, Default)]
pub struct CommitSubgraphOperator;

impl CommitSubgraphOperator {
    fn parse_selected_original_node_ids(artifacts: &FormArtifacts) -> Result<Vec<OriginalNodeId>, FormError> {
        fn parse_array(value: &JsonValue) -> Option<Vec<OriginalNodeId>> {
            let arr = value.as_array()?;
            let mut out = Vec::with_capacity(arr.len());
            for v in arr {
                let n = v.as_i64()?;
                out.push(n as OriginalNodeId);
            }
            Some(out)
        }

        if let Some(selection) = artifacts.get("selection") {
            if let Some(map) = selection.as_object() {
                if let Some(ids) = map.get("node_ids").and_then(parse_array) {
                    return Ok(ids);
                }
                if let Some(ids) = map.get("original_node_ids").and_then(parse_array) {
                    return Ok(ids);
                }
            }
        }

        if let Some(ids) = artifacts.get("node_ids").and_then(parse_array) {
            return Ok(ids);
        }

        Err(FormError::Config(
            "Missing selection artifact. Expected selection.node_ids or node_ids".to_string(),
        ))
    }

    fn induced_topology_for_type(
        base_graph: &DefaultGraphStore,
        relationship_type: &RelationshipType,
        selected_mapped_ids: &HashMap<i64, i64>,
        selected_ordered_old_mapped: &[i64],
        include_incoming: bool,
    ) -> Result<RelationshipTopology, FormError> {
        use std::collections::HashSet;

        let mut types = HashSet::new();
        types.insert(relationship_type.clone());
        let view = base_graph
            .get_graph_with_types(&types)
            .map_err(|e| FormError::Execution(e.to_string()))?;

        let n = selected_ordered_old_mapped.len();
        let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); n];

        for (new_source_index, &old_source) in selected_ordered_old_mapped.iter().enumerate() {
            let stream = view.stream_relationships(old_source, view.default_property_value());
            for cursor in stream {
                let old_target = cursor.target_id();
                if let Some(&new_target) = selected_mapped_ids.get(&old_target) {
                    outgoing[new_source_index].push(new_target);
                }
            }
        }

        let incoming = if include_incoming {
            let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); n];
            for (source, neighbors) in outgoing.iter().enumerate() {
                let source_id = source as i64;
                for &target in neighbors {
                    let idx = target as usize;
                    if idx < incoming.len() {
                        incoming[idx].push(source_id);
                    }
                }
            }
            Some(incoming)
        } else {
            None
        };

        Ok(RelationshipTopology::new(outgoing, incoming))
    }
}

impl FormOperator for CommitSubgraphOperator {
    fn name(&self) -> &str {
        "commitSubgraph"
    }

    fn evaluate(
        &self,
        input: FormInput<'_>,
        _context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput, FormError> {
        use std::collections::HashMap as StdHashMap;

        let selected_original_ids = Self::parse_selected_original_node_ids(input.artifacts)?;
        if selected_original_ids.is_empty() {
            return Err(FormError::Config("Selection must be non-empty".to_string()));
        }

        let base_nodes = input.base_graph.nodes();

        let mut selected_ordered_old_mapped: Vec<i64> = Vec::with_capacity(selected_original_ids.len());
        let mut selected_old_to_new: StdHashMap<i64, i64> = StdHashMap::new();

        for (index, original_id) in selected_original_ids.iter().copied().enumerate() {
            let old_mapped = base_nodes
                .safe_to_mapped_node_id(original_id)
                .ok_or_else(|| FormError::Execution(format!("Unknown node id in selection: {original_id}")))?;
            let new_mapped = index as i64;
            selected_ordered_old_mapped.push(old_mapped);
            selected_old_to_new.insert(old_mapped, new_mapped);
        }

        // Build new IdMap with preserved labels.
        let mut new_id_map = SimpleIdMap::from_original_ids(selected_original_ids.iter().copied());
        for (new_mapped, original_id) in selected_original_ids.iter().copied().enumerate() {
            let new_mapped = new_mapped as i64;
            let old_mapped = base_nodes
                .safe_to_mapped_node_id(original_id)
                .ok_or_else(|| FormError::Execution(format!("Unknown node id in selection: {original_id}")))?;
            for label in base_nodes.node_labels(old_mapped) {
                new_id_map.add_node_label(label.clone());
                new_id_map.add_node_id_to_label(new_mapped, label);
            }
        }

        // Build per-type induced relationship topologies.
        let rel_types = input.base_graph.relationship_types();
        let inverse_indexed = input.base_graph.inverse_indexed_relationship_types();
        let mut relationship_topologies = StdHashMap::new();

        for rel_type in rel_types {
            let topology = Self::induced_topology_for_type(
                &input.base_graph,
                &rel_type,
                &selected_old_to_new,
                &selected_ordered_old_mapped,
                inverse_indexed.contains(&rel_type),
            )?;

            if topology.relationship_count() > 0 {
                relationship_topologies.insert(rel_type, topology);
            }
        }

        // Proof (C): a minimal trace of the apodictic commitment.
        let mut old_mapped_to_new = serde_json::Map::new();
        for (old, new) in selected_old_to_new.iter() {
            old_mapped_to_new.insert(old.to_string(), serde_json::json!(new));
        }

        let mut by_type = serde_json::Map::new();
        let mut kept_total: usize = 0;
        for (rel_type, topo) in relationship_topologies.iter() {
            let kept = topo.relationship_count();
            kept_total += kept;
            by_type.insert(
                rel_type.to_string(),
                serde_json::json!({
                    "kept": kept,
                    "inverse_indexed": inverse_indexed.contains(rel_type)
                }),
            );
        }

        let proof = serde_json::json!({
            "kind": "commitSubgraph",
            "selection": {
                "original_node_ids": selected_original_ids,
                "count": selected_original_ids.len()
            },
            "id_map": {
                "old_mapped_to_new": old_mapped_to_new
            },
            "relationships": {
                "total_kept": kept_total,
                "by_type": by_type
            }
        });

        // NOTE: For now, this operator projects topology only. Property materialization is handled
        // by later operators (A).
        let store = DefaultGraphStore::new(
            GraphStoreConfig::default(),
            GraphName::new("form.commitSubgraph"),
            input.base_graph.database_info().clone(),
            input.base_graph.schema().clone(),
            input.base_graph.capabilities().clone(),
            new_id_map,
            relationship_topologies,
        );

        Ok(FormOperatorOutput {
            graph: Arc::new(store),
            proof,
        })
    }
}

/// Materializes numeric node properties (f64/i64) onto a new graph store.
///
/// This is the “A” operator: it takes problematic artifacts (e.g. ML predictions)
/// and writes them as node properties on the returned ResultStore.
///
/// Artifacts contract (JSON):
///
/// ```json
/// {
///   "node_properties": {
///     "score": {
///       "values": [1.0, 2.0, 3.0],
///       "value_type": "double"
///     },
///     "rank": {
///       "values_by_original_node_id": {"10": 1, "20": 2},
///       "default": 0,
///       "value_type": "long"
///     }
///   }
/// }
/// ```
///
/// - `values` must have length equal to the graph's `node_count()`.
/// - `values_by_original_node_id` maps **original node ids** to values.
/// - If `value_type` is omitted, it is inferred from the first non-null value.
#[derive(Debug, Default)]
pub struct MaterializeNodePropertiesOperator;

impl MaterializeNodePropertiesOperator {
    fn node_labels_for_store(store: &DefaultGraphStore) -> std::collections::HashSet<NodeLabel> {
        store
            .nodes()
            .available_node_labels()
            .into_iter()
            .map(|label| NodeLabel::of(label.name()))
            .collect()
    }

    fn infer_numeric_type(values: &[JsonValue]) -> Result<&'static str, FormError> {
        for v in values {
            if v.is_null() {
                continue;
            }
            if v.is_i64() {
                return Ok("long");
            }
            if v.is_u64() {
                return Ok("long");
            }
            if v.is_f64() {
                return Ok("double");
            }
            return Err(FormError::Config(
                "node_properties values must be numeric (long/double)".to_string(),
            ));
        }
        Ok("double")
    }

    fn parse_value_type(spec: &serde_json::Map<String, JsonValue>) -> Result<&'static str, FormError> {
        if let Some(vt) = spec.get("value_type").and_then(|v| v.as_str()) {
            match vt {
                "double" | "float" | "f64" => return Ok("double"),
                "long" | "int" | "i64" => return Ok("long"),
                other => {
                    return Err(FormError::Config(format!(
                        "Unsupported node property value_type: {other}"
                    )))
                }
            }
        }
        // fall back to inference from provided values
        if let Some(values) = spec.get("values").and_then(|v| v.as_array()) {
            return Self::infer_numeric_type(values);
        }
        if let Some(map) = spec
            .get("values_by_original_node_id")
            .and_then(|v| v.as_object())
        {
            let sample: Vec<JsonValue> = map.values().cloned().collect();
            return Self::infer_numeric_type(&sample);
        }
        Ok("double")
    }

    fn materialize_one_property(
        store: &mut DefaultGraphStore,
        key: &str,
        spec: &serde_json::Map<String, JsonValue>,
    ) -> Result<JsonValue, FormError> {
        let node_count = store.node_count();

        let default_value = spec.get("default").cloned().unwrap_or(JsonValue::Null);
        let value_type = Self::parse_value_type(spec)?;

        let mut missing: usize = 0;

        match value_type {
            "double" => {
                let mut out: Vec<f64> = vec![0.0; node_count];

                if let Some(values) = spec.get("values").and_then(|v| v.as_array()) {
                    if values.len() != node_count {
                        return Err(FormError::Config(format!(
                            "node_properties.{key}.values length ({}) must equal node_count ({node_count})",
                            values.len()
                        )));
                    }
                    for (i, v) in values.iter().enumerate() {
                        if let Some(n) = v.as_f64().or_else(|| v.as_i64().map(|n| n as f64)) {
                            out[i] = n;
                        } else if v.is_null() {
                            missing += 1;
                            out[i] = default_value.as_f64().unwrap_or(0.0);
                        } else {
                            return Err(FormError::Config(format!(
                                "node_properties.{key}.values[{i}] must be numeric"
                            )));
                        }
                    }
                } else if let Some(map) = spec
                    .get("values_by_original_node_id")
                    .and_then(|v| v.as_object())
                {
                    for mapped in 0..node_count {
                        let original = store
                            .nodes()
                            .to_original_node_id(mapped as i64)
                            .ok_or_else(|| FormError::Execution("IdMap missing original id".to_string()))?;
                        let k = original.to_string();
                        let v = map.get(&k).cloned().unwrap_or(JsonValue::Null);
                        if let Some(n) = v.as_f64().or_else(|| v.as_i64().map(|n| n as f64)) {
                            out[mapped] = n;
                        } else {
                            missing += 1;
                            out[mapped] = default_value.as_f64().unwrap_or(0.0);
                        }
                    }
                } else {
                    return Err(FormError::Config(format!(
                        "node_properties.{key} must provide values or values_by_original_node_id"
                    )));
                }

                store
                    .add_node_property_f64(key.to_string(), out)
                    .map_err(|e| FormError::Execution(e.to_string()))?;
            }
            "long" => {
                let mut out: Vec<i64> = vec![0; node_count];

                if let Some(values) = spec.get("values").and_then(|v| v.as_array()) {
                    if values.len() != node_count {
                        return Err(FormError::Config(format!(
                            "node_properties.{key}.values length ({}) must equal node_count ({node_count})",
                            values.len()
                        )));
                    }
                    for (i, v) in values.iter().enumerate() {
                        if let Some(n) = v.as_i64().or_else(|| v.as_u64().map(|n| n as i64)) {
                            out[i] = n;
                        } else if v.is_null() {
                            missing += 1;
                            out[i] = default_value.as_i64().unwrap_or(0);
                        } else {
                            return Err(FormError::Config(format!(
                                "node_properties.{key}.values[{i}] must be integer"
                            )));
                        }
                    }
                } else if let Some(map) = spec
                    .get("values_by_original_node_id")
                    .and_then(|v| v.as_object())
                {
                    for mapped in 0..node_count {
                        let original = store
                            .nodes()
                            .to_original_node_id(mapped as i64)
                            .ok_or_else(|| FormError::Execution("IdMap missing original id".to_string()))?;
                        let k = original.to_string();
                        let v = map.get(&k).cloned().unwrap_or(JsonValue::Null);
                        if let Some(n) = v.as_i64().or_else(|| v.as_u64().map(|n| n as i64)) {
                            out[mapped] = n;
                        } else {
                            missing += 1;
                            out[mapped] = default_value.as_i64().unwrap_or(0);
                        }
                    }
                } else {
                    return Err(FormError::Config(format!(
                        "node_properties.{key} must provide values or values_by_original_node_id"
                    )));
                }

                store
                    .add_node_property_i64(key.to_string(), out)
                    .map_err(|e| FormError::Execution(e.to_string()))?;
            }
            other => {
                return Err(FormError::Config(format!(
                    "Unsupported inferred node property type: {other}"
                )))
            }
        }

        Ok(serde_json::json!({
            "value_type": value_type,
            "missing": missing,
        }))
    }
}

impl FormOperator for MaterializeNodePropertiesOperator {
    fn name(&self) -> &str {
        "materializeNodeProperties"
    }

    fn evaluate(
        &self,
        input: FormInput<'_>,
        _context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput, FormError> {
        let node_props = input
            .artifacts
            .get("node_properties")
            .and_then(|v| v.as_object())
            .ok_or_else(|| {
                FormError::Config(
                    "Missing node_properties artifact (expected object)".to_string(),
                )
            })?;

        let mut store = (*input.base_graph).clone();

        // Ensure the property is discoverable under all store labels.
        let label_set = Self::node_labels_for_store(&store);
        if label_set.is_empty() {
            // still ok; properties become globally discoverable via `node_property_keys()`.
        }

        let mut materialized = serde_json::Map::new();
        for (key, spec_val) in node_props.iter() {
            let spec = spec_val.as_object().ok_or_else(|| {
                FormError::Config(format!("node_properties.{key} must be an object"))
            })?;

            // Materialize via store helpers, then record proof.
            let proof = Self::materialize_one_property(&mut store, key, spec)?;
            materialized.insert(key.clone(), proof);

            // If the store has labels, link the property key to them for label-scoped discovery.
            // (DefaultGraphStore's config-based helpers bypass the label index.)
            if !label_set.is_empty() {
                let values = store
                    .node_property_values(key)
                    .map_err(|e| FormError::Execution(e.to_string()))?;
                store
                    .add_node_property(label_set.clone(), key.clone(), values)
                    .map_err(|e| FormError::Execution(e.to_string()))?;
            }
        }

        let proof = serde_json::json!({
            "kind": "materializeNodeProperties",
            "properties": materialized,
        });

        Ok(FormOperatorOutput {
            graph: Arc::new(store),
            proof,
        })
    }
}

/// Convenience for timing form evaluation.
pub fn time_form_eval<T>(f: impl FnOnce() -> Result<T, FormError>) -> Result<(T, Duration), FormError> {
    let start = Instant::now();
    let out = f()?;
    Ok((out, start.elapsed()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::core::{Context, FormShape, Shape};
    use crate::form::core::shape::Morph;
    use crate::types::random::random_graph::RandomGraphConfig;
    use crate::types::graph_store::GraphStore;
    use std::sync::Arc;

    #[test]
    fn operator_name_uses_first_pattern() {
        let shape = Shape::new(vec![], vec![], HashMap::new(), HashMap::new());
        let ctx = Context::new(vec![], vec![], "strategy".to_string(), vec![]);
        let morph = Morph::new(vec![], vec!["passThrough".to_string()], vec![], vec![]);
        let form = FormShape::new(shape, ctx, morph);

        assert_eq!(operator_name_from_form_shape(&form), Some("passThrough"));
    }

    #[test]
    fn commit_subgraph_projects_selected_nodes_only() {
        let base = Arc::new(DefaultGraphStore::random(&RandomGraphConfig::seeded(42)).unwrap());

        let shape = Shape::new(vec![], vec![], HashMap::new(), HashMap::new());
        let ctx = Context::new(vec![], vec![], "strategy".to_string(), vec![]);
        let morph = Morph::new(vec![], vec!["commitSubgraph".to_string()], vec![], vec![]);
        let program = FormShape::new(shape, ctx, morph);

        let mut artifacts = FormArtifacts::new();
        artifacts.insert(
            "selection".to_string(),
            serde_json::json!({"node_ids": [0, 1, 2]}),
        );

        let op = CommitSubgraphOperator::default();
        let out = op
            .evaluate(
                FormInput {
                    base_graph: base,
                    program: &program,
                    artifacts: &artifacts,
                },
                &mut ExecutionContext::new("user"),
            )
            .unwrap();

        let result = out.graph;
        assert_eq!(out.proof.get("kind").and_then(|v| v.as_str()), Some("commitSubgraph"));
        assert_eq!(
            out.proof
                .pointer("/selection/count")
                .and_then(|v| v.as_u64()),
            Some(3)
        );

        assert_eq!(result.node_count(), 3);

        // The projected graph must not reference nodes outside the committed set.
        let g = result.get_graph();
        for node_id in 0..3 {
            for rel in g.stream_relationships(node_id, g.default_property_value()) {
                assert!(rel.target_id() >= 0 && rel.target_id() < 3);
            }
        }
    }
}
