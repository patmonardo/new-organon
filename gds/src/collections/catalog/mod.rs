//! Collections Catalog (Disk-First)
//!
//! Split into types + catalog logic modules.

pub mod catalog;
pub mod disk;
pub mod polars_io;
pub mod samples;
pub mod schema;
pub mod types;

pub use catalog::*;
pub use disk::*;
pub use polars_io::*;
pub use samples::*;
pub use schema::*;
pub use types::*;
