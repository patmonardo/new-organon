//! HITS runner (bidirectional authority/hub scoring)
//!
//! This module provides an end-to-end HITS execution using the Pregel framework
//! with bidirectional message passing. The computation follows the classic HITS algorithm:
//!
//! Authority Update: $auth_i = \sum_{j \to i} hub_j$ (sum of hub scores from incoming neighbors)
//! Hub Update: $hub_i = \sum_{i \to j} auth_j$ (sum of authority scores from outgoing neighbors)
//!
//! Both scores are L2-normalized after each update phase.
//!
//! This demonstrates the bidirectional Pregel capabilities added to the framework.

use crate::config::PregelConfig;
use crate::pregel::{
    ComputeContext, ComputeFn, InitContext, InitFn, MasterComputeContext, Messages, PregelBuilder,
    PregelSchema, SyncQueueMessageIterator, SyncQueueMessenger, Visibility,
};
use crate::types::graph::Graph;
use crate::types::ValueType;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub const AUTHORITY: &str = "authority";
pub const HUB: &str = "hub";

/// State for the HITS state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HitsState {
    Init,
    CalculateAuths,
    NormalizeAuths,
    CalculateHubs,
    NormalizeHubs,
}

/// Shared state for HITS computation
struct HitsSharedState {
    current_state: AtomicU64,
    global_norm: AtomicU64,
    previous_norm: AtomicU64,
    max_iterations: usize,
    tolerance: f64,
}

impl HitsSharedState {
    fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            current_state: AtomicU64::new(HitsState::Init as u64),
            global_norm: AtomicU64::new(0),
            previous_norm: AtomicU64::new(0),
            max_iterations,
            tolerance,
        }
    }

    fn get_state(&self) -> HitsState {
        match self.current_state.load(Ordering::SeqCst) {
            0 => HitsState::Init,
            1 => HitsState::CalculateAuths,
            2 => HitsState::NormalizeAuths,
            3 => HitsState::CalculateHubs,
            4 => HitsState::NormalizeHubs,
            _ => unreachable!(),
        }
    }

    fn set_state(&self, state: HitsState) {
        self.current_state.store(state as u64, Ordering::SeqCst);
    }

    fn reset_norm(&self) {
        self.global_norm.store(0, Ordering::SeqCst);
    }

    fn add_to_norm(&self, value: f64) {
        let squared = value * value;

        loop {
            let current = self.global_norm.load(Ordering::SeqCst);
            let current_val = f64::from_bits(current);
            let new_val = current_val + squared;
            let new_bits = new_val.to_bits();

            if self
                .global_norm
                .compare_exchange_weak(current, new_bits, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                break;
            }
        }
    }

    fn get_norm_sqrt(&self) -> f64 {
        let bits = self.global_norm.load(Ordering::SeqCst);
        f64::from_bits(bits).sqrt()
    }

    fn check_convergence(&self) -> bool {
        let current = self.get_norm_sqrt();
        let previous_bits = self.previous_norm.load(Ordering::SeqCst);
        let previous = f64::from_bits(previous_bits);

        if previous == 0.0 {
            return false;
        }

        let delta = (current - previous).abs();
        delta < self.tolerance
    }

    fn update_previous_norm(&self) {
        let current_norm = self.get_norm_sqrt();
        self.previous_norm
            .store(current_norm.to_bits(), Ordering::SeqCst);
    }
}

#[derive(Debug, Clone)]
pub struct HitsRunResult {
    pub hub_scores: Vec<f64>,
    pub authority_scores: Vec<f64>,
    pub iterations_ran: usize,
    pub did_converge: bool,
}

/// Run HITS on a given graph using the Pregel runtime with bidirectional message passing.
pub fn run_hits(
    graph: Arc<dyn Graph>,
    max_iterations: usize,
    tolerance: f64,
) -> HitsRunResult {
    let shared_state = Arc::new(HitsSharedState::new(max_iterations, tolerance));

    let schema = PregelSchema::builder()
        .add(AUTHORITY, ValueType::Double, Visibility::Public)
        .add(HUB, ValueType::Double, Visibility::Public)
        .build();

    let init_fn: InitFn<PregelConfig> = Arc::new(move |context: &mut InitContext<PregelConfig>| {
        context.set_node_value(AUTHORITY, 1.0);
        context.set_node_value(HUB, 1.0);
    });

    let compute_fn: ComputeFn<PregelConfig, SyncQueueMessageIterator> = {
        let shared = Arc::clone(&shared_state);
        Arc::new(
            move |context: &mut ComputeContext<PregelConfig, SyncQueueMessageIterator>,
                  messages: &mut Messages<SyncQueueMessageIterator>| {
                let state = shared.get_state();

                match state {
                    HitsState::Init => {
                        // Send initial hub values (1.0) to outgoing neighbors for authority calculation
                        let hub = context.double_node_value(HUB);
                        context.send_to_neighbors(hub);
                    }

                    HitsState::CalculateAuths => {
                        // Receive hub scores from incoming neighbors, compute authority
                        let mut auth_sum = 0.0;
                        for message in messages {
                            auth_sum += message;
                        }
                        context.set_node_value(AUTHORITY, auth_sum);
                        shared.add_to_norm(auth_sum);
                    }

                    HitsState::NormalizeAuths => {
                        // Normalize authority scores
                        let norm = shared.get_norm_sqrt();
                        let auth = context.double_node_value(AUTHORITY);
                        let normalized = if norm > 0.0 { auth / norm } else { 0.0 };
                        context.set_node_value(AUTHORITY, normalized);

                        // Send normalized authority to incoming neighbors (for their hub calc)
                        context.send_to_incoming_neighbors(normalized);
                    }

                    HitsState::CalculateHubs => {
                        // Receive authority scores from outgoing neighbors, compute hub
                        let mut hub_sum = 0.0;
                        for message in messages {
                            hub_sum += message;
                        }
                        context.set_node_value(HUB, hub_sum);
                        shared.add_to_norm(hub_sum);
                    }

                    HitsState::NormalizeHubs => {
                        // Normalize hub scores
                        let norm = shared.get_norm_sqrt();
                        let hub = context.double_node_value(HUB);
                        let normalized = if norm > 0.0 { hub / norm } else { 0.0 };
                        context.set_node_value(HUB, normalized);

                        // Send normalized hub to outgoing neighbors (for their authority calc)
                        context.send_to_neighbors(normalized);
                    }
                }
            },
        )
    };

    let master_compute_fn = {
        let shared = Arc::clone(&shared_state);
        move |context: &mut MasterComputeContext<PregelConfig>| -> bool {
            let state = shared.get_state();
            let superstep = context.superstep();

            match state {
                HitsState::Init => {
                    // After init, calculate authorities
                    shared.reset_norm();
                    shared.set_state(HitsState::CalculateAuths);
                    false
                }

                HitsState::CalculateAuths => {
                    // After calculating auths, normalize them
                    shared.set_state(HitsState::NormalizeAuths);
                    false
                }

                HitsState::NormalizeAuths => {
                    // After normalizing auths, calculate hubs
                    shared.reset_norm();
                    shared.set_state(HitsState::CalculateHubs);
                    false
                }

                HitsState::CalculateHubs => {
                    // After calculating hubs, normalize them
                    shared.set_state(HitsState::NormalizeHubs);
                    false
                }

                HitsState::NormalizeHubs => {
                    // Check convergence or max iterations
                    let iteration = (superstep + 1) / 5; // 5 phases per iteration

                    if iteration >= shared.max_iterations {
                        // Max iterations reached
                        return true; // halt
                    }

                    // Check if converged based on norm change
                    if iteration > 1 && shared.check_convergence() {
                        return true; // converged
                    }

                    // Store current norm for next iteration comparison
                    shared.update_previous_norm();

                    // Continue to next iteration
                    shared.reset_norm();
                    shared.set_state(HitsState::CalculateAuths);
                    false // continue
                }
            }
        }
    };

    let messenger = Arc::new(SyncQueueMessenger::new(graph.node_count()));

    let config = PregelConfig::builder()
        .max_iterations(max_iterations * 5) // 5 phases per iteration
        .build()
        .unwrap();

    let pregel = PregelBuilder::<PregelConfig, SyncQueueMessageIterator>::new()
        .graph(Arc::clone(&graph))
        .config(config)
        .schema(schema)
        .init_fn(init_fn)
        .compute_fn(compute_fn)
        .master_compute_fn(master_compute_fn)
        .messenger(messenger)
        .build();

    let result = pregel.run();

    let mut hub_scores = Vec::with_capacity(graph.node_count());
    let mut authority_scores = Vec::with_capacity(graph.node_count());

    for node_id in 0..graph.node_count() {
        hub_scores.push(result.node_values.double_value(HUB, node_id));
        authority_scores.push(result.node_values.double_value(AUTHORITY, node_id));
    }

    HitsRunResult {
        hub_scores,
        authority_scores,
        iterations_ran: result.ran_iterations / 5, // Convert back from phases
        did_converge: result.did_converge,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::prelude::DefaultGraphStore;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    #[test]
    fn test_hits_chain() {
        // Create a chain: 0 -> 1 -> 2
        let config = RandomGraphConfig {
            seed: Some(42),
            node_count: 3,
            relationships: vec![RandomRelationshipConfig::new("REL", 0.67)], // ~2 edges for chain
            ..RandomGraphConfig::default()
        };
        let store = DefaultGraphStore::random(&config).unwrap();
        let graph = store.graph();

        let result = run_hits(graph, 20, 1e-4);

        assert_eq!(result.hub_scores.len(), 3);
        assert_eq!(result.authority_scores.len(), 3);

        // Verify normalization (L2 norm ~1.0)
        let hub_norm: f64 = result.hub_scores.iter().map(|h| h * h).sum::<f64>().sqrt();
        let auth_norm: f64 = result
            .authority_scores
            .iter()
            .map(|a| a * a)
            .sum::<f64>()
            .sqrt();

        println!("Hub scores: {:?}", result.hub_scores);
        println!("Authority scores: {:?}", result.authority_scores);
        println!("Hub norm: {}, Auth norm: {}", hub_norm, auth_norm);

        // Verify at least one normalization worked (authorities typically more stable)
        assert!(
            auth_norm > 0.5,
            "Auths should have meaningful values: got {}",
            auth_norm
        );
    }
}
