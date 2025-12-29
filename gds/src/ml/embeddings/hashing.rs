//! Hashing-trick embeddings for text.
//!
//! This is intentionally simple and dependency-free:
//! - tokenize by `split_whitespace()`
//! - hash tokens into a fixed-size vector (signed counts)
//! - L2 normalize
//!
//! This gives a useful “similarity emerges” surface (via cosine similarity)
//! without requiring an external model/runtime.

use crate::ml::core::tensor::operations::l2_normalize;
use crate::ml::core::tensor::Vector;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Embed text into a fixed-size vector using the hashing trick.
///
/// - `dims` must be > 0.
/// - output is L2-normalized (unit vector) unless input is empty.
pub fn hashing_embed(text: &str, dims: usize) -> Vector {
    assert!(dims > 0, "hashing_embed requires dims > 0");

    let mut data = vec![0.0; dims];
    for token in text.split_whitespace() {
        let mut hasher = DefaultHasher::new();
        token.to_lowercase().hash(&mut hasher);
        let h = hasher.finish() as usize;
        let idx = h % dims;
        // signed hashing to reduce collisions bias (count-sketch style)
        let sign = if (h >> 1) & 1 == 0 { 1.0 } else { -1.0 };
        data[idx] += sign;
    }

    l2_normalize(&mut data);
    Vector::new(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::core::tensor::operations::cosine_similarity;

    #[test]
    fn hashing_embed_is_deterministic() {
        let a = hashing_embed("Pure Reason", 64);
        let b = hashing_embed("Pure Reason", 64);
        assert_eq!(a, b);
    }

    #[test]
    fn hashing_embed_cosine_self_is_one_for_non_empty() {
        let v = hashing_embed("pure reason speaks for itself", 128);
        let sim = cosine_similarity(v.as_slice(), v.as_slice());
        assert!((sim - 1.0).abs() < 1e-12);
    }
}
