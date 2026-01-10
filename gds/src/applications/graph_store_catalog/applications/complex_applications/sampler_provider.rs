use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;

/// SamplerProvider
///
/// Small deterministic sampler helpers used by sampling/project applications.
pub struct SamplerProvider;

impl SamplerProvider {
    pub fn new() -> Self {
        Self
    }

    /// Select `sample_size` indices from `0..node_count` deterministically using `seed`.
    pub fn select_indices(node_count: usize, sample_size: usize, seed: u64) -> Vec<usize> {
        let mut ids: Vec<usize> = (0..node_count).collect();
        let mut rng = StdRng::seed_from_u64(seed);
        ids.shuffle(&mut rng);
        ids.into_iter().take(sample_size.min(node_count)).collect()
    }
}

impl Default for SamplerProvider {
    fn default() -> Self {
        Self::new()
    }
}
