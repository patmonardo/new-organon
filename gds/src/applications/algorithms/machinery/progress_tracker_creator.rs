//! ProgressTrackerCreator (Java parity).
//!
//! Mirrors `ProgressTrackerCreator` in Java. The key responsibility is creating a
//! request-scoped `ProgressTracker` (typically `TaskProgressTracker`) from:
//! - the algorithm configuration (optional; caller decides what to encode)
//! - a `Task` tree
//! - request-scoped dependencies (job id + task registry)

use crate::concurrency::{Concurrency, TerminationFlag};
use crate::core::utils::progress::{ProgressTracker, Task, TaskProgressTracker};

use super::RequestScopedDependencies;

#[derive(Clone)]
pub struct ProgressTrackerCreator {
    request_scoped_dependencies: RequestScopedDependencies,
}

/// Back-compat alias expected by some call sites.
///
/// Java naming often uses `DefaultProgressTrackerCreator`.
pub type DefaultProgressTrackerCreator = ProgressTrackerCreator;

impl ProgressTrackerCreator {
    pub fn new(request_scoped_dependencies: RequestScopedDependencies) -> Self {
        Self {
            request_scoped_dependencies,
        }
    }

    pub fn create_progress_tracker(
        &self,
        concurrency: Concurrency,
        task: Task,
    ) -> TaskProgressTracker {
        TaskProgressTracker::with_registry(
            task,
            concurrency,
            self.request_scoped_dependencies.job_id.clone(),
            self.request_scoped_dependencies
                .task_registry_factory
                .as_ref(),
        )
    }

    pub fn request_scoped_dependencies(&self) -> &RequestScopedDependencies {
        &self.request_scoped_dependencies
    }

    pub fn termination_flag(&self) -> &TerminationFlag {
        &self.request_scoped_dependencies.termination_flag
    }
}

// Convenience: accept any tracker impl.
pub type BoxedProgressTracker = Box<dyn ProgressTracker + Send>;
