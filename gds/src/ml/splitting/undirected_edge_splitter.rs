use super::edge_splitter::{
    split_positive_examples_with, BaseEdgeSplitter, EdgeSplitter, RelationshipsBuilderFactory,
};
use crate::concurrency::virtual_threads::RunWithConcurrency;
use crate::concurrency::Concurrency;
use crate::core::utils::partition::DegreeFunction;
use crate::core::utils::partition::PartitionUtils;
use crate::projection::factory::RelationshipsBuilder;
use crate::projection::RelationshipType;
use crate::types::graph::id_map::IdMap;
use crate::types::graph::Graph;
use std::collections::HashSet;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

/// Splits an undirected graph into two relationship sets:
/// 1. A holdout set represented as a directed graph
/// 2. The remaining graph that stays undirected
/// For each held out undirected edge, the holdout set gets an edge
/// with the same node pair but with random direction.
pub struct UndirectedEdgeSplitter {
    base: BaseEdgeSplitter,
    seen_relationships: Arc<parking_lot::RwLock<HashSet<(i64, i64)>>>,
}

impl UndirectedEdgeSplitter {
    /// Creates a new UndirectedEdgeSplitter
    pub fn new(
        maybe_seed: Option<u64>,
        root_nodes: Arc<dyn IdMap>,
        source_nodes: Arc<dyn IdMap>,
        target_nodes: Arc<dyn IdMap>,
        selected_relationship_type: RelationshipType,
        remaining_relationship_type: RelationshipType,
        concurrency: usize,
    ) -> Self {
        Self {
            base: BaseEdgeSplitter::new(
                maybe_seed,
                root_nodes,
                source_nodes,
                target_nodes,
                selected_relationship_type,
                remaining_relationship_type,
                concurrency,
            ),
            seen_relationships: Arc::new(parking_lot::RwLock::new(HashSet::new())),
        }
    }

    /// Records a relationship as seen
    fn mark_relationship_seen(&self, source: i64, target: i64) -> bool {
        let mut seen = self.seen_relationships.write();
        let canonical = if source < target {
            (source, target)
        } else {
            (target, source)
        };
        seen.insert(canonical)
    }

    /// Checks if a relationship has been seen
    fn has_seen_relationship(&self, source: i64, target: i64) -> bool {
        let seen = self.seen_relationships.read();
        let canonical = if source < target {
            (source, target)
        } else {
            (target, source)
        };
        seen.contains(&canonical)
    }
}

impl EdgeSplitter for UndirectedEdgeSplitter {
    fn split_positive_examples(
        &mut self,
        graph: Arc<dyn Graph>,
        holdout_fraction: f64,
        remaining_rel_property_key: Option<String>,
        builder_factory: &dyn RelationshipsBuilderFactory,
    ) -> super::edge_splitter::SplitResult {
        self.seen_relationships.write().clear();
        let selected_relationship_type = self.base.selected_relationship_type().clone();
        let remaining_relationship_type = self.base.remaining_relationship_type().clone();
        let source_nodes = self.base.source_nodes().clone();
        let target_nodes = self.base.target_nodes().clone();
        split_positive_examples_with(
            self,
            graph,
            holdout_fraction,
            remaining_rel_property_key,
            builder_factory,
            selected_relationship_type,
            remaining_relationship_type,
            source_nodes,
            target_nodes,
        )
    }

    fn sample(&mut self, probability: f64) -> bool {
        self.base.sample(probability)
    }

    fn valid_positive_relationship_candidate_count(
        &self,
        graph: &dyn Graph,
        is_valid_node_pair: Arc<dyn Fn(i64, i64) -> bool + Send + Sync>,
    ) -> usize {
        let valid_relationship_count = Arc::new(AtomicUsize::new(0));

        let node_count = graph.node_count();
        let relationship_count = graph.relationship_count();
        let degrees = {
            let graph = Graph::concurrent_copy(graph);
            Box::new(move |node_id: usize| graph.degree(node_id as i64)) as Box<dyn DegreeFunction>
        };

        // Create tasks for each partition
        let count_valid_relationship_tasks = PartitionUtils::degree_partition(
            node_count,
            relationship_count,
            degrees,
            self.base.concurrency(),
            {
                let graph = Graph::concurrent_copy(graph);
                let valid_relationship_count = valid_relationship_count.clone();
                let is_valid_node_pair = is_valid_node_pair.clone();

                move |partition| {
                    let graph = Graph::concurrent_copy(graph.as_ref());
                    let is_valid_node_pair = is_valid_node_pair.clone();
                    let valid_relationship_count = valid_relationship_count.clone();
                    Box::new(move || {
                        let mut local_count = 0;

                        for node_id in partition.as_partition().iter() {
                            let node_id = node_id as i64;
                            for cursor in
                                graph.stream_relationships(node_id, graph.default_property_value())
                            {
                                let source = cursor.source_id();
                                let target = cursor.target_id();
                                if source < target
                                    && (is_valid_node_pair(source, target)
                                        || is_valid_node_pair(target, source))
                                {
                                    local_count += 2;
                                }
                            }
                        }

                        valid_relationship_count
                            .fetch_add(local_count, std::sync::atomic::Ordering::Relaxed);
                    }) as Box<dyn FnOnce() + Send>
                }
            },
            None,
        );

        // Run tasks concurrently
        let _ = RunWithConcurrency::builder()
            .concurrency(Concurrency::of(self.base.concurrency()))
            .tasks(count_valid_relationship_tasks)
            .run();

        valid_relationship_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn positive_sampling(
        &mut self,
        graph: &dyn Graph,
        selected_rels_builder: &mut dyn RelationshipsBuilder,
        remaining_rels_builder: &mut dyn RelationshipsBuilder,
        remaining_rel_property_key: Option<&str>,
        selected_rel_count: &mut usize,
        remaining_rel_count: &mut usize,
        node_id: i64,
        is_valid_node_pair: &dyn Fn(i64, i64) -> bool,
        positive_samples_remaining: &mut usize,
        candidate_edges_remaining: &mut usize,
    ) {
        for cursor in graph.stream_relationships(node_id, graph.default_property_value()) {
            let source = cursor.source_id();
            let target = cursor.target_id();
            let weight = cursor.property();

            if source < target
                && (is_valid_node_pair(source, target) || is_valid_node_pair(target, source))
                && !self.has_seen_relationship(source, target)
            {
                self.mark_relationship_seen(source, target);

                if *candidate_edges_remaining == 0 {
                    break;
                }

                let probability = if *candidate_edges_remaining == 0 {
                    0.0
                } else {
                    *positive_samples_remaining as f64 / *candidate_edges_remaining as f64
                };

                if *positive_samples_remaining > 0 && self.sample(probability) {
                    if let Some((root_source, root_target)) = if is_valid_node_pair(source, target)
                    {
                        self.base.to_root_ids(graph, source, target)
                    } else {
                        self.base.to_root_ids(graph, target, source)
                    } {
                        selected_rels_builder.add_from_internal(
                            root_source,
                            root_target,
                            super::edge_splitter::POSITIVE,
                        );
                        *selected_rel_count += 1;
                        *positive_samples_remaining = positive_samples_remaining.saturating_sub(2);
                    }
                } else if let Some((root_source, root_target)) =
                    self.base.to_root_ids(graph, source, target)
                {
                    match remaining_rel_property_key {
                        Some(_) => {
                            remaining_rels_builder.add_from_internal(
                                root_source,
                                root_target,
                                weight,
                            );
                        }
                        None => {
                            remaining_rels_builder
                                .add_from_internal_no_property(root_source, root_target);
                        }
                    }
                    *remaining_rel_count += 1;
                }

                *candidate_edges_remaining = candidate_edges_remaining.saturating_sub(2);
            }
        }
    }
}
