//! SCC Computation Runtime
//!
//! This module implements a correct Strongly Connected Components (SCC)
//! computation using an iterative Kosaraju algorithm.
//!
//! Notes:
//! - We intentionally implement the algorithm directly (no recursive DFS) to
//!   avoid stack overflows on large graphs.
//! - We do not copy Neo4j's Java source; we align behavior/semantics.

use crate::concurrency::TerminationFlag;

/// SCC computation result
#[derive(Debug, Clone)]
pub struct SccComputationResult {
    /// Component ID for each node
    pub components: Vec<u64>,
    /// Number of strongly connected components found
    pub component_count: usize,
    /// Computation time in milliseconds
    pub computation_time_ms: u64,
}

impl SccComputationResult {
    /// Create a new SCC computation result
    pub fn new(components: Vec<u64>, component_count: usize, computation_time_ms: u64) -> Self {
        Self {
            components,
            component_count,
            computation_time_ms,
        }
    }
}

/// SCC computation runtime.
///
/// The runtime is currently stateless; it exists to mirror the
/// storage/computation split used across procedures.
#[derive(Debug, Default, Clone)]
pub struct SccComputationRuntime;

impl SccComputationRuntime {
    pub fn new() -> Self {
        Self
    }

    /// Compute SCCs given both outgoing and incoming adjacency lists.
    ///
    /// Returns a vector mapping node index → component id, and the number of components.
    pub fn compute(
        &mut self,
        outgoing: &[Vec<usize>],
        incoming: &[Vec<usize>],
        termination_flag: &TerminationFlag,
    ) -> Result<(Vec<u64>, usize), String> {
        let node_count = outgoing.len();
        if incoming.len() != node_count {
            return Err("incoming adjacency size mismatch".to_string());
        }

        // First pass: compute finishing order on the original graph.
        let mut visited = vec![false; node_count];
        let mut order: Vec<usize> = Vec::with_capacity(node_count);

        for start in 0..node_count {
            if !termination_flag.running() {
                return Err("Algorithm terminated by user".to_string());
            }
            if visited[start] {
                continue;
            }

            // Iterative DFS with explicit neighbor index.
            let mut stack: Vec<(usize, usize)> = Vec::new();
            visited[start] = true;
            stack.push((start, 0));

            while let Some((node, next_idx)) = stack.pop() {
                if !termination_flag.running() {
                    return Err("Algorithm terminated by user".to_string());
                }

                if next_idx < outgoing[node].len() {
                    // Re-push current frame with advanced cursor.
                    stack.push((node, next_idx + 1));

                    let neighbor = outgoing[node][next_idx];
                    if neighbor < node_count && !visited[neighbor] {
                        visited[neighbor] = true;
                        stack.push((neighbor, 0));
                    }
                } else {
                    // All neighbors processed.
                    order.push(node);
                }
            }
        }

        // Second pass: traverse the reversed graph in reverse finishing order.
        let mut assigned = vec![false; node_count];
        let mut components: Vec<u64> = vec![u64::MAX; node_count];
        let mut component_id: u64 = 0;

        for &start in order.iter().rev() {
            if !termination_flag.running() {
                return Err("Algorithm terminated by user".to_string());
            }
            if assigned[start] {
                continue;
            }

            let mut stack: Vec<usize> = vec![start];
            assigned[start] = true;
            components[start] = component_id;

            while let Some(node) = stack.pop() {
                if !termination_flag.running() {
                    return Err("Algorithm terminated by user".to_string());
                }

                for &neighbor in &incoming[node] {
                    if neighbor < node_count && !assigned[neighbor] {
                        assigned[neighbor] = true;
                        components[neighbor] = component_id;
                        stack.push(neighbor);
                    }
                }
            }

            component_id += 1;
        }

        Ok((components, component_id as usize))
    }
}
