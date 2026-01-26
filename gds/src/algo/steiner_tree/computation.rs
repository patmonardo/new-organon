use crate::algo::steiner_tree::spec::{PRUNED, ROOT_NODE};
use crate::types::graph::NodeId;
use std::collections::VecDeque;

/// Pure state runtime for Steiner Tree.
///
/// Storage owns graph access and drives the algorithm loop.
pub struct SteinerTreeComputationRuntime {
    node_count: usize,
    delta: f64,

    // Final output state (tree)
    parent: Vec<i64>,
    parent_cost: Vec<f64>,
    in_tree: Vec<bool>,

    // Search state (per-iteration)
    distances: Vec<f64>,
    predecessor: Vec<i64>,
    predecessor_edge_weight: Vec<f64>,
    bins: Vec<VecDeque<NodeId>>,
}

impl SteinerTreeComputationRuntime {
    pub fn new(delta: f64, node_count: usize) -> Self {
        Self {
            node_count,
            delta,
            parent: vec![PRUNED; node_count],
            parent_cost: vec![0.0; node_count],
            in_tree: vec![false; node_count],
            distances: vec![f64::INFINITY; node_count],
            predecessor: vec![PRUNED; node_count],
            predecessor_edge_weight: vec![0.0; node_count],
            bins: Vec::new(),
        }
    }

    pub fn initialize_tree(&mut self, source: NodeId) {
        self.parent.fill(PRUNED);
        self.parent_cost.fill(0.0);
        self.in_tree.fill(false);

        let source_idx = source as usize;
        if source_idx < self.node_count {
            self.parent[source_idx] = ROOT_NODE;
            self.in_tree[source_idx] = true;
        }
    }

    pub fn parent_array(&self) -> &[i64] {
        &self.parent
    }

    pub fn parent_cost_array(&self) -> &[f64] {
        &self.parent_cost
    }

    pub fn reset_search(&mut self, merged_to_source: &[bool]) -> VecDeque<NodeId> {
        self.distances.fill(f64::INFINITY);
        self.predecessor.fill(PRUNED);
        self.predecessor_edge_weight.fill(0.0);
        self.bins.clear();

        let mut frontier = VecDeque::new();
        for (idx, merged) in merged_to_source.iter().enumerate() {
            if *merged {
                self.distances[idx] = 0.0;
                frontier.push_back(idx as NodeId);
            }
        }
        frontier
    }

    pub fn distance(&self, node: NodeId) -> f64 {
        self.distances
            .get(node as usize)
            .copied()
            .unwrap_or(f64::INFINITY)
    }

    pub fn predecessor(&self, node: NodeId) -> Option<NodeId> {
        let p = *self.predecessor.get(node as usize)?;
        if p >= 0 {
            Some(p as NodeId)
        } else {
            None
        }
    }

    pub fn predecessor_edge_weight(&self, node: NodeId) -> f64 {
        self.predecessor_edge_weight
            .get(node as usize)
            .copied()
            .unwrap_or(0.0)
    }

    pub fn try_relax(&mut self, source: NodeId, target: NodeId, weight: f64) -> bool {
        let source_idx = source as usize;
        let target_idx = target as usize;
        if source_idx >= self.node_count || target_idx >= self.node_count {
            return false;
        }

        let new_distance = self.distances[source_idx] + weight;
        if new_distance < self.distances[target_idx] {
            self.distances[target_idx] = new_distance;
            self.predecessor[target_idx] = source as i64;
            self.predecessor_edge_weight[target_idx] = weight;
            true
        } else {
            false
        }
    }

    pub fn add_to_bin(&mut self, node: NodeId, bin_index: usize) {
        while self.bins.len() <= bin_index {
            self.bins.push(VecDeque::new());
        }
        self.bins[bin_index].push_back(node);
    }

    pub fn find_next_non_empty_bin(&self, start_index: usize) -> Option<usize> {
        for i in start_index..self.bins.len() {
            if !self.bins[i].is_empty() {
                return Some(i);
            }
        }
        None
    }

    pub fn drain_bin(&mut self, bin_index: usize) -> Vec<NodeId> {
        if bin_index < self.bins.len() {
            self.bins[bin_index].drain(..).collect()
        } else {
            Vec::new()
        }
    }

    /// Merge a predecessor-chain path into the output tree.
    pub fn merge_path_into_tree(
        &mut self,
        terminal: NodeId,
        merged_to_source: &mut [bool],
    ) -> bool {
        let mut current = terminal;
        let mut merged_any = false;

        while (current as usize) < self.node_count && !merged_to_source[current as usize] {
            let pred = match self.predecessor(current) {
                Some(p) => p,
                None => break,
            };

            let current_idx = current as usize;
            merged_to_source[current_idx] = true;
            merged_any = true;

            if !self.in_tree[current_idx] {
                self.parent[current_idx] = pred as i64;
                self.parent_cost[current_idx] = self.predecessor_edge_weight(current);
                self.in_tree[current_idx] = true;
            }

            current = pred;
        }

        merged_any
    }

    pub fn prune_non_terminal_leaves(&mut self, is_terminal: &[bool], source: NodeId) {
        let node_count = self.node_count;
        let mut child_count = vec![0u32; node_count];

        for node_id in 0..node_count {
            let parent = self.parent[node_id];
            if parent >= 0 {
                child_count[parent as usize] += 1;
            }
        }

        let mut queue = VecDeque::new();
        for node_id in 0..node_count {
            if self.parent[node_id] == PRUNED || self.parent[node_id] == ROOT_NODE {
                continue;
            }
            if node_id == source as usize {
                continue;
            }
            if child_count[node_id] == 0 && !is_terminal[node_id] {
                queue.push_back(node_id as NodeId);
            }
        }

        while let Some(node) = queue.pop_front() {
            let node_idx = node as usize;
            let parent = self.parent[node_idx];
            if parent < 0 {
                continue;
            }

            self.parent[node_idx] = PRUNED;
            self.parent_cost[node_idx] = 0.0;
            self.in_tree[node_idx] = false;

            let parent_idx = parent as usize;
            child_count[parent_idx] = child_count[parent_idx].saturating_sub(1);

            if parent_idx != source as usize
                && self.parent[parent_idx] != ROOT_NODE
                && self.parent[parent_idx] != PRUNED
                && child_count[parent_idx] == 0
                && !is_terminal[parent_idx]
            {
                queue.push_back(parent_idx as NodeId);
            }
        }
    }

    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    #[allow(dead_code)]
    pub fn delta(&self) -> f64 {
        self.delta
    }
}
