//! ApproxMaxKCut computation: GRASP with local search

use super::spec::{ApproxMaxKCutConfig, ApproxMaxKCutResult};
use super::storage::ApproxMaxKCutStorageRuntime;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::sync::Arc;

pub struct ApproxMaxKCutComputationRuntime {
    config: ApproxMaxKCutConfig,
}

impl ApproxMaxKCutComputationRuntime {
    pub fn new(config: ApproxMaxKCutConfig) -> Self {
        Self { config }
    }

    /// Compute approximate maximum k-cut
    ///
    /// # Arguments
    /// * `node_count` - Total number of nodes in the graph
    /// * `get_neighbors` - Closure that returns (target, weight) pairs for a node's neighbors
    pub fn compute<G>(&self, node_count: usize, get_neighbors: G) -> ApproxMaxKCutResult
    where
        G: Fn(usize) -> Vec<(usize, f64)> + Sync,
    {
        let storage = Arc::new(ApproxMaxKCutStorageRuntime::new(
            node_count,
            self.config.k,
            self.config.minimize,
        ));

        let mut rng = StdRng::seed_from_u64(self.config.random_seed);

        // GRASP iterations
        for _iteration in 0..self.config.iterations {
            // Phase 1: Random placement
            self.place_nodes_randomly(&storage, node_count, &mut rng);

            // Phase 2: Local search for improvements
            self.local_search(&storage, node_count, &get_neighbors);

            // Update best solution if improved
            storage.update_best_if_improved(self.config.minimize);
        }

        let (best_communities, best_cost) = storage.get_best_solution();

        ApproxMaxKCutResult {
            communities: best_communities,
            cut_cost: best_cost,
        }
    }

    /// Randomly assign nodes to communities
    fn place_nodes_randomly(
        &self,
        storage: &Arc<ApproxMaxKCutStorageRuntime>,
        node_count: usize,
        rng: &mut StdRng,
    ) {
        let mut communities = vec![0u8; node_count];
        let mut cardinalities = vec![0usize; self.config.k as usize];

        // First, ensure minimum community sizes are met
        let mut node_idx = 0;
        for (community, &min_size) in self.config.min_community_sizes.iter().enumerate() {
            for _ in 0..min_size {
                if node_idx < node_count {
                    communities[node_idx] = community as u8;
                    cardinalities[community] += 1;
                    node_idx += 1;
                }
            }
        }

        // Assign remaining nodes randomly
        for i in node_idx..node_count {
            let community = rng.gen_range(0..self.config.k);
            communities[i] = community;
            cardinalities[community as usize] += 1;
        }

        // Shuffle to randomize
        for i in (1..node_count).rev() {
            let j = rng.gen_range(0..=i);
            communities.swap(i, j);
        }

        storage.set_communities(communities);
    }

    /// Local search: iteratively swap nodes to improve cut cost
    fn local_search<G>(
        &self,
        storage: &Arc<ApproxMaxKCutStorageRuntime>,
        node_count: usize,
        get_neighbors: &G,
    ) where
        G: Fn(usize) -> Vec<(usize, f64)> + Sync,
    {
        const MAX_ITERATIONS: usize = 100;
        let mut improved = true;
        let mut iterations = 0;

        while improved && iterations < MAX_ITERATIONS {
            improved = false;
            iterations += 1;

            // Compute node-to-community weights
            self.compute_node_to_community_weights(storage, node_count, get_neighbors);

            // Try swapping each node to a better community
            let communities = storage.get_communities();
            let mut new_communities = communities.clone();
            let mut cardinalities = vec![0usize; self.config.k as usize];

            for &c in &communities {
                cardinalities[c as usize] += 1;
            }

            for node in 0..node_count {
                let current_community = communities[node];
                let best_community = self.find_best_community_for_node(
                    storage,
                    node,
                    current_community,
                    &cardinalities,
                );

                if best_community != current_community {
                    // Check if we can move from current community (min size constraint)
                    if cardinalities[current_community as usize]
                        > self.config.min_community_sizes[current_community as usize]
                    {
                        new_communities[node] = best_community;
                        cardinalities[current_community as usize] -= 1;
                        cardinalities[best_community as usize] += 1;
                        improved = true;
                    }
                }
            }

            if improved {
                storage.set_communities(new_communities);
            }

            // Compute current cost
            self.compute_cost(storage, node_count, get_neighbors);
        }
    }

    /// Compute weights from each node to each community
    fn compute_node_to_community_weights<G>(
        &self,
        storage: &Arc<ApproxMaxKCutStorageRuntime>,
        node_count: usize,
        get_neighbors: &G,
    ) where
        G: Fn(usize) -> Vec<(usize, f64)>,
    {
        let communities = storage.get_communities();
        let k = self.config.k as usize;
        let mut weights = vec![0.0; node_count * k];

        for node in 0..node_count {
            let neighbors = get_neighbors(node);

            for (target, mut weight) in neighbors {
                if node == target {
                    continue; // Skip self-loops
                }

                if !self.config.has_relationship_weight_property {
                    weight = 1.0;
                }

                let target_community = communities[target] as usize;
                weights[node * k + target_community] += weight;
            }
        }

        *storage.node_to_community_weights.lock().unwrap() = weights;
    }

    /// Find the best community for a node based on improvement cost
    fn find_best_community_for_node(
        &self,
        storage: &Arc<ApproxMaxKCutStorageRuntime>,
        node: usize,
        current_community: u8,
        _cardinalities: &[usize],
    ) -> u8 {
        let weights = storage.node_to_community_weights.lock().unwrap();
        let k = self.config.k as usize;
        let node_offset = node * k;

        let mut best_community = current_community;
        let mut best_weight = weights[node_offset + current_community as usize];

        for community in 0..k {
            let weight = weights[node_offset + community];

            // For maximizing cut cost: prefer communities with LESS weight (more edges crossing)
            // For minimizing cut cost: prefer communities with MORE weight (fewer edges crossing)
            let is_better = if self.config.minimize {
                weight > best_weight
            } else {
                weight < best_weight
            };

            if is_better {
                best_weight = weight;
                best_community = community as u8;
            }
        }

        best_community
    }

    /// Compute the cut cost for the current solution
    fn compute_cost<G>(
        &self,
        storage: &Arc<ApproxMaxKCutStorageRuntime>,
        node_count: usize,
        get_neighbors: &G,
    ) where
        G: Fn(usize) -> Vec<(usize, f64)>,
    {
        let communities = storage.get_communities();
        let mut cost = 0.0;

        for node in 0..node_count {
            let node_community = communities[node];
            let neighbors = get_neighbors(node);

            for (target, mut weight) in neighbors {
                if !self.config.has_relationship_weight_property {
                    weight = 1.0;
                }

                let target_community = communities[target];
                if node_community != target_community {
                    cost += weight;
                }
            }
        }

        *storage.current_cost.lock().unwrap() = cost;
    }
}
