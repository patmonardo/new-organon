use crate::core::utils::progress::ProgressTracker;
use crate::pregel::{
    ComputeContext, DefaultValue, InitContext, MasterComputeContext, Messages, Pregel, PregelSchema,
    PregelSchemaBuilder, SyncQueueMessageIterator, SyncQueueMessenger, Visibility,
};
use crate::ValueType;
use crate::types::graph::Graph;
use std::sync::Arc;

use super::storage::{HitsPregelRuntimeConfig, HitsRunResult};

const HUB_KEY: &str = "hub";
const AUTH_KEY: &str = "authority";
const HUB_TMP_KEY: &str = "hub_tmp";
const AUTH_TMP_KEY: &str = "authority_tmp";
const HUB_PREV_KEY: &str = "hub_prev";
const AUTH_PREV_KEY: &str = "authority_prev";

fn hits_schema() -> PregelSchema {
    PregelSchemaBuilder::new()
        .add_with_default(
            HUB_KEY,
            DefaultValue::Double(1.0),
            Visibility::Public,
        )
        .add_with_default(
            AUTH_KEY,
            DefaultValue::Double(1.0),
            Visibility::Public,
        )
        .add_with_default(
            HUB_TMP_KEY,
            DefaultValue::Double(0.0),
            Visibility::Private,
        )
        .add_with_default(
            AUTH_TMP_KEY,
            DefaultValue::Double(0.0),
            Visibility::Private,
        )
        .add_with_default(
            HUB_PREV_KEY,
            DefaultValue::Double(1.0),
            Visibility::Private,
        )
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
/// Storage owns orchestration (projection, choosing messenger/config). This function is the
/// pure-ish Pregel kernel wiring: schema + init/compute/master.
pub fn run_hits(
    graph: Arc<dyn Graph>,
    max_iterations: usize,
    tolerance: f64,
    concurrency: usize,
    _progress_tracker: &mut dyn ProgressTracker,
) -> HitsRunResult {
    let supersteps = 1usize.saturating_add(max_iterations.saturating_mul(4));

    let config = HitsPregelRuntimeConfig {
        concurrency: concurrency.max(1),
        max_iterations: supersteps,
    };

    let schema = hits_schema();

    let init_fn = Arc::new(|context: &mut InitContext<HitsPregelRuntimeConfig>| {
        context.set_node_value(HUB_KEY, 1.0);
        context.set_node_value(AUTH_KEY, 1.0);
        context.set_node_value(HUB_PREV_KEY, 1.0);
        context.set_node_value(AUTH_PREV_KEY, 1.0);
        context.set_node_value(HUB_TMP_KEY, 0.0);
        context.set_node_value(AUTH_TMP_KEY, 0.0);
        context.set_node_value("value", 0.0);
    });

    let compute_fn = Arc::new(
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
    );

    let master_compute_fn = move |context: &mut MasterComputeContext<HitsPregelRuntimeConfig>| {
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
    };

    let messenger = Arc::new(SyncQueueMessenger::new(graph.node_count()));

    let result = Pregel::new(
        Arc::clone(&graph),
        config,
        schema,
        init_fn,
        compute_fn,
        messenger,
        None,
    )
    .with_master_compute_fn(master_compute_fn)
    .run();

    let node_values = Arc::clone(&result.node_values);
    let node_count = graph.node_count();

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
