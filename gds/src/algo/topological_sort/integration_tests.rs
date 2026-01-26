#[cfg(test)]
mod tests {
    use crate::algo::topological_sort::TopologicalSortComputationRuntime;
    use crate::types::graph::NodeId;

    #[test]
    fn test_simple_dag() {
        // 0 -> 1 -> 2
        let edges: Vec<Vec<(NodeId, f64)>> = vec![vec![(1, 1.0)], vec![(2, 1.0)], vec![]];

        let get_neighbors = |node: NodeId| edges[node as usize].clone();

        let mut runtime = TopologicalSortComputationRuntime::new(3, false);
        let result = runtime.compute(3, get_neighbors);

        assert_eq!(result.sorted_nodes.len(), 3);
        assert_eq!(result.sorted_nodes, vec![0, 1, 2]);
    }

    #[test]
    fn test_with_max_distance() {
        // 0 -> 1 (weight 2) -> 2 (weight 3)
        let edges: Vec<Vec<(NodeId, f64)>> = vec![vec![(1, 2.0)], vec![(2, 3.0)], vec![]];

        let get_neighbors = |node: NodeId| edges[node as usize].clone();

        let mut runtime = TopologicalSortComputationRuntime::new(3, true);
        let result = runtime.compute(3, get_neighbors);

        assert_eq!(result.sorted_nodes, vec![0, 1, 2]);
        let distances = result.max_source_distances.unwrap();
        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 2.0);
        assert_eq!(distances[2], 5.0);
    }

    #[test]
    fn test_diamond_dag() {
        // 0 -> 1 -> 3
        //  \-> 2 ->/
        let edges: Vec<Vec<(NodeId, f64)>> = vec![
            vec![(1, 1.0), (2, 2.0)],
            vec![(3, 1.0)],
            vec![(3, 1.0)],
            vec![],
        ];

        let get_neighbors = |node: NodeId| edges[node as usize].clone();

        let mut runtime = TopologicalSortComputationRuntime::new(4, true);
        let result = runtime.compute(4, get_neighbors);

        assert_eq!(result.sorted_nodes.len(), 4);
        assert_eq!(result.sorted_nodes[0], 0);
        // Longest path to node 3 should be through node 2
        let distances = result.max_source_distances.unwrap();
        assert_eq!(distances[3], 3.0); // 0->2->3
    }

    #[test]
    fn test_disconnected_graph() {
        // 0 -> 1, 2 -> 3 (two separate chains)
        let edges: Vec<Vec<(NodeId, f64)>> = vec![vec![(1, 1.0)], vec![], vec![(3, 1.0)], vec![]];

        let get_neighbors = |node: NodeId| edges[node as usize].clone();

        let mut runtime = TopologicalSortComputationRuntime::new(4, false);
        let result = runtime.compute(4, get_neighbors);

        assert_eq!(result.sorted_nodes.len(), 4);
        // Should include all nodes from both components
    }
}
