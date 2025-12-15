//! Path finding algorithm facades
//!
//! Finds paths between nodes with various optimality criteria.
//! Supports single-source, all-pairs, and multi-target queries.

pub mod dijkstra;
pub mod bfs;
pub mod dfs;
pub mod astar;
pub mod bellman_ford;
pub mod delta_stepping;
pub mod yens;
pub mod all_shortest_paths;
pub mod spanning_tree;

// Re-export for easy access
pub use dijkstra::{DijkstraBuilder, DijkstraStats};
pub use bfs::{BfsBuilder, BfsStats};
pub use dfs::{DfsBuilder, DfsStats};
pub use astar::{AStarBuilder, AStarStats, Heuristic};
pub use bellman_ford::{BellmanFordBuilder, BellmanFordStats};
pub use delta_stepping::{DeltaSteppingBuilder, DeltaSteppingStats};
pub use yens::{YensBuilder, YensStats};
pub use all_shortest_paths::{AllShortestPathsBuilder, AllShortestPathsRow, AllShortestPathsStats};
pub use spanning_tree::{SpanningTreeBuilder, SpanningTreeRow, SpanningTreeStats};

