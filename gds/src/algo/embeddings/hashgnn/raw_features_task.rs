//! Java: `RawFeaturesTask`.
//!
//! Depends on the `FeatureExtraction` subsystem (property extractors) which is
//! not fully wired in Rust GDS yet.

use crate::collections::HugeObjectArray;
use crate::concurrency::TerminationFlag;
use crate::core::utils::paged::HugeAtomicBitSet;
use crate::core::utils::partition::Partition;
use crate::ml::core::features::{self, FeatureConsumer};
use crate::types::graph::Graph;
use std::sync::Arc;

pub struct RawFeaturesTask;

impl RawFeaturesTask {
    pub fn compute(
        graph: Arc<dyn Graph>,
        partitions: Vec<Partition>,
        feature_properties: Vec<String>,
        termination_flag: &TerminationFlag,
    ) -> (HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>, u64) {
        let extractors = features::property_extractors(graph.as_ref(), &feature_properties);
        let input_dimension = features::feature_count(&extractors);

        let mut out: HugeObjectArray<Option<Arc<HugeAtomicBitSet>>> =
            HugeObjectArray::new(graph.node_count());

        let mut total_feature_count: u64 = 0;

        struct Consumer<'a> {
            bitset: &'a HugeAtomicBitSet,
        }

        impl FeatureConsumer for Consumer<'_> {
            fn accept_scalar(&mut self, node_offset: u64, offset: usize, value: f64) {
                if value == 1.0 {
                    self.bitset.set(offset);
                } else if value != 0.0 {
                    panic!(
                        "Feature properties may only contain values 0 and 1 unless `binarizeFeatures` is used. Node {} has a feature property containing value {}",
                        node_offset, value
                    );
                }
            }

            fn accept_array(&mut self, node_offset: u64, offset: usize, values: &[f64]) {
                for (i, &value) in values.iter().enumerate() {
                    if value == 1.0 {
                        self.bitset.set(offset + i);
                    } else if value != 0.0 {
                        panic!(
                            "Feature properties may only contain values 0 and 1 unless `binarizeFeatures` is used. Node {} has a feature property containing value {}",
                            node_offset, value
                        );
                    }
                }
            }
        }

        for partition in partitions {
            termination_flag.assert_running();
            partition.consume(|node_id| {
                termination_flag.assert_running();
                let bitset = Arc::new(HugeAtomicBitSet::new(input_dimension));

                let mut consumer = Consumer {
                    bitset: bitset.as_ref(),
                };

                features::extract(node_id as u64, node_id as u64, &extractors, &mut consumer);

                total_feature_count += bitset.cardinality() as u64;
                out.set(node_id, Some(bitset));
            });
        }

        (out, total_feature_count)
    }
}
