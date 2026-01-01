//! KCore Decomposition Computation Runtime
//!
//! **Translation Source**: `org.neo4j.gds.kcore.KCoreDecomposition`
//!
//! K-core decomposition finds maximal subgraphs where every node has degree >= k.
//! Uses iterative SCAN/ACT phases to progressively remove low-degree nodes.

const UNASSIGNED: i32 = -1;

/// K-core decomposition result
#[derive(Clone)]
pub struct KCoreComputationResult {
    pub core_values: Vec<i32>,
    pub degeneracy: i32,
}

/// K-core decomposition computation runtime
pub struct KCoreComputationRuntime {
    // Placeholder for future state if needed
}

impl Default for KCoreComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl KCoreComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    /// Compute k-core decomposition for a graph
    pub fn compute(
        &mut self,
        node_count: usize,
        get_neighbors: impl Fn(usize) -> Vec<usize>,
    ) -> KCoreComputationResult {
        if node_count == 0 {
            return KCoreComputationResult {
                core_values: Vec::new(),
                degeneracy: 0,
            };
        }

        // Initialize degrees and core values
        let mut current_degrees = vec![0i32; node_count];
        let mut core_values = vec![UNASSIGNED; node_count];
        let mut remaining_nodes = 0usize;

        for node_id in 0..node_count {
            let neighbors = get_neighbors(node_id);
            let degree = neighbors.len() as i32;
            current_degrees[node_id] = degree;

            if degree == 0 {
                core_values[node_id] = 0;
            } else {
                remaining_nodes += 1;
            }
        }

        // Iteratively find k-cores
        let mut scanning_degree = 1i32;
        let mut degeneracy = 0i32;
        let mut examination_stack: Vec<usize> = Vec::new();

        while remaining_nodes > 0 {
            // SCAN Phase: Find nodes with degree == scanning_degree
            examination_stack.clear();
            let mut smallest_active_degree = i32::MAX;

            for node_id in 0..node_count {
                if core_values[node_id] == UNASSIGNED {
                    let node_degree = current_degrees[node_id];
                    if node_degree >= scanning_degree {
                        if node_degree == scanning_degree {
                            examination_stack.push(node_id);
                        }
                        smallest_active_degree = smallest_active_degree.min(node_degree);
                    }
                }
            }

            if smallest_active_degree == i32::MAX {
                break;
            }

            if smallest_active_degree == scanning_degree {
                // ACT Phase: Process all nodes at current scanning degree
                degeneracy = scanning_degree;
                let mut nodes_examined = 0usize;

                while let Some(node_id) = examination_stack.pop() {
                    core_values[node_id] = scanning_degree;
                    nodes_examined += 1;

                    // Relax: decrement neighbor degrees and push if they reach scanning_degree
                    let neighbors = get_neighbors(node_id);
                    for &neighbor in &neighbors {
                        if neighbor >= node_count || core_values[neighbor] != UNASSIGNED {
                            continue;
                        }

                        current_degrees[neighbor] -= 1;
                        if current_degrees[neighbor] == scanning_degree {
                            examination_stack.push(neighbor);
                        }
                    }
                }

                remaining_nodes -= nodes_examined;
                scanning_degree += 1;
            } else {
                // Jump to next smallest active degree
                scanning_degree = smallest_active_degree;
            }
        }

        KCoreComputationResult {
            core_values,
            degeneracy,
        }
    }
}
