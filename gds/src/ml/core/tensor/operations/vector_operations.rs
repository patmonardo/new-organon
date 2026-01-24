//! Vector operations - translated from FloatVectorOperations.java

/// Compute dot product between two vectors.
///
/// Panics if the lengths differ.
pub fn dot(lhs: &[f64], rhs: &[f64]) -> f64 {
    assert_eq!(
        lhs.len(),
        rhs.len(),
        "dot product requires equal lengths, got {} and {}",
        lhs.len(),
        rhs.len()
    );

    lhs.iter().zip(rhs.iter()).map(|(a, b)| a * b).sum()
}

/// Compute cosine similarity between two vectors.
///
/// Returns 0.0 if either vector has zero norm.
/// Panics if the lengths differ.
pub fn cosine_similarity(lhs: &[f64], rhs: &[f64]) -> f64 {
    assert_eq!(
        lhs.len(),
        rhs.len(),
        "cosine similarity requires equal lengths, got {} and {}",
        lhs.len(),
        rhs.len()
    );

    let mut dot_product = 0.0;
    let mut lhs_norm_sq = 0.0;
    let mut rhs_norm_sq = 0.0;

    for i in 0..lhs.len() {
        let a = lhs[i];
        let b = rhs[i];
        dot_product += a * b;
        lhs_norm_sq += a * a;
        rhs_norm_sq += b * b;
    }

    let denom = (lhs_norm_sq * rhs_norm_sq).sqrt();
    if denom > 0.0 {
        dot_product / denom
    } else {
        0.0
    }
}

/// Add rhs to lhs in place.
pub fn add_in_place(lhs: &mut [f64], rhs: &[f64]) {
    let length = lhs.len().min(rhs.len());

    for i in 0..length {
        lhs[i] += rhs[i];
    }
}

/// Add weighted rhs to lhs in place: lhs[i] += weight * rhs[i]
pub fn add_weighted_in_place(lhs: &mut [f64], rhs: &[f64], weight: f64) {
    let length = lhs.len().min(rhs.len());

    for i in 0..length {
        lhs[i] += weight * rhs[i];
    }
}

/// Scale vector by scalar (in-place).
pub fn scale(lhs: &mut [f64], scalar: f64) {
    for value in lhs.iter_mut() {
        *value *= scalar;
    }
}

/// Scale vector by scalar and write to output.
pub fn scale_to(lhs: &[f64], scalar: f64, out: &mut [f64]) {
    assert_eq!(out.len(), lhs.len());

    for (i, &value) in lhs.iter().enumerate() {
        out[i] = value * scalar;
    }
}

/// Calculate L2 norm (Euclidean length) of vector.
pub fn l2_norm(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().map(|&v| v * v).sum();
    sum.sqrt()
}

/// Normalize vector to unit length (L2 normalization).
pub fn l2_normalize(array: &mut [f64]) {
    let euclidean_length = l2_norm(array);
    if euclidean_length > 0.0 {
        scale(array, 1.0 / euclidean_length);
    }
}

/// Check if any element in vector matches predicate.
pub fn any_match<F>(vector: &[f64], mut predicate: F) -> bool
where
    F: FnMut(f64) -> bool,
{
    vector.iter().any(|&v| predicate(v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot() {
        let lhs = vec![1.0, 2.0, 3.0];
        let rhs = vec![4.0, 5.0, 6.0];
        assert_eq!(dot(&lhs, &rhs), 32.0);
    }

    #[test]
    fn test_cosine_similarity_perfect() {
        let v1 = vec![1.0, 2.0, 3.0];
        let v2 = vec![2.0, 4.0, 6.0];
        let sim = cosine_similarity(&v1, &v2);
        assert!((sim - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let v1 = vec![1.0, 0.0];
        let v2 = vec![0.0, 1.0];
        let sim = cosine_similarity(&v1, &v2);
        assert!((sim - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_cosine_similarity_zero_vector_returns_zero() {
        let v1 = vec![0.0, 0.0, 0.0];
        let v2 = vec![1.0, 2.0, 3.0];
        let sim = cosine_similarity(&v1, &v2);
        assert_eq!(sim, 0.0);
    }

    #[test]
    fn test_add_in_place() {
        let mut lhs = vec![1.0, 2.0, 3.0];
        let rhs = vec![4.0, 5.0, 6.0];
        add_in_place(&mut lhs, &rhs);
        assert_eq!(lhs, vec![5.0, 7.0, 9.0]);
    }

    #[test]
    fn test_add_weighted_in_place() {
        let mut lhs = vec![1.0, 2.0, 3.0];
        let rhs = vec![4.0, 5.0, 6.0];
        add_weighted_in_place(&mut lhs, &rhs, 2.0);
        assert_eq!(lhs, vec![9.0, 12.0, 15.0]);
    }

    #[test]
    fn test_scale() {
        let mut data = vec![1.0, 2.0, 3.0];
        scale(&mut data, 2.0);
        assert_eq!(data, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_l2_norm() {
        let data = vec![3.0, 4.0];
        assert_eq!(l2_norm(&data), 5.0);
    }

    #[test]
    fn test_l2_normalize() {
        let mut data = vec![3.0, 4.0];
        l2_normalize(&mut data);
        assert!((data[0] - 0.6).abs() < 1e-10);
        assert!((data[1] - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_any_match() {
        let data = vec![1.0, 2.0, 3.0];
        assert!(any_match(&data, |v| v > 2.0));
        assert!(!any_match(&data, |v| v > 10.0));
    }
}
