//! Java: `BinarizeTask`.
//!
//! Depends on the `FeatureExtraction` subsystem (property extractors + consumers) which is
//! not fully wired in Rust GDS yet.

use super::hash_gnn_parameters::BinarizeFeaturesConfig;
use crate::collections::HugeObjectArray;
use crate::concurrency::TerminationFlag;
use crate::core::utils::paged::HugeAtomicBitSet;
use crate::core::utils::partition::Partition;
use crate::ml::core::features::{self, FeatureConsumer};
use crate::types::graph::Graph;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::f64::consts::PI;
use std::sync::Arc;

pub struct BinarizeTask;

impl BinarizeTask {
    pub fn compute(
        cfg: BinarizeFeaturesConfig,
        graph: Arc<dyn Graph>,
        partitions: Vec<Partition>,
        feature_properties: Vec<String>,
        random_seed: u64,
        termination_flag: &TerminationFlag,
    ) -> (HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>, u64) {
        let extractors = features::property_extractors(graph.as_ref(), &feature_properties);
        let input_dimension = features::feature_count(&extractors);

        let output_dimension = cfg.dimension;
        let threshold = cfg.threshold;

        let mut rng = ChaCha8Rng::seed_from_u64(random_seed);

        // Java: propertyEmbeddings is a random matrix [inputDimension x outputDimension]
        // with Gaussian entries.
        let mut property_embeddings: Vec<Vec<f32>> = Vec::with_capacity(input_dimension);
        for _ in 0..input_dimension {
            let mut row = vec![0.0f32; output_dimension];
            for v in &mut row {
                *v = next_gaussian_f32(&mut rng);
            }
            property_embeddings.push(row);
        }

        let mut out: HugeObjectArray<Option<Arc<HugeAtomicBitSet>>> =
            HugeObjectArray::new(graph.node_count());

        let mut total_feature_count: u64 = 0;

        struct Consumer<'a> {
            embedding: &'a mut [f32],
            property_embeddings: &'a [Vec<f32>],
        }

        impl FeatureConsumer for Consumer<'_> {
            fn accept_scalar(&mut self, _node_offset: u64, offset: usize, value: f64) {
                let value = value as f32;
                let row = &self.property_embeddings[offset];
                for (i, &w) in row.iter().enumerate() {
                    self.embedding[i] += value * w;
                }
            }

            fn accept_array(&mut self, _node_offset: u64, offset: usize, values: &[f64]) {
                for (j, &value) in values.iter().enumerate() {
                    let value = value as f32;
                    let row = &self.property_embeddings[offset + j];
                    for (i, &w) in row.iter().enumerate() {
                        self.embedding[i] += value * w;
                    }
                }
            }
        }

        for partition in partitions {
            termination_flag.assert_running();
            partition.consume(|node_id| {
                termination_flag.assert_running();

                let mut embedding = vec![0.0f32; output_dimension];
                let mut consumer = Consumer {
                    embedding: &mut embedding,
                    property_embeddings: &property_embeddings,
                };

                features::extract(node_id as u64, node_id as u64, &extractors, &mut consumer);

                let bitset = Arc::new(HugeAtomicBitSet::new(output_dimension));
                for (i, &embedding_val) in embedding.iter().enumerate().take(output_dimension) {
                    if embedding_val as f64 > threshold {
                        bitset.set(i);
                    }
                }

                total_feature_count += bitset.cardinality() as u64;
                out.set(node_id, Some(bitset));
            });
        }

        (out, total_feature_count)
    }
}

fn next_gaussian_f32(rng: &mut impl Rng) -> f32 {
    // Box–Muller transform
    let mut u1: f64 = rng.gen();
    if u1 <= 0.0 {
        u1 = f64::MIN_POSITIVE;
    }
    let u2: f64 = rng.gen();
    let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
    z0 as f32
}
