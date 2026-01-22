//! Rust GDS - Graph Data Science library
//!
//! A modular graph data structure and algorithms library.
#![allow(ambiguous_wide_pointer_comparisons)]
#![allow(clippy::all)]
// Real algorithm implementations are in procedures/ module
// (Previously had speculative stubs here - all moved to procedures/)

pub mod algo;
pub mod collections;
pub mod concurrency;
pub mod config;
pub mod core;
pub mod errors;
pub mod form;
pub mod mem;
pub mod ml;
pub mod pregel;
pub mod prints;
pub mod procedures;
pub mod projection;
pub mod types;
pub mod values;

pub mod applications;

// pub use core::*;
// pub use ml::*;
// pub use procedures::*;
// pub use projection::*;
pub use prints::*;
pub use types::*;
pub use values::*;

// User-facing facade entrypoint (live-by-default).
pub use procedures::GraphFacade;

// Re-export procedure macros for procedures module
#[cfg(feature = "procedures")]
#[allow(unused_imports)]
pub use projection::codegen::algorithm::*;
