//! Memory Facade
//!
//! Provides memory tracking and management operations, mirroring Java MemoryFacade.
//! Handles tracking memory usage per user and task, listing memory consumption,
//! and providing memory summaries.

use std::sync::Arc;

use crate::core::utils::progress::JobId;
use crate::mem::{
    MemoryReservationExceededException, MemoryTracker, UserEntityMemory, UserMemorySummary,
};
use crate::types::user::User;

/// Memory Facade for tracking and managing memory usage
pub struct MemoryFacade {
    memory_tracker: Arc<MemoryTracker>,
    user: User,
}

impl MemoryFacade {
    pub fn new(user: User, memory_tracker: Arc<MemoryTracker>) -> Self {
        Self {
            memory_tracker,
            user,
        }
    }

    /// Track memory usage for a task
    pub fn track(
        &self,
        task_name: &str,
        job_id: JobId,
        memory_estimate: u64,
    ) -> Result<(), MemoryReservationExceededException> {
        self.memory_tracker
            .try_to_track(self.user.username(), task_name, &job_id, memory_estimate)
    }

    /// List memory usage
    pub fn list(&self) -> Vec<UserEntityMemory> {
        self.memory_tracker.list(self.user.username())
    }

    /// Get memory summary
    pub fn memory_summary(&self) -> Vec<UserMemorySummary> {
        vec![self.memory_tracker.memory_summary(self.user.username())]
    }
}
