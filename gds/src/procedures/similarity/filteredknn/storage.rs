use super::computation::{FilteredKnnComputationResult, FilteredKnnComputationRuntime};
use crate::procedures::similarity::knn::metrics::{
    KnnNodePropertySpec, SimilarityComputer, SimilarityMetric,
};
use crate::projection::eval::procedure::AlgorithmError;
use crate::projection::NodeLabel;
use crate::types::graph_store::GraphStore;
use std::collections::HashSet;
use std::sync::Arc;

pub struct FilteredKnnStorageRuntime {
    _concurrency: usize,
}

impl FilteredKnnStorageRuntime {
    pub fn new(concurrency: usize) -> Self {
        Self {
            _concurrency: concurrency,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_single(
        &self,
        computation: &FilteredKnnComputationRuntime,
        graph_store: &impl GraphStore,
        node_property: &str,
        k: usize,
        similarity_cutoff: f64,
        metric: SimilarityMetric,
        source_node_labels: &[NodeLabel],
        target_node_labels: &[NodeLabel],
    ) -> Result<Vec<FilteredKnnComputationResult>, AlgorithmError> {
        let values = graph_store
            .node_property_values(node_property)
            .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

        let similarity =
            <dyn SimilarityComputer>::of_property_values(node_property, values, metric)
                .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

        let (source_allowed, target_allowed) =
            Self::build_filters(graph_store, source_node_labels, target_node_labels)?;

        Ok(computation.compute(
            graph_store.node_count(),
            k,
            similarity_cutoff,
            similarity,
            source_allowed.as_deref(),
            target_allowed.as_deref(),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn compute_multi(
        &self,
        computation: &FilteredKnnComputationRuntime,
        graph_store: &impl GraphStore,
        node_properties: &[KnnNodePropertySpec],
        k: usize,
        similarity_cutoff: f64,
        source_node_labels: &[NodeLabel],
        target_node_labels: &[NodeLabel],
    ) -> Result<Vec<FilteredKnnComputationResult>, AlgorithmError> {
        if node_properties.is_empty() {
            return Err(AlgorithmError::InvalidGraph(
                "Missing `node_properties`".to_string(),
            ));
        }

        let mut props: Vec<(
            String,
            Arc<dyn crate::types::properties::node::NodePropertyValues>,
            SimilarityMetric,
        )> = Vec::with_capacity(node_properties.len());

        for spec in node_properties {
            let name = spec.name.trim();
            if name.is_empty() {
                return Err(AlgorithmError::InvalidGraph(
                    "`node_properties` contains an empty property name".to_string(),
                ));
            }
            let values = graph_store
                .node_property_values(name)
                .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;
            props.push((name.to_string(), values, spec.metric));
        }

        let similarity = <dyn SimilarityComputer>::of_properties(props)
            .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?;

        let (source_allowed, target_allowed) =
            Self::build_filters(graph_store, source_node_labels, target_node_labels)?;

        Ok(computation.compute(
            graph_store.node_count(),
            k,
            similarity_cutoff,
            similarity,
            source_allowed.as_deref(),
            target_allowed.as_deref(),
        ))
    }

    #[allow(clippy::type_complexity)]
    fn build_filters(
        graph_store: &impl GraphStore,
        source_node_labels: &[NodeLabel],
        target_node_labels: &[NodeLabel],
    ) -> Result<(Option<Vec<bool>>, Option<Vec<bool>>), AlgorithmError> {
        let source = Self::build_label_filter(graph_store, source_node_labels)?;
        let target = Self::build_label_filter(graph_store, target_node_labels)?;
        Ok((source, target))
    }

    fn build_label_filter(
        graph_store: &impl GraphStore,
        labels: &[NodeLabel],
    ) -> Result<Option<Vec<bool>>, AlgorithmError> {
        if labels.is_empty() || labels.iter().any(|l| l.is_all_nodes()) {
            return Ok(None);
        }

        for label in labels {
            if label.is_all_nodes() {
                continue;
            }
            if !graph_store.has_node_label(label) {
                return Err(AlgorithmError::InvalidGraph(format!(
                    "Unknown node label `{}`",
                    label.name()
                )));
            }
        }

        let node_count = graph_store.node_count();
        let id_map = graph_store.nodes();
        let label_set: HashSet<NodeLabel> = labels.iter().cloned().collect();

        let mut allowed = vec![false; node_count];
        for (mapped, allowed_slot) in allowed.iter_mut().enumerate().take(node_count) {
            let mapped_i64 = mapped as i64;
            let mut ok = false;
            for label in &label_set {
                if id_map.has_label(mapped_i64, label) {
                    ok = true;
                    break;
                }
            }
            *allowed_slot = ok;
        }

        Ok(Some(allowed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::prelude::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    #[test]
    fn compute_single_respects_source_and_target_label_filters() {
        let config = RandomGraphConfig {
            node_count: 24,
            node_labels: vec!["A".to_string(), "B".to_string()],
            seed: Some(7),
            ..RandomGraphConfig::default()
        };
        let store = DefaultGraphStore::random(&config).unwrap();

        let node_count = store.node_count();
        let id_map = store.nodes();
        let a = NodeLabel::of("A");
        let b = NodeLabel::of("B");

        let a_count = (0..node_count)
            .filter(|&i| id_map.has_label(i as i64, &a))
            .count();
        let b_count = (0..node_count)
            .filter(|&i| id_map.has_label(i as i64, &b))
            .count();
        assert!(a_count > 0, "seeded graph should contain label A");
        assert!(b_count > 0, "seeded graph should contain label B");

        let runtime = FilteredKnnStorageRuntime::new(4);
        let computation = FilteredKnnComputationRuntime::new();

        let rows = runtime
            .compute_single(
                &computation,
                &store,
                "random_score",
                3,
                0.0,
                SimilarityMetric::Default,
                &[a.clone()],
                &[b.clone()],
            )
            .unwrap();

        for row in &rows {
            assert!(
                id_map.has_label(row.source as i64, &a),
                "source {} must have label A",
                row.source
            );
            assert!(
                id_map.has_label(row.target as i64, &b),
                "target {} must have label B",
                row.target
            );
        }
    }
}
