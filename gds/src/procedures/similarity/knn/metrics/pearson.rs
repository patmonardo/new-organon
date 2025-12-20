/// Pearson correlation coefficient mapped from [-1, 1] to [0, 1] via (r+1)/2.
pub fn float_metric(a: &[f32], b: &[f32]) -> f64 {
    let n = a.len().min(b.len());
    if n == 0 {
        return 0.0;
    }

    let mut sum_a = 0.0f64;
    let mut sum_b = 0.0f64;
    for i in 0..n {
        sum_a += a[i] as f64;
        sum_b += b[i] as f64;
    }
    let mean_a = sum_a / n as f64;
    let mean_b = sum_b / n as f64;

    let mut num = 0.0f64;
    let mut da = 0.0f64;
    let mut db = 0.0f64;
    for i in 0..n {
        let a_delta = a[i] as f64 - mean_a;
        let b_delta = b[i] as f64 - mean_b;
        num += a_delta * b_delta;
        da += a_delta * a_delta;
        db += b_delta * b_delta;
    }

    let denom = (da * db).sqrt();
    if denom == 0.0 {
        return 0.0;
    }
    let r = num / denom;
    ((r + 1.0) / 2.0).clamp(0.0, 1.0)
}

/// Pearson correlation coefficient mapped from [-1, 1] to [0, 1] via (r+1)/2.
pub fn double_metric(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    if n == 0 {
        return 0.0;
    }

    let mut sum_a = 0.0f64;
    let mut sum_b = 0.0f64;
    for i in 0..n {
        sum_a += a[i];
        sum_b += b[i];
    }
    let mean_a = sum_a / n as f64;
    let mean_b = sum_b / n as f64;

    let mut num = 0.0f64;
    let mut da = 0.0f64;
    let mut db = 0.0f64;
    for i in 0..n {
        let a_delta = a[i] - mean_a;
        let b_delta = b[i] - mean_b;
        num += a_delta * b_delta;
        da += a_delta * a_delta;
        db += b_delta * b_delta;
    }

    let denom = (da * db).sqrt();
    if denom == 0.0 {
        return 0.0;
    }
    let r = num / denom;
    ((r + 1.0) / 2.0).clamp(0.0, 1.0)
}
