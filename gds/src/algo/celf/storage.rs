//! CELF Storage Layer
//!
//! Internal data structures for CELF computation.

use super::CELFComputationRuntime;
use crate::collections::BitSet;
use crate::collections::HugeDoubleArray;
use crate::concurrency::{TerminatedException, TerminationFlag};
use crate::core::utils::paged::HugeLongArrayStack;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::{Orientation, RelationshipType};
use crate::types::graph::NodeId;
use crate::types::graph::Graph;
use crate::types::prelude::GraphStore;
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::sync::Arc;

/// Priority queue for node spreads with custom ordering
/// (higher spread = higher priority; ties broken by smaller node ID)
pub struct SpreadPriorityQueue {
    /// Node IDs in heap order
    heap: Vec<usize>,
    /// Position of each node in the heap (for updates)
    positions: Vec<usize>,
    /// Spread values (costs) for each node
    spreads: HugeDoubleArray,
}

impl SpreadPriorityQueue {
    pub fn new(node_count: usize) -> Self {
        let spreads = HugeDoubleArray::new(node_count);
        let mut heap = Vec::with_capacity(node_count);
        let mut positions = vec![usize::MAX; node_count];

        // Initialize with all nodes and valid positions.
        for i in 0..node_count {
            positions[i] = i;
            heap.push(i);
        }

        let mut q = Self {
            heap,
            positions,
            spreads,
        };

        // Values default to 0.0; heapify to establish a stable ordering.
        q.heapify();
        q
    }

    /// Add or update a node's spread value
    pub fn set(&mut self, node: usize, spread: f64) {
        self.spreads.set(node, spread);
        let pos = self.positions[node];
        if pos == usize::MAX {
            // Not present (should be uncommon): add and bubble.
            let pos = self.heap.len();
            self.heap.push(node);
            self.positions[node] = pos;
            self.bubble_up(pos);
            return;
        }

        // Update existing - reheapify.
        self.bubble_up(pos);
        self.bubble_down(pos);
    }

    /// Get the node with highest spread
    pub fn top(&self) -> usize {
        self.heap[0]
    }

    /// Get the spread of the top node
    pub fn top_spread(&self) -> f64 {
        self.spreads.get(self.heap[0])
    }

    /// Get spread of a specific node
    pub fn spread(&self, node: usize) -> f64 {
        self.spreads.get(node)
    }

    /// Remove and return the top node
    pub fn pop(&mut self) -> usize {
        let top = self.heap[0];
        let last = self.heap.len() - 1;

        if last > 0 {
            self.heap.swap(0, last);
            self.positions[self.heap[0]] = 0;
        }

        self.heap.pop();
        self.positions[top] = usize::MAX;

        if !self.heap.is_empty() {
            self.bubble_down(0);
        }

        top
    }

    /// Get the i-th element (0-indexed) without removing
    pub fn get_ith(&self, i: usize) -> usize {
        self.heap[i]
    }

    /// Number of nodes in queue
    pub fn size(&self) -> usize {
        self.heap.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn heapify(&mut self) {
        if self.heap.is_empty() {
            return;
        }
        // Standard heapify: bubble down from last parent.
        let last_parent = (self.heap.len() / 2).saturating_sub(1);
        for i in (0..=last_parent).rev() {
            self.bubble_down(i);
        }
    }

    fn less_than(&self, a: usize, b: usize) -> bool {
        let spread_a = self.spreads.get(a);
        let spread_b = self.spreads.get(b);
        // Higher spread = less (higher priority)
        // Ties broken by smaller node ID
        if (spread_a - spread_b).abs() < f64::EPSILON {
            a < b
        } else {
            spread_a > spread_b
        }
    }

    fn bubble_up(&mut self, mut pos: usize) {
        let node = self.heap[pos];
        while pos > 0 {
            let parent = (pos - 1) / 2;
            let parent_node = self.heap[parent];
            if !self.less_than(node, parent_node) {
                break;
            }
            self.heap[pos] = parent_node;
            self.positions[parent_node] = pos;
            pos = parent;
        }
        self.heap[pos] = node;
        self.positions[node] = pos;
    }

    fn bubble_down(&mut self, mut pos: usize) {
        let node = self.heap[pos];
        let len = self.heap.len();

        loop {
            let mut min_child = pos;
            let left = 2 * pos + 1;
            let right = left + 1;

            if left < len && self.less_than(self.heap[left], self.heap[min_child]) {
                min_child = left;
            }
            if right < len && self.less_than(self.heap[right], self.heap[min_child]) {
                min_child = right;
            }

            if min_child == pos {
                break;
            }

            let child_node = self.heap[min_child];
            self.heap[pos] = child_node;
            self.positions[child_node] = pos;
            pos = min_child;
        }

        self.heap[pos] = node;
        self.positions[node] = pos;
    }
}

/// Storage runtime for CELF: handles graph access and orchestrates controller call.
pub struct CELFStorageRuntime<'a, G: GraphStore> {
    graph: Arc<dyn Graph>,
    _marker: PhantomData<&'a G>,
}

impl<'a, G: GraphStore> CELFStorageRuntime<'a, G> {
    pub fn new(graph_store: &'a G) -> Result<Self, AlgorithmError> {
        let rel_types: HashSet<RelationshipType> = HashSet::new();
        let graph = graph_store
            .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
            .map_err(|e| AlgorithmError::Graph(e.to_string()))?;

        Ok(Self {
            graph,
            _marker: PhantomData,
        })
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn neighbors(&self, node_idx: usize) -> Vec<usize> {
        let node_id = match NodeId::try_from(node_idx as i64) {
            Ok(id) => id,
            Err(_) => return Vec::new(),
        };

        let fallback = self.graph.default_property_value();
        self.graph
            .stream_relationships(node_id, fallback)
            .map(|cursor| cursor.target_id())
            .filter(|target| *target >= 0)
            .map(|target| target as usize)
            .collect()
    }

    pub fn compute_celf(
        &self,
        computation: &CELFComputationRuntime,
        termination: &TerminationFlag,
    ) -> Result<HashMap<u64, f64>, TerminatedException> {
        // compute() uses parallel executors that obey the provided termination flag.
        // No additional controller-side loops here; storage owns graph access and neighbors.
        let neighbors = |n: usize| self.neighbors(n);
        Ok(computation.compute(neighbors, termination))
    }
}
/// Storage for Independent Cascade simulation state
pub struct ICStorage {
    /// Nodes activated by seed set
    pub seed_active: BitSet,
    /// Nodes activated by candidate
    pub candidate_active: BitSet,
    /// Stack of nodes to process
    pub active_stack: HugeLongArrayStack,
}

impl ICStorage {
    pub fn new(node_count: usize) -> Self {
        Self {
            seed_active: BitSet::new(node_count),
            candidate_active: BitSet::new(node_count),
            active_stack: HugeLongArrayStack::new(node_count),
        }
    }

    pub fn clear_seed(&mut self) {
        self.seed_active.clear_all();
    }

    pub fn clear_candidate(&mut self) {
        self.candidate_active.clear_all();
    }
}

/// Result builder for seed set
pub struct SeedSetBuilder {
    seed_nodes: Vec<(usize, f64)>,
}

impl Default for SeedSetBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SeedSetBuilder {
    pub fn new() -> Self {
        Self {
            seed_nodes: Vec::new(),
        }
    }

    pub fn add_seed(&mut self, node: usize, spread: f64) {
        self.seed_nodes.push((node, spread));
    }

    pub fn build(self) -> HashMap<u64, f64> {
        self.seed_nodes
            .into_iter()
            .map(|(node, spread)| (node as u64, spread))
            .collect()
    }
}
