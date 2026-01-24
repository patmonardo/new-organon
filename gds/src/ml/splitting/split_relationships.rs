use super::config::SplitRelationshipsParameters;
use super::directed_edge_splitter::DirectedEdgeSplitter;
use super::edge_splitter::EdgeSplitter;
use super::edge_splitter::RelationshipsBuilderFactory;
use super::edge_splitter::SplitResult;
use super::undirected_edge_splitter::UndirectedEdgeSplitter;
use crate::ml::sampling::negative::NegativeSampler;
use crate::ml::sampling::negative::RandomNegativeSampler;
use crate::projection::factory::RelationshipsBuilder;
use crate::types::graph::id_map::IdMap;
use crate::types::graph::Graph;
use std::sync::Arc;

/// Mirrors Java `SplitRelationships` algorithm.
///
/// Note: This struct operates on already-projected graphs and id maps.
/// GraphStore-level wiring is handled higher up the stack.
#[derive(Clone)]
pub struct SplitRelationships {
    graph: Arc<dyn Graph>,
    master_graph: Arc<dyn Graph>,
    root_nodes: Arc<dyn IdMap>,
    source_nodes: Arc<dyn IdMap>,
    target_nodes: Arc<dyn IdMap>,
    parameters: SplitRelationshipsParameters,
    builder_factory: Arc<dyn RelationshipsBuilderFactory>,
}

impl SplitRelationships {
    pub fn new(
        graph: Arc<dyn Graph>,
        master_graph: Arc<dyn Graph>,
        root_nodes: Arc<dyn IdMap>,
        source_nodes: Arc<dyn IdMap>,
        target_nodes: Arc<dyn IdMap>,
        parameters: SplitRelationshipsParameters,
        builder_factory: Arc<dyn RelationshipsBuilderFactory>,
    ) -> Self {
        Self {
            graph,
            master_graph,
            root_nodes,
            source_nodes,
            target_nodes,
            parameters,
            builder_factory,
        }
    }

    pub fn compute(&self) -> SplitResult {
        let mut splitter: Box<dyn EdgeSplitter> = if self.graph.schema().is_undirected() {
            Box::new(UndirectedEdgeSplitter::new(
                self.parameters.random_seed,
                self.root_nodes.clone(),
                self.source_nodes.clone(),
                self.target_nodes.clone(),
                self.parameters.holdout_relationship_type.clone(),
                self.parameters.remaining_relationship_type.clone(),
                self.parameters.concurrency.value(),
            ))
        } else {
            Box::new(DirectedEdgeSplitter::new(
                self.parameters.random_seed,
                self.root_nodes.clone(),
                self.source_nodes.clone(),
                self.target_nodes.clone(),
                self.parameters.holdout_relationship_type.clone(),
                self.parameters.remaining_relationship_type.clone(),
                self.parameters.concurrency.value(),
            ))
        };

        let mut split_result = splitter.split_positive_examples(
            self.graph.clone(),
            self.parameters.holdout_fraction,
            self.parameters.relationship_weight_property.clone(),
            self.builder_factory.as_ref(),
        );

        let negative_samples = (split_result.selected_rel_count as f64
            * self.parameters.negative_sampling_ratio)
            .floor() as usize;

        let negative_sampler = RandomNegativeSampler::new(
            self.master_graph.clone(),
            negative_samples,
            0,
            self.source_nodes.clone(),
            self.target_nodes.clone(),
            self.parameters.random_seed,
        );

        let mut null_builder = NullRelationshipsBuilder;
        negative_sampler.produce_negative_samples(
            split_result.selected_relationships.as_mut(),
            &mut null_builder,
        );

        split_result
    }
}

#[derive(Debug, Default)]
struct NullRelationshipsBuilder;

impl RelationshipsBuilder for NullRelationshipsBuilder {
    fn add_from_internal(&mut self, _source: u64, _target: u64, _property_value: f64) {}

    fn add_from_internal_with_properties(
        &mut self,
        _source: u64,
        _target: u64,
        _property_values: &[f64],
    ) {
    }

    fn add_from_internal_no_property(&mut self, _source: u64, _target: u64) {}
}
