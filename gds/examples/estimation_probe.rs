//! Estimation probe (memory / "estimate" support).
//!
//! Purpose:
//! - Build a small in-memory random graph
//! - Call `estimate_memory()` on a few representative algorithms
//! - Print min/max bytes + human-readable sizes
//!
//! This is intentionally *non-invasive*: no refactors, just visibility.
//!
//! Run with:
//!   cargo run -p gds --example estimation_probe

mod enabled {
    use gds::mem::Estimate;
    use gds::mem::MemoryRange;
    use gds::procedures::centrality::DegreeCentralityFacade;
    use gds::procedures::centrality::PageRankFacade;
    use gds::procedures::community::k1coloring::K1ColoringFacade;
    use gds::procedures::community::{LouvainFacade, SccFacade, WccFacade};
    use gds::procedures::pathfinding::{BfsBuilder, DfsBuilder};
    use gds::types::prelude::{DefaultGraphStore, GraphStore, RandomGraphConfig};
    use gds::types::random::RandomRelationshipConfig;

    pub fn main() {
        if let Err(err) = run() {
            eprintln!("estimation_probe failed: {err}");
            std::process::exit(1);
        }
    }

    fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("=== Estimation probe ===\n");

        let config = RandomGraphConfig {
            graph_name: "estimation-demo".into(),
            database_name: "in-memory".into(),
            node_count: 5_000,
            node_labels: vec!["Node".into()],
            relationships: vec![RandomRelationshipConfig::new("REL", 0.002)],
            directed: false,
            inverse_indexed: false,
            seed: Some(42),
        };

        let store = DefaultGraphStore::random(&config)?;
        println!(
            "Graph created: nodes={} relationships={} rel_types={:?}",
            store.node_count(),
            store.relationship_count(),
            store.relationship_types(),
        );

        let store = std::sync::Arc::new(store);

        println!("\n-- Procedure-level estimate_memory() coverage snapshot --");

        // ---- BFS ----
        let bfs = BfsBuilder::new(std::sync::Arc::clone(&store))
            .source(0)
            .max_depth(3)
            .track_paths(true);
        let bfs_mem = bfs.estimate_memory();
        print_range("BFS", bfs_mem);

        // ---- DFS ----
        let dfs = DfsBuilder::new(std::sync::Arc::clone(&store))
            .source(0)
            .max_depth(3)
            .track_paths(true);
        let dfs_mem = dfs.estimate_memory();
        print_range("DFS", dfs_mem);

        // ---- PageRank ----
        let pr = PageRankFacade::new(std::sync::Arc::clone(&store))
            .iterations(20)
            .damping_factor(0.85)
            .tolerance(1e-4);
        let pr_mem = pr.estimate_memory();
        print_range("PageRank", pr_mem);

        // ---- Degree centrality ----
        let degree = DegreeCentralityFacade::new(std::sync::Arc::clone(&store));
        let degree_mem = degree.estimate_memory();
        print_range("DegreeCentrality", degree_mem);

        // ---- WCC ----
        let wcc = WccFacade::new(std::sync::Arc::clone(&store)).concurrency(1);
        let wcc_mem = wcc.estimate_memory();
        print_range("WCC", wcc_mem);

        // ---- Louvain ----
        let louvain = LouvainFacade::new(std::sync::Arc::clone(&store)).concurrency(1);
        let louvain_mem = louvain.estimate_memory();
        print_range("Louvain", louvain_mem);

        // ---- SCC (placeholder today) ----
        let scc = SccFacade::new(std::sync::Arc::clone(&store)).concurrency(1);
        let scc_mem = scc.estimate_memory()?;
        print_range("SCC (placeholder today)", scc_mem);

        // ---- K1Coloring (currently placeholder) ----
        let k1 = K1ColoringFacade::new(std::sync::Arc::clone(&store)).concurrency(1);
        let k1_mem = k1.estimate_memory()?;
        print_range("K1Coloring (placeholder today)", k1_mem);

        println!("\nNotes:");
        println!(
            "- These estimates are heuristic and not yet unified under a single executor mode."
        );
        println!("- Some procedures return placeholders (K1Coloring currently does).");
        println!("- Memory guard enforcement is not wired to these estimates yet.");

        Ok(())
    }

    fn print_range(label: &str, range: MemoryRange) {
        let min = range.min();
        let max = range.max();
        let is_placeholder_1mb = min == 0 && max == 1024 * 1024;
        let tag = if is_placeholder_1mb {
            " [placeholder?]"
        } else {
            ""
        };
        println!(
            "- {:<28}{} min={} ({})  max={} ({})",
            label,
            tag,
            min,
            Estimate::human_readable(min),
            max,
            Estimate::human_readable(max),
        );
    }
}

fn main() {
    enabled::main();
}
