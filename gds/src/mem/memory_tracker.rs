//! Memory tracker
//!
//! Tracks memory reservations for tasks and graph stores, mirroring the Java `mem` package.

use crate::core::utils::progress::{JobId, TaskStoreListener, UserTask};

use super::{
    GraphStoreAddedEvent, GraphStoreMemoryContainer, GraphStoreRemovedEvent,
    MemoryReservationExceededException, TaskMemoryContainer, UserEntityMemory, UserMemorySummary,
};

use std::collections::HashSet;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct MemoryTracker {
    max_memory_in_bytes: u64,
    graph_store_memory_container: Arc<RwLock<GraphStoreMemoryContainer>>,
    task_memory_container: Arc<RwLock<TaskMemoryContainer>>,
}

impl MemoryTracker {
    pub fn new(max_memory_in_bytes: u64) -> Self {
        Self {
            max_memory_in_bytes,
            graph_store_memory_container: Arc::new(RwLock::new(GraphStoreMemoryContainer::new())),
            task_memory_container: Arc::new(RwLock::new(TaskMemoryContainer::new())),
        }
    }

    pub fn with_containers(
        max_memory_in_bytes: u64,
        graph_store_memory_container: GraphStoreMemoryContainer,
        task_memory_container: TaskMemoryContainer,
    ) -> Self {
        Self {
            max_memory_in_bytes,
            graph_store_memory_container: Arc::new(RwLock::new(graph_store_memory_container)),
            task_memory_container: Arc::new(RwLock::new(task_memory_container)),
        }
    }

    pub fn available_memory(&self) -> u64 {
        let graph_reserved = self
            .graph_store_memory_container
            .read()
            .expect("graph_store_memory_container lock poisoned")
            .graph_store_reserved_memory();
        let task_reserved = self
            .task_memory_container
            .read()
            .expect("task_memory_container lock poisoned")
            .task_reserved_memory();

        self.max_memory_in_bytes
            .saturating_sub(graph_reserved.saturating_add(task_reserved))
    }

    pub fn max_memory(&self) -> u64 {
        self.max_memory_in_bytes
    }

    pub fn graph_store_reserved_memory(&self) -> u64 {
        self.graph_store_memory_container
            .read()
            .expect("graph_store_memory_container lock poisoned")
            .graph_store_reserved_memory()
    }

    pub fn task_reserved_memory(&self) -> u64 {
        self.task_memory_container
            .read()
            .expect("task_memory_container lock poisoned")
            .task_reserved_memory()
    }

    pub fn on_graph_store_added(&self, event: &GraphStoreAddedEvent) {
        self.graph_store_memory_container
            .write()
            .expect("graph_store_memory_container lock poisoned")
            .add_graph_event(event);
    }

    pub fn on_graph_store_removed(&self, event: &GraphStoreRemovedEvent) {
        self.graph_store_memory_container
            .write()
            .expect("graph_store_memory_container lock poisoned")
            .remove_graph_event(event);
    }

    pub fn try_to_track(
        &self,
        username: &str,
        task_name: &str,
        job_id: &JobId,
        memory_in_bytes: u64,
    ) -> Result<(), MemoryReservationExceededException> {
        let available = self.available_memory();
        if memory_in_bytes > available {
            return Err(MemoryReservationExceededException::new(
                memory_in_bytes,
                available,
            ));
        }

        self.task_memory_container
            .write()
            .expect("task_memory_container lock poisoned")
            .reserve(username, task_name, job_id, memory_in_bytes);

        Ok(())
    }

    pub fn track(&self, username: &str, task_name: &str, job_id: &JobId, memory_in_bytes: u64) {
        self.try_to_track(username, task_name, job_id, memory_in_bytes)
            .expect("memory reservation exceeded")
    }

    pub fn list(&self, username: &str) -> Vec<UserEntityMemory> {
        let mut entities = Vec::new();

        entities.extend(
            self.graph_store_memory_container
                .read()
                .expect("graph_store_memory_container lock poisoned")
                .list_graphs(username),
        );
        entities.extend(
            self.task_memory_container
                .read()
                .expect("task_memory_container lock poisoned")
                .list_tasks(username),
        );

        entities
    }

    pub fn list_all(&self) -> Vec<UserEntityMemory> {
        let mut entities = Vec::new();

        entities.extend(
            self.graph_store_memory_container
                .read()
                .expect("graph_store_memory_container lock poisoned")
                .list_all_graphs(),
        );
        entities.extend(
            self.task_memory_container
                .read()
                .expect("task_memory_container lock poisoned")
                .list_all_tasks(),
        );

        entities
    }

    pub fn memory_summary(&self, username: &str) -> UserMemorySummary {
        let graphs = self
            .graph_store_memory_container
            .read()
            .expect("graph_store_memory_container lock poisoned")
            .memory_of_graphs(username);
        let tasks = self
            .task_memory_container
            .read()
            .expect("task_memory_container lock poisoned")
            .memory_of_tasks(username);

        UserMemorySummary::new(username.to_string(), graphs, tasks)
    }

    pub fn memory_summary_all(&self) -> Vec<UserMemorySummary> {
        let users = self.users();
        users
            .into_iter()
            .map(|user| self.memory_summary(&user))
            .collect()
    }

    pub fn users(&self) -> HashSet<String> {
        let users = self
            .graph_store_memory_container
            .read()
            .expect("graph_store_memory_container lock poisoned")
            .graph_users(None);

        self.task_memory_container
            .read()
            .expect("task_memory_container lock poisoned")
            .task_users(Some(users))
    }
}

impl TaskStoreListener for MemoryTracker {
    fn on_task_added(&self, _user_task: &UserTask) {
        // Memory is tracked explicitly via `track`/`try_to_track`.
    }

    fn on_task_removed(&self, username: &str, job_id: &JobId) {
        self.task_memory_container
            .write()
            .expect("task_memory_container lock poisoned")
            .remove_task_by_job(username, job_id);
    }

    fn on_store_cleared(&self) {
        *self
            .task_memory_container
            .write()
            .expect("task_memory_container lock poisoned") = TaskMemoryContainer::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_and_releases_task_memory_on_remove() {
        let tracker = MemoryTracker::new(10_000);
        let job_id = JobId::from("job-1");

        tracker
            .try_to_track("alice", "pagerank", &job_id, 2_000)
            .unwrap();
        assert_eq!(tracker.available_memory(), 8_000);

        tracker.on_task_removed("alice", &job_id);
        assert_eq!(tracker.available_memory(), 10_000);
    }

    #[test]
    fn errors_when_insufficient_memory() {
        let tracker = MemoryTracker::new(1_000);
        let job_id = JobId::from("job-1");

        let err = tracker
            .try_to_track("alice", "pagerank", &job_id, 2_000)
            .unwrap_err();

        assert_eq!(err.bytes_required(), 2_000);
        assert_eq!(err.bytes_available(), 1_000);
    }
}
