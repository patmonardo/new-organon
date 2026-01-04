//! Task hierarchy and progress tracking for graph algorithms.
//!
//! This module provides a comprehensive task system for tracking
//! long-running graph computations:
//!
//! - **Task Types**: Leaf, intermediate, and iterative tasks
//! - **Progress Tracking**: Volume-based progress with batched updates
//! - **Visitor Pattern**: Flexible traversal of task hierarchies
//! - **Status Management**: Lifecycle tracking (pending → running → finished)
//!
//! # Architecture
//!
//! The task system follows a composite pattern with visitor support:
//!
//! ```text
//! Task (base)
//! ├── LeafTask (terminal nodes with progress)
//! ├── Task (intermediate nodes with subtasks)
//! └── IterativeTask (repeating subtasks)
//! ```

pub mod depth_aware_task_visitor;
pub mod iterative_task;
pub mod leaf_task;
pub mod log_level;
pub mod progress;
pub mod progress_tracker;
pub mod status;
pub mod task;
pub mod task_progress_logger;
pub mod task_progress_tracker;
pub mod task_traversal;
pub mod task_visitor;
#[allow(clippy::module_inception)]
pub mod tasks;

pub use depth_aware_task_visitor::DepthAwareTaskVisitor;
pub use iterative_task::{IterativeTask, IterativeTaskMode};
pub use leaf_task::LeafTask;
pub use log_level::LogLevel;
pub use progress::Progress;
pub use progress_tracker::{NoopProgressTracker, ProgressTracker, NULL_TRACKER};
pub use status::Status;
pub use task::{Task, UNKNOWN_VOLUME};
pub use task_progress_logger::TaskProgressLogger;
pub use task_progress_tracker::TaskProgressTracker;
pub use task_traversal::TaskTraversal;
pub use task_visitor::{TaskLike, TaskVisitor};
pub use tasks::Tasks;
