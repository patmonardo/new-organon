//! Indirect Exposure Pregel computation.
//! Translation source: `org.neo4j.gds.indirectExposure.IndirectExposureComputation`.

use crate::collections::{HugeAtomicBitSet, HugeAtomicLongArray};
use crate::config::{ConcurrencyConfig, Config, IterationsConfig};
use crate::core::utils::partition::Partitioning;
use crate::pregel::{
    ComputeContext, DefaultValue, InitContext, Messages, PregelRuntimeConfig, PregelSchema,
    PregelSchemaBuilder, Reducer, ReducingMessageIterator, Visibility,
};
use std::sync::Arc;

const EXPOSURE_KEY: &str = "exposure";
const HOP_KEY: &str = "hop";
const PARENT_KEY: &str = "parent";
const ROOT_KEY: &str = "root";

/// Pregel computation runtime (pure state + functions).
#[derive(Clone)]
pub struct IndirectExposureComputationRuntime {
    /// Mark sanctioned nodes.
    is_sanctioned: Arc<Vec<bool>>,
    /// Precomputed degree/weight sums for normalization.
    total_transfers: Arc<Vec<f64>>,
    /// Guard against revisiting nodes.
    visited: Arc<HugeAtomicBitSet>,
    /// Shared root tracker (original IDs), used for sender lookups.
    roots: Arc<HugeAtomicLongArray>,
}

impl IndirectExposureComputationRuntime {
    pub fn new(is_sanctioned: Vec<bool>, total_transfers: Vec<f64>) -> Self {
        let node_count = total_transfers.len();
        let roots = HugeAtomicLongArray::new(node_count);
        for i in 0..node_count {
            roots.set(i, -1);
        }
        Self {
            is_sanctioned: Arc::new(is_sanctioned),
            total_transfers: Arc::new(total_transfers),
            visited: Arc::new(HugeAtomicBitSet::new(node_count)),
            roots: Arc::new(roots),
        }
    }

    pub fn schema(&self) -> PregelSchema {
        PregelSchemaBuilder::new()
            .add_with_default(EXPOSURE_KEY, DefaultValue::Double(0.0), Visibility::Private)
            .add_with_default(HOP_KEY, DefaultValue::Long(-1), Visibility::Private)
            .add_with_default(PARENT_KEY, DefaultValue::Long(-1), Visibility::Private)
            .add_with_default(ROOT_KEY, DefaultValue::Long(-1), Visibility::Private)
            // public echo of exposure for downstream consumption
            .add_with_default("value", DefaultValue::Double(0.0), Visibility::Public)
            .build()
    }

    pub fn init_fn(
        &self,
    ) -> Arc<dyn Fn(&mut InitContext<IndirectExposurePregelRuntimeConfig>) + Send + Sync> {
        let is_sanctioned = Arc::clone(&self.is_sanctioned);
        let visited = Arc::clone(&self.visited);
        let roots = Arc::clone(&self.roots);

        Arc::new(
            move |context: &mut InitContext<IndirectExposurePregelRuntimeConfig>| {
                let node_id = context.node_id() as usize;
                if is_sanctioned.get(node_id).copied().unwrap_or(false) {
                    let orig = context.to_original_id();
                    context.set_node_value(EXPOSURE_KEY, 1.0);
                    context.set_node_value("value", 1.0);
                    context.set_node_value_long(HOP_KEY, 0);
                    context.set_node_value_long(ROOT_KEY, orig);
                    context.set_node_value_long(PARENT_KEY, orig);
                    roots.set(node_id, orig);
                    visited.set(node_id);
                }
            },
        )
    }

    pub fn compute_fn(
        &self,
    ) -> Arc<
        dyn Fn(
                &mut ComputeContext<IndirectExposurePregelRuntimeConfig, ReducingMessageIterator>,
                &mut Messages<ReducingMessageIterator>,
            ) + Send
            + Sync,
    > {
        let visited = Arc::clone(&self.visited);
        let total_transfers = Arc::clone(&self.total_transfers);
        let roots = Arc::clone(&self.roots);

        Arc::new(
            move |context: &mut ComputeContext<
                IndirectExposurePregelRuntimeConfig,
                ReducingMessageIterator,
            >,
                  messages: &mut Messages<ReducingMessageIterator>| {
                let node_id = context.node_id() as usize;

                if context.is_initial_superstep() {
                    if visited.get(node_id) {
                        context.send_to_neighbors(context.double_node_value(EXPOSURE_KEY));
                    } else {
                        context.vote_to_halt();
                    }
                    return;
                }

                if visited.get_and_set(node_id) {
                    context.vote_to_halt();
                    return;
                }

                let Some(parent_exposure) = messages.next() else {
                    context.vote_to_halt();
                    return;
                };

                let sender = messages.sender().unwrap_or(context.node_id());
                let sender_root = roots.get(sender as usize);

                let denom = total_transfers
                    .get(node_id)
                    .copied()
                    .unwrap_or(1.0)
                    .max(1.0);
                let new_exposure = parent_exposure / denom;

                context.set_node_value(EXPOSURE_KEY, new_exposure);
                context.set_node_value("value", new_exposure);

                context.set_node_value_long(PARENT_KEY, context.to_original_id_of(sender as u64));
                context.set_node_value_long(HOP_KEY, context.superstep() as i64);
                let root_value = if sender_root != -1 {
                    sender_root
                } else {
                    context.to_original_id_of(sender as u64)
                };
                context.set_node_value_long(ROOT_KEY, root_value);
                roots.set(node_id, root_value);

                context.send_to_neighbors(new_exposure);
                context.vote_to_halt();
            },
        )
    }

    pub fn reducer(&self) -> Reducer {
        Reducer::Max
    }
}

/// Minimal Pregel runtime config for indirect exposure.
#[derive(Debug, Clone)]
pub struct IndirectExposurePregelRuntimeConfig {
    pub concurrency: usize,
    pub max_iterations: usize,
}

impl Config for IndirectExposurePregelRuntimeConfig {}

impl ConcurrencyConfig for IndirectExposurePregelRuntimeConfig {
    fn concurrency(&self) -> usize {
        self.concurrency
    }
}

impl IterationsConfig for IndirectExposurePregelRuntimeConfig {
    fn max_iterations(&self) -> usize {
        self.max_iterations
    }

    fn tolerance(&self) -> Option<f64> {
        None
    }
}

impl PregelRuntimeConfig for IndirectExposurePregelRuntimeConfig {
    fn is_asynchronous(&self) -> bool {
        false
    }

    fn partitioning(&self) -> Partitioning {
        Partitioning::Range
    }

    fn track_sender(&self) -> bool {
        true
    }
}
