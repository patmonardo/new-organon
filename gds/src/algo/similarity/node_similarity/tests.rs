#[cfg(test)]
use super::{Cosine, Jaccard, Overlap, SimilarityMetric};

#[test]
fn test_jaccard_similarity() {
    let metric = Jaccard::new(0.0);
    let v1 = vec![1, 2, 3];
    let v2 = vec![2, 3, 4];
    // Intersection: {2, 3} (2)
    // Union: {1, 2, 3, 4} (4)
    // Jaccard: 0.5
    let sim = metric.compute_similarity(&v1, &v2); // Vectors are sorted
    assert!((sim - 0.5).abs() < 1e-6);
}

#[test]
fn test_jaccard_disjoint() {
    let metric = Jaccard::new(0.0);
    let v1 = vec![1, 2];
    let v2 = vec![3, 4];
    let sim = metric.compute_similarity(&v1, &v2);
    assert_eq!(sim, 0.0);
}

#[test]
fn test_cosine_similarity() {
    let metric = Cosine::new(0.0);
    let v1 = vec![1, 2, 3];
    let v2 = vec![2, 3, 4];
    // Intersection: 2
    // Sqrt(3) * Sqrt(3) = 3
    // Cosine: 2/3 = 0.666...
    let sim = metric.compute_similarity(&v1, &v2);
    assert!((sim - 2.0 / 3.0).abs() < 1e-6);
}

#[test]
fn test_overlap_similarity() {
    let metric = Overlap::new(0.0);
    let v1 = vec![1, 2, 3]; // len 3
    let v2 = vec![2, 3, 4, 5]; // len 4
                               // Intersection: 2
                               // Min size: 3
                               // Overlap: 2/3
    let sim = metric.compute_similarity(&v1, &v2);
    assert!((sim - 2.0 / 3.0).abs() < 1e-6);
}

#[test]
fn test_weighted_jaccard() {
    let metric = Jaccard::new(0.0);
    let v1 = vec![1, 2, 3];
    let w1 = vec![1.0, 1.0, 1.0];

    let v2 = vec![2, 3, 4];
    let w2 = vec![0.5, 2.0, 1.0];

    // 1: w1=1.0 only -> max+=1.0
    // 2: w1=1.0, w2=0.5 -> min+=0.5, max+=1.0
    // 3: w1=1.0, w2=2.0 -> min+=1.0, max+=2.0
    // 4: w2=1.0 only -> max+=1.0

    // Min sum: 0.5 + 1.0 = 1.5
    // Max sum: 1.0 (from 1) + 1.0 (from 2) + 2.0 (from 3) + 1.0 (from 4) = 5.0
    // Sim: 1.5 / 5.0 = 0.3

    let sim = metric.compute_weighted_similarity(&v1, &v2, &w1, &w2);
    assert!((sim - 0.3).abs() < 1e-6);
}
