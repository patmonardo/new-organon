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
pub mod kspanningtree;
pub mod steiner_tree;
pub mod prize_collecting_steiner_tree;
pub mod topological_sort;
pub mod dag_longest_path;
pub mod random_walk;

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
pub use kspanningtree::{KSpanningTreeBuilder, KSpanningTreeRow, KSpanningTreeStats};
pub use steiner_tree::{SteinerTreeBuilder, SteinerTreeAlgorithm};
pub use prize_collecting_steiner_tree::{PCSTreeBuilder, PCSTreeAlgorithm};
pub use topological_sort::{TopologicalSortBuilder, TopologicalSortRow, TopologicalSortStats};
pub use dag_longest_path::{DagLongestPathBuilder, DagLongestPathRow, DagLongestPathStats};
pub use random_walk::{RandomWalkBuilder, RandomWalkRow, RandomWalkStats};

