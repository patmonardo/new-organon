//! ApproxMaxKCut storage: community assignments and improvement tracking

use std::sync::Mutex;

/// Storage for approximate maximum k-cut computation
pub struct ApproxMaxKCutStorageRuntime {
    /// Current community assignment for each node
    pub communities: Mutex<Vec<u8>>,
    /// Best community assignment found so far
    pub best_communities: Mutex<Vec<u8>>,
    /// Current cut cost
    pub current_cost: Mutex<f64>,
    /// Best cut cost found so far
    pub best_cost: Mutex<f64>,
    /// Node-to-community weights for local search (node_count * k sized)
    pub node_to_community_weights: Mutex<Vec<f64>>,
}

impl ApproxMaxKCutStorageRuntime {
    pub fn new(node_count: usize, k: u8, minimize: bool) -> Self {
        let initial_cost = if minimize { f64::MAX } else { f64::MIN };

        Self {
            communities: Mutex::new(vec![0; node_count]),
            best_communities: Mutex::new(vec![0; node_count]),
            current_cost: Mutex::new(0.0),
            best_cost: Mutex::new(initial_cost),
            node_to_community_weights: Mutex::new(vec![0.0; node_count * k as usize]),
        }
    }

    /// Get current communities
    pub fn get_communities(&self) -> Vec<u8> {
        self.communities.lock().unwrap().clone()
    }

    /// Set communities
    pub fn set_communities(&self, communities: Vec<u8>) {
        *self.communities.lock().unwrap() = communities;
    }

    /// Get best solution found
    pub fn get_best_solution(&self) -> (Vec<u8>, f64) {
        let communities = self.best_communities.lock().unwrap().clone();
        let cost = *self.best_cost.lock().unwrap();
        (communities, cost)
    }

    /// Update best solution if current is better
    pub fn update_best_if_improved(&self, minimize: bool) -> bool {
        let current_cost = *self.current_cost.lock().unwrap();
        let mut best_cost = self.best_cost.lock().unwrap();

        let is_better = if minimize {
            current_cost < *best_cost
        } else {
            current_cost > *best_cost
        };

        if is_better {
            *best_cost = current_cost;
            let current_communities = self.communities.lock().unwrap().clone();
            *self.best_communities.lock().unwrap() = current_communities;
            true
        } else {
            false
        }
    }
}
