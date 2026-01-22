// Concurrency package - Java GDS compatible concurrency primitives
//
// This package provides the foundational types for parallel graph algorithm execution.
// It mirrors the Java GDS concurrency package while leveraging Rust's safety guarantees.

/// Default concurrency / pool size for the Open GDS (community) profile.
///
/// Keep this value as the single source of truth for "open" defaults.
pub mod atomics;
pub mod parallel_util;
pub mod pool;
pub mod validator;
pub mod virtual_threads;

mod batch_size;
mod concurrency_level;
mod rayon_pool;
mod termination;

pub use batch_size::*;
pub use concurrency_level::*;
pub use termination::*;

pub use atomics::*;
pub use parallel_util::*;
pub use pool::*;
pub use validator::*;
pub use virtual_threads::*;

// `rayon_pool` is intentionally kept private to avoid leaking rayon-specific
// implementation details. Do not `pub use` its items.
// Re-export only the small API surface we need from the private `rayon_pool`.
pub(crate) use rayon_pool::install_with_concurrency;
