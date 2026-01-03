use crate::algo::similarity::{NodeSimilarityConfig, NodeSimilarityMetric, NodeSimilarityResult};
use crate::core::utils::progress::{ProgressTracker, Tasks};
use crate::procedures::builder_base::ConfigValidator;
use crate::procedures::traits::Result;
use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::orientation::Orientation;
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteredNodeSimilarityStats {
    #[serde(rename = "nodesCompared")]
    pub nodes_compared: u64,
    #[serde(rename = "similarityPairs")]
    pub similarity_pairs: u64,
    #[serde(rename = "similarityDistribution")]
    pub similarity_distribution: HashMap<String, f64>,
    #[serde(rename = "computeMillis")]
    pub compute_millis: u64,
    pub success: bool,
}

pub struct FilteredNodeSimilarityBuilder {
    graph_store: Arc<DefaultGraphStore>,
    metric: NodeSimilarityMetric,
    similarity_cutoff: f64,
    top_k: usize,
    top_n: usize,
    concurrency: usize,
    weight_property: Option<String>,
    source_node_label: Option<NodeLabel>,
    target_node_label: Option<NodeLabel>,
}

impl FilteredNodeSimilarityBuilder {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            metric: NodeSimilarityMetric::Jaccard,
            similarity_cutoff: 0.1,
            top_k: 10,
            top_n: 0,
            concurrency: 4,
            weight_property: None,
            source_node_label: None,
            target_node_label: None,
        }
    }

    pub fn metric(mut self, metric: NodeSimilarityMetric) -> Self {
        self.metric = metric;
        self
    }

    pub fn similarity_cutoff(mut self, cutoff: f64) -> Self {
        self.similarity_cutoff = cutoff;
        self
    }

    pub fn top_k(mut self, k: usize) -> Self {
        self.top_k = k;
        self
    }

    pub fn top_n(mut self, n: usize) -> Self {
        self.top_n = n;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn weight_property(mut self, property: String) -> Self {
        self.weight_property = Some(property);
        self
    }

    pub fn source_node_label(mut self, label: NodeLabel) -> Self {
        self.source_node_label = Some(label);
        self
    }

    pub fn target_node_label(mut self, label: NodeLabel) -> Self {
        self.target_node_label = Some(label);
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.similarity_cutoff, 0.0, 1.0, "similarity_cutoff")?;
        ConfigValidator::in_range(self.top_k as f64, 1.0, 1_000_000.0, "top_k")?;
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        if let Some(prop) = &self.weight_property {
            ConfigValidator::non_empty_string(prop, "weight_property")?;
        }

        if let Some(label) = &self.source_node_label {
            if !GraphStore::has_node_label(self.graph_store.as_ref(), label) {
                return Err(AlgorithmError::Execution(format!(
                    "Unknown sourceNodeLabel '{}'",
                    label.name()
                )));
            }
        }

        if let Some(label) = &self.target_node_label {
            if !GraphStore::has_node_label(self.graph_store.as_ref(), label) {
                return Err(AlgorithmError::Execution(format!(
                    "Unknown targetNodeLabel '{}'",
                    label.name()
                )));
            }
        }

        Ok(())
    }

    fn build_config(&self) -> NodeSimilarityConfig {
        NodeSimilarityConfig {
            similarity_metric: self.metric,
            similarity_cutoff: self.similarity_cutoff,
            top_k: self.top_k,
            top_n: self.top_n,
            concurrency: self.concurrency,
            weight_property: self.weight_property.clone(),
        }
    }

    fn compute_results(&self) -> Result<Vec<NodeSimilarityResult>> {
        self.validate()?;

        let rel_types = HashSet::<RelationshipType>::new();
        let graph = self
            .graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

        let mut progress_tracker = ProgressTracker::with_concurrency(
            Tasks::leaf("filtered_node_similarity", graph.node_count()),
            self.concurrency,
        );
        progress_tracker.begin_subtask(graph.node_count());

        let id_map = GraphStore::nodes(self.graph_store.as_ref());

        let mut source_nodes: Option<HashSet<crate::types::graph::id_map::MappedNodeId>> =
            self.source_node_label.as_ref().map(|label| {
                let mut labels = HashSet::new();
                labels.insert(label.clone());
                id_map.iter_with_labels(&labels).collect()
            });

        let mut target_nodes: Option<HashSet<crate::types::graph::id_map::MappedNodeId>> =
            self.target_node_label.as_ref().map(|label| {
                let mut labels = HashSet::new();
                labels.insert(label.clone());
                id_map.iter_with_labels(&labels).collect()
            });

        if source_nodes.as_ref().is_some_and(|nodes| nodes.is_empty()) {
            return Err(AlgorithmError::Execution(
                "sourceNodeLabel selection is empty".to_string(),
            ));
        }
        if target_nodes.as_ref().is_some_and(|nodes| nodes.is_empty()) {
            return Err(AlgorithmError::Execution(
                "targetNodeLabel selection is empty".to_string(),
            ));
        }

        if source_nodes.is_none() {
            // If only target label is provided, treat it as both source+target filter.
            source_nodes = target_nodes.clone();
        }
        if target_nodes.is_none() {
            // If only source label is provided, treat it as both source+target filter.
            target_nodes = source_nodes.clone();
        }

        let config = self.build_config();
        let results = crate::algo::similarity::filterednodesim::compute_filtered_node_similarity(
            graph.as_ref(),
            &config,
            source_nodes.as_ref(),
            target_nodes.as_ref(),
        );

        progress_tracker.log_progress(graph.node_count());
        progress_tracker.end_subtask();

        Ok(results)
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = NodeSimilarityResult>>> {
        let results = self.compute_results()?;
        Ok(Box::new(results.into_iter()))
    }

    pub fn stats(self) -> Result<FilteredNodeSimilarityStats> {
        let results = self.compute_results()?;

        let mut sources = HashSet::new();
        let tuples: Vec<(u64, u64, f64)> = results
            .iter()
            .map(|r| {
                sources.insert(r.source);
                (r.source, r.target, r.similarity)
            })
            .collect();

        let stats = crate::algo::core::result::similarity::similarity_stats(
            || tuples.into_iter(),
            true,
        );

        Ok(FilteredNodeSimilarityStats {
            nodes_compared: sources.len() as u64,
            similarity_pairs: results.len() as u64,
            similarity_distribution: stats.summary(),
            compute_millis: stats.compute_millis,
            success: stats.success,
        })
    }

    pub fn mutate(self, property: &str) -> Result<()> {
        self.validate()?;
        ConfigValidator::non_empty_string(property, "property_name")?;

        Err(AlgorithmError::Execution(
            "Filtered Node Similarity mutate/write is not implemented yet".to_string(),
        ))
    }

    pub fn write(self, property: &str) -> Result<()> {
        self.validate()?;
        ConfigValidator::non_empty_string(property, "property_name")?;

        Err(AlgorithmError::Execution(
            "Filtered Node Similarity mutate/write is not implemented yet".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::GraphStoreConfig;
    use crate::types::graph::id_map::IdMap;
    use crate::types::graph::id_map::SimpleIdMap;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph_store::{Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation};
    use crate::types::graph_store::GraphName;
    use crate::types::schema::{Direction, GraphSchema, MutableGraphSchema};

    #[test]
    fn filtered_node_similarity_filters_to_label_pair_space() {
        let cfg = GraphStoreConfig::default();
        let graph_name = GraphName::new("g");
        let db_info = DatabaseInfo::new(
            DatabaseId::new("test-db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        );
        let capabilities = Capabilities::default();

        let label_a = NodeLabel::of("A");
        let label_b = NodeLabel::of("B");
        let rel_type = RelationshipType::of("R");

        let mut schema = MutableGraphSchema::empty();
        schema.node_schema_mut().add_label(label_a.clone());
        schema.node_schema_mut().add_label(label_b.clone());
        schema
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema: GraphSchema = schema.build();

        let mut id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);
        id_map.add_node_label(label_a.clone());
        id_map.add_node_label(label_b.clone());
        id_map.add_node_id_to_label(0, label_a.clone());
        id_map.add_node_id_to_label(1, label_a.clone());
        id_map.add_node_id_to_label(2, label_b.clone());
        id_map.add_node_id_to_label(3, label_b.clone());

        // Bipartite-ish: 0->2 and 1->2 gives 0 and 1 identical neighborhoods.
        let outgoing = vec![vec![2], vec![2], vec![], vec![]];
        let incoming = vec![vec![], vec![], vec![0, 1], vec![]];
        let topo = RelationshipTopology::new(outgoing, Some(incoming));
        let mut topologies = std::collections::HashMap::new();
        topologies.insert(rel_type, topo);

        let store = Arc::new(DefaultGraphStore::new(
            cfg,
            graph_name,
            db_info,
            schema,
            capabilities,
            id_map,
            topologies,
        ));

        let results: Vec<_> = FilteredNodeSimilarityBuilder::new(Arc::clone(&store))
            .source_node_label(label_a.clone())
            .target_node_label(label_a)
            .similarity_cutoff(0.0)
            .top_k(10)
            .concurrency(1)
            .stream()
            .unwrap()
            .collect();

        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.source < 2 && r.target < 2));
        assert!(results.iter().any(|r| (r.source, r.target) == (0, 1) || (r.source, r.target) == (1, 0)));
    }
}
