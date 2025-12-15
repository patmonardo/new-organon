//! K1Coloring Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.k1coloring.K1Coloring`
//!
//! Greedy graph coloring algorithm ensuring no two adjacent nodes share the same color.
//! Uses iterative phases: Coloring (assign colors) and Validation (detect conflicts).

use crate::collections::BitSet;

/// K1Coloring computation result
#[derive(Clone)]
pub struct K1ColoringComputationResult {
    pub colors: Vec<u64>,
    pub ran_iterations: u64,
    pub did_converge: bool,
}

/// K1Coloring computation runtime
pub struct K1ColoringComputationRuntime {
    /// Current color assignments for each node
    colors: Vec<u64>,
    /// Nodes pending coloring (current iteration)
    nodes_to_color_current: BitSet,
    /// Nodes pending coloring (next iteration)
    nodes_to_color_next: BitSet,
    /// Max iterations
    max_iterations: u64,
    /// Current iteration count
    ran_iterations: u64,
    /// Forbidden colors for current node being colored
    forbidden_colors: BitSet,
    /// Scratch list of forbidden color indices to clear (sparse reset)
    forbidden_touched: Vec<usize>,
}

pub const INITIAL_FORBIDDEN_COLORS: usize = 1000;
const INITIAL_COLOR: u64 = INITIAL_FORBIDDEN_COLORS as u64;

impl K1ColoringComputationRuntime {
    pub fn new(node_count: usize, max_iterations: u64) -> Self {
        Self {
            colors: vec![INITIAL_COLOR; node_count],
            nodes_to_color_current: BitSet::new(node_count),
            nodes_to_color_next: BitSet::new(node_count),
            max_iterations,
            ran_iterations: 0,
            forbidden_colors: BitSet::new(INITIAL_FORBIDDEN_COLORS),
            forbidden_touched: Vec::new(),
        }
    }

    /// Run K1Coloring algorithm
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> K1ColoringComputationResult {
        // (Re)initialize state for this run.
        if self.colors.len() != node_count {
            self.colors = vec![INITIAL_COLOR; node_count];
            self.nodes_to_color_current = BitSet::new(node_count);
            self.nodes_to_color_next = BitSet::new(node_count);
        } else {
            self.colors.fill(INITIAL_COLOR);
            self.nodes_to_color_current.clear_all();
            self.nodes_to_color_next.clear_all();
        }
        self.ran_iterations = 0;
        self.forbidden_colors.clear_all();
        self.forbidden_touched.clear();

        // Initialize: all nodes need coloring.
        for i in 0..node_count {
            self.nodes_to_color_current.set(i);
        }

        // Iterative coloring and validation
        while self.ran_iterations < self.max_iterations
            && self.nodes_to_color_current.cardinality() > 0
        {
            // Phase 1: Color all nodes in current set
            self.coloring_phase(node_count, &get_neighbors);

            // Phase 2: Validate and mark conflicts for next iteration
            self.validation_phase(node_count, &get_neighbors);

            self.ran_iterations += 1;

            // Swap for next iteration
            std::mem::swap(
                &mut self.nodes_to_color_current,
                &mut self.nodes_to_color_next,
            );
        }

        let did_converge = self.ran_iterations < self.max_iterations;

        K1ColoringComputationResult {
            colors: self.colors.clone(),
            ran_iterations: self.ran_iterations,
            did_converge,
        }
    }

    /// Phase 1: Assign colors to all nodes in current set
    fn coloring_phase(&mut self, node_count: usize, get_neighbors: &impl Fn(usize) -> Vec<usize>) {
        let mut maybe = self.nodes_to_color_current.next_set_bit(0);
        while let Some(node_id) = maybe {
            // Mark neighbor colors as forbidden (sparse reset).
            let neighbors = get_neighbors(node_id);
            for &neighbor in &neighbors {
                if neighbor == node_id || neighbor >= node_count {
                    continue;
                }
                let color_idx = self.colors[neighbor] as usize;
                if !self.forbidden_colors.get(color_idx) {
                    self.forbidden_colors.set(color_idx);
                    self.forbidden_touched.push(color_idx);
                }
            }

            let mut next_color: usize = 0;
            while self.forbidden_colors.get(next_color) {
                next_color += 1;
            }
            self.colors[node_id] = next_color as u64;

            for &idx in &self.forbidden_touched {
                self.forbidden_colors.clear(idx);
            }
            self.forbidden_touched.clear();

            maybe = self.nodes_to_color_current.next_set_bit(node_id + 1);
        }
    }

    /// Phase 2: Validate coloring and mark conflicts
    fn validation_phase(
        &mut self,
        node_count: usize,
        get_neighbors: &impl Fn(usize) -> Vec<usize>,
    ) {
        self.nodes_to_color_next.clear_all();

        let mut maybe = self.nodes_to_color_current.next_set_bit(0);
        while let Some(node_id) = maybe {
            let node_color = self.colors[node_id];
            let neighbors = get_neighbors(node_id);

            for &neighbor in &neighbors {
                if neighbor == node_id || neighbor >= node_count {
                    continue;
                }

                // Mirrors Java:
                // if colors[source]==colors[target] && !nextNodesToColor.get(target) { nextNodesToColor.set(source); break; }
                if node_color == self.colors[neighbor] && !self.nodes_to_color_next.get(neighbor) {
                    self.nodes_to_color_next.set(node_id);
                    break;
                }
            }

            maybe = self.nodes_to_color_current.next_set_bit(node_id + 1);
        }
    }
}
