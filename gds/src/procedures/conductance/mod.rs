//! Conductance: community quality metric based on edge boundaries
//!
//! Evaluates a division of nodes into communities based on the proportion
//! of relationships that cross community boundaries.
//!
//! Conductance = external_edges / (external_edges + internal_edges)
//!
//! Lower conductance indicates better community separation.

pub mod spec;
pub mod storage;
pub mod computation;

#[cfg(test)]
mod integration_tests;
