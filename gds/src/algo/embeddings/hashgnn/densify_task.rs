use crate::collections::HugeObjectArray;
use crate::core::utils::paged::HugeAtomicBitSet;
use crate::core::utils::partition::Partition;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::sync::Arc;

const SPARSITY: usize = 3;
const ENTRY_PROBABILITY: f64 = 1.0 / (2.0 * SPARSITY as f64);

/// Java: `DensifyTask implements Runnable`
pub struct DensifyTask;

impl DensifyTask {
    pub fn compute(
        partitions: Vec<Partition>,
        output_dimension: usize,
        rng_seed: u64,
        binary_features: &HugeObjectArray<Option<Arc<HugeAtomicBitSet>>>,
    ) -> HugeObjectArray<Vec<f64>> {
        let node_count = binary_features.size();
        let binary_dimension = binary_features
            .get(0)
            .as_ref()
            .expect("binary features not initialized")
            .size();

        let projection_matrix = projection_matrix(rng_seed, output_dimension, binary_dimension);

        let mut dense_features: HugeObjectArray<Vec<f64>> = HugeObjectArray::new(node_count);

        // Sequential for now.
        for partition in partitions {
            partition.consume(|node_id| {
                let binary_vector = binary_features
                    .get(node_id)
                    .as_ref()
                    .expect("binary features not initialized");
                let mut dense = vec![0.0f64; output_dimension];

                binary_vector.for_each_set_bit(|bit| {
                    let row = &projection_matrix[bit];
                    for i in 0..output_dimension {
                        dense[i] += row[i] as f64;
                    }
                });

                dense_features.set(node_id, dense);
            });
        }

        dense_features
    }
}

fn projection_matrix(seed: u64, dense_dimension: usize, binary_dimension: usize) -> Vec<Vec<f32>> {
    let entry_value = (SPARSITY as f64).sqrt() / (dense_dimension as f64).sqrt();
    let entry_value = entry_value as f32;

    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut matrix = Vec::with_capacity(binary_dimension);
    for _ in 0..binary_dimension {
        let mut row = vec![0.0f32; dense_dimension];
        for (_d, row_val) in row.iter_mut().enumerate().take(dense_dimension) {
            *row_val = compute_random_entry(&mut rng, entry_value);
        }
        matrix.push(row);
    }
    matrix
}

fn compute_random_entry<R: Rng>(rng: &mut R, entry_value: f32) -> f32 {
    let r: f64 = rng.gen();
    if r < ENTRY_PROBABILITY {
        entry_value
    } else if r < ENTRY_PROBABILITY * 2.0 {
        -entry_value
    } else {
        0.0
    }
}
