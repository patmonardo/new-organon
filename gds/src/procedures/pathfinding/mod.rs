//! Path finding algorithm facades
//!
//! Finds paths between nodes with various optimality criteria.
//! Supports single-source, all-pairs, and multi-target queries.

pub mod all_shortest_paths;
pub mod astar;
pub mod bellman_ford;
pub mod bfs;
pub mod dag_longest_path;
pub mod delta_stepping;
pub mod dfs;
pub mod dijkstra;
pub mod kspanningtree;
pub mod prize_collecting_steiner_tree;
pub mod random_walk;
pub mod spanning_tree;
pub mod steiner_tree;
pub mod topological_sort;
pub mod yens;

mod path_result_mapping;

pub(crate) use path_result_mapping::core_to_procedure_path_result;

// Re-export for easy access
pub use all_shortest_paths::{AllShortestPathsBuilder, AllShortestPathsRow, AllShortestPathsStats};
pub use astar::{AStarBuilder, AStarStats, Heuristic};
pub use bellman_ford::{BellmanFordBuilder, BellmanFordStats};
pub use bfs::{BfsBuilder, BfsStats};
pub use dag_longest_path::{DagLongestPathBuilder, DagLongestPathRow, DagLongestPathStats};
pub use delta_stepping::{DeltaSteppingBuilder, DeltaSteppingStats};
pub use dfs::{DfsBuilder, DfsStats};
pub use dijkstra::{DijkstraBuilder, DijkstraStats};
pub use kspanningtree::{KSpanningTreeBuilder, KSpanningTreeRow, KSpanningTreeStats};
pub use prize_collecting_steiner_tree::{PCSTreeBuilder, PCSTreeRow, PCSTreeStats};
pub use random_walk::{RandomWalkBuilder, RandomWalkRow, RandomWalkStats};
pub use spanning_tree::{SpanningTreeBuilder, SpanningTreeRow, SpanningTreeStats};
pub use steiner_tree::{SteinerTreeBuilder, SteinerTreeRow, SteinerTreeStats};
pub use topological_sort::{TopologicalSortBuilder, TopologicalSortRow, TopologicalSortStats};
pub use yens::{YensBuilder, YensStats};
