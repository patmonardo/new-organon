use crate::pregel::{
    ComputeContext, DefaultValue, InitContext, MasterComputeContext, Messages, PregelResult,
    PregelSchema, PregelSchemaBuilder, SyncQueueMessageIterator, Visibility,
};
use crate::ValueType;
use std::sync::Arc;

use super::{HitsPregelRuntimeConfig, HitsRunResult};

const HUB_KEY: &str = "hub";
const AUTH_KEY: &str = "authority";
const HUB_TMP_KEY: &str = "hub_tmp";
const AUTH_TMP_KEY: &str = "authority_tmp";
const HUB_PREV_KEY: &str = "hub_prev";
const AUTH_PREV_KEY: &str = "authority_prev";

fn hits_schema() -> PregelSchema {
    PregelSchemaBuilder::new()
        .add_with_default(HUB_KEY, DefaultValue::Double(1.0), Visibility::Public)
        .add_with_default(AUTH_KEY, DefaultValue::Double(1.0), Visibility::Public)
        .add_with_default(HUB_TMP_KEY, DefaultValue::Double(0.0), Visibility::Private)
        .add_with_default(AUTH_TMP_KEY, DefaultValue::Double(0.0), Visibility::Private)
        .add_with_default(HUB_PREV_KEY, DefaultValue::Double(1.0), Visibility::Private)
        .add_with_default(
            AUTH_PREV_KEY,
            DefaultValue::Double(1.0),
            Visibility::Private,
        )
        // Keep a canonical node value as well (some infrastructure expects one)
        .add_public("value", ValueType::Double)
        .build()
}

/// Run HITS on a pre-projected graph.
///
/// Pure state runtime for HITS.
///
/// Provides Pregel schema/init/compute/master functions and finalization without owning graph
/// access or execution orchestration.
#[derive(Clone, Debug)]
pub struct HitsComputationRuntime {
    tolerance: f64,
}

impl HitsComputationRuntime {
    pub fn new(tolerance: f64) -> Self {
        Self { tolerance }
    }

    pub fn schema(&self) -> PregelSchema {
        hits_schema()
    }

    pub fn init_fn(&self) -> Arc<dyn Fn(&mut InitContext<HitsPregelRuntimeConfig>) + Send + Sync> {
        Arc::new(|context: &mut InitContext<HitsPregelRuntimeConfig>| {
            context.set_node_value(HUB_KEY, 1.0);
            context.set_node_value(AUTH_KEY, 1.0);
            context.set_node_value(HUB_PREV_KEY, 1.0);
            context.set_node_value(AUTH_PREV_KEY, 1.0);
            context.set_node_value(HUB_TMP_KEY, 0.0);
            context.set_node_value(AUTH_TMP_KEY, 0.0);
            context.set_node_value("value", 0.0);
        })
    }

    pub fn compute_fn(
        &self,
    ) -> Arc<
        dyn Fn(
                &mut ComputeContext<HitsPregelRuntimeConfig, SyncQueueMessageIterator>,
                &mut Messages<SyncQueueMessageIterator>,
            ) + Send
            + Sync,
    > {
        Arc::new(
            |context: &mut ComputeContext<HitsPregelRuntimeConfig, SyncQueueMessageIterator>,
             messages: &mut Messages<SyncQueueMessageIterator>| {
                let superstep = context.superstep();

                // Superstep 0: seed by sending hubs along outgoing edges.
                if superstep == 0 {
                    let hub = context.double_node_value(HUB_KEY);
                    context.send_to_neighbors(hub);
                    return;
                }

                match (superstep - 1) % 4 {
                    // CALC_AUTHS: sum incoming hubs
                    0 => {
                        let mut sum = 0.0f64;
                        for m in messages.by_ref() {
                            sum += m;
                        }
                        context.set_node_value(AUTH_TMP_KEY, sum);
                    }
                    // SEND_AUTHS: send (normalized) authority backwards (to incoming neighbors)
                    1 => {
                        let auth = context.double_node_value(AUTH_KEY);
                        context.send_to_incoming_neighbors(auth);
                    }
                    // CALC_HUBS: sum incoming authorities
                    2 => {
                        let mut sum = 0.0f64;
                        for m in messages.by_ref() {
                            sum += m;
                        }
                        context.set_node_value(HUB_TMP_KEY, sum);
                    }
                    // SEND_HUBS: send (normalized) hubs along outgoing edges
                    _ => {
                        let hub = context.double_node_value(HUB_KEY);
                        context.send_to_neighbors(hub);
                    }
                }
            },
        )
    }

    pub fn master_compute_fn(
        &self,
    ) -> impl Fn(&mut MasterComputeContext<HitsPregelRuntimeConfig>) -> bool + Send + Sync + 'static
    {
        let tolerance = self.tolerance;

        move |context: &mut MasterComputeContext<HitsPregelRuntimeConfig>| {
            let superstep = context.superstep();
            if superstep == 0 {
                return false;
            }

            match (superstep - 1) % 4 {
                // NORMALIZE_AUTHS
                0 => {
                    let mut sum_sq = 0.0f64;
                    let node_count = context.node_count();
                    for node_id in 0..node_count {
                        let v = context.double_node_value(node_id, AUTH_TMP_KEY);
                        sum_sq += v * v;
                    }

                    let norm = sum_sq.sqrt();
                    let denom = if norm > 0.0 { norm } else { 1.0 };

                    for node_id in 0..node_count {
                        let prev = context.double_node_value(node_id, AUTH_KEY);
                        let next = context.double_node_value(node_id, AUTH_TMP_KEY) / denom;

                        context.set_double_node_value(node_id, AUTH_PREV_KEY, prev);
                        context.set_double_node_value(node_id, AUTH_KEY, next);
                    }

                    false
                }
                // NORMALIZE_HUBS + convergence check
                2 => {
                    let mut sum_sq = 0.0f64;
                    let node_count = context.node_count();
                    for node_id in 0..node_count {
                        let v = context.double_node_value(node_id, HUB_TMP_KEY);
                        sum_sq += v * v;
                    }

                    let norm = sum_sq.sqrt();
                    let denom = if norm > 0.0 { norm } else { 1.0 };

                    let mut max_delta = 0.0f64;
                    for node_id in 0..node_count {
                        let prev_hub = context.double_node_value(node_id, HUB_KEY);
                        let next_hub = context.double_node_value(node_id, HUB_TMP_KEY) / denom;

                        let prev_auth = context.double_node_value(node_id, AUTH_PREV_KEY);
                        let next_auth = context.double_node_value(node_id, AUTH_KEY);

                        let d_hub = (prev_hub - next_hub).abs();
                        let d_auth = (prev_auth - next_auth).abs();
                        max_delta = max_delta.max(d_hub.max(d_auth));

                        context.set_double_node_value(node_id, HUB_PREV_KEY, prev_hub);
                        context.set_double_node_value(node_id, HUB_KEY, next_hub);
                    }

                    max_delta <= tolerance
                }
                _ => false,
            }
        }
    }

    pub fn finalize(&self, result: &PregelResult, node_count: usize) -> HitsRunResult {
        let node_values = Arc::clone(&result.node_values);

        let mut hubs = vec![0.0f64; node_count];
        let mut auths = vec![0.0f64; node_count];

        for node_id in 0..node_count {
            hubs[node_id] = node_values.double_value(HUB_KEY, node_id);
            auths[node_id] = node_values.double_value(AUTH_KEY, node_id);
        }

        // Translate pregel supersteps back into algorithm iterations.
        let ran_supersteps = result.ran_iterations;
        let ran_iterations = if ran_supersteps <= 1 {
            0
        } else {
            // After the initial seed step, each full HITS iteration consumes 4 supersteps.
            ((ran_supersteps - 1) / 4).max(1)
        };

        HitsRunResult {
            hub_scores: hubs,
            authority_scores: auths,
            iterations_ran: ran_iterations,
            did_converge: result.did_converge,
        }
    }
}
