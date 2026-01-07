//! Java: `GraphSageAlgorithmFactory`.

use crate::algo::embeddings::graphsage::algo::graph_sage::GraphSage;
use crate::algo::embeddings::graphsage::algo::graph_sage_model_resolver::GraphSageModelResolver;
use crate::concurrency::Concurrency;
use crate::concurrency::TerminationFlag;
use crate::core::model::ModelCatalog;
use crate::core::utils::progress::TaskProgressTracker;
use crate::types::graph::Graph;
use std::sync::Arc;

#[derive(Clone)]
pub struct GraphSageAlgorithmFactory<MC: ModelCatalog> {
    model_catalog: Arc<MC>,
}

impl<MC: ModelCatalog> GraphSageAlgorithmFactory<MC> {
    pub fn new(model_catalog: Arc<MC>) -> Self {
        Self { model_catalog }
    }

    pub fn build(
        &self,
        graph: Arc<dyn Graph>,
        model_user: &str,
        model_name: &str,
        concurrency: Concurrency,
        batch_size: usize,
        progress_tracker: TaskProgressTracker,
    ) -> GraphSage {
        // Java: validateRelationshipWeightPropertyValue(...)
        if graph.has_relationship_property() {
            validate_relationship_weight_property_value(graph.as_ref());
        }

        let model =
            GraphSageModelResolver::resolve_model(&*self.model_catalog, model_user, model_name)
                .expect("resolve GraphSage model");

        GraphSage::new(
            graph,
            model,
            concurrency,
            batch_size,
            progress_tracker,
            TerminationFlag::default(),
        )
    }
}

fn validate_relationship_weight_property_value(graph: &dyn Graph) {
    let fallback = graph.default_property_value();
    for node_id in 0..graph.node_count() {
        for rel in graph.stream_relationships_weighted(node_id as i64, fallback) {
            let w = rel.weight();
            if !w.is_finite() {
                panic!("GraphSage relationship weights must be finite (found {w})");
            }
        }
    }
}
