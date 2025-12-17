//! Form Processor - Apodictic ResultStore projector.
//!
//! The processor is intentionally small: it loads the base graph from the existing
//! procedure `ExecutionContext` catalog, selects a Form operator via the FormShape's
//! `morph.patterns[0]`, and returns an `Arc<DefaultGraphStore>`.

use std::collections::HashMap;

use crate::projection::eval::procedure::ExecutionContext;
use crate::substrate::{RealityFabric, WitnessFabric};
use crate::types::graph_store::DefaultGraphStore;

use super::form_spec::{
    time_form_eval, CommitSubgraphOperator, FormError, FormInput, FormOperator, FormOperatorOutput,
    FormRequest, FormResult, MaterializeNodePropertiesOperator,
    MaterializeRelationshipPropertiesOperator, PassThroughFormOperator,
};

/// A registry of Form operators.
#[derive(Default)]
pub struct FormCatalog {
    ops: HashMap<String, Box<dyn FormOperator>>,
}

impl FormCatalog {
    pub fn new() -> Self {
        Self { ops: HashMap::new() }
    }

    pub fn insert(&mut self, op: impl FormOperator + 'static) {
        self.ops.insert(op.name().to_string(), Box::new(op));
    }

    pub fn get(&self, name: &str) -> Option<&dyn FormOperator> {
        self.ops.get(name).map(|b| b.as_ref())
    }
}

/// Executes Form evaluation and returns a ResultStore (GraphStore).
pub struct FormProcessor {
    catalog: FormCatalog,
    context: ExecutionContext,
}

impl FormProcessor {
    pub fn new(context: ExecutionContext) -> Self {
        let mut catalog = FormCatalog::new();
        catalog.insert(PassThroughFormOperator::default());
        catalog.insert(CommitSubgraphOperator::default());
        catalog.insert(MaterializeNodePropertiesOperator::default());
        catalog.insert(MaterializeRelationshipPropertiesOperator::default());
        Self { catalog, context }
    }

    pub fn context(&self) -> &ExecutionContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut ExecutionContext {
        &mut self.context
    }

    pub fn catalog_mut(&mut self) -> &mut FormCatalog {
        &mut self.catalog
    }

    pub fn evaluate(&mut self, request: &FormRequest) -> Result<FormResult, FormError> {
        self.evaluate_with_witness(request, None)
    }

    /// Evaluate a request while recording witness events through a RealityFabric.
    ///
    /// This makes Form Eval the Wheel's **center conjunction**: it is not only a
    /// transformation, but a witnessed transformation.
    pub fn evaluate_with_fabric<S, C, W: WitnessFabric>(
        &mut self,
        request: &FormRequest,
        fabric: &RealityFabric<S, C, W>,
    ) -> Result<FormResult, FormError> {
        self.evaluate_with_witness(request, Some((&fabric.control.trace_id, &fabric.witness)))
    }

    fn evaluate_with_witness(
        &mut self,
        request: &FormRequest,
        witness: Option<(&Option<String>, &dyn WitnessFabric)>,
    ) -> Result<FormResult, FormError> {
        let base_graph = self
            .context
            .load_graph(&request.graph_name)
            .map_err(|e| FormError::Context(e.to_string()))?;

        let patterns = &request.program.morph.patterns;
        if patterns.is_empty() {
            return Err(FormError::Config(
                "FormShape.morph.patterns must be non-empty".to_string(),
            ));
        }

        let mut current_graph = base_graph;
        let mut step_proofs: Vec<serde_json::Value> = Vec::with_capacity(patterns.len());
        let mut operator_names: Vec<String> = Vec::with_capacity(patterns.len());

        if let Some((trace_id, witness)) = witness.as_ref() {
            witness.record(serde_json::json!({
                "kind": "form.eval.start",
                "trace_id": trace_id.as_deref(),
                "base_graph": request.graph_name,
                "output_graph": request.output_graph_name,
                "operators": patterns,
            }));
        }

        let ((final_graph, final_proof), elapsed) = time_form_eval(|| {
            let mut last_proof = serde_json::json!(null);
            for op_name in patterns {
                let op = self
                    .catalog
                    .get(op_name)
                    .ok_or_else(|| FormError::UnknownOperator(op_name.to_string()))?;

                let (FormOperatorOutput { graph, proof }, step_elapsed) = time_form_eval(|| {
                    op.evaluate(
                        FormInput {
                            base_graph: current_graph,
                            program: &request.program,
                            artifacts: &request.artifacts,
                        },
                        &mut self.context,
                    )
                })?;

                operator_names.push(op_name.to_string());
                step_proofs.push(serde_json::json!({
                    "operator": op_name,
                    "execution_time_ms": step_elapsed.as_millis(),
                    "proof": proof,
                }));

                if let Some((trace_id, witness)) = witness.as_ref() {
                    witness.record(serde_json::json!({
                        "kind": "form.eval.step",
                        "trace_id": trace_id.as_deref(),
                        "operator": op_name,
                        "execution_time_ms": step_elapsed.as_millis(),
                        "proof": step_proofs.last(),
                    }));
                }

                current_graph = graph;
                last_proof = step_proofs
                    .last()
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!(null));
            }

            Ok((current_graph, last_proof))
        })?;

        let graph = final_graph;

        if let Some(name) = &request.output_graph_name {
            self.context.add_graph(name.clone(), graph.clone());
        }

        let meta = serde_json::json!({
            "operators": operator_names,
            "base_graph": request.graph_name,
            "output_graph": request.output_graph_name,
            "execution_time_ms": elapsed.as_millis(),
        });

        let proof = serde_json::json!({
            "meta": meta,
            "steps": step_proofs,
            "final": final_proof,
        });

        let operator = patterns.join(" -> ");

        if let Some((trace_id, witness)) = witness.as_ref() {
            witness.record(serde_json::json!({
                "kind": "form.eval.end",
                "trace_id": trace_id.as_deref(),
                "operator": operator,
                "execution_time_ms": elapsed.as_millis(),
                "output_graph": request.output_graph_name,
                "final_proof": final_proof,
            }));
        }

        Ok(FormResult {
            graph,
            execution_time: elapsed,
            operator,
            proof,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::{Context, FormShape, Morph, Shape};
    use crate::substrate::FabricControl;
    use crate::types::random::random_graph::RandomGraphConfig;
    use crate::types::graph_store::GraphStore;
    use crate::types::ValueType;
    use serde_json::Value as JsonValue;
    use std::sync::{Arc, Mutex};
    use std::sync::Arc;

    #[derive(Clone, Default)]
    struct CollectWitness(Arc<Mutex<Vec<JsonValue>>>);

    impl WitnessFabric for CollectWitness {
        fn record(&self, event: JsonValue) {
            self.0.lock().unwrap().push(event);
        }
    }

    #[test]
    fn passthrough_projects_base_graph() {
        let graph = Arc::new(DefaultGraphStore::random(&RandomGraphConfig::seeded(7)).unwrap());
        let mut ctx = ExecutionContext::new("user");
        ctx.add_graph("g", graph.clone());

        let shape = Shape::new(vec![], vec![], HashMap::new(), HashMap::new());
        let fctx = Context::new(vec![], vec![], "strategy".to_string(), vec![]);
        let morph = Morph::new(vec!["passThrough".to_string()]);
        let program = FormShape::new(shape, fctx, morph);

        let request = FormRequest::new("g", program);
        let mut processor = FormProcessor::new(ctx);
        let result = processor.evaluate(&request).unwrap();

        assert_eq!(result.operator, "passThrough");
        assert_eq!(result.graph.node_count(), graph.node_count());
        assert_eq!(result.graph.relationship_count(), graph.relationship_count());
    }

    #[test]
    fn commit_then_materialize_node_properties() {
        let graph = Arc::new(DefaultGraphStore::random(&RandomGraphConfig::seeded(9)).unwrap());
        let mut ctx = ExecutionContext::new("user");
        ctx.add_graph("g", graph);

        let shape = Shape::new(vec![], vec![], HashMap::new(), HashMap::new());
        let fctx = Context::new(vec![], vec![], "strategy".to_string(), vec![]);
        let morph = Morph::new(vec![
            "commitSubgraph".to_string(),
            "materializeNodeProperties".to_string(),
        ]);
        let program = FormShape::new(shape, fctx, morph);

        let mut request = FormRequest::new("g", program);
        request.artifacts.insert(
            "selection".to_string(),
            serde_json::json!({"node_ids": [0, 1, 2]}),
        );
        request.artifacts.insert(
            "node_properties".to_string(),
            serde_json::json!({
                "score": {
                    "values_by_original_node_id": {"0": 1.0, "1": 2.0, "2": 3.0},
                    "value_type": "double"
                }
            }),
        );

        let mut processor = FormProcessor::new(ctx);
        let result = processor.evaluate(&request).unwrap();

        assert_eq!(result.operator, "commitSubgraph -> materializeNodeProperties");
        assert_eq!(result.graph.node_count(), 3);

        let values = result
            .graph
            .node_property_values("score")
            .expect("score property exists");
        assert_eq!(values.value_type(), ValueType::Double);

        // The committed IdMap preserves selection order [0,1,2] as mapped ids [0,1,2].
        assert_eq!(values.double_value(0).unwrap(), 1.0);
        assert_eq!(values.double_value(1).unwrap(), 2.0);
        assert_eq!(values.double_value(2).unwrap(), 3.0);
    }

    #[test]
    fn form_eval_records_witness_events_when_fabric_is_provided() {
        let graph = Arc::new(DefaultGraphStore::random(&RandomGraphConfig::seeded(11)).unwrap());
        let mut ctx = ExecutionContext::new("user");
        ctx.add_graph("g", graph.clone());

        let shape = Shape::new(vec![], vec![], HashMap::new(), HashMap::new());
        let fctx = Context::new(vec![], vec![], "strategy".to_string(), vec![]);
        let morph = Morph::new(vec!["passThrough".to_string()]);
        let program = FormShape::new(shape, fctx, morph);

        let request = FormRequest::new("g", program);

        let witness = CollectWitness::default();
        let fabric = RealityFabric {
            storage: (),
            compute: (),
            control: FabricControl {
                trace_id: Some("t-form-1".to_string()),
                ..Default::default()
            },
            time: Default::default(),
            witness: witness.clone(),
        };

        let mut processor = FormProcessor::new(ctx);
        let result = processor.evaluate_with_fabric(&request, &fabric).unwrap();
        assert_eq!(result.operator, "passThrough");

        let events = witness.0.lock().unwrap();
        assert!(events.iter().any(|e| e["kind"] == "form.eval.start"));
        assert!(events.iter().any(|e| e["kind"] == "form.eval.step"));
        assert!(events.iter().any(|e| e["kind"] == "form.eval.end"));
        assert!(events
            .iter()
            .filter(|e| e["trace_id"] == "t-form-1")
            .count()
            >= 2);
    }
}
