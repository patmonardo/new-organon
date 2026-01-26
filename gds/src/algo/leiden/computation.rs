//! Leiden computation runtime.
//!
//! This is a phase-structured implementation inspired by the Java GDS Leiden pipeline:
//! - init volumes
//! - local move (queue-based)
//! - refinement (ensure connected communities)
//! - aggregation (contract graph and iterate levels)

use super::spec::{LeidenConfig, LeidenResult};
use crate::concurrency::TerminationFlag;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Duration;

#[derive(Clone, Debug)]
pub(crate) struct AdjacencyGraph {
    pub node_count: usize,
    /// Undirected adjacency list.
    pub adj: Vec<Vec<(usize, f64)>>,
}

impl AdjacencyGraph {
    pub fn new(node_count: usize, adj: Vec<Vec<(usize, f64)>>) -> Self {
        Self { node_count, adj }
    }

    pub fn degree(&self, node: usize) -> f64 {
        self.adj[node].iter().map(|(_, w)| *w).sum()
    }

    pub fn total_edge_weight(&self) -> f64 {
        let total_degree: f64 = (0..self.node_count).map(|i| self.degree(i)).sum();
        total_degree / 2.0
    }
}

#[derive(Clone, Debug)]
pub struct LeidenComputationResult {
    pub communities: Vec<u64>,
    pub modularity: f64,
    pub levels: usize,
    pub converged: bool,
}

pub struct LeidenComputationRuntime {}

impl LeidenComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub(crate) fn compute(
        &mut self,
        graph: &AdjacencyGraph,
        config: &LeidenConfig,
        termination_flag: &TerminationFlag,
    ) -> Result<LeidenComputationResult, String> {
        let n = graph.node_count;
        if n == 0 {
            return Ok(LeidenComputationResult {
                communities: Vec::new(),
                modularity: 0.0,
                levels: 0,
                converged: true,
            });
        }

        // === Init volumes ===
        // Total edge weight `m` changes after aggregation, so we treat it as a per-level value.
        let mut m = graph.total_edge_weight();
        if m <= 0.0 {
            let communities = renumber_communities(starting_communities(n, config));
            return Ok(LeidenComputationResult {
                communities,
                modularity: 0.0,
                levels: 0,
                converged: true,
            });
        }

        let mut rng = StdRng::seed_from_u64(config.random_seed);

        // Dendrogram-lift: for each original node, track the current (working-graph) node id.
        // At level 0, working nodes match original nodes.
        let mut original_to_working: Vec<usize> = (0..n).collect();

        // Working graph + assignments at current level.
        let mut working_graph = graph.clone();
        let mut working_communities: Vec<u64> =
            renumber_communities(starting_communities(n, config));

        // Output communities over original nodes; updated after each level.
        let mut output_communities = working_communities.clone();

        let mut last_modularity = modularity(&working_graph, &working_communities, m, config.gamma);
        let mut levels = 0usize;
        let mut converged = false;

        for _level in 0..config.max_iterations {
            termination_flag.assert_running();
            levels += 1;

            // Recompute total edge weight for this working graph level.
            m = working_graph.total_edge_weight();
            if m <= 0.0 {
                converged = true;
                break;
            }

            let node_volumes = node_volumes_for(&working_graph);

            // Local move (queue-based) + refinement on the current working graph.
            local_move_phase(
                &working_graph,
                &mut working_communities,
                &node_volumes,
                m,
                config.gamma,
                termination_flag,
            )?;

            refinement_phase(
                &working_graph,
                &mut working_communities,
                config.theta,
                &mut rng,
                termination_flag,
            )?;

            // Renumber to keep IDs dense (helps aggregation).
            working_communities = renumber_communities(working_communities);

            let new_modularity = modularity(&working_graph, &working_communities, m, config.gamma);
            let improvement = new_modularity - last_modularity;
            last_modularity = new_modularity;

            // Lift community ids back to original nodes.
            output_communities = vec![0u64; n];
            for (original, &working_node) in original_to_working.iter().enumerate() {
                output_communities[original] = working_communities[working_node];
            }

            if improvement.abs() < config.tolerance {
                converged = true;
                break;
            }

            // Aggregation: contract communities into a smaller graph.
            let (next_graph, next_mapping) = aggregate_graph(&working_graph, &working_communities);
            if next_graph.node_count == working_graph.node_count {
                // No change in number of nodes => no further meaningful aggregation.
                converged = true;
                break;
            }

            // Update the original->working mapping by translating each node's working id
            // through its community-to-aggregated-node mapping.
            for w in &mut original_to_working {
                *w = next_mapping[*w];
            }

            working_graph = next_graph;
            working_communities = (0..working_graph.node_count as u64).collect();
        }

        let final_modularity = last_modularity;

        Ok(LeidenComputationResult {
            communities: output_communities,
            modularity: final_modularity,
            levels,
            converged,
        })
    }

    pub fn into_result(result: LeidenComputationResult) -> LeidenResult {
        let community_count = unique_count(&result.communities) as u64;
        let node_count = result.communities.len();
        LeidenResult {
            communities: result.communities,
            community_count,
            modularity: result.modularity,
            levels: result.levels,
            converged: result.converged,
            node_count,
            execution_time: Duration::default(),
        }
    }
}

impl Default for LeidenComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

fn node_volumes_for(graph: &AdjacencyGraph) -> Vec<f64> {
    (0..graph.node_count).map(|i| graph.degree(i)).collect()
}

fn starting_communities(node_count: usize, config: &LeidenConfig) -> Vec<u64> {
    if let Some(seeds) = &config.seed_communities {
        if seeds.len() == node_count {
            return seeds.clone();
        }
        if seeds.is_empty() {
            // Treat empty as "no seeds".
        }
    }
    (0..node_count as u64).collect()
}

fn local_move_phase(
    graph: &AdjacencyGraph,
    communities: &mut [u64],
    node_volumes: &[f64],
    m: f64,
    gamma: f64,
    termination_flag: &TerminationFlag,
) -> Result<(), String> {
    let n = graph.node_count;

    // Community total volume (sum of node volumes).
    let mut community_totals: HashMap<u64, f64> = HashMap::new();
    for i in 0..n {
        *community_totals.entry(communities[i]).or_insert(0.0) += node_volumes[i];
    }

    let mut in_queue = vec![true; n];
    let mut queue: VecDeque<usize> = (0..n).collect();

    while let Some(node) = queue.pop_front() {
        termination_flag.assert_running();
        in_queue[node] = false;

        let current = communities[node];
        let k_i = node_volumes[node];
        if k_i == 0.0 {
            continue;
        }

        // Sum weights to each neighboring community.
        let mut w_to_comm: HashMap<u64, f64> = HashMap::new();
        for (nbr, w) in &graph.adj[node] {
            let c = communities[*nbr];
            *w_to_comm.entry(c).or_insert(0.0) += *w;
        }

        let w_to_current = *w_to_comm.get(&current).unwrap_or(&0.0);
        let d_a = *community_totals.get(&current).unwrap_or(&0.0);

        let mut best = current;
        let mut best_delta = 0.0;

        for (&candidate, &w_to_candidate) in &w_to_comm {
            if candidate == current {
                continue;
            }

            let d_b = *community_totals.get(&candidate).unwrap_or(&0.0);

            // ΔQ derived from community contribution changes (see module docs).
            let delta = (w_to_candidate - w_to_current) / m
                - gamma * (k_i * (d_b - d_a) + k_i * k_i) / (2.0 * m * m);

            if delta > best_delta {
                best_delta = delta;
                best = candidate;
            }
        }

        if best != current && best_delta > 1e-12 {
            communities[node] = best;

            *community_totals.entry(current).or_insert(0.0) -= k_i;
            *community_totals.entry(best).or_insert(0.0) += k_i;

            // Add neighbors to queue.
            for (nbr, _) in &graph.adj[node] {
                if !in_queue[*nbr] {
                    in_queue[*nbr] = true;
                    queue.push_back(*nbr);
                }
            }
        }
    }

    Ok(())
}

fn refinement_phase(
    graph: &AdjacencyGraph,
    communities: &mut [u64],
    theta: f64,
    rng: &mut StdRng,
    termination_flag: &TerminationFlag,
) -> Result<(), String> {
    // Java refinement is probabilistic; we implement the key invariant: communities must be connected.
    // `theta` is used as a minor random tie-breaker to decide which component keeps the original id.

    let n = graph.node_count;
    let mut visited = vec![false; n];
    let mut seen_first_component: HashSet<u64> = HashSet::new();
    let mut next_comm = communities.iter().copied().max().unwrap_or(0) + 1;

    for start in 0..n {
        termination_flag.assert_running();

        if visited[start] {
            continue;
        }

        let original_comm = communities[start];

        // BFS restricted to nodes of the same original community.
        let mut stack = vec![start];
        visited[start] = true;
        let mut component_nodes = vec![start];

        while let Some(u) = stack.pop() {
            for (v, _) in &graph.adj[u] {
                let v = *v;
                if visited[v] {
                    continue;
                }
                if communities[v] != original_comm {
                    continue;
                }
                visited[v] = true;
                stack.push(v);
                component_nodes.push(v);
            }
        }

        if seen_first_component.insert(original_comm) {
            continue;
        }

        // With some probability controlled by theta, we keep the original id for this component
        // and re-label the earlier component instead. This is a tiny nod toward Java's randomness.
        let swap_ids = theta > 0.0 && rng.gen::<f64>() < theta;

        if swap_ids {
            // Re-label *all nodes currently in original_comm* that are NOT in this component.
            // This keeps the component's id stable.
            let relabel_to = next_comm;
            next_comm += 1;

            let component_set: HashSet<usize> = component_nodes.iter().copied().collect();
            for node in 0..n {
                if communities[node] == original_comm && !component_set.contains(&node) {
                    communities[node] = relabel_to;
                }
            }
        } else {
            // Normal behavior: split this disconnected component into a new community.
            let new_id = next_comm;
            next_comm += 1;
            for node in component_nodes {
                communities[node] = new_id;
            }
        }
    }

    Ok(())
}

fn aggregate_graph(graph: &AdjacencyGraph, communities: &[u64]) -> (AdjacencyGraph, Vec<usize>) {
    let n = graph.node_count;
    if n == 0 {
        return (AdjacencyGraph::new(0, Vec::new()), Vec::new());
    }

    // Map community ids to compact [0..k) ids.
    let mut comm_ids = communities.to_vec();
    comm_ids.sort_unstable();
    comm_ids.dedup();

    let mut comm_to_new: HashMap<u64, usize> = HashMap::with_capacity(comm_ids.len());
    for (idx, c) in comm_ids.into_iter().enumerate() {
        comm_to_new.insert(c, idx);
    }

    let k = comm_to_new.len();

    // Node->aggregated node mapping (via its community).
    let mut node_to_agg: Vec<usize> = vec![0; n];
    for node in 0..n {
        node_to_agg[node] = *comm_to_new.get(&communities[node]).unwrap();
    }

    // Aggregate inter-community edges.
    let mut edge_weights: HashMap<(usize, usize), f64> = HashMap::new();
    for u in 0..n {
        let cu = node_to_agg[u];
        for (v, w) in &graph.adj[u] {
            if u < *v {
                let cv = node_to_agg[*v];
                if cu == cv {
                    // Self-loop handling is intentionally skipped (Java has TODOs for it).
                    continue;
                }
                let (a, b) = if cu < cv { (cu, cv) } else { (cv, cu) };
                *edge_weights.entry((a, b)).or_insert(0.0) += *w;
            }
        }
    }

    let mut adj = vec![Vec::new(); k];
    for ((a, b), w) in edge_weights {
        adj[a].push((b, w));
        adj[b].push((a, w));
    }

    (AdjacencyGraph::new(k, adj), node_to_agg)
}

fn modularity(graph: &AdjacencyGraph, communities: &[u64], m: f64, gamma: f64) -> f64 {
    if m <= 0.0 {
        return 0.0;
    }

    let n = graph.node_count;

    let mut tot: HashMap<u64, f64> = HashMap::new();
    for i in 0..n {
        let k_i = graph.degree(i);
        *tot.entry(communities[i]).or_insert(0.0) += k_i;
    }

    // Internal edge weights per community, counting each undirected edge once.
    let mut internal: HashMap<u64, f64> = HashMap::new();
    for u in 0..n {
        let cu = communities[u];
        for (v, w) in &graph.adj[u] {
            if u < *v && cu == communities[*v] {
                *internal.entry(cu).or_insert(0.0) += *w;
            }
        }
    }

    let mut q = 0.0;
    for (&c, &d_c) in &tot {
        let l_c = *internal.get(&c).unwrap_or(&0.0);
        q += l_c / m - gamma * (d_c / (2.0 * m)).powi(2);
    }

    q
}

fn renumber_communities(mut communities: Vec<u64>) -> Vec<u64> {
    if communities.is_empty() {
        return communities;
    }

    let mut ids = communities.clone();
    ids.sort_unstable();
    ids.dedup();

    let mut map: HashMap<u64, u64> = HashMap::with_capacity(ids.len());
    for (new, old) in ids.into_iter().enumerate() {
        map.insert(old, new as u64);
    }

    for c in &mut communities {
        *c = *map.get(c).unwrap_or(c);
    }

    communities
}

fn unique_count(communities: &[u64]) -> usize {
    let mut ids = communities.to_vec();
    ids.sort_unstable();
    ids.dedup();
    ids.len()
}
