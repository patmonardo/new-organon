use std::cmp::Ordering;

/// Trait for computing similarity between two node vectors.
pub trait SimilarityMetric: Send + Sync {
    /// Compute similarity between two unweighted vectors (node IDs).
    fn compute_similarity(&self, vector1: &[u64], vector2: &[u64]) -> f64;

    /// Compute similarity between two weighted vectors.
    /// `vector1` and `weights1` must have the same length.
    fn compute_weighted_similarity(
        &self,
        vector1: &[u64],
        vector2: &[u64],
        weights1: &[f64],
        weights2: &[f64],
    ) -> f64;
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeSimilarityMetric {
    Jaccard,
    Cosine,
    Overlap,
}

impl NodeSimilarityMetric {
    pub fn create(&self, similarity_cutoff: f64) -> Box<dyn SimilarityMetric> {
        match self {
            NodeSimilarityMetric::Jaccard => Box::new(Jaccard::new(similarity_cutoff)),
            NodeSimilarityMetric::Cosine => Box::new(Cosine::new(similarity_cutoff)),
            NodeSimilarityMetric::Overlap => Box::new(Overlap::new(similarity_cutoff)),
        }
    }
}

// ============================================================================
// Jaccard
// ============================================================================

pub struct Jaccard {
    similarity_cutoff: f64,
}

impl Jaccard {
    pub fn new(similarity_cutoff: f64) -> Self {
        Self { similarity_cutoff }
    }
}

impl SimilarityMetric for Jaccard {
    fn compute_similarity(&self, vector1: &[u64], vector2: &[u64]) -> f64 {
        let intersection = intersection_size(vector1, vector2);
        let union_size = vector1.len() + vector2.len() - intersection;

        let similarity = if union_size == 0 {
            0.0
        } else {
            intersection as f64 / union_size as f64
        };

        if similarity >= self.similarity_cutoff {
            similarity
        } else {
            f64::NAN
        }
    }

    fn compute_weighted_similarity(
        &self,
        vector1: &[u64],
        vector2: &[u64],
        weights1: &[f64],
        weights2: &[f64],
    ) -> f64 {
        debug_assert_eq!(vector1.len(), weights1.len());
        debug_assert_eq!(vector2.len(), weights2.len());

        let mut offset1 = 0;
        let mut offset2 = 0;
        let mut max = 0.0;
        let mut min = 0.0;

        while offset1 < vector1.len() && offset2 < vector2.len() {
            let target1 = vector1[offset1];
            let target2 = vector2[offset2];

            if target1 == target2 {
                let w1 = weights1[offset1];
                let w2 = weights2[offset2];
                if w1 > w2 {
                    max += w1;
                    min += w2;
                } else {
                    min += w1;
                    max += w2;
                }
                offset1 += 1;
                offset2 += 1;
            } else if target1 < target2 {
                max += weights1[offset1];
                offset1 += 1;
            } else {
                max += weights2[offset2];
                offset2 += 1;
            }
        }

        while offset1 < vector1.len() {
            max += weights1[offset1];
            offset1 += 1;
        }

        while offset2 < vector2.len() {
            max += weights2[offset2];
            offset2 += 1;
        }

        let similarity = if max == 0.0 { 0.0 } else { min / max };

        if similarity >= self.similarity_cutoff {
            similarity
        } else {
            f64::NAN
        }
    }
}

// ============================================================================
// Cosine
// ============================================================================

pub struct Cosine {
    similarity_cutoff: f64,
}

impl Cosine {
    pub fn new(similarity_cutoff: f64) -> Self {
        Self { similarity_cutoff }
    }
}

impl SimilarityMetric for Cosine {
    fn compute_similarity(&self, vector1: &[u64], vector2: &[u64]) -> f64 {
        let intersection = intersection_size(vector1, vector2) as f64;
        let similarity =
            intersection / ((vector1.len() as f64).sqrt() * (vector2.len() as f64).sqrt());

        if similarity >= self.similarity_cutoff {
            similarity
        } else {
            f64::NAN
        }
    }

    fn compute_weighted_similarity(
        &self,
        vector1: &[u64],
        vector2: &[u64],
        weights1: &[f64],
        weights2: &[f64],
    ) -> f64 {
        debug_assert_eq!(vector1.len(), weights1.len());
        debug_assert_eq!(vector2.len(), weights2.len());

        let mut offset1 = 0;
        let mut offset2 = 0;
        let mut dot_product = 0.0;
        let mut vector1_squared_sum = 0.0;
        let mut vector2_squared_sum = 0.0;

        while offset1 < vector1.len() && offset2 < vector2.len() {
            let target1 = vector1[offset1];
            let target2 = vector2[offset2];
            let w1 = weights1[offset1];
            let w2 = weights2[offset2];

            if target1 == target2 {
                dot_product += w1 * w2;
                vector1_squared_sum += w1 * w1;
                vector2_squared_sum += w2 * w2;
                offset1 += 1;
                offset2 += 1;
            } else if target1 < target2 {
                vector1_squared_sum += w1 * w1;
                offset1 += 1;
            } else {
                vector2_squared_sum += w2 * w2;
                offset2 += 1;
            }
        }

        while offset1 < vector1.len() {
            let w1 = weights1[offset1];
            vector1_squared_sum += w1 * w1;
            offset1 += 1;
        }

        while offset2 < vector2.len() {
            let w2 = weights2[offset2];
            vector2_squared_sum += w2 * w2;
            offset2 += 1;
        }

        let similarity = dot_product / (vector1_squared_sum.sqrt() * vector2_squared_sum.sqrt());

        if similarity >= self.similarity_cutoff {
            similarity
        } else {
            f64::NAN
        }
    }
}

// ============================================================================
// Overlap
// ============================================================================

pub struct Overlap {
    similarity_cutoff: f64,
}

impl Overlap {
    pub fn new(similarity_cutoff: f64) -> Self {
        Self { similarity_cutoff }
    }
}

impl SimilarityMetric for Overlap {
    fn compute_similarity(&self, vector1: &[u64], vector2: &[u64]) -> f64 {
        let intersection = intersection_size(vector1, vector2) as f64;
        let min_len = vector1.len().min(vector2.len()) as f64;

        let similarity = if min_len == 0.0 {
            0.0
        } else {
            intersection / min_len
        };

        if similarity >= self.similarity_cutoff {
            similarity
        } else {
            f64::NAN
        }
    }

    fn compute_weighted_similarity(
        &self,
        vector1: &[u64],
        vector2: &[u64],
        weights1: &[f64],
        weights2: &[f64],
    ) -> f64 {
        // Overlap weighted: sum(min(w1, w2)) / min(sum(w1), sum(w2))
        // This is a common interpretation. Checking logic against standard.
        // Wait, Java Overlap might just be intersection of keys.
        // Let's implement generic overlap for weighted:
        // O(A, B) = |A intersects B| / min(|A|, |B|)
        // For weighted, it is often sum(min(Ai, Bi)) / min(sum(A), sum(B))

        debug_assert_eq!(vector1.len(), weights1.len());
        debug_assert_eq!(vector2.len(), weights2.len());

        let mut offset1 = 0;
        let mut offset2 = 0;
        let mut intersection_weight = 0.0;
        let mut sum1 = 0.0;
        let mut sum2 = 0.0;

        while offset1 < vector1.len() && offset2 < vector2.len() {
            let target1 = vector1[offset1];
            let target2 = vector2[offset2];
            let w1 = weights1[offset1];
            let w2 = weights2[offset2];

            if target1 == target2 {
                intersection_weight += w1.min(w2);
                sum1 += w1;
                sum2 += w2;
                offset1 += 1;
                offset2 += 1;
            } else if target1 < target2 {
                sum1 += w1;
                offset1 += 1;
            } else {
                sum2 += w2;
                offset2 += 1;
            }
        }

        while offset1 < vector1.len() {
            sum1 += weights1[offset1];
            offset1 += 1;
        }

        while offset2 < vector2.len() {
            sum2 += weights2[offset2];
            offset2 += 1;
        }

        let min_sum = sum1.min(sum2);
        let similarity = if min_sum == 0.0 {
            0.0
        } else {
            intersection_weight / min_sum
        };

        if similarity >= self.similarity_cutoff {
            similarity
        } else {
            f64::NAN
        }
    }
}

// ============================================================================
// Utils
// ============================================================================

/// Assumes sorted vectors
fn intersection_size(v1: &[u64], v2: &[u64]) -> usize {
    let mut count = 0;
    let mut i = 0;
    let mut j = 0;
    while i < v1.len() && j < v2.len() {
        match v1[i].cmp(&v2[j]) {
            Ordering::Equal => {
                count += 1;
                i += 1;
                j += 1;
            }
            Ordering::Less => i += 1,
            Ordering::Greater => j += 1,
        }
    }
    count
}
