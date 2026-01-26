use crate::algo::prize_collecting_steiner_tree::spec::{PRUNED, ROOT_NODE};
use crate::types::graph::NodeId;

/// Computation runtime for Prize-Collecting Steiner Tree.
///
/// Pure state only: maintains the current tree (parent pointers + parent costs),
/// node inclusion flags, and pruning operations. Graph access and the main loop
/// live in the storage runtime.
pub struct PCSTreeComputationRuntime {
    node_count: usize,
    prizes: Vec<f64>,

    parent: Vec<i64>,
    parent_cost: Vec<f64>,
    in_tree: Vec<bool>,
}

impl PCSTreeComputationRuntime {
    pub fn new(prizes: Vec<f64>, node_count: usize) -> Self {
        Self {
            node_count,
            prizes,
            parent: vec![PRUNED; node_count],
            parent_cost: vec![0.0; node_count],
            in_tree: vec![false; node_count],
        }
    }

    pub fn reset(&mut self) {
        self.parent.fill(PRUNED);
        self.parent_cost.fill(0.0);
        self.in_tree.fill(false);
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }

    pub fn prizes(&self) -> &[f64] {
        &self.prizes
    }

    pub fn parent_array(&self) -> &[i64] {
        &self.parent
    }

    pub fn parent_cost_array(&self) -> &[f64] {
        &self.parent_cost
    }

    pub fn is_in_tree(&self, node: NodeId) -> bool {
        self.in_tree.get(node as usize).copied().unwrap_or(false)
    }

    pub fn prize(&self, node: NodeId) -> f64 {
        self.prizes.get(node as usize).copied().unwrap_or(0.0)
    }

    pub fn max_prize_node(&self) -> Option<NodeId> {
        self.prizes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx as NodeId)
    }

    pub fn include_root(&mut self, root: NodeId) {
        let idx = root as usize;
        if idx >= self.node_count {
            return;
        }
        self.parent[idx] = ROOT_NODE;
        self.parent_cost[idx] = 0.0;
        self.in_tree[idx] = true;
    }

    pub fn include_edge(&mut self, parent: NodeId, node: NodeId, weight: f64) {
        let node_idx = node as usize;
        if node_idx >= self.node_count {
            return;
        }
        self.parent[node_idx] = parent as i64;
        self.parent_cost[node_idx] = weight;
        self.in_tree[node_idx] = true;
    }

    pub fn prune_negative_subtrees(&mut self) {
        // Identify root.
        let Some(root) = self
            .parent
            .iter()
            .enumerate()
            .find(|(_, &p)| p == ROOT_NODE)
            .map(|(idx, _)| idx)
        else {
            return;
        };

        // Build children lists for current tree.
        let mut children: Vec<Vec<usize>> = vec![Vec::new(); self.node_count];
        for (node, &p) in self.parent.iter().enumerate() {
            if p >= 0 {
                children[p as usize].push(node);
            }
        }

        // Post-order traversal.
        let mut stack: Vec<usize> = vec![root];
        let mut order: Vec<usize> = Vec::new();
        while let Some(n) = stack.pop() {
            order.push(n);
            for &c in &children[n] {
                stack.push(c);
            }
        }

        let mut subtree_value = vec![0.0_f64; self.node_count];
        for &n in order.iter().rev() {
            if self.parent[n] == PRUNED {
                subtree_value[n] = 0.0_f64;
                continue;
            }

            let mut value = self.prizes[n];
            if self.parent[n] >= 0 {
                value -= self.parent_cost[n];
            }

            for &c in &children[n] {
                value += subtree_value[c].max(0.0_f64);
            }
            subtree_value[n] = value;
        }

        // Prune any child-subtree with non-positive value.
        let mut prune_stack: Vec<usize> = Vec::new();
        let mut walk: Vec<usize> = vec![root];
        while let Some(n) = walk.pop() {
            for &c in &children[n] {
                if subtree_value[c] <= 0.0 {
                    prune_stack.push(c);
                } else {
                    walk.push(c);
                }
            }
        }

        while let Some(n) = prune_stack.pop() {
            if self.parent[n] == PRUNED {
                continue;
            }
            self.parent[n] = PRUNED;
            self.parent_cost[n] = 0.0;
            self.in_tree[n] = false;
            for &c in &children[n] {
                prune_stack.push(c);
            }
        }
    }
}
