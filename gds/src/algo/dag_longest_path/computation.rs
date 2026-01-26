//! DagLongestPath Computation
//!
//! **Translation Source**: `org.neo4j.gds.dag.longestPath.DagLongestPath`
//!
//! Finds longest paths in a DAG using parallel topological traversal with ForkJoin-like tasks.

use super::spec::{DagLongestPathResult, PathRow};
use super::DagLongestPathStorageRuntime;
use crate::types::graph::NodeId;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

/// Parallel task for longest path computation
struct LongestPathTask {
    source_id: NodeId,
    get_neighbors: Arc<dyn Fn(NodeId) -> Vec<(NodeId, f64)> + Send + Sync>,
    storage: Arc<DagLongestPathStorageRuntime>,
    pending_tasks: Arc<AtomicUsize>,
}

impl LongestPathTask {
    fn new(
        source_id: NodeId,
        get_neighbors: Arc<dyn Fn(NodeId) -> Vec<(NodeId, f64)> + Send + Sync>,
        storage: Arc<DagLongestPathStorageRuntime>,
        pending_tasks: Arc<AtomicUsize>,
    ) -> Self {
        Self {
            source_id,
            get_neighbors,
            storage,
            pending_tasks,
        }
    }

    fn execute(self) {
        // Process all relationships from this source node
        for (target, weight) in (self.get_neighbors)(self.source_id) {
            self.longest_path_traverse(self.source_id, target, weight);

            // Decrement in-degree
            let prev_degree =
                self.storage.in_degrees[target as usize].fetch_sub(1, Ordering::SeqCst);

            // If this was the last incoming edge, fork a new task
            if prev_degree == 1_usize {
                self.pending_tasks.fetch_add(1, Ordering::SeqCst);

                let new_task = LongestPathTask::new(
                    target,
                    Arc::clone(&self.get_neighbors)
                        as Arc<dyn Fn(NodeId) -> Vec<(NodeId, f64)> + Send + Sync>,
                    Arc::clone(&self.storage),
                    Arc::clone(&self.pending_tasks),
                );

                // Execute synchronously to maintain correctness.
                new_task.execute();
            }
        }

        // Decrement pending tasks
        self.pending_tasks.fetch_sub(1, Ordering::SeqCst);
    }

    fn longest_path_traverse(&self, source: NodeId, target: NodeId, weight: f64) {
        let source_distance = self.storage.get_distance(source as usize);
        let potential_distance = source_distance + weight;

        // Try to update the target distance if we found a longer path
        self.storage.compare_and_update_distance(
            target as usize,
            potential_distance,
            source as usize,
        );
    }
}

pub struct DagLongestPathComputationRuntime {
    storage: Arc<DagLongestPathStorageRuntime>,
}

impl DagLongestPathComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            storage: Arc::new(DagLongestPathStorageRuntime::new(node_count)),
        }
    }

    /// Compute longest paths in DAG using parallel topological traversal
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(NodeId) -> Vec<(NodeId, f64)> + Send + Sync + 'static,
    ) -> DagLongestPathResult {
        // Reset storage each run so repeated invocations on the same runtime do not carry
        // over in-degree, distance, or predecessor state from prior graphs.
        self.storage = Arc::new(DagLongestPathStorageRuntime::new(node_count));

        let get_neighbors = Arc::new(get_neighbors);

        // Phase 1: Initialize in-degrees
        for node_id in 0..(node_count as i64) {
            for (target, _) in get_neighbors(node_id) {
                self.storage.in_degrees[target as usize].fetch_add(1, Ordering::SeqCst);
            }
        }

        #[cfg(test)]
        {
            // Debug: expose in-degrees during tests to help track intermittent failures
            let indeg: Vec<usize> = self
                .storage
                .in_degrees
                .iter()
                .map(|a| a.load(Ordering::SeqCst))
                .collect();
            eprintln!(
                "[dag_longest_path debug] in_degrees after init: {:?}",
                indeg
            );
        }

        // Phase 2: Create tasks for source nodes (in-degree 0)
        // Collect source nodes first to avoid race where worker threads decrement
        // in-degrees before the main thread has inspected all nodes.
        let pending_tasks = Arc::new(AtomicUsize::new(0));
        let mut task_handles = Vec::new();
        let mut source_nodes: Vec<i64> = Vec::new();

        for node_id in 0..(node_count as i64) {
            if self.storage.in_degrees[node_id as usize].load(Ordering::SeqCst) == 0_usize {
                source_nodes.push(node_id);
            }
        }

        for node_id in source_nodes {
            // Initialize source node distance
            self.storage
                .set_distance_tag(node_id as usize, 0.0, "source_init");
            self.storage
                .set_predecessor_tag(node_id as usize, node_id as usize, "source_init");

            // Create and spawn task
            pending_tasks.fetch_add(1, Ordering::SeqCst);
            let task = LongestPathTask::new(
                node_id,
                Arc::clone(&get_neighbors)
                    as Arc<dyn Fn(NodeId) -> Vec<(NodeId, f64)> + Send + Sync>,
                Arc::clone(&self.storage),
                Arc::clone(&pending_tasks),
            );

            let handle = thread::spawn(move || {
                task.execute();
            });
            task_handles.push(handle);
        }

        // Wait for all tasks to complete
        for handle in task_handles {
            let _ = handle.join();
        }

        // Wait for all pending tasks to complete
        while pending_tasks.load(Ordering::SeqCst) > 0 {
            thread::yield_now();
        }

        // Phase 3: Build path results
        self.build_paths(node_count)
    }

    fn build_paths(&self, node_count: usize) -> DagLongestPathResult {
        let mut paths = Vec::new();
        let mut path_index = 0u64;

        for target_node in 0..node_count {
            let distance = self.storage.get_distance(target_node);

            // Skip unreachable nodes (still have -infinity)
            if distance.is_infinite() && distance.is_sign_negative() {
                continue;
            }

            // Backtrack to build path
            let mut node_ids = Vec::new();
            let mut costs = Vec::new();
            let mut current = target_node;

            // Walk back through predecessors until we reach a source node
            loop {
                node_ids.push(current as NodeId);
                costs.push(self.storage.get_distance(current));

                match self.storage.get_predecessor(current) {
                    Some(pred) if pred != current => current = pred,
                    _ => break, // Reached a source node (predecessor == self)
                }
            }

            // Reverse to get path from source to target
            node_ids.reverse();
            costs.reverse();

            let source_node = node_ids[0];

            paths.push(PathRow {
                index: path_index,
                source_node,
                target_node: target_node as NodeId,
                total_cost: distance,
                node_ids,
                costs,
            });

            path_index += 1;
        }

        DagLongestPathResult { paths }
    }
}
