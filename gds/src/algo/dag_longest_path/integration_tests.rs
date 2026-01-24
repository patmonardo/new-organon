#[cfg(test)]
mod tests {
    use crate::algo::dag_longest_path::DagLongestPathComputationRuntime;

    #[test]
    fn test_simple_dag() {
        // 0 -> 1 -> 2
        let edges: Vec<Vec<(i64, f64)>> = vec![vec![(1, 1.0)], vec![(2, 1.0)], vec![]];

        let get_neighbors = move |node: i64| edges[node as usize].clone();

        let mut runtime = DagLongestPathComputationRuntime::new(3);
        let result = runtime.compute(3, get_neighbors);

        // Should have paths from 0 to all reachable nodes
        assert!(!result.paths.is_empty());

        // Find path to node 2
        let path_to_2 = result.paths.iter().find(|p| p.target_node == 2).unwrap();
        assert_eq!(path_to_2.source_node, 0);
        assert_eq!(path_to_2.total_cost, 2.0);
        assert_eq!(path_to_2.node_ids, vec![0, 1, 2]);
    }

    #[test]
    fn test_diamond_dag() {
        // 0 -> 1 (weight 1) -> 3 (weight 1)
        //  \-> 2 (weight 2) ->/  (weight 1)
        let edges: Vec<Vec<(i64, f64)>> = vec![
            vec![(1, 1.0), (2, 2.0)],
            vec![(3, 1.0)],
            vec![(3, 1.0)],
            vec![],
        ];

        let get_neighbors = move |node: i64| edges[node as usize].clone();

        let mut runtime = DagLongestPathComputationRuntime::new(4);
        let result = runtime.compute(4, get_neighbors);

        // Find path to node 3
        let path_to_3 = result.paths.iter().find(|p| p.target_node == 3).unwrap();

        // Longest path should be 0->2->3 (cost 3.0)
        assert_eq!(path_to_3.total_cost, 3.0);
        assert_eq!(path_to_3.node_ids, vec![0, 2, 3]);
    }

    #[test]
    fn test_weighted_dag() {
        // 0 -> 1 (weight 5) -> 2 (weight 3)
        let edges: Vec<Vec<(i64, f64)>> = vec![vec![(1, 5.0)], vec![(2, 3.0)], vec![]];

        let get_neighbors = move |node: i64| edges[node as usize].clone();

        let mut runtime = DagLongestPathComputationRuntime::new(3);
        let result = runtime.compute(3, get_neighbors);

        let path_to_2 = result.paths.iter().find(|p| p.target_node == 2).unwrap();
        assert_eq!(path_to_2.total_cost, 8.0);
    }

    #[test]
    fn test_disconnected_dag() {
        // 0 -> 1, 2 -> 3 (two separate chains)
        let edges: Vec<Vec<(i64, f64)>> = vec![vec![(1, 1.0)], vec![], vec![(3, 1.0)], vec![]];

        let get_neighbors = move |node: i64| edges[node as usize].clone();

        let mut runtime = DagLongestPathComputationRuntime::new(4);
        let result = runtime.compute(4, get_neighbors);

        // Should have paths from both components
        assert!(result.paths.len() >= 2);
    }
}
