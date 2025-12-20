use crate::collections::HugeLongArray;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Produces negative samples according to a cumulative distribution.
///
/// Java: `NegativeSampleProducer`
pub struct NegativeSampleProducer {
    context_node_distribution: HugeLongArray,
    cumulative_probability: i64,
    rng: ChaCha8Rng,
}

impl NegativeSampleProducer {
    pub fn new(context_node_distribution: HugeLongArray, random_seed: u64) -> Self {
        let last = context_node_distribution.size().saturating_sub(1);
        let cumulative_probability = context_node_distribution.get(last);
        Self {
            context_node_distribution,
            cumulative_probability,
            rng: ChaCha8Rng::seed_from_u64(random_seed),
        }
    }

    pub fn next(&mut self) -> i64 {
        let cp = self.cumulative_probability;
        if cp <= 0 {
            return 0;
        }

        let r = self.rng.gen_range(0..cp);

        // Find the first index where cumulative >= r (standard alias-table-like sampling
        // using a cumulative distribution array).
        let mut lo = 0usize;
        let mut hi = self.context_node_distribution.size();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.context_node_distribution.get(mid) >= r {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo as i64
    }
}


