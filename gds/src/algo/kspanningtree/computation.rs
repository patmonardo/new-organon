//! KSpanningTree Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.kspanningtree.KSpanningTree`
//!
//! Computes k-spanning tree by:
//! 1. Computing MST using Prim's algorithm
//! 2. Progressively cutting k weakest edges

use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Result of k-spanning tree computation
#[derive(Clone)]
pub struct KSpanningTreeResult {
    pub parent: Vec<i64>,
    pub cost_to_parent: Vec<f64>,
    pub total_cost: f64,
    pub root: u64,
    pub node_count: usize,
}

/// Priority queue element for edge tracking
#[derive(Debug, Clone)]
struct QueueElement {
    node_id: usize,
    cost: f64,
}

impl Eq for QueueElement {}

impl PartialEq for QueueElement {
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id
    }
}

impl Ord for QueueElement {
    fn cmp(&self, other: &Self) -> Ordering {
        // Min-heap: reverse comparison
        other
            .cost
            .partial_cmp(&self.cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// KSpanningTree computation runtime - Pure state management for k-limiting logic
pub struct KSpanningTreeComputationRuntime {
    parent: Vec<i64>,
    cost_to_parent: Vec<f64>,
    total_cost: f64,
    // Additional state for k-limiting algorithm
    exterior: Vec<bool>,  // BitSet equivalent - tracks exterior nodes
    included: Vec<bool>,  // BitSet equivalent - tracks included nodes
    out_degree: Vec<u64>, // Out-degree tracking
    priority_queue: BinaryHeap<QueueElement>, // Main priority queue
    trim_queue: BinaryHeap<QueueElement>, // Queue for trimming
}

impl KSpanningTreeComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            parent: vec![-1i64; node_count],
            cost_to_parent: vec![-1.0f64; node_count],
            total_cost: 0.0,
            exterior: vec![false; node_count],
            included: vec![false; node_count],
            out_degree: vec![0u64; node_count],
            priority_queue: BinaryHeap::new(),
            trim_queue: BinaryHeap::new(),
        }
    }

    /// Compute a k-limited spanning tree directly from a neighbor function.
    ///
    /// This is a lightweight, test-friendly entrypoint that does not require a full `Graph`.
    /// It grows a tree from `source_node` using a Prim-like expansion until `k` nodes are
    /// included (or no more nodes are reachable).
    pub fn compute(
        &mut self,
        node_count: usize,
        source_node: usize,
        k: usize,
        objective: &str,
        get_neighbors: impl Fn(usize) -> Vec<(usize, f64)>,
    ) -> KSpanningTreeResult {
        let is_min = objective == "min";

        self.parent = vec![-1i64; node_count];
        self.cost_to_parent = vec![-1.0f64; node_count];
        self.total_cost = 0.0;
        self.included = vec![false; node_count];

        #[derive(Debug, Clone)]
        struct EdgeCandidate {
            node: usize,
            parent: i64,
            cost: f64,
            is_min: bool,
        }

        impl Eq for EdgeCandidate {}
        impl PartialEq for EdgeCandidate {
            fn eq(&self, other: &Self) -> bool {
                self.node == other.node && self.parent == other.parent
            }
        }
        impl Ord for EdgeCandidate {
            fn cmp(&self, other: &Self) -> Ordering {
                // `BinaryHeap` is max-first; adapt so that the "best" edge pops first.
                let best = if self.is_min {
                    other.cost.partial_cmp(&self.cost)
                } else {
                    self.cost.partial_cmp(&other.cost)
                };
                best.unwrap_or(Ordering::Equal)
                    .then_with(|| other.node.cmp(&self.node))
                    .then_with(|| other.parent.cmp(&self.parent))
            }
        }
        impl PartialOrd for EdgeCandidate {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut heap: BinaryHeap<EdgeCandidate> = BinaryHeap::new();
        heap.push(EdgeCandidate {
            node: source_node,
            parent: -1,
            cost: 0.0,
            is_min,
        });

        let mut included_count = 0usize;
        while let Some(candidate) = heap.pop() {
            if included_count >= k {
                break;
            }
            if candidate.node >= node_count || self.included[candidate.node] {
                continue;
            }

            self.included[candidate.node] = true;
            self.parent[candidate.node] = candidate.parent;
            self.cost_to_parent[candidate.node] = if candidate.parent == -1 {
                0.0
            } else {
                candidate.cost
            };
            if candidate.parent != -1 {
                self.total_cost += candidate.cost;
            }
            included_count += 1;

            for (neighbor, weight) in get_neighbors(candidate.node) {
                if neighbor < node_count && !self.included[neighbor] {
                    heap.push(EdgeCandidate {
                        node: neighbor,
                        parent: candidate.node as i64,
                        cost: weight,
                        is_min,
                    });
                }
            }
        }

        // Mark nodes not included
        for i in 0..node_count {
            if !self.included[i] {
                self.parent[i] = -1;
                self.cost_to_parent[i] = -1.0;
            }
        }

        KSpanningTreeResult {
            parent: self.parent.clone(),
            cost_to_parent: self.cost_to_parent.clone(),
            total_cost: self.total_cost,
            root: source_node as u64,
            node_count,
        }
    }

    /// Initialize computation runtime with MST data from spanning tree algorithm
    ///
    /// **Translation Source**: `org.neo4j.gds.kspanningtree.KSpanningTree.init()`
    pub fn initialize_from_mst(
        &mut self,
        parent: &[i64],
        cost_to_parent: &[f64],
        total_weight: f64,
        source_node: usize,
        node_count: usize,
    ) {
        self.parent.copy_from_slice(parent);
        self.cost_to_parent.copy_from_slice(cost_to_parent);
        self.total_cost = total_weight;

        // Initialize state for k-limiting
        self.exterior = vec![false; node_count];
        self.included = vec![false; node_count];
        self.out_degree = vec![0u64; node_count];
        self.priority_queue = BinaryHeap::new();
        self.trim_queue = BinaryHeap::new();

        // Add source node to priority queue
        self.priority_queue.push(QueueElement {
            node_id: source_node,
            cost: 0.0,
        });
    }

    /// Apply k-limiting logic to reduce spanning tree to exactly k nodes
    ///
    /// **Translation Source**: `org.neo4j.gds.kspanningtree.KSpanningTree.growApproach()`
    pub fn apply_k_limiting(
        &mut self,
        k: usize,
        is_min: bool,
        get_neighbors: impl Fn(usize) -> Vec<(usize, f64)>,
    ) {
        let mut nodes_in_tree = 0usize;

        // Reset state
        self.exterior.fill(false);
        self.included.fill(false);
        self.out_degree.fill(0);
        self.priority_queue.clear();
        self.trim_queue.clear();

        // Start with source node
        let source_node = self.find_root();
        self.priority_queue.push(QueueElement {
            node_id: source_node,
            cost: 0.0,
        });

        while !self.priority_queue.is_empty() && nodes_in_tree < k {
            let current = self.priority_queue.pop().unwrap();
            let node_id = current.node_id;
            let cost = current.cost;

            if self.included[node_id] {
                continue;
            }

            // Check if we should add this node
            let should_add = if nodes_in_tree < k {
                true
            } else {
                // Find worst leaf to potentially replace
                let leaf_to_trim = self.find_next_valid_leaf();
                if let Some(trim_node) = leaf_to_trim {
                    let trim_cost = self.cost_to_parent[trim_node];
                    self.should_replace_edge(cost, trim_cost, is_min)
                } else {
                    false
                }
            };

            if should_add {
                self.add_node_to_tree(node_id, cost, &get_neighbors);
                nodes_in_tree += 1;
            }
        }

        // Clean up nodes not in final tree
        self.prune_untouched_nodes();
    }

    /// Find the root node (node with parent == -1)
    fn find_root(&self) -> usize {
        self.parent.iter().position(|&p| p == -1).unwrap_or(0)
    }

    /// Find next valid leaf to trim
    fn find_next_valid_leaf(&self) -> Option<usize> {
        // This is a simplified version - in practice would need more complex logic
        // to find actual leaves in the current tree
        None // Placeholder
    }

    /// Determine if we should replace an edge based on cost
    fn should_replace_edge(&self, new_cost: f64, old_cost: f64, is_min: bool) -> bool {
        if is_min {
            new_cost < old_cost
        } else {
            new_cost > old_cost
        }
    }

    /// Add node to the k-spanning tree
    fn add_node_to_tree(
        &mut self,
        node_id: usize,
        cost: f64,
        get_neighbors: &impl Fn(usize) -> Vec<(usize, f64)>,
    ) {
        self.included[node_id] = true;
        self.total_cost += cost;

        // Update exterior set and trim queue
        self.update_exterior(node_id, cost);

        // Relax neighbors
        self.relax_neighbors(node_id, get_neighbors);
    }

    /// Update exterior set when adding a node
    fn update_exterior(&mut self, node_id: usize, cost: f64) {
        if !self.trim_queue.iter().any(|e| e.node_id == node_id) {
            self.trim_queue.push(QueueElement { node_id, cost });
        } else {
            // Update existing entry - this is simplified
            // In practice would need to update the priority queue
        }
        self.exterior[node_id] = true;
    }

    /// Relax neighbors of a node
    fn relax_neighbors(
        &mut self,
        node_id: usize,
        get_neighbors: &impl Fn(usize) -> Vec<(usize, f64)>,
    ) {
        for (neighbor, _weight) in get_neighbors(node_id) {
            if self.parent.get(neighbor).copied() == Some(node_id as i64) && self.included[neighbor]
            {
                // This neighbor is a child - update out-degree
                self.out_degree[node_id] += 1;
            }
        }
    }

    /// Remove nodes not included in final tree
    fn prune_untouched_nodes(&mut self) {
        for i in 0..self.parent.len() {
            if !self.included[i] {
                self.parent[i] = -1;
                self.cost_to_parent[i] = -1.0;
            }
        }
    }

    // Getters for final results
    pub fn get_parent(&self) -> &Vec<i64> {
        &self.parent
    }

    pub fn get_cost_to_parent(&self) -> &Vec<f64> {
        &self.cost_to_parent
    }

    pub fn get_total_cost(&self) -> f64 {
        self.total_cost
    }
}
