//! DAG Longest Path
//!
//! **Translation Source**: `org.neo4j.gds.dag.longestPath`
//!
//! Finds longest paths in a directed acyclic graph (DAG) using topological
//! ordering and dynamic programming.

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
mod integration_tests;
#[cfg(test)]
mod stress_tests;
