#[cfg(test)]
mod tests {
    use crate::algo::random_walk::RandomWalkComputationRuntime;

    #[test]
    fn test_simple_walk() {
        // Simple path: 0 -> 1 -> 2
        let edges: Vec<Vec<usize>> = vec![vec![1], vec![2], vec![]];

        let get_neighbors = |node: usize| edges[node].clone();

        let runtime = RandomWalkComputationRuntime::new(
            1,       // walks_per_node
            3,       // walk_length
            1.0,     // return_factor
            1.0,     // in_out_factor
            vec![0], // start from node 0
            42,      // random_seed
        );

        let result = runtime.compute(3, get_neighbors);

        assert_eq!(result.walks.len(), 1);
        assert_eq!(result.walks[0], vec![0, 1, 2]);
    }

    #[test]
    fn test_multiple_walks_per_node() {
        // Simple path: 0 -> 1 -> 2
        let edges: Vec<Vec<usize>> = vec![vec![1], vec![2], vec![]];

        let get_neighbors = |node: usize| edges[node].clone();

        let runtime = RandomWalkComputationRuntime::new(
            3,       // walks_per_node
            3,       // walk_length
            1.0,     // return_factor
            1.0,     // in_out_factor
            vec![0], // start from node 0
            42,      // random_seed
        );

        let result = runtime.compute(3, get_neighbors);

        assert_eq!(result.walks.len(), 3);
        // All walks should start from node 0
        assert!(result.walks.iter().all(|w| w[0] == 0));
    }

    #[test]
    fn test_walk_with_branching() {
        // 0 -> 1, 0 -> 2 (branching from 0)
        let edges: Vec<Vec<usize>> = vec![vec![1, 2], vec![], vec![]];

        let get_neighbors = |node: usize| edges[node].clone();

        let runtime = RandomWalkComputationRuntime::new(
            5,       // walks_per_node (multiple to test randomness)
            2,       // walk_length
            1.0,     // return_factor
            1.0,     // in_out_factor
            vec![0], // start from node 0
            42,      // random_seed
        );

        let result = runtime.compute(3, get_neighbors);

        assert_eq!(result.walks.len(), 5);
        // All walks should start from 0 and go to either 1 or 2
        for walk in &result.walks {
            assert_eq!(walk[0], 0);
            assert!(walk[1] == 1 || walk[1] == 2);
        }
    }

    #[test]
    fn test_walk_stops_at_dead_end() {
        // 0 -> 1 (dead end at 1)
        let edges: Vec<Vec<usize>> = vec![vec![1], vec![]];

        let get_neighbors = |node: usize| edges[node].clone();

        let runtime = RandomWalkComputationRuntime::new(
            1,       // walks_per_node
            10,      // walk_length (longer than possible)
            1.0,     // return_factor
            1.0,     // in_out_factor
            vec![0], // start from node 0
            42,      // random_seed
        );

        let result = runtime.compute(2, get_neighbors);

        assert_eq!(result.walks.len(), 1);
        // Walk should stop at node 1 (dead end)
        assert_eq!(result.walks[0], vec![0, 1]);
    }

    #[test]
    fn test_walks_from_all_nodes() {
        // Triangle: 0 <-> 1 <-> 2 <-> 0
        let edges: Vec<Vec<usize>> = vec![vec![1, 2], vec![0, 2], vec![0, 1]];

        let get_neighbors = |node: usize| edges[node].clone();

        let runtime = RandomWalkComputationRuntime::new(
            1,      // walks_per_node
            3,      // walk_length
            1.0,    // return_factor
            1.0,    // in_out_factor
            vec![], // empty = walk from all nodes
            42,     // random_seed
        );

        let result = runtime.compute(3, get_neighbors);

        // Should have 3 walks (one per node)
        assert_eq!(result.walks.len(), 3);

        // Check that walks start from different nodes
        let start_nodes: std::collections::HashSet<u64> =
            result.walks.iter().map(|w| w[0]).collect();
        assert_eq!(start_nodes.len(), 3);
    }
}
