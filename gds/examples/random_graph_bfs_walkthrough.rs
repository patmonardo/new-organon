//! Random graph + BFS walkthrough.
//!
//! Purpose:
//! - Demonstrate `DefaultGraphStore::random()` deterministically (seeded)
//! - Run a real procedure (`BfsBuilder`) on that graph
//! - Print results in a "learning-first" format
//!
//! Run with:
//!   cargo run -p gds --example random_graph_bfs_walkthrough

mod enabled {
    use gds::procedures::pathfinding::BfsBuilder;
    use gds::procedures::pathfinding::DfsBuilder;
    use gds::procedures::pathfinding::DijkstraBuilder;
    use gds::projection::orientation::Orientation;
    use gds::types::prelude::{DefaultGraphStore, GraphStore, RandomGraphConfig};
    use gds::types::random::RandomRelationshipConfig;
    use std::collections::HashMap;

    pub fn main() {
        if let Err(err) = run() {
            eprintln!("random_graph_bfs_walkthrough failed: {err}");
            std::process::exit(1);
        }
    }

    fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("=== Random graph + BFS walkthrough ===\n");

        if cfg!(debug_assertions) {
            println!("(Note) Progress logs print to stderr in debug builds.\n");
        }

        // 1) Generate a deterministic random graph.
        //
        // The generator is currently Erdos–Rényi style per relationship type:
        // for each possible pair (u,v) it flips a coin with probability `p`.
        let config = RandomGraphConfig {
            graph_name: "bfs-demo".into(),
            database_name: "in-memory".into(),
            node_count: 12,
            node_labels: vec!["Person".into(), "Movie".into()],
            relationships: vec![RandomRelationshipConfig::new("KNOWS", 0.35)],
            directed: false,
            inverse_indexed: false,
            seed: Some(7),
        };

        let store = DefaultGraphStore::random(&config)?;

        println!(
            "Graph created: name='{}' nodes={} relationships={} labels={:?} rel_types={:?}",
            config.graph_name,
            store.node_count(),
            store.relationship_count(),
            store.node_labels(),
            store.relationship_types(),
        );

        // 2) Inspect a couple of properties the generator adds.
        // - node property: `random_score` (Double)
        // - graph property: `edge_density` (Double)
        println!(
            "Property keys: node={:?} graph={:?}",
            store.node_property_keys(),
            store.graph_property_keys()
        );

        let density = store
            .graph_property_values("edge_density")?
            .double_values()
            .next()
            .unwrap_or(0.0);
        println!("edge_density={density:.4}");

        let random_score = store.node_property_values("random_score")?;
        let sample_scores: Vec<f64> = (0..3u64)
            .filter_map(|id| random_score.double_value(id).ok())
            .collect();
        println!("random_score sample (nodes 0..2) = {:?}\n", sample_scores);

        // 3) Run a procedure: BFS.
        //
        // We keep this small + readable:
        // - source: 0
        // - max_depth: 3 (so output is bounded)
        // - track_paths: true (so we can *see* the traversal paths)
        let store = std::sync::Arc::new(store);

        let stats = BfsBuilder::new(std::sync::Arc::clone(&store))
            .source(0)
            .max_depth(3)
            .track_paths(true)
            .stats()?;

        println!(
            "BFS stats: visited={} max_depth_reached={} targets_found={} all_targets_reached={} avg_branching_factor={:.3} execution_time_ms={}",
            stats.nodes_visited,
            stats.max_depth_reached,
            stats.targets_found,
            stats.all_targets_reached,
            stats.avg_branching_factor,
            stats.execution_time_ms,
        );

        let results: Vec<_> = BfsBuilder::new(std::sync::Arc::clone(&store))
            .source(0)
            .max_depth(3)
            .track_paths(true)
            .stream()?
            .take(10)
            .collect();

        println!("\nFirst 10 BFS results (target, distance, path):");
        for r in &results {
            println!("- target={} dist={} path={:?}", r.target, r.cost, r.path);
        }

        // 3b) Run a second procedure: DFS.
        //
        // This shares the same underlying progress tracker instrumentation as BFS now.
        let dfs_stats = DfsBuilder::new(std::sync::Arc::clone(&store))
            .source(0)
            .max_depth(3)
            .track_paths(true)
            .stats()?;

        println!(
            "\nDFS stats: visited={} max_depth_reached={} targets_found={} all_targets_reached={} backtrack_operations={} avg_branch_depth={:.3} execution_time_ms={}",
            dfs_stats.nodes_visited,
            dfs_stats.max_depth_reached,
            dfs_stats.targets_found,
            dfs_stats.all_targets_reached,
            dfs_stats.backtrack_operations,
            dfs_stats.avg_branch_depth,
            dfs_stats.execution_time_ms,
        );

        let dfs_results: Vec<_> = DfsBuilder::new(std::sync::Arc::clone(&store))
            .source(0)
            .max_depth(3)
            .track_paths(true)
            .stream()?
            .take(10)
            .collect();

        println!("\nFirst 10 DFS results (target, depth, path):");
        for r in &dfs_results {
            println!("- target={} depth={} path={:?}", r.target, r.cost, r.path);
        }

        // 4) Run a weighted procedure: Dijkstra.
        //
        // The random graph generator now populates relationship property `weight` for each
        // relationship type, so Dijkstra costs should reflect edge weights (not hop count).
        let dijkstra_target = 9u64;
        let mut dijkstra_paths = DijkstraBuilder::new(std::sync::Arc::clone(&store))
            .source(0)
            .target(dijkstra_target)
            .weight_property("weight")
            .direction("outgoing")
            .stream()?
            .collect::<Vec<_>>();

        dijkstra_paths.sort_by(|a, b| a.cost.total_cmp(&b.cost));
        if let Some(best) = dijkstra_paths.first() {
            println!(
                "\nDijkstra best path to target {}: cost={:.4} path={:?}",
                dijkstra_target, best.cost, best.path
            );

            // --- Exposition: how that cost was computed ---
            // Dijkstra consumes relationship weights via the Graph view's selected relationship property.
            // Here we build the same kind of view (selectors => "weight") and then look up the weight for
            // each hop in the returned path.
            let rel_types = store.relationship_types();
            let selectors: HashMap<_, _> = rel_types
                .iter()
                .map(|t| (t.clone(), "weight".to_string()))
                .collect();

            let graph_view = store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    Orientation::Natural,
                )
                .map_err(|e| format!("failed to build graph view for weight inspection: {e}"))?;

            let mut sum = 0.0f64;
            if best.path.len() >= 2 {
                println!("Edge weights along path:");
            }
            for window in best.path.windows(2) {
                let u = window[0] as i64;
                let v = window[1] as i64;

                // Find the (u -> v) weight in the outgoing weighted relationship stream.
                // This matches how weighted algorithms consume relationship properties.
                let mut w_opt: Option<f64> = None;
                for cursor in graph_view.stream_relationships_weighted(u, 1.0) {
                    if cursor.target_id() == v {
                        w_opt = Some(cursor.weight());
                        break;
                    }
                }

                match w_opt {
                    Some(w) => {
                        sum += w;
                        println!("- {} -> {}  weight={:.4}  running_sum={:.4}", u, v, w, sum);
                    }
                    None => {
                        println!("- {} -> {}  weight=(missing?)", u, v);
                    }
                }
            }

            if best.path.len() >= 2 {
                println!(
                    "Total of edge weights: {:.4} (Dijkstra reported: {:.4})",
                    sum, best.cost
                );
            }

            // 5) Final exposition: BFS vs Dijkstra (same target, different objective).
            //
            // - BFS minimizes hop-count (treats every edge as cost=1).
            // - Dijkstra minimizes total weight (sum of edge weights along the path).
            //
            // These can disagree: a longer path in hops can be cheaper in total weight.
            let bfs_to_target = BfsBuilder::new(std::sync::Arc::clone(&store))
                .source(0)
                .target(dijkstra_target)
                .track_paths(true)
                .stream()?
                .find(|r| r.target == dijkstra_target);

            println!("\nBFS vs Dijkstra (target={}):", dijkstra_target);
            match bfs_to_target.as_ref() {
                Some(r) => {
                    let hops = r.path.len().saturating_sub(1) as u64;
                    println!("- BFS (unweighted): hops={} path={:?}", hops, r.path);
                }
                None => println!("- BFS (unweighted): target not reached"),
            }

            let dijkstra_hops = best.path.len().saturating_sub(1) as u64;
            println!(
                "- Dijkstra (weighted): cost={:.4} hops={} path={:?}",
                best.cost, dijkstra_hops, best.path
            );
            if let Some(r) = bfs_to_target.as_ref() {
                let bfs_hops = r.path.len().saturating_sub(1) as u64;
                if dijkstra_hops > bfs_hops {
                    println!(
                        "  Note: Dijkstra used more hops ({} > {}) but won on weight.",
                        dijkstra_hops, bfs_hops
                    );
                } else if dijkstra_hops < bfs_hops {
                    println!(
                        "  Note: Dijkstra used fewer hops ({} < {}) and also minimized weight.",
                        dijkstra_hops, bfs_hops
                    );
                } else {
                    println!(
                        "  Note: Both used the same hop-count ({}), but objectives differ.",
                        dijkstra_hops
                    );
                }
            }
        } else {
            println!(
                "\nDijkstra: no path found to target {} (graph may be disconnected at this seed/p)",
                dijkstra_target
            );
        }

        Ok(())
    }
}

fn main() {
    enabled::main();
}
