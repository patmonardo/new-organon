//! Core storage abstraction for task management.

use super::{JobId, Task, TaskStoreListener, UserTask};

/// Interface for storing and querying user tasks.
///
/// The TaskStore provides the persistence layer for tracking running tasks.
/// It supports queries by username, job ID, or both.
pub trait TaskStore: Send + Sync {
    /// Store a task for a user and job.
    fn store(&self, username: String, job_id: JobId, task: Task);

    /// Remove a task for a user and job.
    fn remove(&self, username: &str, job_id: &JobId);

    /// Query all tasks.
    fn query_all(&self) -> Vec<UserTask>;

    /// Query tasks by job ID.
    fn query_by_job_id(&self, job_id: &JobId) -> Vec<UserTask>;

    /// Query tasks by username.
    fn query_by_username(&self, username: &str) -> Vec<UserTask>;

    /// Query specific task by username and job ID.
    fn query(&self, username: &str, job_id: &JobId) -> Option<UserTask>;

    /// Check if store is empty.
    fn is_empty(&self) -> bool;

    /// Get total task count.
    fn task_count(&self) -> usize;

    /// Add a listener for task store events.
    fn add_listener(&self, listener: Box<dyn TaskStoreListener>);
}
