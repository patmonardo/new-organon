//! HITS Computation Runtime - Simplified single-threaded implementation

use crate::projection::Orientation;
use crate::types::graph::Graph;
use std::sync::Arc;

/// State machine for HITS algorithm phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HitsState {
    Init,
    CalculateAuths,
    NormalizeAuths,
    CalculateHubs,
    NormalizeHubs,
}

impl HitsState {
    pub fn advance(self) -> Self {
        match self {
            HitsState::Init => HitsState::NormalizeAuths,
            HitsState::CalculateAuths => HitsState::NormalizeAuths,
            HitsState::NormalizeAuths => HitsState::CalculateHubs,
            HitsState::CalculateHubs => HitsState::NormalizeHubs,
            HitsState::NormalizeHubs => HitsState::CalculateAuths,
        }
    }
}

/// Result of HITS computation
#[derive(Debug, Clone)]
pub struct HitsComputationResult {
    /// Hub scores for each node
    pub hub_scores: Vec<f64>,
    /// Authority scores for each node
    pub authority_scores: Vec<f64>,
    /// Number of iterations run
    pub iterations: usize,
    /// Whether the algorithm converged
    pub converged: bool,
}

/// Computation Runtime for HITS
pub struct HitsComputationRuntime {
    max_iterations: usize,
    tolerance: f64,
}

impl HitsComputationRuntime {
    pub fn new(max_iterations: usize, tolerance: f64) -> Self {
        Self {
            max_iterations,
            tolerance,
        }
    }

    pub fn compute(&self, graph: Arc<dyn Graph>) -> HitsComputationResult {
        let node_count = graph.node_count();

        if node_count == 0 {
            return HitsComputationResult {
                hub_scores: vec![],
                authority_scores: vec![],
                iterations: 0,
                converged: true,
            };
        }

        let mut hub_scores = vec![1.0; node_count];
        let mut auth_scores = vec![1.0; node_count];
        let mut hub_scores_new = vec![0.0; node_count];
        let mut auth_scores_new = vec![0.0; node_count];

        let mut state = HitsState::Init;
        let mut global_norm: f64 = 0.0;
        let mut iteration = 0;
        let mut converged = false;

        let fallback = graph.default_property_value();

        // Build incoming neighbor lists (for reverse/authority calculation)
        let mut incoming: Vec<Vec<usize>> = vec![Vec::new(); node_count];
        for node_id in 0..node_count {
            let neighbors = graph
                .stream_relationships(node_id as i64, fallback)
                .map(|cursor| cursor.target_id())
                .filter(|target| *target >= 0 && (*target as usize) < node_count)
                .map(|target| target as usize)
                .collect::<Vec<_>>();
            for &neighbor in &neighbors {
                incoming[neighbor].push(node_id);
            }
        }

        // Main iteration loop
        for _ in 0..self.max_iterations * 5 {
            // 5 states per iteration
            match state {
                HitsState::Init => {
                    // Initialize auth scores with in-degree
                    global_norm = 0.0;
                    for node_id in 0..node_count {
                        let degree = incoming[node_id].len() as f64;
                        auth_scores[node_id] = degree;
                        global_norm += degree * degree;
                    }
                }
                HitsState::CalculateAuths => {
                    // Authority = sum of hub scores of incoming neighbors
                    global_norm = 0.0;
                    for node_id in 0..node_count {
                        let mut auth = 0.0;
                        for &neighbor in &incoming[node_id] {
                            auth += hub_scores[neighbor];
                        }
                        auth_scores_new[node_id] = auth;
                        global_norm += auth * auth;
                    }
                }
                HitsState::NormalizeAuths => {
                    // Normalize authority scores
                    let norm = global_norm.sqrt();
                    if norm > 0.0 {
                        for node_id in 0..node_count {
                            auth_scores[node_id] = auth_scores_new[node_id] / norm;
                        }
                    }
                    global_norm = 0.0;
                }
                HitsState::CalculateHubs => {
                    // Hub = sum of authority scores of outgoing neighbors
                    global_norm = 0.0;
                    for node_id in 0..node_count {
                        let mut hub = 0.0;
                        let neighbors = graph
                            .stream_relationships(node_id as i64, fallback)
                            .map(|cursor| cursor.target_id())
                            .filter(|target| *target >= 0 && (*target as usize) < node_count)
                            .map(|target| target as usize);
                        for neighbor in neighbors {
                            hub += auth_scores[neighbor];
                        }
                        hub_scores_new[node_id] = hub;
                        global_norm += hub * hub;
                    }
                }
                HitsState::NormalizeHubs => {
                    // Normalize hub scores
                    let norm = global_norm.sqrt();
                    if norm > 0.0 {
                        for node_id in 0..node_count {
                            hub_scores[node_id] = hub_scores_new[node_id] / norm;
                        }
                    }
                    iteration += 1;

                    // Check convergence
                    if iteration > 1 {
                        let max_diff = (0..node_count)
                            .map(|i| {
                                let hub_diff = (hub_scores[i] - hub_scores_new[i] / norm).abs();
                                let auth_diff = (auth_scores[i] - auth_scores_new[i] / norm).abs();
                                hub_diff.max(auth_diff)
                            })
                            .fold(0.0, f64::max);

                        if max_diff < self.tolerance {
                            converged = true;
                            break;
                        }
                    }

                    if iteration >= self.max_iterations {
                        break;
                    }

                    global_norm = 0.0;
                }
            }

            state = state.advance();
        }

        HitsComputationResult {
            hub_scores,
            authority_scores: auth_scores,
            iterations: iteration,
            converged,
        }
    }
}
