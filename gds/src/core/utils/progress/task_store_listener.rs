//! Listener interface for TaskStore lifecycle events.

use super::{JobId, UserTask};

/// Listener for task store events.
///
/// Implementations can observe when tasks are added, removed, or when
/// the store is cleared.
pub trait TaskStoreListener: Send + Sync {
    /// Called when a task is added to the store.
    fn on_task_added(&self, user_task: &UserTask);

    /// Called when a task is removed from the store.
    fn on_task_removed(&self, username: &str, job_id: &JobId);

    /// Called when the store is cleared.
    fn on_store_cleared(&self);
}
