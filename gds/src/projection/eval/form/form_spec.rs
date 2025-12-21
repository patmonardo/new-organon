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

use crate::projection::eval::procedure::ExecutionContext;
use crate::substrate::FormStoreSurface;
use crate::types::graph::id_map::{MappedNodeId, OriginalNodeId};
use crate::types::graph_store::DefaultGraphStore;
use crate::types::graph_store::GraphName;

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
pub struct FormInput<'a, S = DefaultGraphStore> {
    pub base_graph: Arc<S>,
    pub program: &'a crate::form::FormShape,
    pub artifacts: &'a FormArtifacts,
}

/// The apodictic result of Form evaluation.
#[derive(Debug, Clone)]
pub struct FormResult<S = DefaultGraphStore> {
    pub graph: Arc<S>,
    pub execution_time: Duration,
    pub operator: String,
    /// A structured trace/proof describing how the ResultStore was projected.
    pub proof: JsonValue,
}

/// Output produced by a concrete Form operator.
#[derive(Debug, Clone)]
pub struct FormOperatorOutput<S = DefaultGraphStore> {
    pub graph: Arc<S>,
    pub proof: JsonValue,
}

/// A Form operator projects a ResultStore (GraphStore) from a base graph and artifacts.
pub trait FormOperator<S = DefaultGraphStore>: Send + Sync {
    fn name(&self) -> &str;

    fn evaluate(
        &self,
        input: FormInput<'_, S>,
        context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput<S>, FormError>;
}

/// A complete Form evaluation request.
#[derive(Debug, Clone)]
pub struct FormRequest {
    /// Load the base graph from the procedure ExecutionContext catalog.
    pub graph_name: String,

    /// The Form program (Shape + Context + Morph).
    pub program: crate::form::FormShape,

    /// Cross-stage artifacts (Procedure/ML outputs, configs, model handles).
    pub artifacts: FormArtifacts,

    /// If provided, store the resulting graph into the catalog under this name.
    pub output_graph_name: Option<String>,
}

impl FormRequest {
    pub fn new(graph_name: impl Into<String>, program: crate::form::FormShape) -> Self {
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
pub fn operator_name_from_form_shape(shape: &crate::form::FormShape) -> Option<&str> {
    shape.morph.patterns.first().map(|s| s.as_str())
}

/// A minimal “identity” form: returns the base graph as the ResultStore.
#[derive(Debug, Default)]
pub struct PassThroughFormOperator;

impl<S> FormOperator<S> for PassThroughFormOperator
where
    S: Send + Sync + 'static,
{
    fn name(&self) -> &str {
        "passThrough"
    }

    fn evaluate(
        &self,
        input: FormInput<'_, S>,
        _context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput<S>, FormError> {
        Ok(FormOperatorOutput {
            graph: input.base_graph,
            proof: serde_json::json!({ "kind": "passThrough" }),
        })
    }
}

/// First moment: Essence (essentiality / presupposed).
///
/// This operator does not transform the graph. It only emits a proof marker that
/// later moments can presuppose.
///
/// Working convention:
/// - “Essence” here is a kernel marker for presupposed essentiality: the minimal
///   condition for any further form-shaping operations.
#[derive(Debug, Default)]
pub struct EssenceFormOperator;

impl<S> FormOperator<S> for EssenceFormOperator
where
    S: Send + Sync + 'static,
{
    fn name(&self) -> &str {
        "essence"
    }

    fn evaluate(
        &self,
        input: FormInput<'_, S>,
        _context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput<S>, FormError> {
        Ok(FormOperatorOutput {
            graph: input.base_graph,
            proof: serde_json::json!({
                "kind": "essence",
                "essentiality": true
            }),
        })
    }
}

/// Back-compat alias: `cit`.
///
/// We accept `morph.patterns = ["cit", ...]` to avoid breaking older programs,
/// but emit the canonical proof marker `kind = "essence"`.
#[derive(Debug, Default)]
pub struct CitFormOperator;

impl<S> FormOperator<S> for CitFormOperator
where
    S: Send + Sync + 'static,
{
    fn name(&self) -> &str {
        "cit"
    }

    fn evaluate(
        &self,
        input: FormInput<'_, S>,
        _context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput<S>, FormError> {
        Ok(FormOperatorOutput {
            graph: input.base_graph,
            proof: serde_json::json!({
                "kind": "essence",
                "essentiality": true,
                "invoked_as": "cit"
            }),
        })
    }
}

/// Second moment: Shine (positedness / Essence→Shine).
///
/// This operator is intentionally minimal today: it does not change the base graph.
/// Instead, it **re-marks** the immediate content as positedness (Essence→Shine),
/// emitting a proof marker that can be composed into later Reflection operators.
///
/// Philosophical anchors (conventions, not enforcement):
/// - Hegel: Essence → Shine (Schein)
/// - Yoga: YS IV.3 “nirmāṇa-cittāni asmitā-mātra” (constructed minds as mere I-am-ness)
#[derive(Debug, Default)]
pub struct ShineFormOperator;

impl<S> FormOperator<S> for ShineFormOperator
where
    S: Send + Sync + 'static,
{
    fn name(&self) -> &str {
        "shine"
    }

    fn evaluate(
        &self,
        input: FormInput<'_, S>,
        _context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput<S>, FormError> {
        let mut shape_rules = serde_json::Map::new();
        for (k, v) in input.program.shape.validation_rules.iter() {
            shape_rules.insert(k.clone(), serde_json::json!(v));
        }

        Ok(FormOperatorOutput {
            graph: input.base_graph,
            proof: serde_json::json!({
                "kind": "shine",
                "presupposes": "essence",
                "positedness": true,
                "anchors": {
                    "hegel": "Essence→Shine",
                    "yoga": "YS IV.3 nirmāṇa-cittāni asmitā-mātra"
                },
                "program": {
                    "shape": {
                        "validation_rules": shape_rules
                    }
                }
            }),
        })
    }
}

/// Third moment: Reflection (determination of reflection; reflective consciousness / citta).
///
/// Today this is a proof marker only: it does not alter the base graph.
#[derive(Debug, Default)]
pub struct ReflectionFormOperator;

impl<S> FormOperator<S> for ReflectionFormOperator
where
    S: Send + Sync + 'static,
{
    fn name(&self) -> &str {
        "reflection"
    }

    fn evaluate(
        &self,
        input: FormInput<'_, S>,
        _context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput<S>, FormError> {
        Ok(FormOperatorOutput {
            graph: input.base_graph,
            proof: serde_json::json!({
                "kind": "reflection",
                "presupposes": "shine",
                "citta": true,
                "anchors": {
                    "hegel": "Shine→Reflection"
                },
                "program": {
                    "context": {
                        "dependencies": input.program.context.dependencies,
                        "execution_order": input.program.context.execution_order,
                        "runtime_strategy": input.program.context.runtime_strategy,
                        "conditions": input.program.context.conditions
                    }
                }
            }),
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
    fn parse_selected_original_node_ids(
        artifacts: &FormArtifacts,
    ) -> Result<Vec<OriginalNodeId>, FormError> {
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
}

impl<S> FormOperator<S> for CommitSubgraphOperator
where
    S: FormStoreSurface<Store = S> + Send + Sync + 'static,
{
    fn name(&self) -> &str {
        "commitSubgraph"
    }

    fn evaluate(
        &self,
        input: FormInput<'_, S>,
        _context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput<S>, FormError> {
        let selected_original_ids = Self::parse_selected_original_node_ids(input.artifacts)?;
        let inverse_indexed = input.base_graph.inverse_indexed_relationship_types();
        let (store, old_mapped_to_new, kept_by_type) = input
            .base_graph
            .commit_induced_subgraph_by_original_node_ids(
                GraphName::new("form.commitSubgraph"),
                &selected_original_ids,
            )
            .map_err(|e| FormError::Execution(e.to_string()))?;

        // Proof (C): a minimal trace of the apodictic commitment.
        let mut old_mapped_to_new_json = serde_json::Map::new();
        for (old, new) in old_mapped_to_new.iter() {
            old_mapped_to_new_json.insert(old.to_string(), serde_json::json!(new));
        }

        let mut by_type = serde_json::Map::new();
        let mut kept_total: usize = 0;
        for (rel_type, kept) in kept_by_type.iter() {
            kept_total += *kept;
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
                "old_mapped_to_new": old_mapped_to_new_json
            },
            "relationships": {
                "total_kept": kept_total,
                "by_type": by_type
            }
        });

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

    fn parse_value_type(
        spec: &serde_json::Map<String, JsonValue>,
    ) -> Result<&'static str, FormError> {
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
        store: &mut impl FormStoreSurface,
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
                            .to_original_node_id(mapped as MappedNodeId)
                            .ok_or_else(|| {
                                FormError::Execution("IdMap missing original id".to_string())
                            })?;
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
                store
                    .ensure_node_property_discoverable(key)
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
                            .to_original_node_id(mapped as MappedNodeId)
                            .ok_or_else(|| {
                                FormError::Execution("IdMap missing original id".to_string())
                            })?;
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
                store
                    .ensure_node_property_discoverable(key)
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

impl<S> FormOperator<S> for MaterializeNodePropertiesOperator
where
    S: FormStoreSurface<Store = S> + Clone + Send + Sync + 'static,
{
    fn name(&self) -> &str {
        "materializeNodeProperties"
    }

    fn evaluate(
        &self,
        input: FormInput<'_, S>,
        _context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput<S>, FormError> {
        let node_props = input
            .artifacts
            .get("node_properties")
            .and_then(|v| v.as_object())
            .ok_or_else(|| {
                FormError::Config("Missing node_properties artifact (expected object)".to_string())
            })?;

        let mut store = (*input.base_graph).clone();

        let mut materialized = serde_json::Map::new();
        for (key, spec_val) in node_props.iter() {
            let spec = spec_val.as_object().ok_or_else(|| {
                FormError::Config(format!("node_properties.{key} must be an object"))
            })?;

            // Materialize via store helpers, then record proof.
            let proof = Self::materialize_one_property(&mut store, key, spec)?;
            materialized.insert(key.clone(), proof);
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

/// Materializes numeric relationship properties (f64) onto a new graph store.
///
/// This is an extension of (A) into the relational domain: it writes determinations
/// about relationships as explicit typed property vectors.
///
/// Artifacts contract (JSON):
///
/// ```json
/// {
///   "relationship_properties": {
///     "R": {
///       "weight": {
///         "values": [0.5, 1.2, 3.0],
///         "value_type": "double"
///       }
///     }
///   }
/// }
/// ```
///
/// Optionally, you may provide endpoint-addressed values:
///
/// ```json
/// {
///   "relationship_properties": {
///     "R": {
///       "weight": {
///         "edges_by_original_node_id": [[10, 20], [10, 30]],
///         "values": [0.5, 1.2],
///         "default": 0.0,
///         "value_type": "double"
///       }
///     }
///   }
/// }
/// ```
///
/// - When `edges_by_original_node_id` is provided, `values` must have the same length.
/// - Parallel edges are supported by consuming endpoint-specified values in order.
#[derive(Debug, Default)]
pub struct MaterializeRelationshipPropertiesOperator;

impl MaterializeRelationshipPropertiesOperator {
    fn parse_value_type(
        spec: &serde_json::Map<String, JsonValue>,
    ) -> Result<&'static str, FormError> {
        if let Some(vt) = spec.get("value_type").and_then(|v| v.as_str()) {
            match vt {
                "double" | "float" | "f64" => return Ok("double"),
                other => {
                    return Err(FormError::Config(format!(
                        "Unsupported relationship property value_type: {other}"
                    )))
                }
            }
        }
        Ok("double")
    }

    fn parse_edges_by_original(
        value: &JsonValue,
    ) -> Result<Vec<(OriginalNodeId, OriginalNodeId)>, FormError> {
        let arr = value.as_array().ok_or_else(|| {
            FormError::Config("edges_by_original_node_id must be an array".to_string())
        })?;

        let mut out: Vec<(OriginalNodeId, OriginalNodeId)> = Vec::with_capacity(arr.len());
        for (i, pair) in arr.iter().enumerate() {
            let pair_arr = pair.as_array().ok_or_else(|| {
                FormError::Config(format!("edges_by_original_node_id[{i}] must be [src, dst]"))
            })?;
            if pair_arr.len() != 2 {
                return Err(FormError::Config(format!(
                    "edges_by_original_node_id[{i}] must have length 2"
                )));
            }
            let src = pair_arr[0].as_i64().ok_or_else(|| {
                FormError::Config(format!("edges_by_original_node_id[{i}][0] must be i64"))
            })? as OriginalNodeId;
            let dst = pair_arr[1].as_i64().ok_or_else(|| {
                FormError::Config(format!("edges_by_original_node_id[{i}][1] must be i64"))
            })? as OriginalNodeId;
            out.push((src, dst));
        }
        Ok(out)
    }

    fn materialize_one_property(
        store: &mut impl FormStoreSurface,
        relationship_type: &crate::projection::RelationshipType,
        key: &str,
        spec: &serde_json::Map<String, JsonValue>,
    ) -> Result<JsonValue, FormError> {
        let value_type = Self::parse_value_type(spec)?;
        if value_type != "double" {
            return Err(FormError::Config(
                "Only double relationship properties are supported".to_string(),
            ));
        }

        let default_value = spec.get("default").and_then(|v| v.as_f64()).unwrap_or(0.0);

        let mut missing: usize = 0;
        let mut unused: usize = 0;

        // Case 1: direct store-order values
        if spec.get("edges_by_original_node_id").is_none() {
            let values = spec
                .get("values")
                .and_then(|v| v.as_array())
                .ok_or_else(|| {
                    FormError::Config(format!(
                        "relationship_properties.{relationship_type}.{key} must provide values"
                    ))
                })?;

            let expected = store.relationship_count_for_type(relationship_type);
            if values.len() != expected {
                return Err(FormError::Config(format!(
                    "relationship_properties.{relationship_type}.{key}.values length ({}) must equal relationship_count_for_type ({expected})",
                    values.len()
                )));
            }

            let mut out: Vec<f64> = vec![default_value; expected];
            for (i, v) in values.iter().enumerate() {
                if let Some(n) = v.as_f64().or_else(|| v.as_i64().map(|n| n as f64)) {
                    out[i] = n;
                } else if v.is_null() {
                    missing += 1;
                    out[i] = default_value;
                } else {
                    return Err(FormError::Config(format!(
                        "relationship_properties.{relationship_type}.{key}.values[{i}] must be numeric"
                    )));
                }
            }

            store
                .add_relationship_property_f64(relationship_type.clone(), key.to_string(), out)
                .map_err(|e| FormError::Execution(e.to_string()))?;

            return Ok(serde_json::json!({
                "value_type": value_type,
                "missing": missing,
                "unused": unused,
            }));
        }

        // Case 2: endpoint-addressed values (original node ids)
        let edges_by_original =
            Self::parse_edges_by_original(spec.get("edges_by_original_node_id").unwrap())?;
        let values = spec
            .get("values")
            .and_then(|v| v.as_array())
            .ok_or_else(|| {
                FormError::Config(format!(
                    "relationship_properties.{relationship_type}.{key} must provide values"
                ))
            })?;
        if values.len() != edges_by_original.len() {
            return Err(FormError::Config(format!(
                "relationship_properties.{relationship_type}.{key}.values length ({}) must equal edges_by_original_node_id length ({})",
                values.len(),
                edges_by_original.len()
            )));
        }

        // Build original->mapped lookup using the store's IdMap.
        let mut mapped_by_original: HashMap<OriginalNodeId, MappedNodeId> =
            HashMap::with_capacity(store.node_count());
        for mapped in 0..store.node_count() {
            let original = store
                .to_original_node_id(mapped as MappedNodeId)
                .ok_or_else(|| FormError::Execution("IdMap missing original id".to_string()))?;
            mapped_by_original.insert(original, mapped as MappedNodeId);
        }

        // Map endpoints -> queue of values (supports parallel edges)
        let mut queue_by_edge: HashMap<(MappedNodeId, MappedNodeId), Vec<f64>> = HashMap::new();
        for (i, (src_o, dst_o)) in edges_by_original.into_iter().enumerate() {
            let src = *mapped_by_original.get(&src_o).ok_or_else(|| {
                FormError::Config(
                    "edges_by_original_node_id contains unknown source original id".to_string(),
                )
            })?;
            let dst = *mapped_by_original.get(&dst_o).ok_or_else(|| {
                FormError::Config(
                    "edges_by_original_node_id contains unknown target original id".to_string(),
                )
            })?;

            let v = &values[i];
            let n = v
                .as_f64()
                .or_else(|| v.as_i64().map(|n| n as f64))
                .unwrap_or(default_value);
            if v.is_null() {
                missing += 1;
            }
            queue_by_edge.entry((src, dst)).or_default().push(n);
        }

        let edges_in_order = store
            .relationship_edges_in_store_order(relationship_type)
            .map_err(|e| FormError::Execution(e.to_string()))?;
        let expected = store.relationship_count_for_type(relationship_type);
        if edges_in_order.len() != expected {
            return Err(FormError::Execution(
                "relationship_edges_in_store_order length mismatch".to_string(),
            ));
        }

        let mut out: Vec<f64> = vec![default_value; expected];
        for (i, (src, dst)) in edges_in_order.into_iter().enumerate() {
            if let Some(q) = queue_by_edge.get_mut(&(src, dst)) {
                if !q.is_empty() {
                    out[i] = q.remove(0);
                    continue;
                }
            }

            missing += 1;
            out[i] = default_value;
        }

        for (_edge, remaining) in queue_by_edge.into_iter() {
            unused += remaining.len();
        }

        store
            .add_relationship_property_f64(relationship_type.clone(), key.to_string(), out)
            .map_err(|e| FormError::Execution(e.to_string()))?;

        Ok(serde_json::json!({
            "value_type": value_type,
            "missing": missing,
            "unused": unused,
        }))
    }
}

impl<S> FormOperator<S> for MaterializeRelationshipPropertiesOperator
where
    S: FormStoreSurface<Store = S> + Clone + Send + Sync + 'static,
{
    fn name(&self) -> &str {
        "materializeRelationshipProperties"
    }

    fn evaluate(
        &self,
        input: FormInput<'_, S>,
        _context: &mut ExecutionContext,
    ) -> Result<FormOperatorOutput<S>, FormError> {
        let rel_props = input
            .artifacts
            .get("relationship_properties")
            .and_then(|v| v.as_object())
            .ok_or_else(|| {
                FormError::Config(
                    "Missing relationship_properties artifact (expected object)".to_string(),
                )
            })?;

        let mut store = (*input.base_graph).clone();
        let mut materialized = serde_json::Map::new();

        for (rel_type_str, by_key_val) in rel_props.iter() {
            let by_key = by_key_val.as_object().ok_or_else(|| {
                FormError::Config(format!(
                    "relationship_properties.{rel_type_str} must be an object"
                ))
            })?;
            let rel_type = crate::projection::RelationshipType::of(rel_type_str);

            let mut key_map = serde_json::Map::new();
            for (key, spec_val) in by_key.iter() {
                let spec = spec_val.as_object().ok_or_else(|| {
                    FormError::Config(format!(
                        "relationship_properties.{rel_type_str}.{key} must be an object"
                    ))
                })?;

                let proof = Self::materialize_one_property(&mut store, &rel_type, key, spec)?;
                key_map.insert(key.clone(), proof);
            }

            materialized.insert(rel_type_str.clone(), JsonValue::Object(key_map));
        }

        let proof = serde_json::json!({
            "kind": "materializeRelationshipProperties",
            "properties": materialized,
        });

        Ok(FormOperatorOutput {
            graph: Arc::new(store),
            proof,
        })
    }
}

/// Convenience for timing form evaluation.
pub fn time_form_eval<T>(
    f: impl FnOnce() -> Result<T, FormError>,
) -> Result<(T, Duration), FormError> {
    let start = Instant::now();
    let out = f()?;
    Ok((out, start.elapsed()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::{Context, FormShape, Morph, Shape};
    use crate::types::graph_store::GraphStore;
    use crate::types::random::random_graph::RandomGraphConfig;
    use std::sync::Arc;

    #[test]
    fn operator_name_uses_first_pattern() {
        let shape = Shape::new(vec![], vec![], HashMap::new(), HashMap::new());
        let ctx = Context::new(vec![], vec![], "strategy".to_string(), vec![]);
        let morph = Morph::new(vec!["passThrough".to_string()]);
        let form = FormShape::new(shape, ctx, morph);

        assert_eq!(operator_name_from_form_shape(&form), Some("passThrough"));
    }

    #[test]
    fn commit_subgraph_projects_selected_nodes_only() {
        let base = Arc::new(DefaultGraphStore::random(&RandomGraphConfig::seeded(42)).unwrap());

        let shape = Shape::new(vec![], vec![], HashMap::new(), HashMap::new());
        let ctx = Context::new(vec![], vec![], "strategy".to_string(), vec![]);
        let morph = Morph::new(vec!["commitSubgraph".to_string()]);
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
        assert_eq!(
            out.proof.get("kind").and_then(|v| v.as_str()),
            Some("commitSubgraph")
        );
        assert_eq!(
            out.proof
                .pointer("/selection/count")
                .and_then(|v| v.as_u64()),
            Some(3)
        );

        assert_eq!(
            crate::types::graph_store::GraphStore::node_count(result.as_ref()),
            3
        );

        // The projected graph must not reference nodes outside the committed set.
        let g = result.get_graph();
        for node_id in 0..3 {
            for rel in g.stream_relationships(node_id, g.default_property_value()) {
                assert!(rel.target_id() >= 0 && rel.target_id() < 3);
            }
        }
    }
}
