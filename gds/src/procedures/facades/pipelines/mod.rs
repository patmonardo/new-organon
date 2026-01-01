//! Pipelines Procedure Facade (API translation)
//!
//! This module is a mostly-direct translation of Neo4j GDS's Java
//! `pipelines-facade-api` surface into Rust.
//!
//! The goal here is *not* idiomatic Rust yet — it's to make the moving parts
//! visible so we can later simplify and integrate against the pipeline executor.

pub mod facade;
pub mod types;

pub use facade::*;
pub use types::*;
