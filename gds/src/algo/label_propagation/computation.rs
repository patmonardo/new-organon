//! Label Propagation computation runtime
//!
//! Java parity references:
//! - `org.neo4j.gds.labelpropagation.LabelPropagation`
//! - `InitStep` / `ComputeStep` / `ComputeStepConsumer`
//!
//! Notes:
//! - Uses deterministic in-place (Gauss–Seidel) updates in node-id order.
//!   This avoids oscillations on bipartite components while preserving a stable tie-break.
//! - Voting is weighted by relationship weight * target-node weight.
//! - Tie-breaker matches Java: smallest label ID wins when weights equal.

use crate::concurrency::{install_with_concurrency, Concurrency};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LabelPropResult {
    pub labels: Vec<u64>,
    pub did_converge: bool,
    pub ran_iterations: u64,
}

#[derive(Debug, Clone)]
pub struct LabelPropComputationRuntime {
    node_count: usize,
    max_iterations: u64,
    concurrency: usize,
    node_weights: Vec<f64>,
    initial_labels: Option<Vec<u64>>, // must match node_count when present
}

impl LabelPropComputationRuntime {
    pub fn new(node_count: usize, max_iterations: u64) -> Self {
        Self {
            node_count,
            max_iterations,
            concurrency: 1,
            node_weights: vec![1.0; node_count],
            initial_labels: None,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency.max(1);
        self
    }

    pub fn with_weights(mut self, weights: Vec<f64>) -> Self {
        if weights.len() == self.node_count {
            self.node_weights = weights;
        }
        self
    }

    /// Sets the initial labels for all nodes.
    ///
    /// This corresponds to Java's `InitStep` output.
    pub fn with_seeds(mut self, labels: Vec<u64>) -> Self {
        if labels.len() == self.node_count {
            self.initial_labels = Some(labels);
        }
        self
    }

    pub fn compute<F>(&mut self, node_count: u64, neighbors: F) -> LabelPropResult
    where
        F: Fn(usize) -> Vec<(usize, f64)> + Sync,
    {
        let node_count = node_count as usize;
        if node_count == 0 {
            return LabelPropResult {
                labels: Vec::new(),
                did_converge: true,
                ran_iterations: 0,
            };
        }

        // Init step (either provided labels or identity labels).
        let mut labels: Vec<u64> = if let Some(init) = self.initial_labels.take() {
            init
        } else {
            (0..node_count as u64).collect()
        };

        let concurrency = Concurrency::from_usize(self.concurrency);

        let mut ran_iterations = 0u64;
        let mut did_converge = false;

        while ran_iterations < self.max_iterations {
            let any_changed = AtomicBool::new(false);

            // Deterministic in-place update in node order.
            // Concurrency is currently not used; we keep the install wrapper to match runtime setup.
            install_with_concurrency(concurrency, || {
                let mut tally = VoteTally::new();

                for node_id in 0..node_count {
                    tally.clear();

                    let current_label = labels[node_id];
                    let mut best_label = current_label;
                    let mut best_weight = f64::NEG_INFINITY;

                    for (target, rel_weight) in neighbors(node_id) {
                        let node_weight = *self.node_weights.get(target).unwrap_or(&1.0);
                        let vote_weight = rel_weight * node_weight;
                        let candidate_label = labels[target];
                        tally.add_vote(candidate_label, vote_weight);
                    }

                    for (&label, &weight) in tally.votes.iter() {
                        if weight > best_weight || (weight == best_weight && label < best_label) {
                            best_weight = weight;
                            best_label = label;
                        }
                    }

                    if best_label != current_label {
                        labels[node_id] = best_label;
                        any_changed.store(true, Ordering::Relaxed);
                    }
                }
            });

            ran_iterations += 1;
            if !any_changed.load(Ordering::Relaxed) {
                did_converge = true;
                break;
            }
        }

        LabelPropResult {
            labels,
            did_converge,
            ran_iterations,
        }
    }
}

struct VoteTally {
    votes: HashMap<u64, f64>,
}

impl VoteTally {
    fn new() -> Self {
        Self {
            votes: HashMap::with_capacity(64),
        }
    }

    fn clear(&mut self) {
        self.votes.clear();
    }

    fn add_vote(&mut self, label: u64, weight: f64) {
        *self.votes.entry(label).or_insert(0.0) += weight;
    }
}
