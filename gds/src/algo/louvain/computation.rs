use super::spec::LouvainResult;
use crate::algo::modularity_optimization::{
    ModularityOptimizationComputationRuntime, ModularityOptimizationConfig,
    ModularityOptimizationInput,
};
use std::collections::HashMap;
use std::time::Duration;

pub struct LouvainComputationRuntime {
    max_levels: usize,
    tolerance: f64,
    gamma: f64,
}

impl LouvainComputationRuntime {
    pub fn new() -> Self {
        Self {
            max_levels: 10,
            tolerance: 1e-6,
            gamma: 1.0,
        }
    }

    pub fn compute(&mut self, input: &ModularityOptimizationInput) -> LouvainResult {
        if input.node_count == 0 {
            return LouvainResult {
                data: Vec::new(),
                node_count: 0,
                execution_time: Duration::default(),
            };
        }

        let original_node_count = input.node_count;
        let mut mapping: Vec<usize> = (0..original_node_count).collect();

        let mut working_graph = input.clone();
        let mut last_modularity = f64::NEG_INFINITY;

        let mut modopt = ModularityOptimizationComputationRuntime::new();
        let modopt_config = ModularityOptimizationConfig {
            max_iterations: 20,
            tolerance: self.tolerance,
            gamma: self.gamma,
        };

        for _level in 0..self.max_levels {
            let degrees = working_graph.degrees();
            let two_m = working_graph.total_relationship_weight(&degrees);
            if two_m == 0.0 {
                let mut data = vec![0u64; original_node_count];
                for i in 0..original_node_count {
                    data[i] = mapping[i] as u64;
                }
                return LouvainResult {
                    data,
                    node_count: original_node_count,
                    execution_time: Duration::default(),
                };
            }

            // Java parity: run modularity optimization for this level.
            let level_result = modopt.compute(&working_graph, &modopt_config);
            let compressed: Vec<usize> = level_result
                .communities
                .iter()
                .map(|&c| c as usize)
                .collect();
            let new_node_count = 1 + compressed.iter().copied().max().unwrap_or(0);

            // Update original-node mapping through this level.
            for i in 0..original_node_count {
                let current_node = mapping[i];
                mapping[i] = compressed[current_node];
            }

            let modularity_now = level_result.modularity;

            let improvement = modularity_now - last_modularity;
            last_modularity = modularity_now;

            // Stop if no improvement or no aggregation possible.
            if improvement.abs() <= self.tolerance || new_node_count == working_graph.node_count {
                break;
            }

            // Build the next level graph.
            working_graph = aggregate(&working_graph, &compressed, new_node_count);
        }

        let mut data = vec![0u64; original_node_count];
        for i in 0..original_node_count {
            data[i] = mapping[i] as u64;
        }
        LouvainResult {
            data,
            node_count: original_node_count,
            execution_time: Duration::default(),
        }
    }
}

fn aggregate(
    graph: &ModularityOptimizationInput,
    assignment: &[usize],
    new_node_count: usize,
) -> ModularityOptimizationInput {
    // Build undirected edges between communities by aggregating weights.
    // We process each undirected edge once by only counting (u,v) where u < v.
    let mut edge_weights: HashMap<(usize, usize), f64> = HashMap::new();

    for u in 0..graph.node_count {
        let cu = assignment[u];
        for &(v, w) in &graph.adj[u] {
            if u >= v {
                continue;
            }
            let cv = assignment[v];
            let (a, b) = if cu <= cv { (cu, cv) } else { (cv, cu) };
            *edge_weights.entry((a, b)).or_insert(0.0) += w;
        }
    }

    let mut new_adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); new_node_count];
    for ((a, b), w) in edge_weights {
        if a == b {
            new_adj[a].push((b, w));
        } else {
            new_adj[a].push((b, w));
            new_adj[b].push((a, w));
        }
    }

    ModularityOptimizationInput::new(new_node_count, new_adj)
}

impl Default for LouvainComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}
