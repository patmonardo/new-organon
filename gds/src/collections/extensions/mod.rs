//! Collections Extensions: Advanced Features for Collections First Approach
//!
//! This module provides extension implementations for Collections,
//! repackaging GDS utilities as Collections Extensions for the Collections First approach.

pub mod compression;
pub mod distributed;
pub mod encryption;
pub mod gpu;
pub mod memory_estimation;
pub mod metrics;
pub mod ml;
pub mod ndarray;
pub mod paging;
pub mod partitioning;
pub mod queue;
pub mod random;
pub mod stack;

pub use compression::*;
pub use memory_estimation::*;
pub use metrics::*;
pub use paging::*;
pub use partitioning::*;
pub use queue::*;
pub use random::*;
pub use stack::*;
