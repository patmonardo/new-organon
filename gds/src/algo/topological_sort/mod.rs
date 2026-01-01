//! Topological Sort
//!
//! **Translation Source**: `org.neo4j.gds.dag.topologicalsort`
//!
//! Orders nodes in a directed acyclic graph (DAG) such that for every edge (u, v),
//! u appears before v in the ordering. Optionally computes longest path distances.

pub mod computation;
pub mod spec;
pub mod storage;

#[cfg(test)]
mod integration_tests;
