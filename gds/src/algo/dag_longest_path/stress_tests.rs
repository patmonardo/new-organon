#[cfg(test)]
mod stress {
    use crate::algo::dag_longest_path::DagLongestPathComputationRuntime;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    // Compute longest path distances serially using DP (topological order by node id)
    fn serial_longest_paths(node_count: usize, edges: &Vec<Vec<(i64, f64)>>) -> Vec<f64> {
        let mut dist = vec![f64::NEG_INFINITY; node_count];
        for i in 0..node_count {
            if dist[i] == f64::NEG_INFINITY {
                // treat as new source
                dist[i] = 0.0;
            }
            for &(t, w) in &edges[i] {
                let tidx = t as usize;
                let candidate = dist[i] + w;
                if candidate > dist[tidx] {
                    dist[tidx] = candidate;
                }
            }
        }
        dist
    }

    #[test]
    fn stress_random_dags() {
        let mut rng = StdRng::seed_from_u64(42);
        // moderate sizes to exercise concurrency
        for n in 2..20usize {
            for _iter in 0..50 {
                // build DAG with edges only i -> j for j > i
                let mut edges: Vec<Vec<(i64, f64)>> = vec![Vec::new(); n];
                for i in 0..n {
                    for j in (i + 1)..n {
                        if rng.gen_bool(0.25) {
                            let w: f64 = rng.gen_range(1.0..5.0);
                            edges[i].push((j as i64, w));
                        }
                    }
                }

                let expected = serial_longest_paths(n, &edges);

                let get_neighbors = {
                    let edges_clone = edges.clone();
                    move |node: i64| edges_clone[node as usize].clone()
                };

                let mut runtime = DagLongestPathComputationRuntime::new(n);
                let result = runtime.compute(n, get_neighbors);

                // For each node, compare distances
                for row in result.paths.iter() {
                    let tgt = row.target_node as usize;
                    let observed = row.total_cost;
                    let expect = expected[tgt];
                    // Both could be -inf for unreachable nodes; compare with tolerance
                    if expect.is_infinite() && expect.is_sign_negative() {
                        assert!(observed.is_infinite() && observed.is_sign_negative());
                    } else {
                        let diff = (observed - expect).abs();
                        if diff >= 1e-8 {
                            eprintln!("Mismatch detected for n={}:", n);
                            eprintln!("Edges: {:?}", edges);
                            eprintln!("Expected distances: {:?}", expected);
                            eprintln!("Observed row: {:?}", row);
                            panic!(
                                "observed {} != expected {} for node {}",
                                observed, expect, tgt
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn repeat_specific_small_graph() {
        // Graph: 0 -> 2 with random weight
        for _ in 0..200 {
            let edges: Vec<Vec<(i64, f64)>> = vec![vec![(2, 2.5672564545505523)], vec![], vec![]];
            let expected = serial_longest_paths(3, &edges);

            let get_neighbors = {
                let edges_clone = edges.clone();
                move |node: i64| edges_clone[node as usize].clone()
            };

            let mut runtime = DagLongestPathComputationRuntime::new(3);
            let result = runtime.compute(3, get_neighbors);

            let path_to_2 = result.paths.iter().find(|p| p.target_node == 2).unwrap();
            assert!((path_to_2.total_cost - expected[2]).abs() < 1e-8);
        }
    }
}
