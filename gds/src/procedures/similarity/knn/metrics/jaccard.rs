use crate::core::utils::Intersections;

pub fn metric(left: &[i64], right: &[i64]) -> f64 {
    if left.is_empty() && right.is_empty() {
        return 0.0;
    }
    let intersection = Intersections::intersection3(left, right) as f64;
    let union = (left.len() + right.len()) as f64 - intersection;
    if union == 0.0 {
        0.0
    } else {
        intersection / union
    }
}
