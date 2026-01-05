//! Betweenness Centrality Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.betweenness.BetweennessCentrality`
//!
//! Two-phase algorithm:
//! 1. FORWARD: BFS from each source, track shortest paths (sigma) and predecessors
//! 2. BACKWARD: Propagate dependencies (delta) back through the DAG
//!
//! Formula: betweenness(v) = sum of (sigma[s,v] / sigma[s,t]) * delta[t]
//!                           for all s,t where path goes through v

use crate::collections::HugeAtomicDoubleArray;
use crate::concurrency::virtual_threads::{Executor, WorkerContext};
use crate::concurrency::{Concurrency, TerminatedException, TerminationFlag};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};

#[derive(Clone)]
pub struct BetweennessCentralityComputationResult {
    pub centralities: Vec<f64>,
}

pub struct BetweennessCentralityComputationRuntime {
    centralities: Vec<f64>,
    sigma: Vec<u64>,               // shortest path counts from source
    delta: Vec<f64>,               // dependencies
    distances: Vec<i32>,           // BFS distances from source
    predecessors: Vec<Vec<usize>>, // predecessors in shortest path DAG
    backward_nodes: Vec<usize>,    // stack for backward phase
}

impl BetweennessCentralityComputationRuntime {
    pub fn new(node_count: usize) -> Self {
        Self {
            centralities: vec![0.0f64; node_count],
            sigma: vec![0u64; node_count],
            delta: vec![0.0f64; node_count],
            distances: vec![-1i32; node_count],
            predecessors: vec![Vec::new(); node_count],
            backward_nodes: Vec::new(),
        }
    }

    pub fn compute(
        &mut self,
        node_count: usize,
        divisor: f64,
        get_neighbors: &impl Fn(usize) -> Vec<usize>,
    ) -> BetweennessCentralityComputationResult {
        // Reset centralities (accumulator)
        for c in self.centralities.iter_mut() {
            *c = 0.0;
        }

        // Process each node as a source
        for source_node in 0..node_count {
            self.forward_phase(source_node, get_neighbors);
            self.backward_phase(source_node);
        }

        if divisor != 0.0 {
            for c in self.centralities.iter_mut() {
                *c /= divisor;
            }
        }

        BetweennessCentralityComputationResult {
            centralities: self.centralities.clone(),
        }
    }

    /// Parallel (and optionally sampled) unweighted betweenness centrality.
    ///
    /// This follows the Brandes algorithm but parallelizes across source nodes.
    /// Results are accumulated into a shared atomic double array.
    pub fn compute_parallel_unweighted(
        node_count: usize,
        sources: &[usize],
        divisor: f64,
        concurrency: usize,
        termination: &TerminationFlag,
        on_source_done: std::sync::Arc<dyn Fn() + Send + Sync>,
        get_neighbors: &(impl Fn(usize) -> Vec<usize> + Send + Sync),
    ) -> Result<BetweennessCentralityComputationResult, TerminatedException> {
        if node_count == 0 || sources.is_empty() {
            return Ok(BetweennessCentralityComputationResult {
                centralities: Vec::new(),
            });
        }

        let centralities = HugeAtomicDoubleArray::new(node_count);
        let executor = Executor::new(Concurrency::of(concurrency.max(1)));

        let task_state = WorkerContext::new(move || UnweightedTaskState::new(node_count));

        executor.parallel_for(0, sources.len(), termination, |i| {
            let source = sources[i];
            if source >= node_count {
                return;
            }
            task_state.with(|state| {
                state.run_source(source, node_count, termination, get_neighbors, &centralities);
            });
            (on_source_done.as_ref())();
        })?;

        let mut out = vec![0.0f64; node_count];
        if divisor != 0.0 {
            for i in 0..node_count {
                out[i] = centralities.get(i) / divisor;
            }
        } else {
            for i in 0..node_count {
                out[i] = centralities.get(i);
            }
        }

        Ok(BetweennessCentralityComputationResult { centralities: out })
    }

    /// Parallel (and optionally sampled) weighted betweenness centrality.
    ///
    /// Weighted variant uses Dijkstra in the forward phase (Brandes, 2001).
    pub fn compute_parallel_weighted(
        node_count: usize,
        sources: &[usize],
        divisor: f64,
        concurrency: usize,
        termination: &TerminationFlag,
        on_source_done: std::sync::Arc<dyn Fn() + Send + Sync>,
        get_neighbors_weighted: &(impl Fn(usize) -> Vec<(usize, f64)> + Send + Sync),
    ) -> Result<BetweennessCentralityComputationResult, TerminatedException> {
        if node_count == 0 || sources.is_empty() {
            return Ok(BetweennessCentralityComputationResult {
                centralities: Vec::new(),
            });
        }

        let centralities = HugeAtomicDoubleArray::new(node_count);
        let executor = Executor::new(Concurrency::of(concurrency.max(1)));

        let task_state = WorkerContext::new(move || WeightedTaskState::new(node_count));

        executor.parallel_for(0, sources.len(), termination, |i| {
            let source = sources[i];
            if source >= node_count {
                return;
            }
            task_state.with(|state| {
                state.run_source(
                    source,
                    node_count,
                    termination,
                    get_neighbors_weighted,
                    &centralities,
                );
            });
            (on_source_done.as_ref())();
        })?;

        let mut out = vec![0.0f64; node_count];
        if divisor != 0.0 {
            for i in 0..node_count {
                out[i] = centralities.get(i) / divisor;
            }
        } else {
            for i in 0..node_count {
                out[i] = centralities.get(i);
            }
        }

        Ok(BetweennessCentralityComputationResult { centralities: out })
    }

    /// Phase 1: Forward BFS from source node
    /// Computes sigma (path counts) and predecessors
    fn forward_phase(&mut self, source_node: usize, get_neighbors: &impl Fn(usize) -> Vec<usize>) {
        // Reset per-source arrays
        self.sigma.iter_mut().for_each(|s| *s = 0);
        self.distances.iter_mut().for_each(|d| *d = -1);
        for preds in self.predecessors.iter_mut() {
            preds.clear();
        }
        self.backward_nodes.clear();

        // Initialize source
        self.sigma[source_node] = 1;
        self.distances[source_node] = 0;

        // BFS
        let mut queue = VecDeque::new();
        queue.push_back(source_node);

        while let Some(node) = queue.pop_front() {
            self.backward_nodes.push(node); // Record for backward phase
            let node_dist = self.distances[node];
            let node_sigma = self.sigma[node];

            for neighbor in get_neighbors(node) {
                let new_dist = node_dist + 1;

                // First time visiting this neighbor?
                if self.distances[neighbor] < 0 {
                    self.distances[neighbor] = new_dist;
                    queue.push_back(neighbor);
                }

                // If on shortest path
                if self.distances[neighbor] == new_dist {
                    self.sigma[neighbor] = self.sigma[neighbor].saturating_add(node_sigma);
                    self.predecessors[neighbor].push(node);
                }
            }
        }
    }

    /// Phase 2: Backward dependency propagation
    /// Process nodes in reverse BFS order to calculate dependencies
    fn backward_phase(&mut self, source_node: usize) {
        // Reset delta
        self.delta.iter_mut().for_each(|d| *d = 0.0);

        // Process backward_nodes in reverse order (excluding source)
        for &node in self.backward_nodes.iter().rev() {
            if node == source_node {
                continue;
            }

            let node_sigma = self.sigma[node] as f64;
            let node_delta = self.delta[node];

            // For each predecessor of this node
            for &pred in &self.predecessors[node] {
                let pred_sigma = self.sigma[pred] as f64;

                // Dependency contribution from this path
                let contribution = (pred_sigma / node_sigma) * (node_delta + 1.0);

                // Accumulate dependency at predecessor (for its future contribution)
                self.delta[pred] += contribution;
            }

            // Accumulate centrality at this node (except for source nodes)
            // The dependency value represents how much this node benefits other nodes
            if node != source_node {
                self.centralities[node] += self.delta[node];
            }
        }
    }
}

struct UnweightedTaskState {
    sigma: Vec<u64>,
    delta: Vec<f64>,
    distances: Vec<i32>,
    predecessors: Vec<Vec<usize>>,
    stack: Vec<usize>,
    queue: VecDeque<usize>,
    visited: Vec<usize>,
}

impl UnweightedTaskState {
    fn new(node_count: usize) -> Self {
        Self {
            sigma: vec![0; node_count],
            delta: vec![0.0; node_count],
            distances: vec![-1; node_count],
            predecessors: vec![Vec::new(); node_count],
            stack: Vec::with_capacity(node_count.min(1024)),
            queue: VecDeque::new(),
            visited: Vec::with_capacity(node_count.min(1024)),
        }
    }

    fn run_source(
        &mut self,
        source: usize,
        node_count: usize,
        termination: &TerminationFlag,
        get_neighbors: &impl Fn(usize) -> Vec<usize>,
        centralities: &HugeAtomicDoubleArray,
    ) {
        if !termination.running() {
            return;
        }

        self.stack.clear();
        self.queue.clear();
        self.visited.clear();

        self.sigma[source] = 1;
        self.delta[source] = 0.0;
        self.distances[source] = 0;
        self.predecessors[source].clear();
        self.visited.push(source);
        self.queue.push_back(source);

        while let Some(v) = self.queue.pop_front() {
            if !termination.running() {
                return;
            }
            self.stack.push(v);
            let v_dist = self.distances[v];
            let v_sigma = self.sigma[v];
            if v_sigma == 0 {
                continue;
            }

            for w in get_neighbors(v) {
                if w >= node_count {
                    continue;
                }

                let next_dist = v_dist + 1;
                if self.distances[w] < 0 {
                    // First time seen in this source run.
                    self.distances[w] = next_dist;
                    self.sigma[w] = 0;
                    self.delta[w] = 0.0;
                    self.predecessors[w].clear();
                    self.visited.push(w);
                    self.queue.push_back(w);
                }

                if self.distances[w] == next_dist {
                    self.sigma[w] = self.sigma[w].saturating_add(v_sigma);
                    self.predecessors[w].push(v);
                }
            }
        }

        // Backward dependency propagation.
        while let Some(w) = self.stack.pop() {
            if !termination.running() {
                return;
            }
            let w_sigma = self.sigma[w] as f64;
            if w_sigma == 0.0 {
                continue;
            }

            // For each predecessor v of w.
            let contrib_base = 1.0 + self.delta[w];
            for &v in &self.predecessors[w] {
                let v_sigma = self.sigma[v] as f64;
                if v_sigma == 0.0 {
                    continue;
                }
                self.delta[v] += (v_sigma / w_sigma) * contrib_base;
            }

            if w != source {
                centralities.get_and_add(w, self.delta[w]);
            }
        }

        // Reset per-node markers for the next source.
        for &v in &self.visited {
            self.distances[v] = -1;
            self.sigma[v] = 0;
            self.delta[v] = 0.0;
            self.predecessors[v].clear();
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct HeapItem {
    dist: f64,
    node: usize,
}

impl Eq for HeapItem {}

impl PartialEq for HeapItem {
    fn eq(&self, other: &Self) -> bool {
        self.dist.to_bits() == other.dist.to_bits() && self.node == other.node
    }
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse by distance to make BinaryHeap act like a min-heap.
        other
            .dist
            .total_cmp(&self.dist)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct WeightedTaskState {
    sigma: Vec<u64>,
    delta: Vec<f64>,
    distances: Vec<f64>,
    predecessors: Vec<Vec<usize>>,
    stack: Vec<usize>,
    heap: BinaryHeap<HeapItem>,
    visited: Vec<usize>,
}

impl WeightedTaskState {
    fn new(node_count: usize) -> Self {
        Self {
            sigma: vec![0; node_count],
            delta: vec![0.0; node_count],
            distances: vec![f64::INFINITY; node_count],
            predecessors: vec![Vec::new(); node_count],
            stack: Vec::with_capacity(node_count.min(1024)),
            heap: BinaryHeap::new(),
            visited: Vec::with_capacity(node_count.min(1024)),
        }
    }

    fn run_source(
        &mut self,
        source: usize,
        node_count: usize,
        termination: &TerminationFlag,
        get_neighbors_weighted: &impl Fn(usize) -> Vec<(usize, f64)>,
        centralities: &HugeAtomicDoubleArray,
    ) {
        if !termination.running() {
            return;
        }

        const EPS: f64 = 1e-12;

        self.stack.clear();
        self.heap.clear();
        self.visited.clear();

        self.sigma[source] = 1;
        self.delta[source] = 0.0;
        self.distances[source] = 0.0;
        self.predecessors[source].clear();
        self.visited.push(source);
        self.heap.push(HeapItem {
            dist: 0.0,
            node: source,
        });

        while let Some(HeapItem { dist: d, node: v }) = self.heap.pop() {
            if !termination.running() {
                return;
            }
            if d > self.distances[v] + EPS {
                continue;
            }

            self.stack.push(v);
            let v_sigma = self.sigma[v];
            if v_sigma == 0 {
                continue;
            }

            for (w, weight) in get_neighbors_weighted(v) {
                if w >= node_count {
                    continue;
                }
                if !weight.is_finite() || weight < 0.0 {
                    // Dijkstra requires non-negative finite weights.
                    continue;
                }
                let alt = d + weight;

                if self.distances[w].is_infinite() {
                    // First time seen in this source run.
                    self.predecessors[w].clear();
                    self.sigma[w] = 0;
                    self.delta[w] = 0.0;
                    self.visited.push(w);
                }

                if alt + EPS < self.distances[w] {
                    self.distances[w] = alt;
                    self.heap.push(HeapItem { dist: alt, node: w });
                    self.predecessors[w].clear();
                    self.predecessors[w].push(v);
                    self.sigma[w] = v_sigma;
                } else if (alt - self.distances[w]).abs() <= EPS {
                    self.predecessors[w].push(v);
                    self.sigma[w] = self.sigma[w].saturating_add(v_sigma);
                }
            }
        }

        while let Some(w) = self.stack.pop() {
            if !termination.running() {
                return;
            }
            let w_sigma = self.sigma[w] as f64;
            if w_sigma == 0.0 {
                continue;
            }

            let contrib_base = 1.0 + self.delta[w];
            for &v in &self.predecessors[w] {
                let v_sigma = self.sigma[v] as f64;
                if v_sigma == 0.0 {
                    continue;
                }
                self.delta[v] += (v_sigma / w_sigma) * contrib_base;
            }

            if w != source {
                centralities.get_and_add(w, self.delta[w]);
            }
        }

        for &v in &self.visited {
            self.distances[v] = f64::INFINITY;
            self.sigma[v] = 0;
            self.delta[v] = 0.0;
            self.predecessors[v].clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_graph(edges: Vec<(usize, usize)>, node_count: usize) -> HashMap<usize, Vec<usize>> {
        let mut relationships: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..node_count {
            relationships.insert(i, Vec::new());
        }

        for (from, to) in edges {
            relationships.entry(from).or_default().push(to);
            if from != to {
                relationships.entry(to).or_default().push(from);
            }
        }

        for neighbors in relationships.values_mut() {
            neighbors.sort_unstable();
            neighbors.dedup();
        }

        relationships
    }

    #[test]
    fn test_betweenness_single_edge() {
        let graph = create_graph(vec![(0, 1)], 2);
        let mut runtime = BetweennessCentralityComputationRuntime::new(2);
        let get_neighbors = |node| graph.get(&node).cloned().unwrap_or_default();
        let result = runtime.compute(2, 2.0, &get_neighbors);

        // Single edge: no intermediate nodes
        assert!((result.centralities[0]).abs() < 1e-10);
        assert!((result.centralities[1]).abs() < 1e-10);
    }

    #[test]
    fn test_betweenness_path_three_nodes() {
        let graph = create_graph(vec![(0, 1), (1, 2)], 3);
        let mut runtime = BetweennessCentralityComputationRuntime::new(3);
        let get_neighbors = |node| graph.get(&node).cloned().unwrap_or_default();
        let result = runtime.compute(3, 2.0, &get_neighbors);

        // Node 1 is on the path 0-1-2
        // From 0: sigma[0]=1, sigma[1]=1, sigma[2]=1
        //   dependency[1] = (1/1) * (0+1) = 1
        // From 2: sigma[2]=1, sigma[1]=1, sigma[0]=1
        //   dependency[1] += (1/1) * (0+1) = 1
        // BC[1] = (1 + 1) / 2 = 1.0
        assert!((result.centralities[0]).abs() < 1e-10);
        assert!((result.centralities[1] - 1.0).abs() < 1e-10);
        assert!((result.centralities[2]).abs() < 1e-10);
    }

    #[test]
    fn test_betweenness_star_graph() {
        // Center: 0, Leaves: 1,2,3,4
        let graph = create_graph(vec![(0, 1), (0, 2), (0, 3), (0, 4)], 5);
        let mut runtime = BetweennessCentralityComputationRuntime::new(5);
        let get_neighbors = |node| graph.get(&node).cloned().unwrap_or_default();
        let result = runtime.compute(5, 2.0, &get_neighbors);

        // Center is on every path between leaves
        // From each leaf, 3 paths go through center to other leaves
        // 4 leaves × 3 paths = 12 / 2 (undirected) = 6.0
        assert!(
            (result.centralities[0] - 6.0).abs() < 1e-10,
            "Center: expected 6.0, got {}",
            result.centralities[0]
        );

        // Leaves don't lie on paths between other nodes
        for i in 1..5 {
            assert!(
                (result.centralities[i]).abs() < 1e-10,
                "Leaf {}: expected 0.0, got {}",
                i,
                result.centralities[i]
            );
        }
    }

    #[test]
    fn test_betweenness_triangle() {
        let graph = create_graph(vec![(0, 1), (1, 2), (0, 2)], 3);
        let mut runtime = BetweennessCentralityComputationRuntime::new(3);
        let get_neighbors = |node| graph.get(&node).cloned().unwrap_or_default();
        let result = runtime.compute(3, 2.0, &get_neighbors);

        // In a triangle, each node is on shortest paths between other two
        // But multiple shortest paths exist, so dependencies spread
        // All nodes should have equal non-zero centrality
        for i in 0..3 {
            assert!(
                result.centralities[i] >= 0.0,
                "Node {}: centrality should be non-negative",
                i
            );
        }
    }

    #[test]
    fn test_betweenness_linear_four_nodes() {
        // 0-1-2-3
        let graph = create_graph(vec![(0, 1), (1, 2), (2, 3)], 4);
        let mut runtime = BetweennessCentralityComputationRuntime::new(4);
        let get_neighbors = |node| graph.get(&node).cloned().unwrap_or_default();
        let result = runtime.compute(4, 2.0, &get_neighbors);

        // Node 1 and 2 are on internal paths
        // Ends should have 0
        assert!((result.centralities[0]).abs() < 1e-10);
        assert!(result.centralities[1] > 0.0);
        assert!(result.centralities[2] > 0.0);
        assert!((result.centralities[3]).abs() < 1e-10);
    }

    #[test]
    fn test_betweenness_complete_graph_k4() {
        // Complete graph K4
        let graph = create_graph(vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)], 4);
        let mut runtime = BetweennessCentralityComputationRuntime::new(4);
        let get_neighbors = |node| graph.get(&node).cloned().unwrap_or_default();
        let result = runtime.compute(4, 2.0, &get_neighbors);

        // In complete graph, all paths have length 1 (direct edge)
        // No node lies on a shortest path between two others
        for i in 0..4 {
            assert!(
                (result.centralities[i]).abs() < 1e-10,
                "Node {}: expected 0.0, got {}",
                i,
                result.centralities[i]
            );
        }
    }

    #[test]
    fn test_betweenness_diamond_graph() {
        // Diamond: 0-1, 0-2, 1-3, 2-3
        //    0
        //   / \
        //  1   2
        //   \ /
        //    3
        let graph = create_graph(vec![(0, 1), (0, 2), (1, 3), (2, 3)], 4);
        let mut runtime = BetweennessCentralityComputationRuntime::new(4);
        let get_neighbors = |node| graph.get(&node).cloned().unwrap_or_default();
        let result = runtime.compute(4, 2.0, &get_neighbors);

        // Nodes 1 and 2 should have equal centrality (symmetric)
        assert!(
            (result.centralities[1] - result.centralities[2]).abs() < 1e-10,
            "Nodes 1 and 2 should have same centrality"
        );

        // All centralities should be non-negative
        for i in 0..4 {
            assert!(
                result.centralities[i] >= 0.0,
                "Node {}: centrality should be non-negative",
                i
            );
        }
    }
}
