//! Progress tracking infrastructure for long-running graph algorithms.
//!
//! This module provides comprehensive progress tracking for data science pipelines:
//! - Task management and registration
//! - Progress logging with batched updates
//! - Task stores and registries
//! - User and job tracking
//!
//! # Architecture
//!
//! The progress tracking system consists of several layers:
//!
//! 1. **Tasks**: Define what work needs to be done (`Task`)
//! 2. **Storage**: Persist and query running tasks (`TaskStore`)
//! 3. **Registry**: Manage tasks for user sessions (`TaskRegistry`)
//! 4. **Logging**: Report progress efficiently (`ProgressLogger`)
//!
//! # Examples
//!
//! ```rust,ignore
//! use gds::core::utils::progress::*;
//!
//! // Create a task store
//! let store = PerDatabaseTaskStore::new();
//!
//! // Register a task
//! let job_id = JobId::new();
//! let task = Task::new("Graph Algorithm".to_string(), 1000);
//! store.store("user".to_string(), job_id.clone(), task);
//!
//! // Query tasks
//! let user_tasks = store.query_by_username("user");
//! ```

pub mod batching_progress_logger;
pub mod empty_task_store;
pub mod job_id;
pub mod observable_task_store;
pub mod per_database_task_store;
pub mod progress_logger;
pub mod task;
pub mod task_registry;
pub mod task_registry_factory;
pub mod task_store;
pub mod task_store_holder;
pub mod task_store_listener;
pub mod task_store_provider;
pub mod task_store_service;
pub mod tasks;
pub mod user_task;

pub use batching_progress_logger::{BatchingProgressLogger, MAXIMUM_LOG_INTERVAL};
pub use empty_task_store::EmptyTaskStore;
pub use job_id::JobId;
pub use observable_task_store::ObservableTaskStore;
pub use per_database_task_store::PerDatabaseTaskStore;
pub use progress_logger::{MessageFactory, ProgressLogger, NO_MESSAGE};
pub use task::{Task, UNKNOWN_VOLUME};
pub use task_registry::TaskRegistry;
pub use task_registry_factory::{
    EmptyTaskRegistryFactory, LocalTaskRegistryFactory, TaskRegistryFactories, TaskRegistryFactory,
};
pub use task_store::TaskStore;
#[allow(deprecated)]
pub use task_store_holder::TaskStoreHolder;
pub use task_store_listener::TaskStoreListener;
pub use task_store_provider::{SimpleTaskStoreProvider, TaskStoreProvider, TaskStoreProviders};
pub use task_store_service::TaskStoreService;
pub use user_task::UserTask;

// ============================================================================
// Algorithm-facing ProgressTracker + Tasks
// ============================================================================

/// Algorithm-facing progress tracker.
///
/// This is the lightweight handle algorithms can carry to emit:
/// - begin/end subtask events
/// - incremental progress updates
///
/// Internally it is backed by a [`BatchingProgressLogger`], which batches updates
/// to keep overhead low.
#[derive(Clone)]
pub struct ProgressTracker {
    task_name: String,
    concurrency: usize,
    logger: std::sync::Arc<std::sync::Mutex<BatchingProgressLogger>>,
}

impl std::fmt::Debug for ProgressTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProgressTracker")
            .field("task_name", &self.task_name)
            .field("concurrency", &self.concurrency)
            .finish_non_exhaustive()
    }
}

impl ProgressTracker {
    /// Create a tracker for the given task, defaulting to concurrency = 1.
    pub fn new(task: Tasks) -> Self {
        Self::with_concurrency(task, 1)
    }

    /// Create a tracker for the given task with explicit concurrency.
    pub fn with_concurrency(task: Tasks, concurrency: usize) -> Self {
        let concurrency = concurrency.max(1);
        let (task_name, task_volume) = task.name_and_volume();
        let logger = BatchingProgressLogger::new(task_name.clone(), task_volume, concurrency);

        Self {
            task_name,
            concurrency,
            logger: std::sync::Arc::new(std::sync::Mutex::new(logger)),
        }
    }

    /// Begin a subtask with a known work volume.
    ///
    /// Mirrors Java `progressTracker.beginSubTask(volume)`.
    pub fn begin_subtask(&mut self, volume: usize) {
        let mut logger = self.logger.lock().unwrap();
        logger.reset(volume as i64);
        logger.log_start_default();
    }

    /// Begin a subtask when the total work is unknown.
    pub fn begin_subtask_unknown(&mut self) {
        self.begin_subtask(UNKNOWN_VOLUME);
    }

    /// Increment progress by `amount`.
    pub fn log_progress(&mut self, amount: usize) {
        let mut logger = self.logger.lock().unwrap();
        logger.log_progress_amount(amount as i64);
    }

    /// Finish the subtask successfully.
    pub fn end_subtask(&mut self) {
        let mut logger = self.logger.lock().unwrap();
        logger.log_finish_percentage();
        logger.log_finish_default();
        logger.release();
    }

    /// Finish the subtask with failure.
    pub fn end_subtask_with_failure(&mut self) {
        let mut logger = self.logger.lock().unwrap();
        logger.log_finish_with_failure_default();
        logger.release();
    }

    /// Human-friendly task name.
    pub fn task_name(&self) -> &str {
        &self.task_name
    }
}

/// Placeholder for Tasks - defines work to be done.
#[derive(Debug, Clone)]
pub enum Tasks {
    Leaf(String, usize),
    Task(String, Vec<Tasks>),
}

impl Tasks {
    pub fn leaf(name: &str, count: usize) -> Self {
        Tasks::Leaf(name.to_string(), count)
    }

    pub fn task(name: &str, task1: Tasks, task2: Tasks) -> Self {
        Tasks::Task(name.to_string(), vec![task1, task2])
    }

    pub fn name(&self) -> &str {
        match self {
            Tasks::Leaf(name, _) => name,
            Tasks::Task(name, _) => name,
        }
    }

    /// Best-effort task volume.
    ///
    /// - For a leaf, this is the provided count.
    /// - For a composite task, this is the sum of child volumes, unless any child
    ///   uses `UNKNOWN_VOLUME`, in which case this returns `UNKNOWN_VOLUME`.
    pub fn volume(&self) -> usize {
        match self {
            Tasks::Leaf(_, count) => *count,
            Tasks::Task(_, subtasks) => {
                let mut total = 0usize;
                for sub in subtasks {
                    let v = sub.volume();
                    if v == UNKNOWN_VOLUME {
                        return UNKNOWN_VOLUME;
                    }
                    total = total.saturating_add(v);
                }
                total
            }
        }
    }

    fn name_and_volume(self) -> (String, usize) {
        match self {
            Tasks::Leaf(name, count) => (name, count),
            Tasks::Task(name, subtasks) => {
                let mut total = 0usize;
                for sub in subtasks {
                    let v = sub.volume();
                    if v == UNKNOWN_VOLUME {
                        total = UNKNOWN_VOLUME;
                        break;
                    }
                    total = total.saturating_add(v);
                }
                (name, total)
            }
        }
    }
}
