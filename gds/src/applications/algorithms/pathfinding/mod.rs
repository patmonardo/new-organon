//! Pathfinding algorithm dispatch handlers.
//!
//! Fresh restart: this module intentionally avoids the previous experimental layout.
//! We keep a Java-like split:
//! - shared request parsing + response helpers
//! - per-algorithm modules (BFS, DFS)
//! - per-mode entrypoints under each algorithm
//! - explicit step objects for mutate/write.

pub mod all_shortest_paths;
pub mod astar;
pub mod bellman_ford;
pub mod bfs;
pub mod dag_longest_path;
pub mod delta_stepping;
pub mod dfs;
pub mod dijkstra;
pub mod kspanningtree;
pub mod random_walk;
pub mod shared;
pub mod spanning_tree;
pub mod steiner_tree;
pub mod topological_sort;
pub mod yens;

pub use all_shortest_paths::handle_all_shortest_paths;
pub use astar::handle_astar;
pub use bellman_ford::handle_bellman_ford;
pub use bfs::handle_bfs;
pub use dag_longest_path::handle_dag_longest_path;
pub use delta_stepping::handle_delta_stepping;
pub use dfs::handle_dfs;
pub use dijkstra::handle_dijkstra;
pub use kspanningtree::handle_kspanningtree;
pub use random_walk::handle_random_walk;
pub use shared::*;
pub use spanning_tree::handle_spanning_tree;
pub use steiner_tree::handle_steiner_tree;
pub use topological_sort::handle_topological_sort;
pub use yens::handle_yens;
