use crate::concurrency::{Concurrency, TerminationFlag};
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use super::hash_gnn_companion::{next_prime, HashTriple};

const MAX_FINAL_INFLUENCE: f64 = 1e4;
const PRIME_LOWER_BOUND: i32 = 50_000;

/// Java: `HashTask implements Runnable`
pub struct HashTask {
    embedding_dimension: usize,
    scaled_neighbor_influence: f64,
    number_of_relationship_types: usize,
    rng: ChaCha8Rng,
    neighbors_aggregation_hashes: Vec<i32>,
    self_aggregation_hashes: Vec<i32>,
    pre_aggregation_hashes: Vec<Vec<i32>>,
}

/// Java: `record Hashes(...)`
#[derive(Debug, Clone)]
pub struct Hashes {
    pub neighbors_aggregation_hashes: Vec<i32>,
    pub self_aggregation_hashes: Vec<i32>,
    pub pre_aggregation_hashes: Vec<Vec<i32>>,
}

impl HashTask {
    fn new(
        embedding_dimension: usize,
        scaled_neighbor_influence: f64,
        number_of_relationship_types: usize,
        random_seed: u64,
    ) -> Self {
        Self {
            embedding_dimension,
            scaled_neighbor_influence,
            number_of_relationship_types,
            rng: ChaCha8Rng::seed_from_u64(random_seed),
            neighbors_aggregation_hashes: Vec::new(),
            self_aggregation_hashes: Vec::new(),
            pre_aggregation_hashes: Vec::new(),
        }
    }

    pub fn compute(
        embedding_dimension: usize,
        scaled_neighbor_influence: f64,
        number_of_relationship_types: usize,
        _concurrency: Concurrency,
        embedding_density: usize,
        random_seed: u64,
        termination_flag: &TerminationFlag,
    ) -> Vec<Hashes> {
        // Sequential for now.
        let mut out = Vec::with_capacity(embedding_density);
        for seed_offset in 0..embedding_density {
            termination_flag.assert_running();
            let mut task = HashTask::new(
                embedding_dimension,
                scaled_neighbor_influence,
                number_of_relationship_types,
                random_seed + seed_offset as u64,
            );
            task.run();
            out.push(task.hashes());
        }
        out
    }

    pub fn run(&mut self) {
        let final_influence =
            (self.scaled_neighbor_influence).clamp(1.0 / MAX_FINAL_INFLUENCE, MAX_FINAL_INFLUENCE);

        // Java: primeSeed in [PRIME_LOWER_BOUND, MAX_INT/(max(1, finalInfluence)*1.001)]
        let upper = ((i32::MAX as f64) / (final_influence.max(1.0) * 1.001)).round() as i32;
        let prime_seed = if upper > PRIME_LOWER_BOUND {
            self.rng.gen_range(PRIME_LOWER_BOUND..upper)
        } else {
            PRIME_LOWER_BOUND
        };

        let neighbor_prime = next_prime(prime_seed);
        let self_prime = if (self.scaled_neighbor_influence - 1.0).abs() < f64::EPSILON {
            neighbor_prime
        } else {
            next_prime(((neighbor_prime as f64) * final_influence).round() as i32)
        };

        self.neighbors_aggregation_hashes = HashTriple::compute_hashes_from_triple(
            self.embedding_dimension,
            HashTriple::generate_with_c(&mut self.rng, neighbor_prime),
        );
        self.self_aggregation_hashes = HashTriple::compute_hashes_from_triple(
            self.embedding_dimension,
            HashTriple::generate_with_c(&mut self.rng, self_prime),
        );

        self.pre_aggregation_hashes = (0..self.number_of_relationship_types)
            .map(|_| {
                let triple = HashTriple::generate(&mut self.rng);
                HashTriple::compute_hashes_from_triple(self.embedding_dimension, triple)
            })
            .collect();
    }

    fn hashes(self) -> Hashes {
        Hashes {
            neighbors_aggregation_hashes: self.neighbors_aggregation_hashes,
            self_aggregation_hashes: self.self_aggregation_hashes,
            pre_aggregation_hashes: self.pre_aggregation_hashes,
        }
    }
}
