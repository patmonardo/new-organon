use crate::procedures::steiner_tree::spec::{
    SteinerTreeConfig, SteinerTreeResult, PRUNED, ROOT_NODE,
};
use crate::procedures::steiner_tree::storage::SteinerTreeStorage;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, VecDeque};

/// Priority queue entry for Dijkstra's algorithm
#[derive(Debug, Clone)]
struct DijkstraEntry {
    node: usize,
    cost: f64,
    parent: i64,
}

impl PartialEq for DijkstraEntry {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl Eq for DijkstraEntry {}

impl PartialOrd for DijkstraEntry {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse order for min-heap
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for DijkstraEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

/// Computation runtime for Steiner Tree
pub struct SteinerTreeComputationRuntime {
    config: SteinerTreeConfig,
}

impl SteinerTreeComputationRuntime {
    pub fn new(config: SteinerTreeConfig) -> Self {
        Self { config }
    }

    /// Compute Steiner Tree using closure-based interface
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes in the graph
    /// * `get_neighbors` - Function that returns (target_node, weight) pairs for a given node
    pub fn compute<F>(&self, node_count: usize, get_neighbors: F) -> SteinerTreeResult
    where
        F: Fn(usize) -> Vec<(usize, f64)>,
    {
        let mut storage = SteinerTreeStorage::new(node_count);

        // Validate configuration
        if self.config.source_node as usize >= node_count {
            // Return empty result on error
            return storage.into_result();
        }

        let terminals: HashSet<usize> = self
            .config
            .target_nodes
            .iter()
            .map(|&t| t as usize)
            .collect();

        for &terminal in &terminals {
            if terminal >= node_count {
                return storage.into_result();
            }
        }

        if terminals.is_empty() {
            return storage.into_result();
        }

        // Initialize source
        let source = self.config.source_node as usize;
        storage.parent[source] = ROOT_NODE;
        storage.effective_node_count = 1;

        // Run shortest paths to all terminals
        self.run_shortest_paths_to_terminals(&mut storage, &terminals, &get_neighbors);

        // Prune nodes not on paths to terminals if rerouting enabled
        if self.config.apply_rerouting {
            self.prune_non_terminal_leaves(&mut storage, &terminals);
        }

        // Calculate effective target nodes count
        storage.effective_target_nodes_count = terminals
            .iter()
            .filter(|&&t| storage.parent[t] != PRUNED)
            .count() as u64;

        storage.into_result()
    }

    /// Run Dijkstra from source to find paths to all terminals
    fn run_shortest_paths_to_terminals<F>(
        &self,
        storage: &mut SteinerTreeStorage,
        _terminals: &HashSet<usize>,
        get_neighbors: &F,
    ) where
        F: Fn(usize) -> Vec<(usize, f64)>,
    {
        let node_count = storage.parent.len();
        let mut distances = vec![f64::INFINITY; node_count];
        let mut visited = vec![false; node_count];
        let mut heap = BinaryHeap::new();

        // Start from source
        let source = self.config.source_node as usize;
        distances[source] = 0.0;
        heap.push(DijkstraEntry {
            node: source,
            cost: 0.0,
            parent: ROOT_NODE,
        });

        while let Some(entry) = heap.pop() {
            let node_id = entry.node;

            if visited[node_id] {
                continue;
            }
            visited[node_id] = true;

            // Update parent if this is first visit
            storage.parent[node_id] = entry.parent;
            if entry.parent != ROOT_NODE && entry.parent >= 0 {
                storage.parent_cost[node_id] = entry.cost - distances[entry.parent as usize];
                storage.total_cost += storage.parent_cost[node_id];
            }

            if entry.parent != ROOT_NODE {
                storage.effective_node_count += 1;
            }

            // Explore neighbors
            let neighbors = get_neighbors(node_id);
            for (target, weight) in neighbors {
                if visited[target] {
                    continue;
                }

                let new_cost = entry.cost + weight;

                if new_cost < distances[target] {
                    distances[target] = new_cost;
                    heap.push(DijkstraEntry {
                        node: target,
                        cost: new_cost,
                        parent: node_id as i64,
                    });
                }
            }
        }
    }

    /// Prune leaf nodes that are not terminals
    fn prune_non_terminal_leaves(
        &self,
        storage: &mut SteinerTreeStorage,
        terminals: &HashSet<usize>,
    ) {
        let node_count = storage.parent.len();
        let mut child_count = vec![0u32; node_count];

        // Count children for each node
        for node_id in 0..node_count {
            let parent = storage.parent[node_id];
            if parent >= 0 {
                child_count[parent as usize] += 1;
            }
        }

        // Queue nodes for potential pruning
        let mut queue = VecDeque::new();
        for (node_id, &child_count_val) in child_count.iter().enumerate().take(node_count) {
            if storage.parent[node_id] != PRUNED
                && storage.parent[node_id] != ROOT_NODE
                && child_count_val == 0
                && !terminals.contains(&node_id)
            {
                queue.push_back(node_id);
            }
        }

        // Prune leaves that are not terminals
        while let Some(node_id) = queue.pop_front() {
            let parent = storage.parent[node_id];

            if parent < 0 {
                continue;
            }

            // Remove this node
            let cost = storage.parent_cost[node_id];
            storage.parent[node_id] = PRUNED;
            storage.parent_cost[node_id] = 0.0;
            storage.total_cost -= cost;
            storage.effective_node_count -= 1;

            // Update parent's child count
            let parent_idx = parent as usize;
            child_count[parent_idx] -= 1;

            // If parent is now a leaf and not a terminal, queue it
            if child_count[parent_idx] == 0
                && storage.parent[parent_idx] != ROOT_NODE
                && !terminals.contains(&parent_idx)
            {
                queue.push_back(parent_idx);
            }
        }
    }
}
