use super::spec::{ConductanceConfig, ConductanceResult};
use crate::concurrency::virtual_threads::RunWithConcurrency;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::partition::{DegreeFunction, DegreePartition, PartitionUtils};
use crate::types::graph::Graph;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[derive(Default)]
pub struct ConductanceComputationRuntime {}

impl ConductanceComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn count_relationships(
        &mut self,
        graph: Arc<dyn Graph>,
        community_properties: Arc<dyn NodePropertyValues>,
        config: &ConductanceConfig,
        termination_flag: &TerminationFlag,
    ) -> Result<Vec<LocalCounts>, String> {
        let node_count = graph.node_count();
        if node_count == 0 {
            return Ok(Vec::new());
        }

        let concurrency = Concurrency::of(config.concurrency.max(1));

        // Java parity: Count relationships using degree-aware partitioning with the requested concurrency.
        let partition_concurrency = concurrency.value().min(node_count).max(1);
        let degree_partitions: Vec<DegreePartition> = PartitionUtils::degree_partition(
            node_count,
            graph.relationship_count(),
            Box::new(GraphDegrees {
                graph: Arc::clone(&graph),
            }),
            partition_concurrency,
            |p| p,
            Some(config.min_batch_size.max(1)),
        );

        let slots: Arc<Vec<parking_lot::Mutex<Option<LocalCounts>>>> = Arc::new(
            (0..degree_partitions.len())
                .map(|_| parking_lot::Mutex::new(None))
                .collect(),
        );

        let has_weights = config.has_relationship_weight_property;
        let fallback = graph.default_property_value();

        let tasks: Vec<Box<dyn FnOnce() + Send>> = degree_partitions
            .into_iter()
            .enumerate()
            .map(|(task_index, partition)| {
                let graph_copy = Graph::concurrent_copy(graph.as_ref());
                let community_properties = Arc::clone(&community_properties);
                let slots = Arc::clone(&slots);
                let termination_flag = termination_flag.clone();

                Box::new(move || {
                    let mut local_internal: HashMap<u64, f64> = HashMap::new();
                    let mut local_external: HashMap<u64, f64> = HashMap::new();

                    for node in partition.as_partition().iter() {
                        if !termination_flag.running() {
                            break;
                        }

                        let Ok(source_community_raw) = community_properties.long_value(node as u64)
                        else {
                            continue;
                        };
                        if source_community_raw < 0 {
                            continue;
                        }
                        let source_community = source_community_raw as u64;

                        // Java parity: once a community is seen, both counts exist (0.0 allowed).
                        local_internal.entry(source_community).or_insert(0.0);
                        local_external.entry(source_community).or_insert(0.0);

                        for cursor in graph_copy.stream_relationships(node as i64, fallback) {
                            if !termination_flag.running() {
                                break;
                            }

                            let target = cursor.target_id();
                            if target < 0 {
                                continue;
                            }
                            let target_idx = target as usize;

                            let Ok(target_community_raw) =
                                community_properties.long_value(target_idx as u64)
                            else {
                                continue;
                            };
                            if target_community_raw < 0 {
                                continue;
                            }
                            let target_community = target_community_raw as u64;

                            let weight = if has_weights { cursor.property() } else { 1.0 };

                            if source_community == target_community {
                                *local_internal.entry(source_community).or_insert(0.0) += weight;
                            } else {
                                *local_external.entry(source_community).or_insert(0.0) += weight;
                            }
                        }
                    }

                    *slots[task_index].lock() = Some(LocalCounts {
                        internal: local_internal,
                        external: local_external,
                    });
                }) as Box<dyn FnOnce() + Send>
            })
            .collect();

        if !tasks.is_empty() {
            RunWithConcurrency::builder()
                .concurrency(concurrency)
                .tasks(tasks)
                .termination_flag(termination_flag.clone())
                .run()?;
        }

        // Collect local maps from slots in a deterministic order.
        let mut out = Vec::with_capacity(slots.len());
        for slot in slots.iter() {
            if let Some(local) = slot.lock().take() {
                out.push(local);
            }
        }
        Ok(out)
    }

    pub fn accumulate_counts(&mut self, locals: Vec<LocalCounts>) -> RelationshipCounts {
        let mut internal_counts: HashMap<u64, f64> = HashMap::new();
        let mut external_counts: HashMap<u64, f64> = HashMap::new();

        for local in locals {
            for (k, v) in local.internal {
                *internal_counts.entry(k).or_insert(0.0) += v;
            }
            for (k, v) in local.external {
                *external_counts.entry(k).or_insert(0.0) += v;
            }
        }

        RelationshipCounts {
            internal: internal_counts,
            external: external_counts,
        }
    }

    pub fn compute_conductances(&mut self, counts: RelationshipCounts) -> ConductanceResult {
        let mut community_conductances: HashMap<u64, f64> = HashMap::new();
        let mut sum = 0.0f64;
        let mut count = 0usize;

        for (community, external) in counts.external.iter() {
            let Some(internal) = counts.internal.get(community) else {
                continue;
            };

            let denom = external + internal;
            if denom <= 0.0 {
                continue;
            }

            let conductance = external / denom;

            community_conductances.insert(*community, conductance);
            sum += conductance;
            count += 1;
        }

        let global_average_conductance = if count == 0 { 0.0 } else { sum / count as f64 };

        ConductanceResult {
            community_conductances,
            global_average_conductance,
            community_count: count,
            node_count: 0,
            execution_time: Duration::default(),
        }
    }
}

struct GraphDegrees {
    graph: Arc<dyn Graph>,
}

impl DegreeFunction for GraphDegrees {
    fn degree(&self, node: usize) -> usize {
        self.graph.degree(node as i64)
    }
}

pub struct LocalCounts {
    internal: HashMap<u64, f64>,
    external: HashMap<u64, f64>,
}

pub struct RelationshipCounts {
    internal: HashMap<u64, f64>,
    external: HashMap<u64, f64>,
}
