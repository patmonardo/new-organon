use crate::core::utils::Intersections;

/// Cosine similarity mapped from [-1, 1] to [0, 1] via (x+1)/2.
pub fn float_metric(left: &[f32], right: &[f32]) -> f64 {
    let len = left.len().min(right.len());
    if len == 0 {
        return 0.0;
    }
    let cosine = Intersections::cosine_f32(left, right, len);
    ((cosine as f64 + 1.0) / 2.0).clamp(0.0, 1.0)
}

/// Cosine similarity mapped from [-1, 1] to [0, 1] via (x+1)/2.
pub fn double_metric(left: &[f64], right: &[f64]) -> f64 {
    let len = left.len().min(right.len());
    if len == 0 {
        return 0.0;
    }
    let cosine = Intersections::cosine_f64(left, right, len);
    ((cosine + 1.0) / 2.0).clamp(0.0, 1.0)
}
