use std::collections::HashMap;
use std::sync::Arc;

use crate::projection::eval::procedure::ExecutionContext;
use crate::substrate::FormStoreSurface;

use super::form_spec::{
    time_form_eval, CommitSubgraphOperator, FormError, FormInput, FormOperator, FormOperatorOutput,
    FormRequest, FormResult, MaterializeNodePropertiesOperator,
    MaterializeRelationshipPropertiesOperator, PassThroughFormOperator,
};

/// A registry of Form operators, monomorphized for a particular store type.
#[derive(Default)]
pub struct PureFormCatalog<S> {
    ops: HashMap<String, Box<dyn FormOperator<S>>>,
}

impl<S> PureFormCatalog<S> {
    pub fn new() -> Self {
        Self {
            ops: HashMap::new(),
        }
    }

    pub fn insert(&mut self, op: impl FormOperator<S> + 'static) {
        self.ops.insert(op.name().to_string(), Box::new(op));
    }

    pub fn get(&self, name: &str) -> Option<&dyn FormOperator<S>> {
        self.ops.get(name).map(|b| b.as_ref())
    }
}

/// PureForm processor: runs the Form ISA without depending on the procedure graph catalog.
///
/// This is the substrate-facing entrypoint: the base graph is provided directly and may be
/// backed by a substrate-native store.
pub struct PureFormProcessor<S> {
    catalog: PureFormCatalog<S>,
    context: ExecutionContext,
}

impl<S> Default for PureFormProcessor<S>
where
    S: FormStoreSurface<Store = S> + Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<S> PureFormProcessor<S>
where
    S: FormStoreSurface<Store = S> + Clone + Send + Sync + 'static,
{
    /// Construct a PureForm processor with the standard Form operators registered.
    pub fn new() -> Self {
        let mut catalog = PureFormCatalog::new();
        catalog.insert(PassThroughFormOperator::default());
        catalog.insert(CommitSubgraphOperator::default());
        catalog.insert(MaterializeNodePropertiesOperator::default());
        catalog.insert(MaterializeRelationshipPropertiesOperator::default());

        Self {
            catalog,
            context: ExecutionContext::empty(),
        }
    }

    /// Construct a PureForm processor with standard operators and a caller-provided context.
    ///
    /// This lets substrate callers reuse the existing `ExecutionContext` facilities
    /// (metrics, logging knobs, etc.) without going through the procedure graph catalog.
    pub fn new_with_context(context: ExecutionContext) -> Self {
        let mut processor = Self::new();
        processor.context = context;
        processor
    }

    /// Convenience helper: evaluate a Form request in a single call using the standard
    /// operator catalog.
    pub fn evaluate(base_graph: Arc<S>, request: &FormRequest) -> Result<FormResult<S>, FormError> {
        let mut processor = Self::new();
        processor.evaluate_on(base_graph, request)
    }

    pub fn catalog_mut(&mut self) -> &mut PureFormCatalog<S> {
        &mut self.catalog
    }

    pub fn context(&self) -> &ExecutionContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut ExecutionContext {
        &mut self.context
    }

    pub fn evaluate_on(
        &mut self,
        base_graph: Arc<S>,
        request: &FormRequest,
    ) -> Result<FormResult<S>, FormError> {
        if request.output_graph_name.is_some() {
            return Err(FormError::Config(
                "PureFormProcessor does not support output_graph_name".to_string(),
            ));
        }

        let patterns = &request.program.morph.patterns;
        if patterns.is_empty() {
            return Err(FormError::Config(
                "FormShape.morph.patterns must be non-empty".to_string(),
            ));
        }

        let mut current_graph = base_graph;
        let mut step_proofs: Vec<serde_json::Value> = Vec::with_capacity(patterns.len());
        let mut operator_names: Vec<String> = Vec::with_capacity(patterns.len());

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

                current_graph = graph;
                last_proof = step_proofs
                    .last()
                    .cloned()
                    .unwrap_or_else(|| serde_json::json!(null));
            }

            Ok((current_graph, last_proof))
        })?;

        let meta = serde_json::json!({
            "operators": operator_names,
            "base_graph": request.graph_name,
            "execution_time_ms": elapsed.as_millis(),
        });

        let proof = serde_json::json!({
            "meta": meta,
            "steps": step_proofs,
            "final": final_proof,
        });

        let operator = patterns.join(" -> ");

        Ok(FormResult {
            graph: final_graph,
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
    use crate::projection::RelationshipType;
    use crate::substrate::InMemoryFormStore;

    #[test]
    fn pure_form_commit_then_materialize_uses_substrate_store() {
        let mut base = InMemoryFormStore::new(vec![0, 1, 2, 3]).unwrap();
        base.add_relationships_by_original(
            RelationshipType::of("R"),
            vec![(0, 1), (1, 2), (2, 3), (0, 3)],
        )
        .unwrap();

        let base = Arc::new(base);

        let shape = Shape::new(vec![], vec![], HashMap::new(), HashMap::new());
        let fctx = Context::new(vec![], vec![], "strategy".to_string(), vec![]);
        let morph = Morph::new(vec![
            "commitSubgraph".to_string(),
            "materializeNodeProperties".to_string(),
        ]);
        let program = FormShape::new(shape, fctx, morph);

        let mut request = FormRequest::new("base", program);
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

        let result = PureFormProcessor::<InMemoryFormStore>::evaluate(base, &request).unwrap();

        assert_eq!(
            result.operator,
            "commitSubgraph -> materializeNodeProperties"
        );
        assert_eq!(result.graph.node_count(), 3);

        let values = result
            .graph
            .node_property_f64("score")
            .expect("score exists");
        assert_eq!(values, &[1.0, 2.0, 3.0]);

        // Ensure edges were induced (0->1,1->2 kept; 2->3 and 0->3 dropped)
        let rel_type = RelationshipType::of("R");
        let edges = result
            .graph
            .edges_by_type()
            .get(&rel_type)
            .expect("R edges exist");
        assert_eq!(edges.len(), 2);
        assert!(edges.contains(&(0, 1)));
        assert!(edges.contains(&(1, 2)));
    }

    #[test]
    fn pure_form_materialize_relationship_properties_writes_f64_values() {
        let mut base = InMemoryFormStore::new(vec![0, 1, 2, 3]).unwrap();
        base.add_relationships_by_original(
            RelationshipType::of("R"),
            vec![(0, 1), (1, 2), (2, 3), (0, 3)],
        )
        .unwrap();
        let base = Arc::new(base);

        let shape = Shape::new(vec![], vec![], HashMap::new(), HashMap::new());
        let fctx = Context::new(vec![], vec![], "strategy".to_string(), vec![]);
        let morph = Morph::new(vec!["materializeRelationshipProperties".to_string()]);
        let program = FormShape::new(shape, fctx, morph);

        let mut request = FormRequest::new("base", program);
        request.artifacts.insert(
            "relationship_properties".to_string(),
            serde_json::json!({
                "R": {
                    "weight": {
                        "edges_by_original_node_id": [[2,3],[0,1]],
                        "values": [3.0, 0.5],
                        "default": 0.0,
                        "value_type": "double"
                    }
                }
            }),
        );

        let result = PureFormProcessor::<InMemoryFormStore>::evaluate(base, &request).unwrap();
        assert_eq!(result.operator, "materializeRelationshipProperties");

        let rel_type = RelationshipType::of("R");
        let values = result
            .graph
            .relationship_property_f64(&rel_type, "weight")
            .expect("weight exists");
        // Store order: (0->1),(1->2),(2->3),(0->3)
        assert_eq!(values, &[0.5, 0.0, 3.0, 0.0]);
    }
}
