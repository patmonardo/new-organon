//! TopologicalSort Computation
//!
//! **Translation Source**: `org.neo4j.gds.dag.topologicalsort.TopologicalSort`
//!
//! Implements Kahn's algorithm for topological sorting with optional longest path calculation.

use super::spec::TopologicalSortResult;
use super::TopologicalSortStorageRuntime;
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use std::collections::VecDeque;
use std::sync::atomic::Ordering;

pub struct TopologicalSortComputationRuntime {
    storage: TopologicalSortStorageRuntime,
}

impl TopologicalSortComputationRuntime {
    pub fn new(node_count: usize, compute_max_distance: bool) -> Self {
        Self {
            storage: TopologicalSortStorageRuntime::new(node_count, compute_max_distance),
        }
    }

    /// Compute topological sort
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(NodeId) -> Vec<(NodeId, f64)>, // (neighbor, weight)
    ) -> TopologicalSortResult {
        // Phase 1: Initialize in-degrees
        for node_id in 0..(node_count as i64) {
            for (target, _) in get_neighbors(node_id) {
                self.storage.in_degrees[target as usize].fetch_add(1, Ordering::SeqCst);
            }
        }

        // Phase 2: Traverse from nodes with in-degree 0
        let mut queue: VecDeque<NodeId> = VecDeque::new();

        // Find all nodes with in-degree 0
        for node_id in 0..(node_count as i64) {
            if self.storage.in_degrees[node_id as usize].load(Ordering::SeqCst) == 0 {
                queue.push_back(node_id);
                // Initialize distance for source nodes
                if let Some(ref distances) = self.storage.max_source_distances {
                    distances[node_id as usize].store(0.0_f64.to_bits() as i64, Ordering::SeqCst);
                }
            }
        }

        // Process nodes in topological order
        while let Some(source) = queue.pop_front() {
            self.storage.add_node(source);

            let source_distance = if let Some(ref distances) = self.storage.max_source_distances {
                f64::from_bits(distances[source as usize].load(Ordering::SeqCst) as u64)
            } else {
                0.0
            };

            for (target, weight) in get_neighbors(source) {
                // Update longest path distance if computing
                if let Some(ref distances) = self.storage.max_source_distances {
                    loop {
                        let current_bits = distances[target as usize].load(Ordering::SeqCst);
                        let current = f64::from_bits(current_bits as u64);
                        let new_distance = source_distance + weight;

                        if new_distance > current {
                            let new_bits = new_distance.to_bits() as i64;
                            if distances[target as usize]
                                .compare_exchange(
                                    current_bits,
                                    new_bits,
                                    Ordering::SeqCst,
                                    Ordering::SeqCst,
                                )
                                .is_ok()
                            {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }

                // Decrement in-degree
                let prev_in_degree =
                    self.storage.in_degrees[target as usize].fetch_sub(1, Ordering::SeqCst);

                // If in-degree becomes 0, add to queue
                if prev_in_degree == 1 {
                    queue.push_back(target);
                }
            }
        }

        // Build result
        let size = self.storage.size();
        let mut sorted_nodes = Vec::with_capacity(size);

        for i in 0..size {
            let node = self.storage.sorted_nodes[i].load(Ordering::SeqCst);
            if node != -1 {
                sorted_nodes.push(node);
            }
        }

        let max_source_distances = self.storage.max_source_distances.as_ref().map(|distances| {
            (0..node_count)
                .map(|i| f64::from_bits(distances[i].load(Ordering::SeqCst) as u64))
                .collect()
        });

        TopologicalSortResult {
            sorted_nodes,
            max_source_distances,
        }
    }

    /// Compute topological sort using a graph for neighbor access
    pub fn compute_with_graph(&mut self, graph: &dyn Graph) -> TopologicalSortResult {
        let node_count = graph.node_count() as usize;
        let fallback = graph.default_property_value();

        let get_neighbors = |node_idx: NodeId| -> Vec<(NodeId, f64)> {
            graph
                .stream_relationships(node_idx, fallback)
                .filter_map(|cursor| {
                    let target = cursor.target_id();
                    if target < 0 {
                        return None;
                    }
                    let weight = cursor.property();
                    Some((target, weight))
                })
                .collect()
        };

        self.compute(node_count, get_neighbors)
    }
}
