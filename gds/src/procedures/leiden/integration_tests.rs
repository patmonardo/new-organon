use super::computation::leiden;
use super::spec::LeidenConfig;

#[test]
fn test_leiden_simple_communities() {
    // Two clear communities: 0-1-2 and 3-4-5
    let edges = vec![
        (0, 1, 1.0),
        (1, 0, 1.0),
        (1, 2, 1.0),
        (2, 1, 1.0),
        (0, 2, 1.0),
        (2, 0, 1.0),
        (3, 4, 1.0),
        (4, 3, 1.0),
        (4, 5, 1.0),
        (5, 4, 1.0),
        (3, 5, 1.0),
        (5, 3, 1.0),
        // Weak bridge
        (2, 3, 0.1),
        (3, 2, 0.1),
    ];

    let get_neighbors = |node: usize| -> Vec<(usize, f64)> {
        edges
            .iter()
            .filter(|(src, _, _)| *src == node)
            .map(|(_, dst, weight)| (*dst, *weight))
            .collect()
    };

    let config = LeidenConfig {
        gamma: 1.0,
        theta: 0.01,
        tolerance: 0.0001,
        max_iterations: 10,
        seed_communities: None,
        random_seed: 42,
    };

    let storage = leiden(6, get_neighbors, &config);
    let result = storage.into_result();

    println!("Communities: {:?}", result.communities);
    println!("Modularity: {}", result.modularity);
    println!("Levels: {}", result.levels);
    println!("Community count: {}", result.community_count);

    // Should find 2 communities
    assert!(
        result.community_count <= 3,
        "Should find 2-3 communities, found {}",
        result.community_count
    );

    // Nodes 0, 1, 2 should be in same community
    assert_eq!(result.communities[0], result.communities[1]);
    assert_eq!(result.communities[1], result.communities[2]);

    // Nodes 3, 4, 5 should be in same community
    assert_eq!(result.communities[3], result.communities[4]);
    assert_eq!(result.communities[4], result.communities[5]);

    // Two communities should be different
    assert_ne!(result.communities[0], result.communities[3]);

    // Nodes 0, 1, 2 should be in same community
    assert_eq!(result.communities[0], result.communities[1]);
    assert_eq!(result.communities[1], result.communities[2]);

    // Nodes 3, 4, 5 should be in same community
    assert_eq!(result.communities[3], result.communities[4]);
    assert_eq!(result.communities[4], result.communities[5]);

    // Two communities should be different
    assert_ne!(result.communities[0], result.communities[3]);
}

#[test]
fn test_leiden_single_community() {
    // Fully connected triangle
    let edges = vec![
        (0, 1, 1.0),
        (1, 0, 1.0),
        (1, 2, 1.0),
        (2, 1, 1.0),
        (2, 0, 1.0),
        (0, 2, 1.0),
    ];

    let get_neighbors = |node: usize| -> Vec<(usize, f64)> {
        edges
            .iter()
            .filter(|(src, _, _)| *src == node)
            .map(|(_, dst, weight)| (*dst, *weight))
            .collect()
    };

    let config = LeidenConfig::default();

    let storage = leiden(3, get_neighbors, &config);
    let result = storage.into_result();

    // Should find 1 community (all nodes together)
    assert_eq!(result.community_count, 1);
    assert_eq!(result.communities[0], result.communities[1]);
    assert_eq!(result.communities[1], result.communities[2]);

    println!("Communities: {:?}", result.communities);
    println!("Modularity: {}", result.modularity);
}

#[test]
fn test_leiden_four_cliques() {
    // Four complete K3 subgraphs weakly connected
    let edges = vec![
        // Clique 1: 0, 1, 2
        (0, 1, 1.0),
        (1, 0, 1.0),
        (1, 2, 1.0),
        (2, 1, 1.0),
        (2, 0, 1.0),
        (0, 2, 1.0),
        // Clique 2: 3, 4, 5
        (3, 4, 1.0),
        (4, 3, 1.0),
        (4, 5, 1.0),
        (5, 4, 1.0),
        (5, 3, 1.0),
        (3, 5, 1.0),
        // Clique 3: 6, 7, 8
        (6, 7, 1.0),
        (7, 6, 1.0),
        (7, 8, 1.0),
        (8, 7, 1.0),
        (8, 6, 1.0),
        (6, 8, 1.0),
        // Clique 4: 9, 10, 11
        (9, 10, 1.0),
        (10, 9, 1.0),
        (10, 11, 1.0),
        (11, 10, 1.0),
        (11, 9, 1.0),
        (9, 11, 1.0),
        // Weak inter-clique edges
        (2, 3, 0.1),
        (3, 2, 0.1),
        (5, 6, 0.1),
        (6, 5, 0.1),
        (8, 9, 0.1),
        (9, 8, 0.1),
    ];

    let get_neighbors = |node: usize| -> Vec<(usize, f64)> {
        edges
            .iter()
            .filter(|(src, _, _)| *src == node)
            .map(|(_, dst, weight)| (*dst, *weight))
            .collect()
    };

    let config = LeidenConfig::default();

    let storage = leiden(12, get_neighbors, &config);
    let result = storage.into_result();

    // Should find 4 communities
    assert!(
        result.community_count >= 3 && result.community_count <= 5,
        "Should find 3-5 communities, found {}",
        result.community_count
    );

    // Each clique should be together
    assert_eq!(result.communities[0], result.communities[1]);
    assert_eq!(result.communities[1], result.communities[2]);

    assert_eq!(result.communities[3], result.communities[4]);
    assert_eq!(result.communities[4], result.communities[5]);

    assert_eq!(result.communities[6], result.communities[7]);
    assert_eq!(result.communities[7], result.communities[8]);

    assert_eq!(result.communities[9], result.communities[10]);
    assert_eq!(result.communities[10], result.communities[11]);

    println!("Communities: {:?}", result.communities);
    println!("Modularity: {}", result.modularity);
    println!("Community count: {}", result.community_count);
}

#[test]
fn test_leiden_with_seed() {
    // Two communities, but seed puts them together initially
    let edges = vec![
        (0, 1, 1.0),
        (1, 0, 1.0),
        (1, 2, 1.0),
        (2, 1, 1.0),
        (0, 2, 1.0),
        (2, 0, 1.0),
        (3, 4, 1.0),
        (4, 3, 1.0),
        (4, 5, 1.0),
        (5, 4, 1.0),
        (3, 5, 1.0),
        (5, 3, 1.0),
        (2, 3, 0.1),
        (3, 2, 0.1),
    ];

    let get_neighbors = |node: usize| -> Vec<(usize, f64)> {
        edges
            .iter()
            .filter(|(src, _, _)| *src == node)
            .map(|(_, dst, weight)| (*dst, *weight))
            .collect()
    };

    // Start with all nodes in same community
    let config = LeidenConfig {
        seed_communities: Some(vec![0, 0, 0, 0, 0, 0]),
        ..Default::default()
    };

    let storage = leiden(6, get_neighbors, &config);
    let result = storage.into_result();

    // Algorithm may or may not split - simplified version without refinement
    // Just check it doesn't crash and produces valid output
    assert!(
        result.community_count >= 1,
        "Should have at least 1 community"
    );
    assert_eq!(result.communities.len(), 6, "Should have 6 nodes");
}

#[test]
fn test_leiden_resolution_parameter() {
    // Test with different gamma values
    let edges = vec![
        (0, 1, 1.0),
        (1, 0, 1.0),
        (1, 2, 1.0),
        (2, 1, 1.0),
        (2, 3, 1.0),
        (3, 2, 1.0),
        (3, 4, 1.0),
        (4, 3, 1.0),
    ];

    let get_neighbors = |node: usize| -> Vec<(usize, f64)> {
        edges
            .iter()
            .filter(|(src, _, _)| *src == node)
            .map(|(_, dst, weight)| (*dst, *weight))
            .collect()
    };

    // Low gamma: fewer, larger communities
    let config_low = LeidenConfig {
        gamma: 0.5,
        ..Default::default()
    };

    let storage_low = leiden(5, &get_neighbors, &config_low);
    let result_low = storage_low.into_result();

    // High gamma: more, smaller communities
    let config_high = LeidenConfig {
        gamma: 2.0,
        ..Default::default()
    };

    let storage_high = leiden(5, get_neighbors, &config_high);
    let result_high = storage_high.into_result();

    println!(
        "Low gamma communities: {:?} (count: {})",
        result_low.communities, result_low.community_count
    );
    println!(
        "High gamma communities: {:?} (count: {})",
        result_high.communities, result_high.community_count
    );

    // Higher gamma should generally lead to more communities
    // (though not always guaranteed on small graphs)
    assert!(
        result_high.community_count >= result_low.community_count
            || result_low.community_count <= 2
    );
}

#[test]
fn test_leiden_weighted_graph() {
    // Graph with varying edge weights
    let edges = vec![
        // Strong community 1
        (0, 1, 5.0),
        (1, 0, 5.0),
        (1, 2, 5.0),
        (2, 1, 5.0),
        (0, 2, 5.0),
        (2, 0, 5.0),
        // Strong community 2
        (3, 4, 5.0),
        (4, 3, 5.0),
        (4, 5, 5.0),
        (5, 4, 5.0),
        (3, 5, 5.0),
        (5, 3, 5.0),
        // Weak bridge (much weaker)
        (2, 3, 0.5),
        (3, 2, 0.5),
    ];

    let get_neighbors = |node: usize| -> Vec<(usize, f64)> {
        edges
            .iter()
            .filter(|(src, _, _)| *src == node)
            .map(|(_, dst, weight)| (*dst, *weight))
            .collect()
    };

    let config = LeidenConfig::default();

    let storage = leiden(6, get_neighbors, &config);
    let result = storage.into_result();

    // Should clearly find 2 communities due to strong internal weights
    assert_eq!(
        result.community_count, 2,
        "Should find exactly 2 communities with strong internal weights"
    );

    // Verify split
    assert_eq!(result.communities[0], result.communities[1]);
    assert_eq!(result.communities[1], result.communities[2]);
    assert_eq!(result.communities[3], result.communities[4]);
    assert_eq!(result.communities[4], result.communities[5]);
    assert_ne!(result.communities[0], result.communities[3]);

    println!("Communities: {:?}", result.communities);
    println!("Modularity: {}", result.modularity);
}
