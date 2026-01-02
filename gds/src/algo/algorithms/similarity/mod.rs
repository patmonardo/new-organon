//! Similarity Algorithm Infrastructure
//!
//! **Translation Source**: `org.neo4j.gds.algorithms.similarity` package
//!
//! Provides summary/histogram helpers for similarity algorithms that operate on
//! relationship similarity scores (e.g., Node Similarity, KNN, TopK graphs).

mod summary_builder;

pub use summary_builder::*;
