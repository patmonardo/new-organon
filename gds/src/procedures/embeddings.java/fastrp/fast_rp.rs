use super::fast_rp_parameters::FastRPParameters;
use super::fast_rp_result::FastRPResult;
use crate::collections::HugeObjectArray;
use crate::concurrency::{Concurrency, TerminationFlag};
use crate::ml::core::features::{feature_extraction, AnyFeatureExtractor, FeatureConsumer};
use crate::types::graph::Graph;
use std::sync::Arc;

/// Fast Random Projection (FastRP) node embeddings.
///
/// Java: `FastRP extends Algorithm<FastRPResult>`
pub struct FastRP {
    graph: Arc<dyn Graph>,
    parameters: FastRPParameters,
    concurrency: Concurrency,
    min_batch_size: usize,
    feature_extractors: Vec<AnyFeatureExtractor>,
    random_seed: u64,
    termination_flag: TerminationFlag,
}

impl FastRP {
    const SPARSITY: i32 = 3;
    const ENTRY_PROBABILITY: f64 = 1.0 / (2.0 * Self::SPARSITY as f64);
    // Java: EPSILON = 10f / Float.MAX_VALUE
    const EPSILON: f32 = 10.0f32 / f32::MAX;

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        graph: Arc<dyn Graph>,
        parameters: FastRPParameters,
        concurrency: Concurrency,
        min_batch_size: usize,
        feature_extractors: Vec<AnyFeatureExtractor>,
        random_seed: Option<u64>,
        termination_flag: TerminationFlag,
    ) -> Self {
        let seed = random_seed.unwrap_or(42);
        let improved = HighQualityRandom::new(seed).next_u64();

        Self {
            graph,
            parameters,
            concurrency,
            min_batch_size,
            feature_extractors,
            random_seed: improved,
            termination_flag,
        }
    }

    pub fn compute(self) -> FastRPResult {
        // NOTE: We currently run single-threaded for correctness. The storage types
        // used here (Vec<Vec<f32>> / HugeObjectArray) are not concurrency-friendly.
        // Concurrency/min_batch_size are kept for API parity with Java GDS.
        let _ = self.concurrency;
        let _ = self.min_batch_size;

        let embedding_dimension = self.parameters.embedding_dimension;
        let base_embedding_dimension = embedding_dimension.saturating_sub(self.parameters.property_dimension);

        let feature_dim = feature_extraction::feature_count(&self.feature_extractors);
        let property_vectors = self.init_property_vectors(feature_dim);

        let node_count = self.graph.node_count();
        let mut embeddings: Vec<Vec<f32>> = vec![vec![0.0; embedding_dimension]; node_count];
        let mut embedding_a: Vec<Vec<f32>> = vec![vec![0.0; embedding_dimension]; node_count];
        let mut embedding_b: Vec<Vec<f32>> = vec![vec![0.0; embedding_dimension]; node_count];

        self.init_random_vectors(
            base_embedding_dimension,
            embedding_dimension,
            &property_vectors,
            &mut embedding_b,
        );

        self.add_initial_vectors_to_embedding(&embedding_b, &mut embeddings);

        self.propagate_embeddings(
            &mut embeddings,
            &mut embedding_a,
            &mut embedding_b,
        );

        FastRPResult {
            embeddings: HugeObjectArray::from_vec(embeddings),
        }
    }

    fn init_property_vectors(&self, feature_dim: usize) -> Vec<Vec<f32>> {
        let property_dimension = self.parameters.property_dimension;
        if property_dimension == 0 || feature_dim == 0 {
            return vec![vec![]; feature_dim];
        }

        let entry_value =
            ((Self::SPARSITY as f32).sqrt()) / (self.parameters.embedding_dimension as f32).sqrt();

        let mut random = HighQualityRandom::new(self.random_seed);
        let mut property_vectors: Vec<Vec<f32>> =
            vec![vec![0.0f32; property_dimension]; feature_dim];

        for d in 0..property_dimension {
            for i in 0..feature_dim {
                property_vectors[i][d] = compute_random_entry(&mut random, entry_value);
            }
        }

        property_vectors
    }

    fn init_random_vectors(
        &self,
        base_embedding_dimension: usize,
        embedding_dimension: usize,
        property_vectors: &[Vec<f32>],
        embedding_b: &mut [Vec<f32>],
    ) {
        let sqrt_embedding_dimension = (embedding_dimension as f32).sqrt();
        let sqrt_sparsity = (Self::SPARSITY as f32).sqrt();

        // Per-node deterministic reseeding to match Java behaviour.
        let mut random = HighQualityRandom::new(self.random_seed);

        for node_usize in 0..self.graph.node_count() {
            self.termination_flag.assert_running();
            let node_id = node_usize as i64;

            let degree = self.graph.degree(node_id);
            let scaling = if degree == 0 {
                1.0f32
            } else {
                (degree as f32).powf(self.parameters.normalization_strength)
            };
            let entry_value = scaling * sqrt_sparsity / sqrt_embedding_dimension;

            let original_id = self.graph.to_original_node_id(node_id).unwrap_or(node_id);
            random.reseed(self.random_seed ^ (original_id as u64));

            let mut vec = vec![0.0f32; embedding_dimension];
            for i in 0..base_embedding_dimension {
                vec[i] = compute_random_entry(&mut random, entry_value);
            }

            // Property feature contribution for tail dimensions.
            if self.parameters.property_dimension > 0 && !property_vectors.is_empty() {
                let mut adder = PropertyVectorAdder::new(
                    base_embedding_dimension,
                    embedding_dimension,
                    property_vectors,
                    &mut vec,
                );
                feature_extraction::extract(
                    node_id as u64,
                    0,
                    &self.feature_extractors,
                    &mut adder,
                );
            }

            embedding_b[node_usize] = vec;
        }
    }

    fn add_initial_vectors_to_embedding(
        &self,
        embedding_b: &[Vec<f32>],
        embeddings: &mut [Vec<f32>],
    ) {
        if self.parameters.node_self_influence == 0.0 {
            return;
        }

        for node in 0..self.graph.node_count() {
            self.termination_flag.assert_running();
            let initial = &embedding_b[node];
            let l2 = l2_norm_f32(initial);
            let adjusted = if l2 < Self::EPSILON { 1.0 } else { l2 };
            add_weighted_in_place_f32(
                &mut embeddings[node],
                initial,
                self.parameters.node_self_influence / adjusted,
            );
        }
    }

    fn propagate_embeddings(
        &self,
        embeddings: &mut [Vec<f32>],
        embedding_a: &mut [Vec<f32>],
        embedding_b: &mut [Vec<f32>],
    ) {
        let relationship_weight_fallback = if self.parameters.relationship_weight_property.is_some() {
            f64::NAN
        } else {
            1.0
        };

        for (i, &iteration_weight) in self.parameters.iteration_weights.iter().enumerate() {
            self.termination_flag.assert_running();
            let first_iteration = i == 0;

            // Java parity:
            // current = (i % 2 == 0) ? A : B
            // previous = (i % 2 == 0) ? B : A
            let (current, previous): (&mut [Vec<f32>], &[Vec<f32>]) = if i % 2 == 0 {
                (&mut *embedding_a, &*embedding_b)
            } else {
                (&mut *embedding_b, &*embedding_a)
            };

            for node_usize in 0..self.graph.node_count() {
                self.termination_flag.assert_running();
                let node_id = node_usize as i64;

                let current_vec = &mut current[node_usize];
                current_vec.fill(0.0);

                // Collect neighbor embeddings (outgoing edges).
                if self.graph.has_relationship_property() {
                    for cursor in self
                        .graph
                        .stream_relationships_weighted(node_id, relationship_weight_fallback)
                    {
                        let weight = cursor.weight();
                        if first_iteration && weight.is_nan() {
                            let source = cursor.source_id();
                            let target = cursor.target_id();
                            let source_orig = self.graph.to_original_node_id(source).unwrap_or(source);
                            let target_orig = self.graph.to_original_node_id(target).unwrap_or(target);

                            panic!(
                                "Missing relationship property `{}` on relationship between nodes with ids `{}` and `{}`.",
                                self.parameters
                                    .relationship_weight_property
                                    .as_deref()
                                    .unwrap_or(""),
                                source_orig,
                                target_orig
                            );
                        }

                        let target_idx = cursor.target_id() as usize;
                        add_weighted_in_place_f32(current_vec, &previous[target_idx], weight as f32);
                    }
                } else {
                    for cursor in self.graph.stream_relationships(node_id, relationship_weight_fallback) {
                        let target_idx = cursor.target_id() as usize;
                        add_in_place_f32(current_vec, &previous[target_idx]);
                    }
                }

                // Normalize neighbor embeddings:
                let degree = self.graph.degree(node_id);
                let adjusted_degree = if degree == 0 { 1 } else { degree };
                let degree_scale = 1.0f32 / (adjusted_degree as f32);
                scale_f32(current_vec, degree_scale);

                let inv_l2 = 1.0f32 / l2_norm_f32(current_vec);
                let safe_inv = if inv_l2.is_finite() { inv_l2 } else { 1.0 };

                // Update final embedding.
                add_weighted_in_place_f32(
                    &mut embeddings[node_usize],
                    current_vec,
                    safe_inv * iteration_weight,
                );
            }
        }
    }
}

// =============================================================================
// Feature consumer: adds projected property vectors into the embedding tail.
// =============================================================================

struct PropertyVectorAdder<'a> {
    base_embedding_dimension: usize,
    embedding_dimension: usize,
    property_vectors: &'a [Vec<f32>],
    random_vector: &'a mut [f32],
}

impl<'a> PropertyVectorAdder<'a> {
    fn new(
        base_embedding_dimension: usize,
        embedding_dimension: usize,
        property_vectors: &'a [Vec<f32>],
        random_vector: &'a mut [f32],
    ) -> Self {
        Self {
            base_embedding_dimension,
            embedding_dimension,
            property_vectors,
            random_vector,
        }
    }

    #[inline]
    fn add_scaled_property_vector(&mut self, feature_offset: usize, value: f32) {
        if value == 0.0 {
            return;
        }

        let pv = &self.property_vectors[feature_offset];
        // property vectors are sized to property_dimension, appended at base dim.
        for i in self.base_embedding_dimension..self.embedding_dimension {
            let pv_i = pv[i - self.base_embedding_dimension];
            self.random_vector[i] += value * pv_i;
        }
    }
}

impl FeatureConsumer for PropertyVectorAdder<'_> {
    fn accept_scalar(&mut self, _node_offset: u64, offset: usize, value: f64) {
        self.add_scaled_property_vector(offset, value as f32);
    }

    fn accept_array(&mut self, _node_offset: u64, offset: usize, values: &[f64]) {
        for (j, &v) in values.iter().enumerate() {
            self.add_scaled_property_vector(offset + j, v as f32);
        }
    }
}

// =============================================================================
// Randomness: Java's HighQualityRandom translated.
// =============================================================================

struct HighQualityRandom {
    u: u64,
    v: u64,
    w: u64,
}

impl HighQualityRandom {
    fn new(seed: u64) -> Self {
        let mut r = Self {
            u: 0,
            v: 0,
            w: 0,
        };
        r.reseed(seed);
        r
    }

    fn reseed(&mut self, seed: u64) {
        self.v = 4101842887655102017u64;
        self.w = 1u64;
        self.u = seed ^ self.v;
        self.next_u64();
        self.v = self.u;
        self.next_u64();
        self.w = self.v;
        self.next_u64();
    }

    fn next_u64(&mut self) -> u64 {
        self.u = self
            .u
            .wrapping_mul(2862933555777941757u64)
            .wrapping_add(7046029254386353087u64);
        self.v ^= self.v >> 17;
        self.v ^= self.v << 31;
        self.v ^= self.v >> 8;
        self.w = 4294957665u64.wrapping_mul(self.w).wrapping_add(self.w >> 32);
        let mut x = self.u ^ (self.u << 21);
        x ^= x >> 35;
        x ^= x << 4;
        (x.wrapping_add(self.v)) ^ self.w
    }

    fn next_f64(&mut self) -> f64 {
        // Map to [0,1) with 53 bits of precision.
        // Similar in spirit to Java Random#nextDouble.
        let x = self.next_u64() >> 11; // keep top 53 bits
        (x as f64) * (1.0 / ((1u64 << 53) as f64))
    }
}

fn compute_random_entry(random: &mut HighQualityRandom, entry_value: f32) -> f32 {
    let r = random.next_f64();
    if r < FastRP::ENTRY_PROBABILITY {
        entry_value
    } else if r < FastRP::ENTRY_PROBABILITY * 2.0 {
        -entry_value
    } else {
        0.0
    }
}

// =============================================================================
// Vector ops (f32) to match Java float semantics.
// =============================================================================

#[inline]
fn add_in_place_f32(lhs: &mut [f32], rhs: &[f32]) {
    let len = lhs.len().min(rhs.len());
    for i in 0..len {
        lhs[i] += rhs[i];
    }
}

#[inline]
fn add_weighted_in_place_f32(lhs: &mut [f32], rhs: &[f32], weight: f32) {
    let len = lhs.len().min(rhs.len());
    for i in 0..len {
        lhs[i] = rhs[i].mul_add(weight, lhs[i]);
    }
}

#[inline]
fn scale_f32(lhs: &mut [f32], scalar: f32) {
    for x in lhs.iter_mut() {
        *x *= scalar;
    }
}

#[inline]
fn l2_norm_f32(v: &[f32]) -> f32 {
    let mut sum = 0.0f32;
    for &x in v {
        sum = x.mul_add(x, sum);
    }
    sum.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::{RandomGraphConfig, RandomRelationshipConfig};

    #[test]
    fn fastrp_smoke_produces_embeddings() {
        let config = RandomGraphConfig {
            graph_name: "fastrp".into(),
            database_name: "in-memory".into(),
            node_count: 16,
            node_labels: vec!["N".into()],
            relationships: vec![RandomRelationshipConfig::new("R", 0.3)],
            directed: true,
            inverse_indexed: false,
            seed: Some(7),
        };
        let store = DefaultGraphStore::random(&config).unwrap();
        let graph = store.graph();

        let params = FastRPParameters {
            feature_properties: vec![],
            iteration_weights: vec![1.0, 1.0],
            embedding_dimension: 8,
            property_dimension: 0,
            relationship_weight_property: None,
            normalization_strength: 0.0,
            node_self_influence: 1.0,
        };

        let algo = FastRP::new(
            graph,
            params,
            Concurrency::of(1),
            10_000,
            vec![],
            Some(7),
            TerminationFlag::default(),
        );

        let result = algo.compute();
        assert_eq!(result.embeddings.size(), config.node_count);
        assert_eq!(result.embeddings.get(0).len(), 8);
    }
}


