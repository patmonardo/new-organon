/// Euclidean similarity: 1/(1+sqrt(sum((a-b)^2))) over the overlapping prefix.
pub fn float_metric(left: &[f32], right: &[f32]) -> f64 {
    let len = left.len().min(right.len());
    if len == 0 {
        return 0.0;
    }
    let mut sum = 0.0f64;
    for i in 0..len {
        let delta = left[i] as f64 - right[i] as f64;
        sum += delta * delta;
    }
    1.0 / (1.0 + sum.sqrt())
}

/// Euclidean similarity: 1/(1+sqrt(sum((a-b)^2))) over the overlapping prefix.
pub fn double_metric(left: &[f64], right: &[f64]) -> f64 {
    let len = left.len().min(right.len());
    if len == 0 {
        return 0.0;
    }
    let mut sum = 0.0f64;
    for i in 0..len {
        let delta = left[i] - right[i];
        sum += delta * delta;
    }
    1.0 / (1.0 + sum.sqrt())
}
