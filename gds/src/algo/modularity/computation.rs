use super::spec::{CommunityModularity, ModularityResult};
use std::collections::HashMap;
use std::time::Duration;

pub struct ModularityComputationRuntime {}

impl ModularityComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    /// Computes modularity for a given community assignment.
    ///
    /// Assumptions:
    /// - `get_neighbors(i)` yields weighted neighbors for node `i`.
    /// - For an undirected graph projection, the neighbor iteration is expected to
    ///   behave like adjacency: each undirected edge contributes to both endpoint
    ///   neighbor lists. In that common case, the accumulated `total_relationship_weight`
    ///   equals $2m$.
    pub fn compute(
        &self,
        node_count: usize,
        get_community: impl Fn(usize) -> Option<u64>,
        get_neighbors: impl Fn(usize) -> Vec<(usize, f64)>,
    ) -> ModularityResult {
        if node_count == 0 {
            return ModularityResult {
                node_count,
                total_relationship_weight: 0.0,
                total_modularity: 0.0,
                community_count: 0,
                community_modularities: Vec::new(),
                execution_time: Duration::default(),
            };
        }

        // Map arbitrary community ids to dense indices.
        let mut community_to_index: HashMap<u64, usize> = HashMap::new();
        let mut index_to_community: Vec<u64> = Vec::new();

        let mut node_community_index: Vec<Option<usize>> = vec![None; node_count];
        for node in 0..node_count {
            if let Some(comm) = get_community(node) {
                let idx = *community_to_index.entry(comm).or_insert_with(|| {
                    let next = index_to_community.len();
                    index_to_community.push(comm);
                    next
                });
                node_community_index[node] = Some(idx);
            }
        }

        let community_count = index_to_community.len();
        if community_count == 0 {
            return ModularityResult {
                node_count,
                total_relationship_weight: 0.0,
                total_modularity: 0.0,
                community_count: 0,
                community_modularities: Vec::new(),
                execution_time: Duration::default(),
            };
        }

        let mut inside_weight: Vec<f64> = vec![0.0; community_count];
        let mut community_degree: Vec<f64> = vec![0.0; community_count];
        let mut total_relationship_weight = 0.0;

        for node in 0..node_count {
            let Some(node_comm_idx) = node_community_index[node] else {
                // Nodes without a community are ignored (parity with Java behavior where missing/invalid seed is mapped out).
                continue;
            };

            let neighbors = get_neighbors(node);
            let mut node_degree = 0.0;

            for (nbr, w) in neighbors {
                node_degree += w;
                if nbr < node_count {
                    if let Some(nbr_comm_idx) = node_community_index[nbr] {
                        if nbr_comm_idx == node_comm_idx {
                            inside_weight[node_comm_idx] += w;
                        }
                    }
                }
            }

            community_degree[node_comm_idx] += node_degree;
            total_relationship_weight += node_degree;
        }

        if total_relationship_weight == 0.0 {
            return ModularityResult {
                node_count,
                total_relationship_weight,
                total_modularity: 0.0,
                community_count,
                community_modularities: index_to_community
                    .into_iter()
                    .map(|community_id| CommunityModularity {
                        community_id,
                        modularity: 0.0,
                    })
                    .collect(),
                execution_time: Duration::default(),
            };
        }

        // Using the standard modularity decomposition:
        // Q = sum_c ( e_c - a_c^2 ) where
        // e_c = (sum_{i,j in c} A_ij) / (2m) and a_c = (sum_{i in c} k_i) / (2m)
        // Here, `total_relationship_weight` acts as 2m.
        let mut total_modularity = 0.0;
        let mut community_modularities = Vec::with_capacity(community_count);
        for (idx, community_id) in index_to_community.into_iter().enumerate() {
            let e_c = inside_weight[idx] / total_relationship_weight;
            let a_c = community_degree[idx] / total_relationship_weight;
            let q_c = e_c - (a_c * a_c);
            total_modularity += q_c;
            community_modularities.push(CommunityModularity {
                community_id,
                modularity: q_c,
            });
        }

        ModularityResult {
            node_count,
            total_relationship_weight,
            total_modularity,
            community_count,
            community_modularities,
            execution_time: Duration::default(),
        }
    }
}

impl Default for ModularityComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}
