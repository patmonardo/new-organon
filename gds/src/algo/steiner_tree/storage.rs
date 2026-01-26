use crate::algo::steiner_tree::SteinerTreeComputationRuntime;
use crate::algo::steiner_tree::spec::{SteinerTreeConfig, SteinerTreeResult, PRUNED};
use crate::core::utils::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use std::collections::HashSet;
use std::time::Instant;

/// Steiner Tree Storage Runtime (controller).
///
/// Heuristic (Java-parity direction): iteratively connect the closest remaining
/// terminal to the current merged-to-source set using a delta-stepping style
/// shortest path search seeded with multiple sources at distance 0.
pub struct SteinerTreeStorageRuntime {
    pub config: SteinerTreeConfig,
    pub concurrency: usize,
}

impl SteinerTreeStorageRuntime {
    pub fn new(config: SteinerTreeConfig, concurrency: usize) -> Self {
        Self {
            config,
            concurrency,
        }
    }

    pub fn compute_steiner_tree(
        &self,
        computation: &mut SteinerTreeComputationRuntime,
        graph: Option<&dyn Graph>,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<SteinerTreeResult, AlgorithmError> {
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
    pub(crate) fn compute_steiner_tree_with_neighbors<F>(
        &self,
        computation: &mut SteinerTreeComputationRuntime,
        node_count: usize,
        get_neighbors: &F,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<SteinerTreeResult, AlgorithmError>
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
        computation: &mut SteinerTreeComputationRuntime,
        node_count: usize,
        get_neighbors: &F,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<SteinerTreeResult, AlgorithmError>
    where
        F: Fn(NodeId) -> Vec<(NodeId, f64)>,
    {
        if node_count == 0 {
            return Ok(SteinerTreeResult {
                parent_array: vec![],
                relationship_to_parent_cost: vec![],
                total_cost: 0.0,
                effective_node_count: 0,
                effective_target_nodes_count: 0,
            });
        }

        if self.config.delta <= 0.0 {
            return Err(AlgorithmError::InvalidGraph(
                "delta must be > 0".to_string(),
            ));
        }

        let source = self.config.source_node;
        if source < 0 || (source as usize) >= node_count {
            return Err(AlgorithmError::InvalidGraph(
                "source_node out of bounds".to_string(),
            ));
        }
        if self.config.target_nodes.is_empty() {
            return Err(AlgorithmError::InvalidGraph(
                "target_nodes must not be empty".to_string(),
            ));
        }

        let mut terminals: Vec<NodeId> = Vec::new();
        let mut seen = HashSet::new();
        for &t in &self.config.target_nodes {
            if t < 0 || (t as usize) >= node_count {
                return Err(AlgorithmError::InvalidGraph(
                    "target_nodes contains out-of-bounds node".to_string(),
                ));
            }
            if seen.insert(t) {
                terminals.push(t);
            }
        }

        let mut is_terminal = vec![false; node_count];
        for &t in &terminals {
            is_terminal[t as usize] = true;
        }

        computation.initialize_tree(source);

        let mut merged_to_source = vec![false; node_count];
        merged_to_source[source as usize] = true;

        let mut remaining: Vec<NodeId> = terminals.into_iter().filter(|&t| t != source).collect();

        while !remaining.is_empty() {
            self.run_multi_source_delta_stepping(
                computation,
                &merged_to_source,
                get_neighbors,
                progress_tracker,
            );

            // Choose closest reachable terminal.
            let mut best_idx: Option<usize> = None;
            let mut best_dist = f64::INFINITY;
            for (idx, &t) in remaining.iter().enumerate() {
                let d = computation.distance(t);
                if d < best_dist {
                    best_dist = d;
                    best_idx = Some(idx);
                }
            }

            let Some(chosen_idx) = best_idx else {
                break;
            };
            if best_dist == f64::INFINITY {
                break; // no reachable remaining terminal
            }

            let chosen = remaining.swap_remove(chosen_idx);
            let _ = computation.merge_path_into_tree(chosen, &mut merged_to_source);
        }

        // Always prune non-terminal leaves; rerouting is optional and separate.
        computation.prune_non_terminal_leaves(&is_terminal, source);

        // Optional rerouting/post-optimization stage (not implemented yet).
        let _apply_rerouting = self.config.apply_rerouting;

        // Aggregate totals.
        let mut total_cost = 0.0;
        let mut effective_node_count = 0u64;
        let mut effective_target_nodes_count = 0u64;
        for node_id in 0..node_count {
            let parent = computation.parent_array()[node_id];
            if parent == PRUNED {
                continue;
            }
            effective_node_count += 1;
            if is_terminal[node_id] {
                effective_target_nodes_count += 1;
            }
            if parent >= 0 {
                total_cost += computation.parent_cost_array()[node_id];
            }
        }

        Ok(SteinerTreeResult {
            parent_array: computation.parent_array().to_vec(),
            relationship_to_parent_cost: computation.parent_cost_array().to_vec(),
            total_cost,
            effective_node_count,
            effective_target_nodes_count,
        })
    }

    fn run_multi_source_delta_stepping<F>(
        &self,
        computation: &mut SteinerTreeComputationRuntime,
        merged_to_source: &[bool],
        get_neighbors: &F,
        progress_tracker: &mut dyn ProgressTracker,
    ) where
        F: Fn(NodeId) -> Vec<(NodeId, f64)>,
    {
        let mut scanned_relationships: usize = 0;
        const LOG_BATCH: usize = 256;

        let mut frontier = computation.reset_search(merged_to_source);
        let mut current_bin: usize = 0;

        let max_iterations = merged_to_source.len().saturating_mul(2).max(1);
        let mut iteration = 0usize;

        while !frontier.is_empty() && iteration < max_iterations {
            let mut next_frontier = std::collections::VecDeque::new();

            while let Some(node) = frontier.pop_front() {
                let node_distance = computation.distance(node);
                if node_distance >= self.config.delta * current_bin as f64 {
                    let neighbors = get_neighbors(node);
                    scanned_relationships = scanned_relationships.saturating_add(neighbors.len());
                    if scanned_relationships >= LOG_BATCH {
                        progress_tracker.log_progress(scanned_relationships);
                        scanned_relationships = 0;
                    }

                    for (nbr, weight) in neighbors {
                        if nbr < 0 {
                            continue;
                        }
                        if weight.is_nan() || weight.is_infinite() || weight < 0.0 {
                            continue;
                        }
                        if computation.try_relax(node, nbr, weight) {
                            let new_dist = computation.distance(nbr);
                            let dest_bin = (new_dist / self.config.delta) as usize;
                            if dest_bin == current_bin {
                                next_frontier.push_back(nbr);
                            } else {
                                computation.add_to_bin(nbr, dest_bin);
                            }
                        }
                    }
                }
            }

            frontier = next_frontier;
            let Some(next_bin) = computation.find_next_non_empty_bin(current_bin) else {
                break;
            };
            current_bin = next_bin;
            for node in computation.drain_bin(current_bin) {
                frontier.push_back(node);
            }

            iteration += 1;
        }

        if scanned_relationships > 0 {
            progress_tracker.log_progress(scanned_relationships);
        }
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
            // Minimal mock for storage/computation integration tests.
            match node_id {
                0 => vec![(1, 1.0), (2, 1.0)],
                1 => vec![(3, 1.0)],
                2 => vec![(4, 1.0)],
                _ => vec![],
            }
        }
    }
}
