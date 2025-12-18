//! DagLongestPath Computation
//!
//! **Translation Source**: `org.neo4j.gds.dag.longestPath.DagLongestPath`
//!
//! Finds longest paths in a DAG using topological ordering and dynamic programming.

use super::spec::{DagLongestPathResult, PathRow};
use super::storage::DagLongestPathStorageRuntime;
use std::collections::VecDeque;
use std::sync::atomic::Ordering;

pub struct DagLongestPathComputationRuntime {
    storage: DagLongestPathStorageRuntime,
}

impl DagLongestPathComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            storage: DagLongestPathStorageRuntime::new(node_count),
        }
    }

    /// Compute longest paths in DAG
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<(usize, f64)>, // (neighbor, weight)
    ) -> DagLongestPathResult {
        // Phase 1: Initialize in-degrees
        for node_id in 0..node_count {
            for (target, _) in get_neighbors(node_id) {
                self.storage.in_degrees[target].fetch_add(1, Ordering::SeqCst);
            }
        }

        // Phase 2: Initialize source nodes (in-degree 0) with distance 0
        let mut queue: VecDeque<usize> = VecDeque::new();
        for node_id in 0..node_count {
            if self.storage.in_degrees[node_id].load(Ordering::SeqCst) == 0 {
                queue.push_back(node_id);
                self.storage.set_distance(node_id, 0.0);
            }
        }

        // Phase 3: Traverse in topological order, updating longest paths
        while let Some(source) = queue.pop_front() {
            let source_distance = self.storage.get_distance(source);

            // Skip if this node is unreachable
            if source_distance.is_infinite() && source_distance.is_sign_negative() {
                continue;
            }

            for (target, weight) in get_neighbors(source) {
                let new_distance = source_distance + weight;

                // Update if we found a longer path
                if self
                    .storage
                    .compare_and_update_distance(target, new_distance)
                {
                    self.storage.set_predecessor(target, source);
                }

                // Decrement in-degree and add to queue if ready
                let prev_in_degree = self.storage.in_degrees[target].fetch_sub(1, Ordering::SeqCst);
                if prev_in_degree == 1 {
                    queue.push_back(target);
                }
            }
        }

        // Phase 4: Build path results
        self.build_paths(node_count)
    }

    fn build_paths(&self, node_count: usize) -> DagLongestPathResult {
        let mut paths = Vec::new();
        let mut path_index = 0u64;

        for target_node in 0..node_count {
            let distance = self.storage.get_distance(target_node);

            // Skip unreachable nodes
            if distance.is_infinite() && distance.is_sign_negative() {
                continue;
            }

            // Backtrack to build path
            let mut node_ids = Vec::new();
            let mut costs = Vec::new();
            let mut current = target_node;

            // Walk back through predecessors
            loop {
                node_ids.push(current as u64);
                costs.push(self.storage.get_distance(current));

                match self.storage.get_predecessor(current) {
                    Some(pred) => current = pred,
                    None => break,
                }
            }

            // Reverse to get path from source to target
            node_ids.reverse();
            costs.reverse();

            let source_node = node_ids[0];

            paths.push(PathRow {
                index: path_index,
                source_node,
                target_node: target_node as u64,
                total_cost: distance,
                node_ids,
                costs,
            });

            path_index += 1;
        }

        DagLongestPathResult { paths }
    }
}
