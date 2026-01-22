use std::sync::Arc;

use crate::algo::similarity::knn::metrics::SimilarityComputer;
use crate::ml::models::Classifier;
use crate::projection::eval::pipeline::link_pipeline::train::POSITIVE;
use crate::projection::eval::pipeline::link_pipeline::LinkFeatureExtractor;
use crate::types::graph::Graph;

use super::lp_node_filter::LPNodeFilter;

pub struct LinkPredictionSimilarityComputer<'a> {
    link_feature_extractor: &'a LinkFeatureExtractor,
    classifier: &'a dyn Classifier,
}

impl<'a> LinkPredictionSimilarityComputer<'a> {
    pub fn new(
        link_feature_extractor: &'a LinkFeatureExtractor,
        classifier: &'a dyn Classifier,
    ) -> Self {
        Self {
            link_feature_extractor,
            classifier,
        }
    }
}

impl<'a> SimilarityComputer for LinkPredictionSimilarityComputer<'a> {
    fn similarity(&self, first_node_id: u64, second_node_id: u64) -> f64 {
        let features = self
            .link_feature_extractor
            .extract_features_for_pair(first_node_id, second_node_id);
        let probabilities = self.classifier.predict_probabilities(&features);
        probabilities.get(POSITIVE as usize).copied().unwrap_or(0.0)
    }

    fn is_symmetric(&self) -> bool {
        self.link_feature_extractor.is_symmetric()
    }
}

pub trait NeighborFilter: Send + Sync {
    fn exclude_node_pair(&self, first_node_id: i64, second_node_id: i64) -> bool;
    fn lower_bound_of_potential_neighbors(&self, node_id: i64) -> i64;
}

pub trait NeighborFilterFactory: Send + Sync {
    fn create(&self) -> Box<dyn NeighborFilter>;
}

pub struct LinkFilter {
    graph: Arc<dyn Graph>,
    source_node_filter: Arc<dyn LPNodeFilter>,
    target_node_filter: Arc<dyn LPNodeFilter>,
}

impl LinkFilter {
    pub fn new(
        graph: Arc<dyn Graph>,
        source_node_filter: Arc<dyn LPNodeFilter>,
        target_node_filter: Arc<dyn LPNodeFilter>,
    ) -> Self {
        Self {
            graph,
            source_node_filter,
            target_node_filter,
        }
    }
}

impl NeighborFilter for LinkFilter {
    fn exclude_node_pair(&self, first_node_id: i64, second_node_id: i64) -> bool {
        if first_node_id == second_node_id {
            return true;
        }

        let matches_filter = (self.source_node_filter.test(first_node_id)
            && self.target_node_filter.test(second_node_id))
            || (self.source_node_filter.test(second_node_id)
                && self.target_node_filter.test(first_node_id));

        !matches_filter || self.graph.exists(first_node_id, second_node_id)
    }

    fn lower_bound_of_potential_neighbors(&self, node_id: i64) -> i64 {
        let degree = self.graph.degree(node_id) as i64;
        if self.source_node_filter.test(node_id) {
            (self.target_node_filter.valid_node_count() - 1 - degree).max(0)
        } else {
            (self.source_node_filter.valid_node_count() - 1 - degree).max(0)
        }
    }
}

pub struct LinkFilterFactory {
    graph: Arc<dyn Graph>,
    source_node_filter: Arc<dyn LPNodeFilter>,
    target_node_filter: Arc<dyn LPNodeFilter>,
}

impl LinkFilterFactory {
    pub fn new(
        graph: Arc<dyn Graph>,
        source_node_filter: Arc<dyn LPNodeFilter>,
        target_node_filter: Arc<dyn LPNodeFilter>,
    ) -> Self {
        Self {
            graph,
            source_node_filter,
            target_node_filter,
        }
    }
}

impl NeighborFilterFactory for LinkFilterFactory {
    fn create(&self) -> Box<dyn NeighborFilter> {
        Box::new(LinkFilter::new(
            Graph::concurrent_copy(self.graph.as_ref()),
            Arc::clone(&self.source_node_filter),
            Arc::clone(&self.target_node_filter),
        ))
    }
}
