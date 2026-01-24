use super::TriangleComputationRuntime;

#[test]
fn triangle_empty() {
    let mut rt = TriangleComputationRuntime::new();
    let result = rt.compute(0, |_| Vec::new());
    assert_eq!(result.global_triangles, 0);
    assert!(result.local_triangles.is_empty());
}

#[test]
fn triangle_single_triangle() {
    // 0-1-2-0
    let mut rt = TriangleComputationRuntime::new();
    let neighbors = |n: usize| match n {
        0 => vec![1, 2],
        1 => vec![0, 2],
        2 => vec![0, 1],
        _ => vec![],
    };
    let result = rt.compute(3, neighbors);

    assert_eq!(result.global_triangles, 1);
    assert_eq!(result.local_triangles, vec![1, 1, 1]);
}

#[test]
fn triangle_two_share_edge() {
    // Triangles: (0,1,2) and (0,1,3)
    let mut rt = TriangleComputationRuntime::new();
    let neighbors = |n: usize| match n {
        0 => vec![1, 2, 3],
        1 => vec![0, 2, 3],
        2 => vec![0, 1],
        3 => vec![0, 1],
        _ => vec![],
    };
    let result = rt.compute(4, neighbors);

    assert_eq!(result.global_triangles, 2);
    assert_eq!(result.local_triangles, vec![2, 2, 1, 1]);
}
