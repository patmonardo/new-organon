use crate::algo::prize_collecting_steiner_tree::spec::{PCSTreeConfig, PCSTreeResult, PRUNED};
use crate::algo::prize_collecting_steiner_tree::PCSTreeComputationRuntime;
use crate::core::utils::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

/// Prize-Collecting Steiner Tree storage runtime (controller).
///
/// Owns graph access and drives the algorithm loop. The computation runtime is
/// pure state (tree arrays + pruning). This implementation is intentionally
/// simple (parity direction) and can be upgraded over time.
pub struct PCSTreeStorageRuntime {
    pub config: PCSTreeConfig,
    pub concurrency: usize,
}

#[derive(Debug, Clone)]
struct FrontierEntry {
    node: NodeId,
    from: NodeId,
    weight: f64,
    effective_cost: f64,
}

impl PartialEq for FrontierEntry {
    fn eq(&self, other: &Self) -> bool {
        self.effective_cost.eq(&other.effective_cost)
    }
}

impl Eq for FrontierEntry {}

impl PartialOrd for FrontierEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FrontierEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap by effective_cost.
        other
            .effective_cost
            .partial_cmp(&self.effective_cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PCSTreeStorageRuntime {
    pub fn new(config: PCSTreeConfig, concurrency: usize) -> Self {
        Self {
            config,
            concurrency,
        }
    }

    pub fn compute_prize_collecting_steiner_tree(
        &self,
        computation: &mut PCSTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<PCSTreeResult, AlgorithmError> {
        let volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(UNKNOWN_VOLUME);
        if volume == UNKNOWN_VOLUME {
            progress_tracker.begin_subtask_unknown();
        } else {
            progress_tracker.begin_subtask_with_volume(volume);
        }

        let start = Instant::now();
        let node_count = graph.map(|g| g.node_count()).unwrap_or(0);
        let neighbor_fn =
            |node: NodeId| -> Vec<(NodeId, f64)> { self.get_neighbors_with_weights(graph, node) };

        let result = self.compute_core(computation, node_count, &neighbor_fn, progress_tracker);

        match result {
            Ok(ok) => {
                let _elapsed = start.elapsed();
                progress_tracker.end_subtask();
                Ok(ok)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    /// Test/utility entrypoint: run with an explicit neighbor provider.
    #[cfg(test)]
    pub(crate) fn compute_prize_collecting_steiner_tree_with_neighbors<F>(
        &self,
        computation: &mut PCSTreeComputationRuntime,
        node_count: usize,
        get_neighbors: &F,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<PCSTreeResult, AlgorithmError>
    where
        F: Fn(NodeId) -> Vec<(NodeId, f64)>,
    {
        progress_tracker.begin_subtask_unknown();
        let start = Instant::now();
        let out = self.compute_core(computation, node_count, get_neighbors, progress_tracker);
        match out {
            Ok(v) => {
                let _elapsed = start.elapsed();
                progress_tracker.end_subtask();
                Ok(v)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    fn compute_core<F>(
        &self,
        computation: &mut PCSTreeComputationRuntime,
        node_count: usize,
        get_neighbors: &F,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<PCSTreeResult, AlgorithmError>
    where
        F: Fn(NodeId) -> Vec<(NodeId, f64)>,
    {
        if node_count == 0 {
            return Ok(PCSTreeResult {
                parent_array: vec![],
                relationship_to_parent_cost: vec![],
                total_edge_cost: 0.0,
                total_prize: 0.0,
                net_value: 0.0,
                effective_node_count: 0,
            });
        }

        if computation.node_count() != node_count {
            return Err(AlgorithmError::InvalidGraph(
                "computation node_count mismatch".to_string(),
            ));
        }

        if self.config.prizes.len() != node_count || computation.prizes().len() != node_count {
            return Err(AlgorithmError::InvalidGraph(
                "prizes length must match node_count".to_string(),
            ));
        }

        computation.reset();

        let root = computation
            .max_prize_node()
            .ok_or_else(|| AlgorithmError::InvalidGraph("empty prizes".to_string()))?;
        computation.include_root(root);

        let mut scanned_relationships: usize = 0;
        const LOG_BATCH: usize = 256;

        let mut heap = BinaryHeap::new();
        for (nbr, weight) in get_neighbors(root) {
            scanned_relationships = scanned_relationships.saturating_add(1);
            if scanned_relationships >= LOG_BATCH {
                progress_tracker.log_progress(scanned_relationships);
                scanned_relationships = 0;
            }
            if nbr < 0 {
                continue;
            }
            if weight.is_nan() || weight.is_infinite() || weight < 0.0 {
                continue;
            }
            let effective_cost = weight - computation.prize(nbr);
            heap.push(FrontierEntry {
                node: nbr,
                from: root,
                weight,
                effective_cost,
            });
        }

        while let Some(entry) = heap.pop() {
            if computation.is_in_tree(entry.node) {
                continue;
            }

            computation.include_edge(entry.from, entry.node, entry.weight);

            for (nbr, weight) in get_neighbors(entry.node) {
                scanned_relationships = scanned_relationships.saturating_add(1);
                if scanned_relationships >= LOG_BATCH {
                    progress_tracker.log_progress(scanned_relationships);
                    scanned_relationships = 0;
                }
                if nbr < 0 {
                    continue;
                }
                if weight.is_nan() || weight.is_infinite() || weight < 0.0 {
                    continue;
                }
                if computation.is_in_tree(nbr) {
                    continue;
                }
                let effective_cost = weight - computation.prize(nbr);
                heap.push(FrontierEntry {
                    node: nbr,
                    from: entry.node,
                    weight,
                    effective_cost,
                });
            }
        }

        if scanned_relationships > 0 {
            progress_tracker.log_progress(scanned_relationships);
        }

        computation.prune_negative_subtrees();

        let mut total_prize = 0.0;
        let mut total_edge_cost = 0.0;
        let mut effective_node_count = 0u64;
        for node in 0..node_count {
            let p = computation.parent_array()[node];
            if p == PRUNED {
                continue;
            }
            effective_node_count += 1;
            total_prize += computation.prizes()[node];
            if p >= 0 {
                total_edge_cost += computation.parent_cost_array()[node];
            }
        }

        Ok(PCSTreeResult {
            parent_array: computation.parent_array().to_vec(),
            relationship_to_parent_cost: computation.parent_cost_array().to_vec(),
            total_edge_cost,
            total_prize,
            net_value: total_prize - total_edge_cost,
            effective_node_count,
        })
    }

    fn get_neighbors_with_weights(
        &self,
        graph: Option<&dyn Graph>,
        node_id: NodeId,
    ) -> Vec<(NodeId, f64)> {
        if let Some(g) = graph {
            let fallback: f64 = 1.0;
            g.stream_relationships(node_id, fallback)
                .map(|cursor| (cursor.target_id(), cursor.property()))
                .filter(|(t, _)| *t >= 0)
                .collect()
        } else {
            Vec::new()
        }
    }
}
