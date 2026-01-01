use crate::core::utils::Intersections;

pub fn metric(left: &[i64], right: &[i64]) -> f64 {
    let denom = left.len().min(right.len()) as f64;
    if denom == 0.0 {
        return 0.0;
    }
    let intersection = Intersections::intersection3(left, right) as f64;
    intersection / denom
}
