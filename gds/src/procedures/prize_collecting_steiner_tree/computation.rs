use crate::procedures::prize_collecting_steiner_tree::spec::{
    PCSTreeConfig, PCSTreeResult, PRUNED, ROOT_NODE,
};
use crate::procedures::prize_collecting_steiner_tree::storage::PCSTreeStorage;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Priority queue entry for cluster growth
#[derive(Debug, Clone)]
struct ClusterEntry {
    node: usize,
    cost: f64,
    from_cluster: usize,
}

impl PartialEq for ClusterEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for ClusterEntry {}

impl PartialOrd for ClusterEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ClusterEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: reverse comparison
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

/// Computation runtime for Prize-Collecting Steiner Tree
///
/// This is a simplified implementation focusing on the core algorithm.
/// The full Java implementation includes sophisticated data structures
/// (PairingHeap, ClusterStructure) for better performance on large graphs.
pub struct PCSTreeComputationRuntime {
    config: PCSTreeConfig,
}

impl PCSTreeComputationRuntime {
    pub fn new(config: PCSTreeConfig) -> Self {
        Self { config }
    }

    /// Compute Prize-Collecting Steiner Tree using closure-based interface
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes in graph
    /// * `get_neighbors` - Function returning (target, weight) for each node
    pub fn compute<F>(&self, node_count: usize, get_neighbors: F) -> PCSTreeResult
    where
        F: Fn(usize) -> Vec<(usize, f64)>,
    {
        // Validate prizes array
        if self.config.prizes.len() != node_count {
            // Return empty result on error
            return PCSTreeStorage::new(node_count).into_result();
        }

        // Find node with maximum prize as starting point
        let root = self.find_max_prize_node();

        // Phase 1: Growth - build initial tree using prize-aware expansion
        let mut storage = self.grow_tree(node_count, root, &get_neighbors);

        // Phase 2: Pruning - remove subtrees where cost > prize
        self.prune_tree(&mut storage);

        storage.into_result()
    }

    /// Find node with maximum prize to use as root
    fn find_max_prize_node(&self) -> usize {
        self.config
            .prizes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    /// Grow tree from root using prize-aware Prim's algorithm
    /// Each node's effective cost is (edge_cost - prize)
    fn grow_tree<F>(&self, node_count: usize, root: usize, get_neighbors: &F) -> PCSTreeStorage
    where
        F: Fn(usize) -> Vec<(usize, f64)>,
    {
        let mut storage = PCSTreeStorage::new(node_count);
        let mut in_tree = vec![false; node_count];
        let mut heap = BinaryHeap::new();

        // Start from root
        storage.parent[root] = ROOT_NODE;
        storage.total_prize += self.config.prizes[root];
        storage.effective_node_count = 1;
        in_tree[root] = true;

        // Add root's neighbors to heap
        for (neighbor, weight) in get_neighbors(root) {
            if !in_tree[neighbor] {
                // Effective cost = edge_weight - node_prize
                let effective_cost = weight - self.config.prizes[neighbor];
                heap.push(ClusterEntry {
                    node: neighbor,
                    cost: effective_cost,
                    from_cluster: root,
                });
            }
        }

        // Grow tree greedily by best prize/cost ratio
        while let Some(entry) = heap.pop() {
            if in_tree[entry.node] {
                continue;
            }

            // Add node to tree
            let node = entry.node;
            let parent = entry.from_cluster;

            // Get actual edge weight from parent
            let edge_weight = get_neighbors(parent)
                .into_iter()
                .find(|(n, _)| *n == node)
                .map(|(_, w)| w)
                .unwrap_or(0.0);

            storage.parent[node] = parent as i64;
            storage.parent_cost[node] = edge_weight;
            storage.total_edge_cost += edge_weight;
            storage.total_prize += self.config.prizes[node];
            storage.effective_node_count += 1;
            in_tree[node] = true;

            // Add new neighbors to heap
            for (neighbor, weight) in get_neighbors(node) {
                if !in_tree[neighbor] {
                    let effective_cost = weight - self.config.prizes[neighbor];
                    heap.push(ClusterEntry {
                        node: neighbor,
                        cost: effective_cost,
                        from_cluster: node,
                    });
                }
            }
        }

        storage
    }

    /// Prune subtrees where total prize < total edge cost
    /// Uses bottom-up dynamic programming
    fn prune_tree(&self, storage: &mut PCSTreeStorage) {
        let node_count = storage.parent.len();

        // Build children lists
        let mut children: Vec<Vec<usize>> = vec![Vec::new(); node_count];
        let mut root = 0;

        for node in 0..node_count {
            let parent = storage.parent[node];
            if parent == ROOT_NODE {
                root = node;
            } else if parent >= 0 {
                children[parent as usize].push(node);
            }
        }

        // Calculate subtree values (prize - cost) bottom-up
        let mut subtree_value = vec![0.0; node_count];
        self.calculate_subtree_values(
            root,
            &children,
            &storage.parent,
            &storage.parent_cost,
            &mut subtree_value,
        );

        // Prune subtrees with negative value
        self.prune_negative_subtrees(
            root,
            &children,
            &mut storage.parent,
            &mut storage.parent_cost,
            &subtree_value,
        );

        // Recalculate totals
        self.recalculate_totals(storage);
    }

    /// Calculate value of each subtree (sum of prizes - sum of edge costs)
    fn calculate_subtree_values(
        &self,
        node: usize,
        children: &[Vec<usize>],
        parent: &[i64],
        parent_cost: &[f64],
        subtree_value: &mut [f64],
    ) -> f64 {
        // Start with node's own prize
        let mut value = self.config.prizes[node];

        // Subtract edge cost to parent (if not root)
        if parent[node] >= 0 {
            value -= parent_cost[node];
        }

        // Add children's subtree values (if positive)
        for &child in &children[node] {
            let child_value =
                self.calculate_subtree_values(child, children, parent, parent_cost, subtree_value);
            value += child_value.max(0.0);
        }

        subtree_value[node] = value;
        value
    }

    /// Prune subtrees with negative value
    fn prune_negative_subtrees(
        &self,
        node: usize,
        children: &[Vec<usize>],
        parent: &mut [i64],
        parent_cost: &mut [f64],
        subtree_value: &[f64],
    ) {
        for &child in &children[node] {
            if subtree_value[child] <= 0.0 {
                // Prune this entire subtree
                self.prune_subtree(child, children, parent, parent_cost);
            } else {
                // Recursively check child's subtrees
                self.prune_negative_subtrees(child, children, parent, parent_cost, subtree_value);
            }
        }
    }

    /// Recursively prune a subtree
    fn prune_subtree(
        &self,
        node: usize,
        children: &[Vec<usize>],
        parent: &mut [i64],
        parent_cost: &mut [f64],
    ) {
        parent[node] = PRUNED;
        parent_cost[node] = 0.0;

        for &child in &children[node] {
            self.prune_subtree(child, children, parent, parent_cost);
        }
    }

    /// Recalculate totals after pruning
    fn recalculate_totals(&self, storage: &mut PCSTreeStorage) {
        storage.total_edge_cost = 0.0;
        storage.total_prize = 0.0;
        storage.effective_node_count = 0;

        for node in 0..storage.parent.len() {
            if storage.parent[node] != PRUNED {
                storage.total_prize += self.config.prizes[node];
                storage.effective_node_count += 1;

                if storage.parent[node] >= 0 {
                    storage.total_edge_cost += storage.parent_cost[node];
                }
            }
        }
    }
}
