//! A* Pathfinding Algorithm
//!
//! **Translation Source**: `org.neo4j.gds.paths.astar.AStar`
//!
//! This module provides the A* pathfinding algorithm with Haversine heuristic for geographical pathfinding.

pub mod computation;
#[cfg(test)]
pub mod integration_tests;
pub mod spec;
pub mod storage;

// Re-export main types
pub use computation::AStarComputationRuntime;
pub use spec::{ASTARAlgorithmSpec, AStarConfig, AStarResult};
pub use storage::AStarStorageRuntime;
