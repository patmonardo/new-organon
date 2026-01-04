//! No-op TaskStore implementation for disabled progress tracking.

use super::{JobId, Task, TaskStore, TaskStoreListener, UserTask};
use std::sync::Arc;

/// No-op TaskStore that stores nothing and returns empty results.
///
/// Used when progress tracking is disabled. All operations are no-ops,
/// and all queries return empty results.
#[derive(Debug, Clone)]
pub struct EmptyTaskStore;

impl EmptyTaskStore {
    /// Singleton instance.
    pub fn instance() -> Arc<dyn TaskStore> {
        Arc::new(Self)
    }
}

impl TaskStore for EmptyTaskStore {
    fn store(&self, _username: String, _job_id: JobId, _task: Task) {
        // No-op
    }

    fn remove(&self, _username: &str, _job_id: &JobId) {
        // No-op
    }

    fn query_all(&self) -> Vec<UserTask> {
        Vec::new()
    }

    fn query_by_job_id(&self, _job_id: &JobId) -> Vec<UserTask> {
        Vec::new()
    }

    fn query_by_username(&self, _username: &str) -> Vec<UserTask> {
        Vec::new()
    }

    fn query(&self, _username: &str, _job_id: &JobId) -> Option<UserTask> {
        None
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn task_count(&self) -> usize {
        0
    }

    fn add_listener(&self, _listener: Box<dyn TaskStoreListener>) {
        // No-op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_store_always_empty() {
        let store = EmptyTaskStore;

        assert!(store.is_empty());
        assert_eq!(store.task_count(), 0);
    }

    #[test]
    fn test_empty_store_queries_return_empty() {
        let store = EmptyTaskStore;

        assert!(store.query_all().is_empty());
        assert!(store.query_by_job_id(&JobId::new()).is_empty());
        assert!(store.query_by_username("user").is_empty());
        assert!(store.query("user", &JobId::new()).is_none());
    }

    #[test]
    fn test_empty_store_store_is_noop() {
        let store = EmptyTaskStore;
        let task = Task::new("test".to_string(), vec![]);

        store.store("user".to_string(), JobId::new(), task);

        assert!(store.is_empty());
    }
}
