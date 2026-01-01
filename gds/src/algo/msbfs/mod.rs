//! Multi-Source Breadth-First Search (MSBFS)
//!
//! Translation target: Neo4j GDS `org.neo4j.gds.msbfs.*`.
//!
//! The primary implementation exposed here is **Aggregated Neighbor Processing (ANP)**,
//! which provides the key invariant used by Neo4j GDS centrality algorithms:
//! a `(nodeId, depth)` pair is processed at most once per traversal batch.

pub mod anp;

pub use anp::{AggregatedNeighborProcessingMsBfs, OMEGA};
