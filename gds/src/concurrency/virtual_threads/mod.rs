//! Virtual threads abstraction for graph algorithms.
//!
//! This module provides a high-level abstraction over Rayon's work-stealing parallelism
//! that's specifically designed for iterative graph algorithms like Pregel.
//!
//! ## Design Philosophy
//!
//! Java GDS has complex thread management with ExecutorService, WorkerPools, ThreadFactories,
//! and manual synchronization. This is necessary in Java because:
//! - Thread creation is expensive
//! - Manual lifecycle management is required
//! - Synchronization primitives are heavy
//!
//! Rust + Rayon gives us:
//! - **Work-stealing**: Automatic load balancing across CPU cores
//! - **Scoped work**: Structured parallelism with clear join points
//! - **Type safety**: Rust helps avoid many accidental data races, but correctness
//!   still depends on using thread-safe data structures and synchronization
//!
//! ## What We Provide
//!
//! Instead of mimicking Java's complexity, we provide clean abstractions for:
//! - **Executor**: High-level parallel execution with termination support
//! - **Scope**: Synchronization boundary (supersteps in Pregel)
//! - **WorkerContext**: Per-thread state management
//!
//! ## Example: Pregel-Style Iteration
//!
//! ```no_run
//! use gds::concurrency::virtual_threads::Executor;
//! use gds::concurrency::{TerminatedException, TerminationFlag};
//!
//! fn compute_vertex(_node_id: usize) {}
//! fn converged() -> bool { false }
//!
//! fn run() -> Result<(), TerminatedException> {
//!     let node_count = 1_000usize;
//!     let max_iterations = 10usize;
//!     let executor = Executor::default();
//!     let termination = TerminationFlag::running_true();
//!
//!     for _iteration in 0..max_iterations {
//!         executor.scope(&termination, |scope| {
//!             scope.spawn_many(node_count, |node_id| compute_vertex(node_id));
//!         })?;
//!
//!         if converged() {
//!             break;
//!         }
//!     }
//!     Ok(())
//! }
//! ```

mod executor;
mod run_with_concurrency;
mod scope;
mod worker_context;

pub use executor::*;
pub use run_with_concurrency::*;
pub use scope::*;
pub use worker_context::*;
