//! Triangle intersection utilities (Java GDS parity).
//!
//! This is a small, single-threaded (not thread-safe) helper to enumerate triangles
//! by intersecting sorted adjacency lists.

pub mod spec;
pub mod graph_intersect;

pub use graph_intersect::{AdjacencyProvider, GraphIntersect, IntersectionConsumer, RelationshipIntersect};
pub use spec::RelationshipIntersectConfig;
